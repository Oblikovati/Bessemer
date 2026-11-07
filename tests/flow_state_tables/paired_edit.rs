use super::*;
#[test]
fn paired_edits_match_every_source_value_offset_preserve_noops_history_and_fresh_reversal() {
    for case in cases() {
        let state = check_case(&case);
        for group in state.groups() {
            check_group_edits(&state, &case, group);
        }
        assert_eq!(state.decoded_bytes(), case.raw);
        assert_eq!(
            state.source().as_bytes(),
            state_carrier::carrier(417, &case.raw).as_bytes()
        );
    }
}
fn check_group_edits(
    state: &FlowPairedState,
    case: &fixture::StateCase,
    group: FlowPairedStateGroup<'_>,
) {
    for index in 0..group.indices().word_count() {
        check_index_edit(state, case, group, index);
    }
    if let Some(array) = group.wides() {
        for index in 0..array.word_count() {
            check_wide_edit(state, case, group, index);
        }
    }
}
fn check_index_edit(
    state: &FlowPairedState,
    case: &fixture::StateCase,
    group: FlowPairedStateGroup<'_>,
    index: usize,
) {
    let old = group.indices().raw_word(index).unwrap();
    check_noop(
        state,
        &state.replace_index(group.ordinal(), index, old).unwrap(),
    );
    let edit = state.replace_index(group.ordinal(), index, !old).unwrap();
    let offset =
        expected_field(case, group.ordinal(), "index_patterns").offset as usize + index * 4;
    check_edit(state, &edit, offset, &(!old).to_le_bytes());
    let fresh = open(edit.result().decoded_bytes(), state.options()).unwrap();
    let reverse = fresh.replace_index(group.ordinal(), index, old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        state.source().as_bytes()
    );
}
fn check_wide_edit(
    state: &FlowPairedState,
    case: &fixture::StateCase,
    group: FlowPairedStateGroup<'_>,
    index: usize,
) {
    let old = group.wides().unwrap().raw_bits(index).unwrap();
    check_noop(
        state,
        &state.replace_wide(group.ordinal(), index, old).unwrap(),
    );
    let edit = state.replace_wide(group.ordinal(), index, !old).unwrap();
    let offset = expected_field(case, group.ordinal(), "wide_patterns").offset as usize + index * 8;
    check_edit(state, &edit, offset, &(!old).to_le_bytes());
    let fresh = open(edit.result().decoded_bytes(), state.options()).unwrap();
    let reverse = fresh.replace_wide(group.ordinal(), index, old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        state.source().as_bytes()
    );
}
fn check_noop(state: &FlowPairedState, edit: &FlowPairedStateEdit) {
    assert_eq!(edit.result().record(), state.record());
    assert_eq!(edit.result().source().as_bytes(), state.source().as_bytes());
    state_carrier::check_history(edit.carrier_edit());
}
fn check_edit(state: &FlowPairedState, edit: &FlowPairedStateEdit, offset: usize, bytes: &[u8]) {
    let mut expected = state.decoded_bytes().to_vec();
    expected[offset..offset + bytes.len()].copy_from_slice(bytes);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(
        edit.result().source().as_bytes(),
        state_carrier::carrier(417, &expected).as_bytes()
    );
    assert_eq!(edit.source().source().as_bytes(), state.source().as_bytes());
    assert_eq!(edit.result().coverage(), state.coverage());
    assert_eq!(edit.result().options(), state.options());
    state_carrier::check_history(edit.carrier_edit());
}
