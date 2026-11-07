use super::*;
#[test]
fn tagged_framing_rejects_compact_prefixes_every_extension_and_negative_or_impossible_counts() {
    for case in cases() {
        if case.raw.len() <= 256 {
            check_prefixes(&case);
        }
        let mut extra = case.raw.clone();
        extra.push(0);
        assert!(open(&extra, options()).is_err());
        check_bad_counts(&case);
    }
    for version in [i32::MIN, -1, 0, 8, 10, i32::MAX] {
        let mut raw = version.to_le_bytes().to_vec();
        raw.resize(8, 0);
        assert_eq!(
            open(&raw, options()).unwrap_err().class,
            bessemer::FailureClass::Unsupported
        );
    }
}
fn check_prefixes(case: &fixture::StateCase) {
    for end in 0..case.raw.len() {
        assert!(open(&case.raw[..end], options()).is_err());
    }
}
fn check_bad_counts(case: &fixture::StateCase) {
    let mut selected = options();
    selected.maximum_rows = usize::MAX;
    selected.maximum_wide_words = usize::MAX;
    for &offset in &case.counts {
        for count in [-1i32, i32::MAX] {
            let mut raw = case.raw.clone();
            raw[offset as usize..offset as usize + 4].copy_from_slice(&count.to_le_bytes());
            assert!(open(&raw, selected).is_err());
        }
    }
}
#[test]
fn all_tagged_resource_axes_accept_exact_counts_and_reject_one_below() {
    for case in cases() {
        let state = check_case(&case);
        let coverage = state.coverage();
        let exact = FlowTaggedStateOptions {
            maximum_payload_bytes: case.raw.len(),
            maximum_rows: coverage.rows,
            maximum_wide_words: coverage.wide_words,
        };
        assert!(open(&case.raw, exact).is_ok());
        for axis in 0..3 {
            check_reduced_bound(&case.raw, exact, axis);
        }
    }
}
fn check_reduced_bound(raw: &[u8], mut selected: FlowTaggedStateOptions, axis: usize) {
    let limit = match axis {
        0 => &mut selected.maximum_payload_bytes,
        1 => &mut selected.maximum_rows,
        _ => &mut selected.maximum_wide_words,
    };
    if *limit == 0 {
        return;
    }
    *limit -= 1;
    assert!(open(raw, selected).is_err());
}
fn oracle() -> Vec<u8> {
    let mut raw: Vec<_> = [9i32, 3].into_iter().flat_map(i32::to_le_bytes).collect();
    raw.extend_from_slice(&[0, 255, 1]);
    raw.extend([0i32, 1].into_iter().flat_map(i32::to_le_bytes));
    raw.extend_from_slice(&0x7ff0000000000001u64.to_le_bytes());
    raw.extend_from_slice(&i32::MIN.to_le_bytes());
    raw
}
#[test]
fn split_tag_block_zero_bodies_present_empty_wides_and_signed_tail_have_independent_oracles() {
    let raw = oracle();
    assert_eq!(raw.len(), 31);
    let state = open(&raw, options()).unwrap();
    assert_eq!(state.tags_bytes(), &[0, 255, 1]);
    assert_eq!(state.tail(), Some(i32::MIN));
    check_oracle_rows(&state);
    assert!(state.row(0).unwrap().values().is_none());
    assert_eq!(
        state.row(1).unwrap().values().unwrap().payload_interval(),
        (15, 15)
    );
    assert_eq!(
        state.row(2).unwrap().values().unwrap().raw_bits(0),
        Some(0x7ff0000000000001)
    );
    let changed = state.replace_tag(2, 254).unwrap();
    assert_eq!(changed.result().decoded_bytes()[10], 254);
}
fn check_oracle_rows(state: &FlowTaggedState) {
    for (ordinal, interval, width) in [
        (0, (11, 11), None),
        (1, (11, 15), Some(0)),
        (2, (15, 27), Some(1)),
    ] {
        let row = state.row(ordinal).unwrap();
        assert_eq!(row.payload_interval(), interval);
        assert_eq!(row.declared_width(), width);
        assert_eq!(row.tag_offset(), 8 + ordinal);
    }
}
#[test]
fn empty_records_and_all_absent_rows_keep_distinct_tail_presence_under_zero_wide_budgets() {
    let mut selected = options();
    selected.maximum_rows = 0;
    selected.maximum_wide_words = 0;
    let mut raw: Vec<_> = [9i32, 0].into_iter().flat_map(i32::to_le_bytes).collect();
    let state = open(&raw, selected).unwrap();
    assert_eq!(state.tags_bytes(), &[]);
    assert_eq!(state.tail(), None);
    assert_eq!(state.coverage().stored_values, 2);
    raw[4..8].copy_from_slice(&1i32.to_le_bytes());
    raw.push(0);
    raw.extend_from_slice(&(-1i32).to_le_bytes());
    selected.maximum_rows = 1;
    let state = open(&raw, selected).unwrap();
    assert_eq!(state.tail(), Some(-1));
    assert_eq!(state.row(0).unwrap().payload_interval(), (9, 9));
    assert_eq!(state.coverage().present_rows, 0);
    for maximum in [0, 1, 3] {
        selected.maximum_payload_bytes = maximum;
        assert!(open(&raw, selected).is_err());
    }
}
#[test]
fn all_stored_tag_bytes_keep_raw_bits_and_tag_edits_reject_presence_transitions() {
    let mut seen = BTreeSet::new();
    for case in cases() {
        let state = check_case(&case);
        for row in state.rows() {
            seen.insert(row.tag());
            check_invalid_row_edits(&state, row);
        }
        assert_eq!(state.decoded_bytes(), case.raw);
        for ordinal in [state.row_count(), usize::MAX] {
            check_invalid_row(&state, ordinal);
        }
    }
    assert_eq!(seen, (0..=255).collect());
}
fn check_invalid_row_edits(state: &FlowTaggedState, row: FlowTaggedStateRow<'_>) {
    let next = if row.tag() == 0 { 1 } else { 0 };
    assert_eq!(
        state.replace_tag(row.ordinal(), next).unwrap_err().class,
        bessemer::FailureClass::Invalid
    );
    let count = row.declared_width().unwrap_or(0);
    for index in [count, usize::MAX] {
        assert!(state.replace_wide(row.ordinal(), index, 1).is_err());
    }
    if row.tag() == 0 {
        assert_eq!(
            state.replace_wide(row.ordinal(), 0, 1).unwrap_err().class,
            bessemer::FailureClass::Missing
        );
    }
}
fn check_invalid_row(state: &FlowTaggedState, ordinal: usize) {
    assert!(state.replace_tag(ordinal, 1).is_err());
    assert!(state.replace_wide(ordinal, 0, 1).is_err());
    if state.row_count() == 0 {
        assert!(state.replace_tail(1).is_err());
    }
}
#[test]
fn tagged_occurrences_reject_unrelated_records_and_foreign_or_stale_revisions() {
    let raw = oracle();
    let source = state_carrier::carrier(129, &raw);
    let id = source.latest(129).unwrap().unwrap().id;
    let wrong = source.latest(999).unwrap().unwrap().id;
    assert!(source.record_129_tagged_state(&wrong, options()).is_err());
    assert!(
        state_carrier::carrier(129, &raw)
            .record_129_tagged_state(&id, options())
            .is_err()
    );
    let changed = source
        .replace_payload(&id, &[9, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    assert!(
        changed
            .result()
            .record_129_tagged_state(&id, options())
            .is_err()
    );
}
