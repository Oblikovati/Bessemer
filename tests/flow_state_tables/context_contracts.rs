use super::*;
#[test]
fn context_profiles_reject_all_compact_prefixes_extensions_versions_and_negative_object_counts() {
    for case in cases()
        .into_iter()
        .filter(|case| case.name.ends_with("-n1-c3") || case.name.ends_with("-n0-c0"))
    {
        for end in 0..case.raw.len() {
            assert!(open(&case.raw[..end], options(case.context)).is_err());
        }
        let mut extra = case.raw.clone();
        extra.push(0);
        assert!(open(&extra, options(case.context)).is_err());
        check_bad_words(&case);
    }
}
fn check_bad_words(case: &fixture::StateCase) {
    for version in [i32::MIN, -1, 0, 1, 6, i32::MAX] {
        let mut raw = case.raw.clone();
        raw[..4].copy_from_slice(&version.to_le_bytes());
        assert_eq!(
            open(&raw, options(case.context)).unwrap_err().class,
            bessemer::FailureClass::Unsupported
        );
    }
    for count in [i32::MIN, -1] {
        let mut raw = case.raw.clone();
        raw[4..8].copy_from_slice(&count.to_le_bytes());
        assert!(open(&raw, options(case.context)).is_err());
    }
}
#[test]
fn index_context_is_required_only_when_applied_and_never_inferred_from_object_count() {
    for case in cases() {
        let state = check_case(&case);
        if state.context_is_applied() {
            assert_eq!(
                open(&case.raw, options(None)).unwrap_err().class,
                bessemer::FailureClass::Missing
            );
            let count = state.indices().word_count() as i32;
            assert!(open(&case.raw, options(Some(count + 1))).is_err());
            if count > 0 {
                assert!(open(&case.raw, options(Some(count - 1))).is_err());
            }
        } else {
            for context in [None, Some(0), Some(i32::MAX)] {
                let mut selected = options(context);
                selected.maximum_index_words = 0;
                assert_eq!(open(&case.raw, selected).unwrap().indices().word_count(), 0);
            }
        }
    }
}
#[test]
fn payload_and_actual_word_bounds_accept_exact_counts_and_reject_one_below() {
    for case in cases() {
        let state = check_case(&case);
        let mut selected = state.options();
        selected.maximum_payload_bytes = case.raw.len();
        selected.maximum_index_words = state.indices().word_count();
        assert!(open(&case.raw, selected).is_ok());
        selected.maximum_payload_bytes -= 1;
        assert!(open(&case.raw, selected).is_err());
        selected.maximum_payload_bytes += 1;
        if selected.maximum_index_words > 0 {
            selected.maximum_index_words -= 1;
            assert!(open(&case.raw, selected).is_err());
        }
        for maximum in [0, 1, 3] {
            selected.maximum_payload_bytes = maximum;
            assert!(open(&case.raw, selected).is_err());
        }
    }
}
#[test]
fn typed_context_boundary_and_independent_large_object_count_oracle_preserve_unsigned_words() {
    assert!(FlowSimpleLegacyCount::try_from(i32::MAX as u32).is_ok());
    assert!(FlowSimpleLegacyCount::try_from(i32::MAX as u32 + 1).is_err());
    assert!(FlowSimpleLegacyCount::try_from(u32::MAX).is_err());
    let raw: Vec<_> = [3i32, i32::MAX, i32::MIN, -1, 7, i32::MIN]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    let state = open(&raw, options(Some(3))).unwrap();
    assert_eq!(state.object_count(), i32::MAX as u32);
    assert_eq!(
        (0..3)
            .map(|index| state.indices().raw_word(index).unwrap())
            .collect::<Vec<_>>(),
        [0x80000000, 0xffffffff, 7]
    );
    assert_eq!(state.tail(), Some(i32::MIN));
    check_large_empty_context();
}
fn check_large_empty_context() {
    let empty: Vec<_> = [4i32, i32::MAX]
        .into_iter()
        .flat_map(i32::to_le_bytes)
        .collect();
    let mut selected = options(Some(0));
    selected.maximum_index_words = 0;
    let state = open(&empty, selected).unwrap();
    assert!(state.context_is_applied());
    assert_eq!(state.indices().word_count(), 0);
    let mut impossible = options(Some(i32::MAX));
    impossible.maximum_index_words = usize::MAX;
    assert!(open(&empty, impossible).is_err());
}
#[test]
fn context_edits_reject_absent_or_out_of_range_slots_and_foreign_stale_record_ids() {
    for case in cases() {
        let state = check_case(&case);
        for index in [state.indices().word_count(), usize::MAX] {
            assert!(state.replace_index(index, 7).is_err());
        }
        if state.tail().is_none() {
            assert!(state.replace_tail(7).is_err());
        }
        assert_eq!(state.decoded_bytes(), case.raw);
    }
    check_context_occurrence_ids();
}
fn check_context_occurrence_ids() {
    let raw = [4u8, 0, 0, 0, 0, 0, 0, 0];
    let source = state_carrier::carrier(138, &raw);
    let id = source.latest(138).unwrap().unwrap().id;
    let wrong = source.latest(999).unwrap().unwrap().id;
    assert!(
        source
            .record_138_context_state(&wrong, options(None))
            .is_err()
    );
    check_stale_context_id(&source, &id, &raw);
}
fn check_stale_context_id(source: &FlowCarrier, id: &FlowRecordId, raw: &[u8]) {
    let changed = source
        .replace_payload(id, &[5, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    assert!(
        changed
            .result()
            .record_138_context_state(id, options(None))
            .is_err()
    );
    assert!(
        state_carrier::carrier(138, raw)
            .record_138_context_state(id, options(None))
            .is_err()
    );
}
