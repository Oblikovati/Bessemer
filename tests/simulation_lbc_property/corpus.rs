use super::projections::{self, span, words};
use bessemer::simulation::*;
use std::io::Read;
#[allow(dead_code)]
#[path = "../common/simulation_images.rs"]
mod packet;
use packet::Cursor;

struct LbcCase<'a> {
    kind: u32,
    fixed: bool,
    version: i32,
    ordinal: usize,
    replacement: i32,
    accepted: bool,
    raw: &'a [u8],
}

fn owned() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(
        include_bytes!("../fixtures/simulation-lbc-property/owned.bin").as_slice(),
    )
    .read_to_end(&mut raw)
    .unwrap();
    raw
}

#[test]
fn complete_owned_reader_writer_and_field_edit_packets_match_independent_oracles() {
    let expected = owned();
    assert_eq!(project_packet(&expected), expected);
}

#[test]
#[ignore = "requires injected hash-bound native input and complete projection output paths"]
fn export_complete_native_candidate_outcomes_writer_and_field_edits() {
    let input = std::env::var_os("BESSEMER_LBC_PROPERTY_NATIVE_INPUT")
        .expect("expected injected native input path");
    let output = std::env::var_os("BESSEMER_LBC_PROPERTY_PROJECTION_OUTPUT")
        .expect("expected injected projection output path");
    let native = std::fs::read(input).unwrap();
    let mut projected = Vec::new();
    span(&mut projected, &project_packet(&owned()));
    span(&mut projected, &project_packet(&native));
    std::fs::write(output, projected).unwrap();
}

fn project_packet(expected: &[u8]) -> Vec<u8> {
    let mut cursor = Cursor(expected);
    assert_eq!(cursor.take(4), b"LBCP");
    let count = cursor.word();
    let mut output = b"LBCP".to_vec();
    words(&mut output, &[count]);
    for _ in 0..count {
        let case = read_case(&mut cursor);
        let expected_result = cursor.span();
        let actual = project_case(&case);
        assert_eq!(
            actual, expected_result,
            "kind {} writer {} version {} field {} accepted {}",
            case.kind, case.fixed, case.version, case.ordinal, case.accepted
        );
        append_case(&mut output, &case, &actual);
    }
    assert!(cursor.0.is_empty());
    output
}

fn bool_word(cursor: &mut Cursor<'_>) -> bool {
    match cursor.word() {
        0 => false,
        1 => true,
        other => panic!("boolean {other}; expected 0 or 1"),
    }
}

fn read_case<'a>(cursor: &mut Cursor<'a>) -> LbcCase<'a> {
    LbcCase {
        kind: cursor.word(),
        fixed: bool_word(cursor),
        version: cursor.word() as i32,
        ordinal: cursor.word() as usize,
        replacement: cursor.word() as i32,
        accepted: bool_word(cursor),
        raw: cursor.span(),
    }
}

fn append_case(output: &mut Vec<u8>, case: &LbcCase<'_>, actual: &[u8]) {
    words(
        output,
        &[
            case.kind,
            u32::from(case.fixed),
            case.version as u32,
            case.ordinal as u32,
            case.replacement as u32,
            u32::from(case.accepted),
        ],
    );
    span(output, case.raw);
    span(output, actual);
}

fn read_property(case: &LbcCase<'_>) -> Result<SimulationLbcProperty, bessemer::BessemerError> {
    let profile = if case.fixed {
        SimulationLbcPropertyProfile::InstalledFixedWriter
    } else {
        SimulationLbcPropertyProfile::VersionSelected(SimulationPersistVersion::new(case.version)?)
    };
    SimulationLbcProperty::from_bytes(case.raw, profile)
}

fn project_case(case: &LbcCase<'_>) -> Vec<u8> {
    if !case.accepted {
        return project_expected_failure(case);
    }
    let source = read_property(case).unwrap();
    let values: Vec<_> = source.fields().iter().map(|field| field.value).collect();
    assert_eq!(
        SimulationLbcProperty::from_values(&values, source.profile())
            .unwrap()
            .to_bytes(),
        case.raw
    );
    if case.kind == 0 {
        return projections::image(&source);
    }
    assert_eq!(case.kind, 1);
    let edit = source
        .edit_field(&source.field_id(case.ordinal).unwrap(), case.replacement)
        .unwrap();
    assert_eq!(source.to_bytes(), case.raw);
    projections::edit(&edit)
}

fn project_expected_failure(case: &LbcCase<'_>) -> Vec<u8> {
    assert_eq!(case.kind, 0);
    let error = read_property(case).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(!error.operation.is_empty() && !error.entity.is_empty());
    assert!(error.cause.contains("expected"));
    let offending = if case.version < 0 {
        case.version.to_string()
    } else {
        case.raw.len().to_string()
    };
    assert!(error.cause.contains(&offending));
    let mut output = Vec::new();
    words(&mut output, &[0, 1]);
    output
}
