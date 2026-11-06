use super::*;
#[test]
fn exact_body_wide_word_and_name_caps_are_independent_including_unused_zero_count_collections() {
    for case in kind_fixture::cases() {
        let counts = counts(&case);
        let exact = limited(counts.map(|n| n.max(1)));
        assert!(open(&case.raw, exact).is_ok());
        for axis in 0..4 {
            let mut zero = counts.map(|n| n.max(1));
            zero[axis] = 0;
            assert!(open(&case.raw, limited(zero)).is_err());
            if counts[axis] == 0 {
                continue;
            }
            let mut below = counts.map(|n| n.max(1));
            below[axis] -= 1;
            assert!(open(&case.raw, limited(below)).is_err());
        }
    }
}
fn counts(case: &KindTwoCase) -> [usize; 4] {
    let wide = expected::fields(case)
        .iter()
        .map(|(_, _, words)| words.len())
        .sum();
    let words = word_count(case);
    [
        case.footer.end as usize,
        wide,
        case.prefix.text.units as usize,
        words,
    ]
}
fn limited(counts: [usize; 4]) -> FlowMediumKindTwoOptions {
    FlowMediumKindTwoOptions {
        body: FlowMediumBodyOptions {
            maximum_body_bytes: counts[0],
            maximum_wide_words: counts[1],
            maximum_name_units: counts[2],
        },
        maximum_word_patterns: counts[3],
    }
}
#[test]
fn every_small_prefix_and_every_reached_unsupported_version_or_unknown_model_tag_is_rejected() {
    for case in kind_fixture::cases() {
        if case.footer.end <= 512 {
            for end in 0..case.footer.end as usize {
                assert!(open(&case.raw[..end], options()).is_err());
            }
        }
    }
    for kind in 0..6 {
        let case = compact(kind);
        check_versions(&case);
        check_bad_counts(&case);
    }
    check_unknown_tags(&compact(0));
}
fn compact(kind: i32) -> KindTwoCase {
    kind_fixture::cases()
        .into_iter()
        .find(|case| case.name == format!("tag-{kind}-count-0-flag-0"))
        .unwrap()
}
fn check_versions(case: &KindTwoCase) {
    let model_curves = match &case.model.payload {
        ModelPayload::Curves(value) => value.curves.as_slice(),
        _ => &[],
    };
    let offsets = std::iter::once(0)
        .chain(std::iter::once(case.model.start as usize))
        .chain(
            case.prefix
                .curves
                .iter()
                .chain(model_curves)
                .map(|curve| curve.start as usize),
        );
    for offset in offsets {
        for version in 1..=255 {
            let mut raw = case.raw.clone();
            raw[offset] = version;
            assert!(open(&raw, options()).is_err());
        }
    }
}
fn check_bad_counts(case: &KindTwoCase) {
    let offsets = count_offsets(case);
    for offset in offsets {
        for count in [-1i32, i32::MAX] {
            let mut raw = case.raw.clone();
            raw[offset..offset + 4].copy_from_slice(&count.to_le_bytes());
            assert!(open(&raw, options()).is_err());
        }
    }
}
#[test]
fn wrong_kind_absent_roles_invalid_indices_flag_branch_and_terminator_changes_leave_source_unchanged()
 {
    for kind in 0..6 {
        let case = compact(kind);
        let body = open(&case.raw, options()).unwrap();
        assert!(
            backend(&case.raw, 1)
                .first_kind_two_body(options())
                .is_err()
        );
        check_indices(&body);
        check_absence(&body, kind);
        assert!(
            body.replace_curve_flag(FlowMediumCurveScope::Body, 0, 1)
                .is_err()
        );
        assert!(
            body.replace_name_unit(body.name().stored_units() - 1, 1)
                .is_err()
        );
        assert_eq!(body.body_bytes(), &case.raw[..case.footer.end as usize]);
    }
}
fn check_indices(body: &FlowMediumKindTwo) {
    assert!(body.replace_name_unit(usize::MAX, 0).is_err());
    assert!(body.replace_model_state(usize::MAX, 0).is_err());
    assert!(
        body.replace_curve_flag(FlowMediumCurveScope::Body, 7, 0)
            .is_err()
    );
    assert!(
        body.replace_curve_flag(FlowMediumCurveScope::Model, usize::MAX, 0)
            .is_err()
    );
    assert!(
        body.replace_wide_word(FlowMediumKindTwoWideField::BodyScalar, 1, 0)
            .is_err()
    );
    assert!(
        body.wide_words(FlowMediumKindTwoWideField::SharedArray { array: 6 })
            .is_err()
    );
}
fn check_absence(body: &FlowMediumKindTwo, kind: i32) {
    assert!(
        body.wide_words(FlowMediumKindTwoWideField::ModelFirstArray { array: 2 })
            .is_err()
    );
    if kind != 5 {
        assert!(body.replace_model_word(0, 0).is_err());
        assert!(body.replace_model_member_8c(0).is_err());
    }
    if kind != 4 {
        assert!(
            body.wide_words(FlowMediumKindTwoWideField::ModelLastScalar)
                .is_err()
        );
    }
    if [0, 2, 3].contains(&kind) {
        assert!(body.replace_model_state(0, 0).is_err());
    }
    if kind == 5 {
        assert!(body.replace_model_word(0, 0).is_err());
    }
}
#[test]
fn all_public_kind_two_types_are_send_sync_and_eight_workers_match_complete_body_edits_after_input_mutation()
 {
    assert_send_sync::<FlowMediumKindTwoOptions>();
    assert_send_sync::<FlowMediumModelKind>();
    assert_send_sync::<FlowMediumCurveScope>();
    assert_send_sync::<FlowMediumKindTwoWideField>();
    assert_send_sync::<FlowMediumKindTwoCoverage>();
    assert_send_sync::<FlowMediumKindTwoDiagnostic>();
    assert_send_sync::<FlowMediumKindTwo>();
    assert_send_sync::<FlowMediumKindTwoEdit>();
    let cases = kind_fixture::cases();
    let expected = transcript(&cases);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(|| transcript(&cases))).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
