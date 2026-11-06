use super::*;
#[test]
fn every_small_body_prefix_unsupported_version_and_negative_or_impossible_count_is_rejected() {
    for case in fixture::cases() {
        if case.footer.end <= 1024 {
            for end in 0..case.footer.end as usize {
                assert!(open(&case.raw[..end], options()).is_err());
            }
        }
    }
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.name == "flag-0-count-0")
        .unwrap();
    for offset in std::iter::once(0).chain(case.curves.iter().map(|curve| curve.start as usize)) {
        for version in 1..=255 {
            let mut raw = case.raw.clone();
            raw[offset] = version;
            assert!(open(&raw, options()).is_err());
        }
    }
    check_bad_counts(&case);
}
fn check_bad_counts(case: &BodyCase) {
    let last = case.curves.last().unwrap().end as usize;
    for offset in std::iter::once(18)
        .chain(case.curves.iter().map(|curve| curve.start as usize + 9))
        .chain([last, last + 4])
    {
        for count in [-1i32, i32::MAX] {
            let mut raw = case.raw.clone();
            raw[offset..offset + 4].copy_from_slice(&count.to_le_bytes());
            assert!(open(&raw, options()).is_err());
        }
    }
}
#[test]
fn exact_body_word_and_name_budgets_are_independent_and_tail_bytes_do_not_spend_body_budget() {
    for case in fixture::cases() {
        let words = fields(&case).iter().map(|(_, _, words)| words.len()).sum();
        let exact = FlowMediumBodyOptions {
            maximum_body_bytes: case.footer.end as usize,
            maximum_wide_words: words,
            maximum_name_units: (case.text.units as usize).max(1),
        };
        assert!(open(&case.raw, exact).is_ok());
        for axis in 0..3 {
            check_below_budget(&case, exact, axis);
        }
    }
}
fn check_below_budget(case: &BodyCase, exact: FlowMediumBodyOptions, axis: usize) {
    let mut below = exact;
    match axis {
        0 => below.maximum_body_bytes -= 1,
        1 => below.maximum_wide_words -= 1,
        _ => below.maximum_name_units = case.text.units.saturating_sub(1) as usize,
    }
    assert!(open(&case.raw, below).is_err());
    let mut zero = exact;
    match axis {
        0 => zero.maximum_body_bytes = 0,
        1 => zero.maximum_wide_words = 0,
        _ => zero.maximum_name_units = 0,
    }
    assert!(open(&case.raw, zero).is_err());
}
#[test]
fn wrong_kind_absent_arrays_invalid_indices_presence_changes_and_terminal_edits_are_rejected() {
    let case = fixture::cases()
        .into_iter()
        .find(|case| case.name == "flag-0-count-0")
        .unwrap();
    let body = open(&case.raw, options()).unwrap();
    assert!(
        backend(&case.raw, 2)
            .first_kind_one_body(options())
            .is_err()
    );
    check_invalid_edits(&body);
    check_invalid_selections(&body);
    let native = fixture::cases().remove(0);
    let body = open(&native.raw, options()).unwrap();
    assert!(body.replace_curve_flag(0, 0).is_err());
    assert!(
        body.replace_name_unit(body.name().stored_units() - 1, 1)
            .is_err()
    );
    assert_eq!(body.body_bytes(), &native.raw[..native.footer.end as usize]);
}
#[test]
fn all_public_body_types_are_send_sync_and_eight_workers_match_complete_reads_and_edits_after_input_mutation()
 {
    assert_send_sync::<FlowMediumBodyOptions>();
    assert_send_sync::<FlowMediumCurveArray>();
    assert_send_sync::<FlowMediumWideField>();
    assert_send_sync::<FlowMediumBodyCoverage>();
    assert_send_sync::<FlowMediumBodyDiagnostic>();
    assert_send_sync::<FlowMediumKindOne>();
    assert_send_sync::<FlowMediumCurve<'static>>();
    assert_send_sync::<FlowMediumBodyEdit>();
    let cases = fixture::cases();
    let expected = transcript(&cases);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(|| transcript(&cases))).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
fn assert_send_sync<T: Send + Sync>() {}
fn transcript(cases: &[BodyCase]) -> Vec<u8> {
    let mut output = Vec::new();
    for case in cases {
        let mut raw = case.raw.clone();
        let body = open(&raw, options()).unwrap();
        raw.fill(255);
        output.extend_from_slice(body.body_bytes());
        output.extend_from_slice(body.opaque_tail());
        output.extend_from_slice(body.name().native_text_bytes());
        let edited = body.replace_state_byte(body.state_byte() ^ 255).unwrap();
        output.extend_from_slice(
            edited
                .result()
                .backend_prefix()
                .metadata_prefix()
                .source()
                .as_bytes(),
        );
        assert_eq!(body.body_bytes(), &case.raw[..case.footer.end as usize]);
    }
    output
}

fn check_invalid_edits(body: &FlowMediumKindOne) {
    assert!(body.replace_curve_flag(0, 1).is_err());
    assert!(body.replace_curve_flag(7, 0).is_err());
    assert!(body.replace_name_unit(0, 0).is_err());
    assert!(
        body.replace_wide_word(FlowMediumWideField::FinalScalars, 4, 0)
            .is_err()
    );
}
fn check_invalid_selections(body: &FlowMediumKindOne) {
    assert!(
        body.wide_words(FlowMediumWideField::SharedArray { array: 6 })
            .is_err()
    );
    assert!(
        body.wide_words(FlowMediumWideField::CurveScalar { curve: 7 })
            .is_err()
    );
    assert!(
        body.wide_words(FlowMediumWideField::CurveArray {
            curve: 0,
            array: FlowMediumCurveArray::ExtraFirst
        })
        .is_err()
    );
}
