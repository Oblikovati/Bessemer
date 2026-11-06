use super::*;
#[test]
fn fixed_edits_match_every_source_member_setting_and_final_offset_preserve_noops_history_and_fresh_reversal()
 {
    for case in cases() {
        let state = check_case(&case);
        for row in state.rows() {
            check_row_edits(&state, &case, row);
        }
        assert_eq!(state.decoded_bytes(), case.raw);
        assert_eq!(
            state.source().as_bytes(),
            state_carrier::carrier(139, &case.raw).as_bytes()
        );
    }
}
fn check_row_edits(state: &FlowFixedState, case: &fixture::StateCase, row: FlowFixedStateRow<'_>) {
    for member in MEMBERS {
        check_member_edit(state, case, row, member);
    }
    for kind in [
        FlowFixedSettingsKind::Numeric,
        FlowFixedSettingsKind::Integer,
    ] {
        check_settings_edits(state, case, row, kind);
    }
    if let Some(old) = row.final_member() {
        check_final_edit(state, case, row, old);
    }
}
fn check_member_edit(
    state: &FlowFixedState,
    case: &fixture::StateCase,
    row: FlowFixedStateRow<'_>,
    member: FlowFixedStateWideMember,
) {
    let old = row.wide_member(member);
    check_noop(
        state,
        &state
            .replace_wide_member(row.ordinal(), member, old)
            .unwrap(),
    );
    let edit = state
        .replace_wide_member(row.ordinal(), member, !old)
        .unwrap();
    let offset = expected_field(case, row.ordinal(), member_role(member)).offset as usize;
    check_edit(state, &edit, offset, &(!old).to_le_bytes());
    let fresh = open(edit.result().decoded_bytes(), state.options()).unwrap();
    let reverse = fresh
        .replace_wide_member(row.ordinal(), member, old)
        .unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        state.source().as_bytes()
    );
}
fn check_settings_edits(
    state: &FlowFixedState,
    case: &fixture::StateCase,
    row: FlowFixedStateRow<'_>,
    kind: FlowFixedSettingsKind,
) {
    if row.settings_wides().is_none() {
        return;
    }
    for index in 0..kind.slot_count() {
        check_setting_edit(state, case, row, kind, index);
    }
}
fn check_setting_edit(
    state: &FlowFixedState,
    case: &fixture::StateCase,
    row: FlowFixedStateRow<'_>,
    kind: FlowFixedSettingsKind,
    index: usize,
) {
    let (role, old, next) = setting_values(row, kind, index);
    check_setting_noop(state, row.ordinal(), kind, index, old);
    let edit = state
        .replace_setting(row.ordinal(), kind, index, next)
        .unwrap();
    let bytes = encoded(next);
    let offset = expected_field(case, row.ordinal(), role).offset as usize + index * bytes.len();
    check_edit(state, &edit, offset, &bytes);
    let fresh = open(edit.result().decoded_bytes(), state.options()).unwrap();
    let reverse = fresh
        .replace_setting(row.ordinal(), kind, index, old)
        .unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        state.source().as_bytes()
    );
}
fn check_setting_noop(
    state: &FlowFixedState,
    row: usize,
    kind: FlowFixedSettingsKind,
    index: usize,
    old: FlowFixedSlot,
) {
    let noop = state.replace_setting(row, kind, index, old).unwrap();
    check_noop(state, &noop);
}
pub(super) fn setting_values(
    row: FlowFixedStateRow<'_>,
    kind: FlowFixedSettingsKind,
    index: usize,
) -> (&'static str, FlowFixedSlot, FlowFixedSlot) {
    match kind {
        FlowFixedSettingsKind::Numeric => {
            let old = row.settings_wides().unwrap().raw_bits(index).unwrap();
            (
                "settings_wide_patterns",
                FlowFixedSlot::Numeric(old),
                FlowFixedSlot::Numeric(!old),
            )
        }
        FlowFixedSettingsKind::Integer => {
            let old = row.settings_words().unwrap().raw_word(index).unwrap();
            (
                "settings_i32_patterns",
                FlowFixedSlot::Integer(old),
                FlowFixedSlot::Integer(!old),
            )
        }
    }
}
fn encoded(value: FlowFixedSlot) -> Vec<u8> {
    match value {
        FlowFixedSlot::Integer(bits) => bits.to_le_bytes().to_vec(),
        FlowFixedSlot::Numeric(bits) => bits.to_le_bytes().to_vec(),
    }
}
fn check_final_edit(
    state: &FlowFixedState,
    case: &fixture::StateCase,
    row: FlowFixedStateRow<'_>,
    old: i32,
) {
    check_noop(
        state,
        &state.replace_final_member(row.ordinal(), old).unwrap(),
    );
    let edit = state.replace_final_member(row.ordinal(), !old).unwrap();
    let offset = expected_field(case, row.ordinal(), "final_member_e0_i32").offset as usize;
    check_edit(state, &edit, offset, &(!old).to_le_bytes());
    let fresh = open(edit.result().decoded_bytes(), state.options()).unwrap();
    let reverse = fresh.replace_final_member(row.ordinal(), old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        state.source().as_bytes()
    );
}
fn check_noop(state: &FlowFixedState, edit: &FlowFixedStateEdit) {
    assert_eq!(edit.result().record(), state.record());
    assert_eq!(edit.result().source().as_bytes(), state.source().as_bytes());
    state_carrier::check_history(edit.carrier_edit());
}
fn check_edit(state: &FlowFixedState, edit: &FlowFixedStateEdit, offset: usize, bytes: &[u8]) {
    let mut expected = state.decoded_bytes().to_vec();
    expected[offset..offset + bytes.len()].copy_from_slice(bytes);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(
        edit.result().source().as_bytes(),
        state_carrier::carrier(139, &expected).as_bytes()
    );
    assert_eq!(edit.source().source().as_bytes(), state.source().as_bytes());
    assert_eq!(edit.result().options(), state.options());
    assert_eq!(edit.result().coverage(), state.coverage());
    state_carrier::check_history(edit.carrier_edit());
}
