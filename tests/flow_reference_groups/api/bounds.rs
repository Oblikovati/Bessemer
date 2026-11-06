use super::*;

#[test]
fn all_small_prefixes_extensions_and_wrong_mapping_branches_fail_without_partial_results() {
    for case in cases()
        .into_iter()
        .filter(|case| case.frame.decoded.len() <= 4096)
    {
        let mut context = options(&case.frame);
        context.compression = FlowLegacyTableTransport::Raw;
        for end in 0..case.frame.decoded.len() {
            assert_invalid(open(&case.frame.decoded[..end], context));
        }
        let mut extra = case.frame.decoded.clone();
        extra.push(0);
        assert_invalid(open(&extra, context));
        if case.frame.mapping && !case.groups[0].entries.is_empty() {
            context.first_group_mapping_enabled = false;
            assert_invalid(open(&case.frame.decoded, context));
        }
    }
}

fn set_limit(options: &mut FlowReferenceGroupOptions, axis: usize, value: usize) {
    match axis {
        0 => options.maximum_decoded_bytes = value,
        1 => options.maximum_entries = value,
        2 => options.maximum_reference_words = value,
        3 => options.maximum_integer_words = value,
        4 => options.maximum_mapping_words = value,
        5 => options.maximum_wide_words = value,
        _ => panic!("axis {axis}; expected six serialized bounds"),
    }
}

#[test]
fn each_independent_budget_accepts_exact_and_rejects_one_below_or_zero() {
    for case in cases() {
        let coverage = read_case(&case).coverage();
        let counts = [
            coverage.decoded_bytes,
            coverage.entries,
            coverage.reference_words,
            coverage.first_words + coverage.second_words,
            coverage.mapping_words,
            coverage.wide_words,
        ];
        for (axis, count) in counts.into_iter().enumerate() {
            check_limit(&case.frame, axis, count);
        }
    }
}

fn check_limit(frame: &ReferenceFrame, axis: usize, count: usize) {
    let mut context = options(frame);
    set_limit(&mut context, axis, count.max(1));
    assert!(open(&frame.stored, context).is_ok());
    if count > 0 {
        set_limit(&mut context, axis, count - 1);
        assert_invalid(open(&frame.stored, context));
    }
    set_limit(&mut context, axis, 0);
    assert_invalid(open(&frame.stored, context));
}

#[test]
fn signed_domains_delegated_versions_and_minimum_version_one_rows_are_explicit() {
    let case = cases()
        .into_iter()
        .find(|case| case.version == 1 && !case.frame.compressed)
        .unwrap();
    let context = options(&case.frame);
    for version in [-1_i32, 18, i32::MAX] {
        let error = open(&version.to_le_bytes(), context).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
    }
    check_large_counts(context);
    let mut raw = [0_u8; 33];
    raw[0] = 1;
    raw[8] = 1;
    assert_eq!(
        open(&raw, context).unwrap().group(0).unwrap().entry_count(),
        1
    );
}

#[test]
fn complete_zlib_wrapper_checksum_single_stream_and_occurrence_identity_are_required() {
    let case = cases().remove(0);
    let mut corrupt = case.frame.stored.clone();
    let last = corrupt.len() - 1;
    corrupt[last] ^= 1;
    assert_invalid(open(&corrupt, options(&case.frame)));
    let mut extra = case.frame.stored.clone();
    extra.extend_from_slice(&case.frame.stored);
    assert_invalid(open(&extra, options(&case.frame)));
    for end in 0..case.frame.stored.len() {
        assert_invalid(open(&case.frame.stored[..end], options(&case.frame)));
    }
    assert_invalid(open(&[0x1f, 0x8b, 8, 0], options(&case.frame)));
    let wrong = carrier(&case.frame.stored, 415, 4_000_000);
    let id = wrong.latest(415).unwrap().unwrap().id;
    assert_invalid(wrong.record_416_references(&id, options(&case.frame)));
    let valid = read_case(&case);
    assert_invalid(wrong.record_416_references(valid.record(), options(&case.frame)));
}

#[test]
fn header_values_do_not_control_group_count_and_empty_mapping_context_is_retained() {
    let case = cases().remove(0);
    for header in [i32::MIN, -999, 0, 1, 2, i32::MAX] {
        let raw: Vec<u8> = [14, header, 0, 0]
            .into_iter()
            .flat_map(i32::to_le_bytes)
            .collect();
        let mut context = options(&case.frame);
        context.compression = FlowLegacyTableTransport::Raw;
        for mapping in [false, true] {
            context.first_group_mapping_enabled = mapping;
            let view = open(&raw, context).unwrap();
            assert_eq!(view.header(FlowReferenceHeaderField::Stored), Some(header));
            assert_eq!(view.groups().len(), 2);
            assert_eq!(view.coverage().entries, 0);
            check_empty_mapping(&view);
            assert_eq!(view.options().first_group_mapping_enabled, mapping);
        }
    }
}

fn check_large_counts(mut context: FlowReferenceGroupOptions) {
    context.maximum_entries = usize::MAX;
    context.maximum_reference_words = usize::MAX;
    context.maximum_integer_words = usize::MAX;
    for count in [-1_i32, i32::MAX] {
        let raw: Vec<u8> = [14, -999, count, 0]
            .into_iter()
            .flat_map(i32::to_le_bytes)
            .collect();
        assert_invalid(open(&raw, context));
    }
}

fn check_empty_mapping(view: &FlowReferenceGroups) {
    assert!(
        view.groups()
            .all(|group| group.refinement_level().is_none())
    );
}
