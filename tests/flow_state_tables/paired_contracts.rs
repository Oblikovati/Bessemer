use super::*;
#[test]
fn paired_profiles_reject_all_compact_prefixes_extensions_versions_and_impossible_reached_counts() {
    for case in cases() {
        check_prefixes(&case);
        let mut extra = case.raw.clone();
        extra.push(0);
        assert!(open(&extra, options()).is_err());
        check_bad_counts(&case);
    }
    check_bad_versions();
}
fn check_prefixes(case: &fixture::StateCase) {
    if case.raw.len() > 256 {
        return;
    }
    for end in 0..case.raw.len() {
        assert!(open(&case.raw[..end], options()).is_err());
    }
}
fn check_bad_counts(case: &fixture::StateCase) {
    for &offset in &case.counts {
        for count in [-1i32, i32::MAX] {
            let mut raw = case.raw.clone();
            raw[offset as usize..offset as usize + 4].copy_from_slice(&count.to_le_bytes());
            assert!(open(&raw, options()).is_err());
        }
    }
}
fn check_bad_versions() {
    for version in [i32::MIN, -1, 0, 2, i32::MAX] {
        let mut raw = version.to_le_bytes().to_vec();
        raw.resize(12, 0);
        assert_eq!(
            open(&raw, options()).unwrap_err().class,
            bessemer::FailureClass::Unsupported
        );
    }
}
#[test]
fn aggregate_index_wide_and_payload_limits_accept_exact_totals_and_reject_one_below() {
    for case in cases() {
        let state = check_case(&case);
        let coverage = state.coverage();
        let exact = FlowPairedStateOptions {
            maximum_payload_bytes: coverage.payload_bytes,
            maximum_index_words: coverage.index_words,
            maximum_wide_words: coverage.wide_words,
        };
        assert!(open(&case.raw, exact).is_ok());
        for axis in 0..3 {
            check_reduced_bound(&case.raw, exact, axis);
        }
    }
}
fn check_reduced_bound(raw: &[u8], mut selected: FlowPairedStateOptions, axis: usize) {
    let count = match axis {
        0 => &mut selected.maximum_payload_bytes,
        1 => &mut selected.maximum_index_words,
        _ => &mut selected.maximum_wide_words,
    };
    if *count == 0 {
        return;
    }
    *count -= 1;
    assert!(open(raw, selected).is_err());
}
#[test]
fn absent_and_present_empty_wides_have_independent_closed_form_framing_and_quotients() {
    let mut selected = options();
    selected.maximum_index_words = 0;
    selected.maximum_wide_words = 0;
    let raw: Vec<_> = [1i32, 0, 0]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    let state = open(&raw, selected).unwrap();
    for group in state.groups() {
        assert!(group.wides().is_none());
        assert_eq!(group.native_quotient(), None);
    }
    assert_eq!(state.coverage().present_wide_arrays, 0);
    check_present_empty_wides(selected);
}
fn check_present_empty_wides(mut selected: FlowPairedStateOptions) {
    selected.maximum_index_words = 1;
    let raw: Vec<_> = [1i32, 1, -1, 0, 0]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    let state = open(&raw, selected).unwrap();
    let first = state.group(0).unwrap();
    assert_eq!(first.indices().raw_word(0), Some(u32::MAX));
    assert_eq!(first.wides().unwrap().as_bytes(), &[]);
    assert_eq!(first.native_quotient(), Some(0));
    assert!(state.group(1).unwrap().wides().is_none());
    assert_eq!(state.coverage().present_wide_arrays, 1);
}
#[test]
fn minimum_payload_limits_and_unbounded_impossible_counts_fail_without_allocating_elements() {
    let raw = nondivisible();
    for maximum in [0, 1, 3] {
        let mut selected = options();
        selected.maximum_payload_bytes = maximum;
        assert!(open(&raw, selected).is_err());
    }
    let mut selected = options();
    selected.maximum_index_words = usize::MAX;
    selected.maximum_wide_words = usize::MAX;
    for offset in [4, 16, 44, 60] {
        let mut raw = raw.clone();
        raw[offset..offset + 4].copy_from_slice(&i32::MAX.to_le_bytes());
        assert!(open(&raw, selected).is_err());
    }
}
fn nondivisible() -> Vec<u8> {
    let mut raw: Vec<_> = [1i32, 2, i32::MIN, -1, 3]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    raw.extend(
        [0x8000000000000000u64, 0x7ff8000000000042, u64::MAX]
            .into_iter()
            .flat_map(u64::to_le_bytes),
    );
    raw.extend([3i32, 0, 1, 2, 2].into_iter().flat_map(i32::to_le_bytes));
    raw.extend(
        [1u64, 0x7ff0000000000001]
            .into_iter()
            .flat_map(u64::to_le_bytes),
    );
    raw
}
#[test]
fn nondivisible_counts_retain_floor_quotients_exact_offsets_and_raw_special_patterns() {
    let state = open(&nondivisible(), options()).unwrap();
    assert_eq!(state.decoded_bytes().len(), 80);
    let first = state.group(0).unwrap();
    let second = state.group(1).unwrap();
    assert_eq!(first.payload_interval(), (4, 44));
    assert_eq!(second.payload_interval(), (44, 80));
    assert_eq!(first.indices().payload_interval(), (8, 16));
    assert_eq!(second.indices().payload_interval(), (48, 60));
    assert_eq!(first.wides().unwrap().payload_interval(), (20, 44));
    assert_eq!(second.wides().unwrap().payload_interval(), (64, 80));
    assert_eq!(first.native_quotient(), Some(1));
    assert_eq!(second.native_quotient(), Some(0));
    assert_eq!(first.indices().raw_word(0), Some(0x80000000));
    assert_eq!(first.wides().unwrap().raw_bits(1), Some(0x7ff8000000000042));
    assert_eq!(
        second.wides().unwrap().raw_bits(1),
        Some(0x7ff0000000000001)
    );
}
#[test]
fn paired_edits_reject_absent_or_out_of_range_slots_and_keep_original_inputs() {
    for case in cases() {
        let state = check_case(&case);
        for group in state.groups() {
            check_invalid_group_edits(&state, group);
        }
        for group in [2, usize::MAX] {
            assert!(state.replace_index(group, 0, 1).is_err());
            assert!(state.replace_wide(group, 0, 1).is_err());
        }
        assert_eq!(state.decoded_bytes(), case.raw);
    }
}
fn check_invalid_group_edits(state: &FlowPairedState, group: FlowPairedStateGroup<'_>) {
    let ordinal = group.ordinal();
    for index in [group.indices().word_count(), usize::MAX] {
        assert!(state.replace_index(ordinal, index, 1).is_err());
    }
    let count = group.wides().map_or(0, |array| array.word_count());
    for index in [count, usize::MAX] {
        assert!(state.replace_wide(ordinal, index, 1).is_err());
    }
}
#[test]
fn paired_occurrence_ids_reject_unrelated_records_and_foreign_or_stale_revisions() {
    let raw = nondivisible();
    let source = state_carrier::carrier(417, &raw);
    let id = source.latest(417).unwrap().unwrap().id;
    let wrong = source.latest(999).unwrap().unwrap().id;
    assert!(source.record_417_paired_state(&wrong, options()).is_err());
    assert!(
        state_carrier::carrier(417, &raw)
            .record_417_paired_state(&id, options())
            .is_err()
    );
    let changed = source
        .replace_payload(&id, &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    assert!(
        changed
            .result()
            .record_417_paired_state(&id, options())
            .is_err()
    );
}
