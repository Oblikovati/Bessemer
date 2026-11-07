use super::*;
#[derive(Clone, Copy)]
pub(super) enum Field {
    Word(usize),
    State,
}
pub(super) fn fields(view: &FlowOwnerState) -> Vec<Field> {
    let mut result: Vec<_> = (0..view.words().word_count()).map(Field::Word).collect();
    if view.state_byte().is_some() {
        result.push(Field::State);
    }
    result
}
fn original(view: &FlowOwnerState, field: Field) -> (usize, Vec<u8>) {
    match field {
        Field::Word(index) => (
            view.words().payload_interval().0 + index * 4,
            view.words().raw_word(index).unwrap().to_le_bytes().to_vec(),
        ),
        Field::State => (28, vec![view.state_byte().unwrap()]),
    }
}
fn apply(view: &FlowOwnerState, field: Field, bytes: &[u8]) -> FlowSimpleTableEdit<FlowOwnerState> {
    match field {
        Field::Word(index) => view
            .replace_word(index, u32::from_le_bytes(bytes.try_into().unwrap()))
            .unwrap(),
        Field::State => view.replace_state_byte(bytes[0]).unwrap(),
    }
}
pub(super) fn check_edit(view: &FlowOwnerState, field: Field) -> Vec<u8> {
    let (offset, old) = original(view, field);
    let source = view.source().as_bytes().to_vec();
    let no_op = apply(view, field, &old);
    assert_eq!(no_op.result().source().as_bytes(), source);
    state_carrier::check_history(no_op.carrier_edit());
    let changed: Vec<_> = old.iter().map(|value| !value).collect();
    let edit = apply(view, field, &changed);
    check_expected(view, &edit, offset, &changed);
    let identifier = if view.kind() == FlowOwnerStateKind::Legacy122 {
        122
    } else {
        434
    };
    let fresh = open(identifier, edit.result().decoded_bytes(), view.options()).unwrap();
    let restored = apply(&fresh, field, &old);
    assert_eq!(restored.result().source().as_bytes(), source);
    state_carrier::check_history(restored.carrier_edit());
    assert_eq!(view.source().as_bytes(), source);
    edit.result().source().as_bytes().to_vec()
}
fn check_expected(
    view: &FlowOwnerState,
    edit: &FlowSimpleTableEdit<FlowOwnerState>,
    offset: usize,
    replacement: &[u8],
) {
    let mut expected = view.decoded_bytes().to_vec();
    expected[offset..offset + replacement.len()].copy_from_slice(replacement);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().decoded_bytes(), view.decoded_bytes());
    let identifier = if view.kind() == FlowOwnerStateKind::Legacy122 {
        122
    } else {
        434
    };
    assert_eq!(
        projection(edit.result()),
        expected_updates(identifier, &expected)
    );
    assert_eq!(edit.result().version(), view.version());
    assert_eq!(edit.result().kind(), view.kind());
    assert_eq!(edit.result().options(), view.options());
    state_carrier::check_history(edit.carrier_edit());
}
#[test]
fn every_stored_word_and_state_byte_has_literal_patch_noop_history_projection_and_fresh_reversal() {
    for row in cases().0 {
        let view = open(row.identifier, &row.raw, options()).unwrap();
        for field in fields(&view) {
            check_edit(&view, field);
        }
    }
}
#[test]
fn signed_modern_edits_match_raw_word_edits_and_legacy_gate_transitions_recompute_updates() {
    let row = cases()
        .0
        .into_iter()
        .find(|row| row.identifier == 434)
        .unwrap();
    let modern = check_case(&row);
    for index in 0..6 {
        let value = i32::MIN + index as i32;
        let signed = modern.replace_modern_member(index, value).unwrap();
        let raw = modern.replace_word(index, value as u32).unwrap();
        assert_eq!(
            signed.result().source().as_bytes(),
            raw.result().source().as_bytes()
        );
        assert_eq!(signed.result().modern_member(index).unwrap(), value);
    }
    check_gate_transitions();
}
fn check_gate_transitions() {
    let row = cases()
        .0
        .into_iter()
        .find(|row| row.identifier == 122)
        .unwrap();
    let source = check_case(&row);
    let disabled = source.replace_word(0, u32::MAX).unwrap();
    assert!(!disabled.result().updates_enabled());
    assert!(disabled.result().updates().is_none());
    assert_eq!(disabled.result().coverage().projected_updates, 0);
    for gate in [0, 1, 0x7fffffff, 0x80000000, 0xfffffffe] {
        let enabled = disabled.result().replace_word(0, gate).unwrap();
        assert!(enabled.result().updates_enabled());
        assert_eq!(projection(enabled.result()), projection(&source));
    }
    assert!(source.updates_enabled());
}
