use super::*;
#[test]
fn tagged_edits_match_source_offsets_preserve_noops_history_layout_and_fresh_raw_reversals() {
    for case in cases() {
        let state = check_case(&case);
        for ordinal in selected_rows(&state) {
            check_row_edits(&state, &case, state.row(ordinal).unwrap());
        }
        check_tail_edit(&state, &case);
        assert_eq!(state.decoded_bytes(), case.raw);
        assert_eq!(
            state.source().as_bytes(),
            state_carrier::carrier(129, &case.raw).as_bytes()
        );
    }
}
fn check_row_edits(
    state: &FlowTaggedState,
    case: &fixture::StateCase,
    row: FlowTaggedStateRow<'_>,
) {
    check_tag_edit(state, case, row);
    let Some(array) = row.values() else {
        return;
    };
    for index in selected_elements(array.word_count(), case.name.starts_with("native-")) {
        check_wide_edit(state, case, row, index);
    }
}
fn check_tag_edit(state: &FlowTaggedState, case: &fixture::StateCase, row: FlowTaggedStateRow<'_>) {
    let old = row.tag();
    check_noop(state, &state.replace_tag(row.ordinal(), old).unwrap());
    let next = if old == 0 { 0 } else { old % 255 + 1 };
    let edit = state.replace_tag(row.ordinal(), next).unwrap();
    let offset = expected_field(case, "[\"tags_raw\"]").offset as usize + row.ordinal();
    check_edit(state, &edit, offset, &[next]);
    let fresh = open(edit.result().decoded_bytes(), state.options()).unwrap();
    let reverse = fresh.replace_tag(row.ordinal(), old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        state.source().as_bytes()
    );
}
fn check_wide_edit(
    state: &FlowTaggedState,
    case: &fixture::StateCase,
    row: FlowTaggedStateRow<'_>,
    index: usize,
) {
    let old = row.values().unwrap().raw_bits(index).unwrap();
    check_noop(
        state,
        &state.replace_wide(row.ordinal(), index, old).unwrap(),
    );
    let edit = state.replace_wide(row.ordinal(), index, !old).unwrap();
    let offset = row_field(case, row.ordinal(), "value_patterns").offset as usize + index * 8;
    check_edit(state, &edit, offset, &(!old).to_le_bytes());
    let fresh = open(edit.result().decoded_bytes(), state.options()).unwrap();
    let reverse = fresh.replace_wide(row.ordinal(), index, old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        state.source().as_bytes()
    );
}
fn check_tail_edit(state: &FlowTaggedState, case: &fixture::StateCase) {
    let Some(old) = state.tail() else {
        assert!(state.replace_tail(7).is_err());
        return;
    };
    check_noop(state, &state.replace_tail(old).unwrap());
    let edit = state.replace_tail(!old).unwrap();
    let offset = expected_field(case, "[\"tail_i32\"]").offset as usize;
    check_edit(state, &edit, offset, &(!old).to_le_bytes());
    let fresh = open(edit.result().decoded_bytes(), state.options()).unwrap();
    let reverse = fresh.replace_tail(old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        state.source().as_bytes()
    );
}
fn check_noop(state: &FlowTaggedState, edit: &FlowTaggedStateEdit) {
    assert_eq!(edit.result().record(), state.record());
    assert_eq!(edit.result().source().as_bytes(), state.source().as_bytes());
    state_carrier::check_history(edit.carrier_edit());
}
fn check_edit(state: &FlowTaggedState, edit: &FlowTaggedStateEdit, offset: usize, bytes: &[u8]) {
    let mut expected = state.decoded_bytes().to_vec();
    expected[offset..offset + bytes.len()].copy_from_slice(bytes);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(
        edit.result().source().as_bytes(),
        state_carrier::carrier(129, &expected).as_bytes()
    );
    assert_eq!(edit.source().source().as_bytes(), state.source().as_bytes());
    assert_eq!(edit.result().coverage(), state.coverage());
    assert_eq!(edit.result().options(), state.options());
    check_layout(state, edit.result());
    state_carrier::check_history(edit.carrier_edit());
}
fn check_layout(before: &FlowTaggedState, after: &FlowTaggedState) {
    for (left, right) in before.rows().zip(after.rows()) {
        assert_eq!(left.payload_interval(), right.payload_interval());
        assert_eq!(left.tag_offset(), right.tag_offset());
        assert_eq!(left.declared_width(), right.declared_width());
    }
}
