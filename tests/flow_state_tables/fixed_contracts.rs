use super::*;
#[test]
fn fixed_profiles_reject_compact_prefixes_every_extension_and_negative_or_impossible_row_counts() {
    for case in cases() {
        if case.name == "fixed-v0"
            || (1..=4).any(|version| case.name == format!("fixed-v{version}-n1"))
        {
            check_prefixes(&case);
        }
        let mut extra = case.raw.clone();
        extra.push(0);
        assert!(open(&extra, options()).is_err());
        check_bad_counts(&case);
    }
    for version in [i32::MIN, -1, 5, i32::MAX] {
        let raw = version.to_le_bytes();
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
    for &offset in &case.counts {
        for count in [i32::MIN, -1, i32::MAX] {
            let mut raw = case.raw.clone();
            raw[offset as usize..offset as usize + 4].copy_from_slice(&count.to_le_bytes());
            let mut selected = options();
            selected.maximum_rows = usize::MAX;
            assert!(open(&raw, selected).is_err());
        }
    }
}
#[test]
fn fixed_raw_byte_and_row_bounds_accept_exact_totals_and_reject_one_below() {
    for case in cases() {
        let state = check_case(&case);
        let exact = FlowFixedStateOptions {
            maximum_payload_bytes: case.raw.len(),
            maximum_rows: state.row_count(),
        };
        assert!(open(&case.raw, exact).is_ok());
        check_byte_bounds(&case.raw, exact);
        if state.row_count() > 0 {
            let mut selected = exact;
            selected.maximum_rows -= 1;
            assert!(open(&case.raw, selected).is_err());
        }
    }
}
fn check_byte_bounds(raw: &[u8], exact: FlowFixedStateOptions) {
    let mut selected = exact;
    selected.maximum_payload_bytes -= 1;
    assert!(open(raw, selected).is_err());
    for maximum in [0, 1, 3] {
        selected = exact;
        selected.maximum_payload_bytes = maximum;
        assert!(open(raw, selected).is_err());
    }
}
#[test]
fn version_zero_count_absence_and_later_stored_zero_counts_have_independent_exact_oracles() {
    let selected = FlowFixedStateOptions {
        maximum_payload_bytes: 8,
        maximum_rows: 0,
    };
    for version in 0..=4i32 {
        check_empty_version(version, selected);
    }
}
fn check_empty_version(version: i32, selected: FlowFixedStateOptions) {
    let mut raw = version.to_le_bytes().to_vec();
    if version > 0 {
        raw.extend_from_slice(&0i32.to_le_bytes());
    }
    let state = open(&raw, selected).unwrap();
    assert_eq!(
        state.declared_count(),
        if version == 0 { None } else { Some(0) }
    );
    assert_eq!(state.rows().len(), 0);
    assert_eq!(state.row(0).map(|row| row.ordinal()), None);
    assert_eq!(
        state.coverage().stored_values,
        if version == 0 { 1 } else { 2 }
    );
}
#[test]
fn literal_version_one_row_keeps_exact_member_bits_offsets_and_optional_absence() {
    let mut raw: Vec<_> = [1i32, 1].into_iter().flat_map(i32::to_le_bytes).collect();
    raw.extend(
        [0x7ff0000000000001u64, 0x8000000000000000]
            .into_iter()
            .flat_map(u64::to_le_bytes),
    );
    let state = open(&raw, options()).unwrap();
    let row = state.row(0).unwrap();
    assert_eq!(row.payload_interval(), (8, 24));
    assert_eq!(
        row.wide_member(FlowFixedStateWideMember::Member600),
        0x7ff0000000000001
    );
    assert_eq!(
        row.wide_member(FlowFixedStateWideMember::Discarded),
        0x8000000000000000
    );
    assert!(row.settings_wides().is_none());
    assert!(row.settings_words().is_none());
    assert_eq!(row.final_member(), None);
}
#[test]
fn fixed_edits_reject_absent_rows_settings_finals_wrong_slot_kinds_and_invalid_indexes() {
    for case in cases() {
        let state = check_case(&case);
        for row in state.rows() {
            check_invalid_row_edits(&state, row);
        }
        for ordinal in [state.row_count(), usize::MAX] {
            check_invalid_row(&state, ordinal);
        }
        assert_eq!(state.decoded_bytes(), case.raw);
    }
}
fn check_invalid_row_edits(state: &FlowFixedState, row: FlowFixedStateRow<'_>) {
    for kind in [
        FlowFixedSettingsKind::Integer,
        FlowFixedSettingsKind::Numeric,
    ] {
        check_invalid_settings(state, row, kind);
    }
    if row.final_member().is_none() {
        assert!(state.replace_final_member(row.ordinal(), 1).is_err());
    }
}
fn check_invalid_settings(
    state: &FlowFixedState,
    row: FlowFixedStateRow<'_>,
    kind: FlowFixedSettingsKind,
) {
    let (valid, invalid) = slot_pair(kind);
    let error = state
        .replace_setting(row.ordinal(), kind, 0, invalid)
        .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert_eq!(error.operation, "Flow legacy tables");
    assert!(error.entity.contains("139"));
    for index in [kind.slot_count(), usize::MAX] {
        assert!(
            state
                .replace_setting(row.ordinal(), kind, index, valid)
                .is_err()
        );
    }
    if row.settings_wides().is_none() {
        let error = state
            .replace_setting(row.ordinal(), kind, 0, valid)
            .unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Missing);
    }
}
fn slot_pair(kind: FlowFixedSettingsKind) -> (FlowFixedSlot, FlowFixedSlot) {
    match kind {
        FlowFixedSettingsKind::Integer => (FlowFixedSlot::Integer(0), FlowFixedSlot::Numeric(0)),
        FlowFixedSettingsKind::Numeric => (FlowFixedSlot::Numeric(0), FlowFixedSlot::Integer(0)),
    }
}
fn check_invalid_row(state: &FlowFixedState, ordinal: usize) {
    assert!(state.replace_wide_member(ordinal, MEMBERS[0], 0).is_err());
    assert!(
        state
            .replace_setting(
                ordinal,
                FlowFixedSettingsKind::Integer,
                0,
                FlowFixedSlot::Integer(0)
            )
            .is_err()
    );
    assert!(state.replace_final_member(ordinal, 0).is_err());
}
#[test]
fn fixed_occurrence_ids_reject_unrelated_records_and_foreign_or_stale_revisions() {
    let raw = [0; 4];
    let source = state_carrier::carrier(139, &raw);
    let id = source.latest(139).unwrap().unwrap().id;
    let wrong = source.latest(999).unwrap().unwrap().id;
    assert!(source.record_139_fixed_state(&wrong, options()).is_err());
    assert!(
        state_carrier::carrier(139, &raw)
            .record_139_fixed_state(&id, options())
            .is_err()
    );
    let changed = source
        .replace_payload(&id, &[1, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    assert!(
        changed
            .result()
            .record_139_fixed_state(&id, options())
            .is_err()
    );
}