fn assert_send_sync<T: Send + Sync>() {}
fn transcript(cases: &[KindTwoCase]) -> Vec<u8> {
    let mut result = Vec::new();
    for case in cases {
        let mut raw = case.raw.clone();
        let body = open(&raw, options()).unwrap();
        raw.fill(255);
        result.extend_from_slice(body.body_bytes());
        result.extend_from_slice(body.opaque_tail());
        result.extend_from_slice(body.model_state_bytes());
        let edit = body.replace_state_byte(body.state_byte() ^ 255).unwrap();
        result.extend_from_slice(
            edit.result()
                .backend_prefix()
                .metadata_prefix()
                .source()
                .as_bytes(),
        );
        assert_eq!(body.body_bytes(), &case.raw[..case.footer.end as usize]);
    }
    result
}

fn check_unknown_tags(case: &KindTwoCase) {
    let offset = case.model.start as usize - 4;
    for tag in [i32::MIN, -1, 6, 255, i32::MAX] {
        assert!(FlowMediumModelKind::try_from(tag).is_err());
        let mut raw = case.raw.clone();
        raw[offset..offset + 4].copy_from_slice(&tag.to_le_bytes());
        assert_eq!(
            open(&raw, options()).unwrap_err().class,
            bessemer::FailureClass::Unsupported
        );
    }
}

fn count_offsets(case: &KindTwoCase) -> Vec<usize> {
    let last = case.prefix.curves.last().unwrap().end as usize;
    let mut offsets = vec![18, last, last + 4];
    offsets.extend(
        case.prefix
            .curves
            .iter()
            .map(|curve| curve.start as usize + 9),
    );
    match &case.model.payload {
        ModelPayload::Curves(value) => {
            offsets.extend(value.curves.iter().map(|curve| curve.start as usize + 9))
        }
        ModelPayload::Table(_) => {
            offsets.extend([1, 5, 9].map(|offset| case.model.start as usize + offset))
        }
        _ => (),
    }
    offsets
}

#[test]
fn first_kind_two_entry_requires_the_table_key_and_keeps_it_distinct_from_the_body_key() {
    let case = compact(0);
    let body = open(&case.raw, options()).unwrap();
    let prefix = body.backend_prefix().metadata_prefix();
    check_missing_table_keys(prefix);
    let mut raw = prefix.payload_bytes().to_vec();
    raw[19..35].fill(0xa5);
    let edit = prefix
        .source()
        .replace_payload(prefix.record(), &raw)
        .unwrap();
    let after = reopen(edit.result(), prefix.options()).unwrap();
    assert_eq!(after.body_bytes(), body.body_bytes());
    assert_ne!(after.base_key_bytes(), &[0xa5; 16]);
    assert_eq!(after.payload_interval(), body.payload_interval());
}
fn reopen(
    source: &FlowCarrier,
    metadata: FlowMetadataPrefixOptions,
) -> bessemer::Result<FlowMediumKindTwo> {
    let id = source.latest(424)?.unwrap().id;
    source
        .record_424_metadata_prefix(&id, metadata)?
        .medium_backend_prefix()?
        .first_kind_two_body(options())
}

fn check_missing_table_keys(prefix: &FlowMetadataPrefix) {
    for length in 0..16 {
        let raw = &prefix.payload_bytes()[..19 + length];
        let edit = prefix
            .source()
            .replace_payload(prefix.record(), raw)
            .unwrap();
        assert!(reopen(edit.result(), prefix.options()).is_err());
    }
}
