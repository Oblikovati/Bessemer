use super::*;
#[test]
fn context_edits_match_every_source_index_and_tail_preserve_noops_history_and_fresh_reversal() {
    for case in cases() {
        let state = check_case(&case);
        for index in 0..state.indices().word_count() {
            let old = state.indices().raw_word(index).unwrap();
            let noop = state.replace_index(index, old).unwrap();
            check_noop(&state, &noop);
            let edit = state.replace_index(index, !old).unwrap();
            let offset = expected_field(&case, "[\"index_patterns\"]").offset as usize + index * 4;
            check_edit(&state, &edit, offset, &(!old).to_le_bytes());
            let fresh = open(edit.result().decoded_bytes(), state.options()).unwrap();
            let reverse = fresh.replace_index(index, old).unwrap();
            assert_eq!(
                reverse.result().source().as_bytes(),
                state.source().as_bytes()
            );
        }
        check_tail_edit(&state, &case);
    }
}
fn check_tail_edit(state: &FlowContextState, case: &fixture::StateCase) {
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
fn check_noop(state: &FlowContextState, edit: &FlowContextStateEdit) {
    assert_eq!(edit.result().record(), state.record());
    assert_eq!(edit.result().source().as_bytes(), state.source().as_bytes());
    check_history(edit.carrier_edit());
}
fn check_edit(state: &FlowContextState, edit: &FlowContextStateEdit, offset: usize, bytes: &[u8]) {
    let mut expected = state.decoded_bytes().to_vec();
    expected[offset..offset + 4].copy_from_slice(bytes);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(
        edit.result().source().as_bytes(),
        state_carrier::carrier(138, &expected).as_bytes()
    );
    assert_eq!(edit.source().source().as_bytes(), state.source().as_bytes());
    assert_eq!(edit.result().coverage(), state.coverage());
    assert_eq!(edit.result().options(), state.options());
    assert_eq!(edit.result().object_count(), state.object_count());
    check_history(edit.carrier_edit());
}
pub(super) use crate::state_carrier::check_history;
