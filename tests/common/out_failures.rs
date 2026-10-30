use super::reference::LIMITS;
use bessemer::simulation::*;

fn wrapped(body: &[u8]) -> Vec<u8> {
    let count = if body.is_empty() {
        0
    } else {
        (body.len() - 1) / 0xa00000 + 1
    };
    let mut raw = (count as u32).to_le_bytes().to_vec();
    raw.extend_from_slice(body);
    raw
}

fn report(body: &str) -> SimulationOutArchive {
    SimulationOutArchive::from_bytes(&wrapped(body.as_bytes()), LIMITS).unwrap()
}

#[test]
fn decimal_patterns_match_independent_exact_coefficients_and_exponents() {
    for (token, negative, digits, exponent) in [
        ("-0.000e+2", true, vec![0], -1),
        ("+000.00100D+03", false, vec![1, 0, 0], -2),
        ("1.", false, vec![1], 0),
        (".50", false, vec![5, 0], -2),
        ("-123D-2", true, vec![1, 2, 3], -2),
        ("00010", false, vec![1, 0], 0),
    ] {
        let parsed = report(&format!(" Value (X)={token}"));
        let value = &parsed.fields()[0].value;
        assert_eq!(value.is_negative(), negative);
        assert_eq!(value.digits(), digits);
        assert_eq!(value.exponent(), exponent);
    }
}

#[test]
fn malformed_field_like_lines_are_preserved_and_duplicate_symbols_stay_distinct() {
    for body in [
        " Bad (lower)=1",
        "Bad(A)=1e",
        "Bad(A)=.",
        "Bad(A)=1 2",
        "Bad(A B)=2",
    ] {
        let parsed = report(body);
        assert!(parsed.fields().is_empty());
        assert_eq!(parsed.to_bytes(), wrapped(body.as_bytes()));
    }
    let parsed = report("First(A)=1\r\n EQ. -000, zero\r\nOther\r\nSecond(A)=2\r\nEQ.+001, one");
    assert_eq!(parsed.fields().len(), 2);
    assert_ne!(parsed.fields()[0].id, parsed.fields()[1].id);
    assert_eq!(parsed.fields()[0].legends[0].value.as_ref(), "0");
    assert_eq!(parsed.fields()[1].legends[0].value.as_ref(), "1");
}

#[test]
fn encoding_endings_transfer_and_resource_limits_fail_before_publication() {
    for body in [
        b"\xff".as_slice(),
        b"\x01",
        b"\x0b",
        b"\x0c",
        b"x\n",
        b"x\r",
        b"\r\r\n",
        b"",
    ] {
        assert!(SimulationOutArchive::from_bytes(&wrapped(body), LIMITS).is_err());
    }
    for count in [0_u32, 2, u32::MAX] {
        let mut raw = wrapped(b"x");
        raw[..4].copy_from_slice(&count.to_le_bytes());
        assert!(SimulationOutArchive::from_bytes(&raw, LIMITS).is_err());
    }
    let raw = wrapped(b"A(X)=1\r\nB(X)=2");
    for limits in [
        SimulationOutLimits {
            maximum_bytes: 5,
            ..LIMITS
        },
        SimulationOutLimits {
            maximum_lines: 1,
            ..LIMITS
        },
        SimulationOutLimits {
            maximum_fields: 1,
            ..LIMITS
        },
    ] {
        assert!(SimulationOutArchive::from_bytes(&raw, limits).is_err());
    }
}

#[test]
fn complete_line_construction_rejects_hidden_endings_and_phantom_final_lines() {
    let input = |content: &[u8], crlf| SimulationOutLineInput {
        content: content.into(),
        crlf,
    };
    for lines in [
        vec![],
        vec![input(b"", false)],
        vec![input(b"x\n", false)],
        vec![input(b"x", false), input(b"y", true)],
        vec![input(b"x", true), input(b"", false)],
    ] {
        assert!(SimulationOutArchive::from_lines(&lines, LIMITS).is_err());
    }
    let complete = SimulationOutArchive::from_lines(&[input(b"", true)], LIMITS).unwrap();
    assert_eq!(complete.to_bytes(), wrapped(b"\r\n"));
}

#[test]
fn invalid_and_foreign_edits_leave_sources_unchanged() {
    let original = report("Value(A)=1\r\n");
    let foreign = report("Value(A)=1\r\n");
    let id = &original.fields()[0].id;
    for token in [
        "",
        ".",
        "1e",
        " 1",
        "1 ",
        "1\r\nX(B)=2",
        "NaN",
        "1e9999999999999999999999999",
    ] {
        assert!(
            original.edit_numeric_lexeme(id, token).is_err(),
            "{token:?}"
        );
    }
    assert!(
        original
            .edit_numeric_lexeme(&foreign.fields()[0].id, "2")
            .is_err()
    );
    let tight = SimulationOutLimits {
        maximum_bytes: original.to_bytes().len(),
        ..LIMITS
    };
    let bounded = SimulationOutArchive::from_bytes(&original.to_bytes(), tight).unwrap();
    assert!(
        bounded
            .edit_numeric_lexeme(&bounded.fields()[0].id, "123456789")
            .is_err()
    );
    assert_eq!(original.to_bytes(), wrapped(b"Value(A)=1\r\n"));
}

#[test]
fn report_types_are_send_sync_and_concurrent_edits_are_isolated() {
    fn check<T: Send + Sync>() {}
    check::<SimulationOutArchive>();
    check::<SimulationOutLimits>();
    check::<SimulationOutRevision>();
    check::<SimulationOutFieldId>();
    check::<SimulationOutDecimal>();
    check::<SimulationOutLineInput>();
    check::<SimulationOutLine>();
    check::<SimulationOutLegend>();
    check::<SimulationOutField>();
    check::<SimulationOutCheckMode>();
    check::<SimulationOutEditFidelity>();
    check::<SimulationOutCoverage>();
    check::<SimulationOutDiagnostic>();
    check::<SimulationOutFieldHistory>();
    check::<SimulationOutEdit>();
    check::<SimulationOutTotalStatus>();
    check::<SimulationOutTotalComparison>();
    check::<SimulationOutPairedTotals>();
    check::<SimulationOutStudy>();
    check::<SimulationOutStudyEdit>();
    let mut raw = wrapped(b"Value(A)=1\r\n");
    let original = SimulationOutArchive::from_bytes(&raw, LIMITS).unwrap();
    raw.fill(0);
    std::thread::scope(|scope| {
        let tasks: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| edit_revert(&original)))
            .collect();
        for task in tasks {
            task.join().unwrap();
        }
    });
}

fn edit_revert(original: &SimulationOutArchive) {
    for _ in 0..32 {
        let edit = original
            .edit_numeric_lexeme(&original.fields()[0].id, "-2.500D+1")
            .unwrap();
        let reverted = edit
            .archive
            .edit_numeric_lexeme(&edit.archive.fields()[0].id, "1")
            .unwrap();
        assert_eq!(reverted.archive.to_bytes(), wrapped(b"Value(A)=1\r\n"));
        assert_eq!(original.to_bytes(), wrapped(b"Value(A)=1\r\n"));
    }
}
