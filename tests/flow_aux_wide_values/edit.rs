use super::*;
#[derive(Clone, Copy)]
pub(super) enum Selection {
    Legacy,
    Pattern(usize),
}
pub(super) fn selections(row: &AuxWideCase) -> Vec<Selection> {
    let mut result = Vec::new();
    if row.legacy.is_some() {
        result.push(Selection::Legacy);
    }
    let count = row.count as usize;
    if count <= 257 || row.native_count.is_some() {
        result.extend((0..count).map(Selection::Pattern));
        return result;
    }
    result.extend([0, count / 2, count - 1].map(Selection::Pattern));
    result
}
fn original_bytes(view: &FlowAuxWideValues, selection: Selection) -> (usize, Vec<u8>) {
    match selection {
        Selection::Legacy => (4, view.discarded_legacy().unwrap().to_le_bytes().to_vec()),
        Selection::Pattern(index) => {
            let start = view.values().payload_interval().0 + index * 8;
            (
                start,
                view.values()
                    .raw_bits(index)
                    .unwrap()
                    .to_le_bytes()
                    .to_vec(),
            )
        }
    }
}
fn apply(
    view: &FlowAuxWideValues,
    selection: Selection,
    raw: &[u8],
) -> FlowSimpleTableEdit<FlowAuxWideValues> {
    match selection {
        Selection::Legacy => view
            .replace_discarded_legacy(i32::from_le_bytes(raw.try_into().unwrap()))
            .unwrap(),
        Selection::Pattern(index) => view
            .replace_pattern(index, u64::from_le_bytes(raw.try_into().unwrap()))
            .unwrap(),
    }
}
pub(super) fn check_edit(view: &FlowAuxWideValues, selection: Selection) -> Vec<u8> {
    let (offset, original) = original_bytes(view, selection);
    let no_op = apply(view, selection, &original);
    assert_eq!(no_op.result().source().as_bytes(), view.source().as_bytes());
    state_carrier::check_history(no_op.carrier_edit());
    let replacement: Vec<_> = original.iter().map(|byte| byte ^ 0xa5).collect();
    let edit = apply(view, selection, &replacement);
    check_expected(view, &edit, offset, &replacement);
    check_reverse(view, &edit, selection, &original);
    edit.result().source().as_bytes().to_vec()
}
fn check_expected(
    view: &FlowAuxWideValues,
    edit: &FlowSimpleTableEdit<FlowAuxWideValues>,
    offset: usize,
    replacement: &[u8],
) {
    let mut expected = view.decoded_bytes().to_vec();
    expected[offset..offset + replacement.len()].copy_from_slice(replacement);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().decoded_bytes(), view.decoded_bytes());
    assert_eq!(view.source().as_bytes(), edit.source().source().as_bytes());
    assert_eq!(edit.result().stored_count(), view.stored_count());
    assert_eq!(edit.result().version(), view.version());
    assert_eq!(edit.result().options(), view.options());
    state_carrier::check_history(edit.carrier_edit());
}
fn check_reverse(
    view: &FlowAuxWideValues,
    edit: &FlowSimpleTableEdit<FlowAuxWideValues>,
    selection: Selection,
    original: &[u8],
) {
    let stored = edit.result().source().records().nth(1).unwrap().payload;
    let fresh = open(stored, view.options()).unwrap();
    let restored = apply(&fresh, selection, original);
    assert_eq!(restored.result().decoded_bytes(), view.decoded_bytes());
    state_carrier::check_history(restored.carrier_edit());
    assert_eq!(view.decoded_bytes(), edit.source().decoded_bytes());
}
#[test]
fn every_legacy_and_selected_pattern_edit_has_literal_bytes_noop_history_and_fresh_decoded_reverse()
{
    for row in cases() {
        let view = open(&row.stored, options()).unwrap();
        for selection in selections(&row) {
            check_edit(&view, selection);
        }
    }
}
