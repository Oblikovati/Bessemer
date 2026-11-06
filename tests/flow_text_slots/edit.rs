use super::*;
pub(super) fn indices(slot: &TextSlot, native: bool) -> Vec<usize> {
    let count = slot.units as usize;
    if count <= 64 || native {
        return (0..count).collect();
    }
    vec![0, count / 2, count - 1]
}
pub(super) fn check_header(view: &FlowTextSlots) -> Vec<u8> {
    let source = view.source().as_bytes().to_vec();
    let no_op = view.replace_header(view.header()).unwrap();
    assert_eq!(no_op.result().source().as_bytes(), source);
    state_carrier::check_history(no_op.carrier_edit());
    let edit = view.replace_header(!view.header()).unwrap();
    check_expected(view, &edit, 0, &(!view.header()).to_le_bytes());
    let fresh = open(edit.result().decoded_bytes(), view.options()).unwrap();
    let reverse = fresh.replace_header(view.header()).unwrap();
    assert_eq!(reverse.result().source().as_bytes(), source);
    state_carrier::check_history(reverse.carrier_edit());
    edit.result().source().as_bytes().to_vec()
}
pub(super) fn check_unit(view: &FlowTextSlots, ordinal: usize, index: usize) -> Vec<u8> {
    let slot = view.slot(ordinal).unwrap();
    let original = slot.unit(index).unwrap();
    let no_op = view.replace_unit(ordinal, index, original).unwrap();
    assert_eq!(no_op.result().source().as_bytes(), view.source().as_bytes());
    state_carrier::check_history(no_op.carrier_edit());
    if index + 1 == slot.stored_units() {
        return check_terminal(view, ordinal, index);
    }
    let edit = view.replace_unit(ordinal, index, !original).unwrap();
    check_expected(
        view,
        &edit,
        slot.interval().0 + 4 + index * 2,
        &(!original).to_le_bytes(),
    );
    check_reverse_unit(view, edit.result(), ordinal, index, original);
    edit.result().source().as_bytes().to_vec()
}
fn check_reverse_unit(
    view: &FlowTextSlots,
    result: &FlowTextSlots,
    ordinal: usize,
    index: usize,
    original: u16,
) {
    let fresh = open(result.decoded_bytes(), view.options()).unwrap();
    let reverse = fresh.replace_unit(ordinal, index, original).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        view.source().as_bytes()
    );
    state_carrier::check_history(reverse.carrier_edit());
}

fn check_terminal(view: &FlowTextSlots, ordinal: usize, index: usize) -> Vec<u8> {
    let source = view.source().as_bytes().to_vec();
    assert!(view.replace_unit(ordinal, index, 1).is_err());
    assert_eq!(view.source().as_bytes(), source);
    source
}
fn check_expected(
    view: &FlowTextSlots,
    edit: &FlowSimpleTableEdit<FlowTextSlots>,
    offset: usize,
    replacement: &[u8],
) {
    let mut expected = view.decoded_bytes().to_vec();
    expected[offset..offset + replacement.len()].copy_from_slice(replacement);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().decoded_bytes(), view.decoded_bytes());
    assert_eq!(edit.result().options(), view.options());
    state_carrier::check_history(edit.carrier_edit());
    for ordinal in 0..50 {
        assert_eq!(
            edit.result().slot(ordinal).unwrap().native_prefix_bytes(),
            native_prefix(edit.result().slot(ordinal).unwrap().raw_bytes())
        );
    }
}
#[test]
fn all_headers_and_selected_units_preserve_counts_have_literal_patches_noops_history_and_fresh_reversals()
 {
    for row in cases() {
        let view = open(&row.raw, options()).unwrap();
        check_header(&view);
        for slot in &row.slots {
            for index in indices(slot, row.native_header.is_some()) {
                check_unit(&view, slot.ordinal as usize, index);
            }
        }
    }
}
#[test]
fn creating_and_removing_an_interior_nul_recomputes_prefix_and_previous_state_decisions() {
    let raw = first_slot_payload(0, &[65, 66, 0]);
    let view = open(&raw, options()).unwrap();
    let previous =
        FlowTextSlotState::from_units(std::array::from_fn(|_| vec![0xd800, 0, 66]), 150).unwrap();
    let hidden = view.replace_unit(0, 0, 0).unwrap();
    let retained = hidden.result().project(&previous, 150).unwrap();
    assert_eq!(
        retained.result().units(0).unwrap(),
        previous.units(0).unwrap()
    );
    let visible = hidden.result().replace_unit(0, 0, 0xdc00).unwrap();
    let applied = visible.result().project(&previous, 150).unwrap();
    assert_eq!(applied.result().units(0).unwrap(), [0xdc00, 66]);
    assert_eq!(view.slot(0).unwrap().native_prefix_bytes(), [65, 0, 66, 0]);
}
