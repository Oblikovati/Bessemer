use super::*;
#[derive(Clone, Copy)]
pub(super) enum Field {
    First(usize),
    Second(usize),
    Third(usize),
}
pub(super) fn fields(row: &ControlCase) -> Vec<Field> {
    let count = row.count as usize;
    let ordinals: Vec<_> = if count <= 64 {
        (0..count).collect()
    } else {
        vec![0, count / 2, count - 1]
    };
    let mut result = Vec::new();
    for ordinal in ordinals {
        result.extend([Field::First(ordinal), Field::Second(ordinal)]);
        if row.identifier == INDEX {
            result.push(Field::Third(ordinal));
        }
    }
    result
}
fn original(row: &ControlCase, field: Field) -> (usize, Vec<u8>) {
    let index = match field {
        Field::First(index) | Field::Second(index) | Field::Third(index) => index,
    };
    let (prefix, stride) = if row.identifier == SAVED {
        (16, 16)
    } else {
        (4, 20)
    };
    let (field_offset, size) = match field {
        Field::First(_) => (0, if row.identifier == SAVED { 8 } else { 4 }),
        Field::Second(_) => (if row.identifier == SAVED { 8 } else { 4 }, 8),
        Field::Third(_) => (12, 8),
    };
    let start = index * stride + field_offset;
    (prefix + start, row.entries[start..start + size].to_vec())
}
fn apply_saved(
    view: &FlowSavedSpans,
    field: Field,
    raw: &[u8],
) -> FlowSimpleTableEdit<FlowSavedSpans> {
    let bits = u64::from_le_bytes(raw.try_into().unwrap());
    match field {
        Field::First(ordinal) => view.replace_key(ordinal, bits).unwrap(),
        Field::Second(ordinal) => view.replace_value(ordinal, bits).unwrap(),
        Field::Third(_) => panic!("saved pair third field; expected only key/value"),
    }
}
fn apply_index(
    view: &FlowTerminalIndex,
    field: Field,
    raw: &[u8],
) -> FlowSimpleTableEdit<FlowTerminalIndex> {
    match field {
        Field::First(ordinal) => view
            .replace_identifier(ordinal, u32::from_le_bytes(raw.try_into().unwrap()))
            .unwrap(),
        Field::Second(ordinal) => view
            .replace_record_start(ordinal, u64::from_le_bytes(raw.try_into().unwrap()))
            .unwrap(),
        Field::Third(ordinal) => view
            .replace_record_span(ordinal, u64::from_le_bytes(raw.try_into().unwrap()))
            .unwrap(),
    }
}
pub(super) fn check_saved_edit(view: &FlowSavedSpans, row: &ControlCase, field: Field) -> Vec<u8> {
    let (offset, old) = original(row, field);
    let no_op = apply_saved(view, field, &old);
    assert_eq!(no_op.result().source().as_bytes(), view.source().as_bytes());
    assert_eq!(no_op.result().record(), view.record());
    state_carrier::check_history(no_op.carrier_edit());
    let changed: Vec<_> = old.iter().map(|byte| !byte).collect();
    let edit = apply_saved(view, field, &changed);
    assert_eq!(
        edit.result().decoded_bytes(),
        patched(row, offset, &changed)
    );
    assert_eq!(edit.result().options(), view.options());
    assert_eq!(edit.result().diagnostics(), view.diagnostics());
    check_saved_reverse(&edit, row, field, &old);
    state_carrier::check_history(edit.carrier_edit());
    assert_eq!(view.decoded_bytes(), row.decoded);
    edit.result().source().as_bytes().to_vec()
}
fn check_saved_reverse(
    edit: &FlowSimpleTableEdit<FlowSavedSpans>,
    row: &ControlCase,
    field: Field,
    old: &[u8],
) {
    let stored = edit.result().source().records().nth(1).unwrap().payload;
    let fresh = open_saved(stored, edit.result().options()).unwrap();
    let reverse = apply_saved(&fresh, field, old);
    assert_eq!(reverse.result().decoded_bytes(), row.decoded);
    state_carrier::check_history(reverse.carrier_edit());
}
pub(super) fn check_index_edit(
    view: &FlowTerminalIndex,
    row: &ControlCase,
    field: Field,
) -> Vec<u8> {
    let (offset, old) = original(row, field);
    let no_op = apply_index(view, field, &old);
    assert_eq!(no_op.result().source().as_bytes(), view.source().as_bytes());
    assert_eq!(no_op.result().record(), view.record());
    state_carrier::check_history(no_op.carrier_edit());
    let changed: Vec<_> = old.iter().map(|byte| !byte).collect();
    let edit = apply_index(view, field, &changed);
    assert_eq!(
        edit.result().decoded_bytes(),
        patched(row, offset, &changed)
    );
    assert_eq!(edit.result().length_width(), view.length_width());
    assert_eq!(edit.result().options(), view.options());
    assert_eq!(edit.result().diagnostics(), view.diagnostics());
    check_recomputed_footer(edit.result());
    check_index_reverse(&edit, row, field, &old);
    state_carrier::check_history(edit.carrier_edit());
    assert_eq!(view.decoded_bytes(), row.decoded);
    edit.result().source().as_bytes().to_vec()
}
fn patched(row: &ControlCase, offset: usize, replacement: &[u8]) -> Vec<u8> {
    let mut expected = row.decoded.clone();
    expected[offset..offset + replacement.len()].copy_from_slice(replacement);
    expected
}
fn check_index_reverse(
    edit: &FlowSimpleTableEdit<FlowTerminalIndex>,
    row: &ControlCase,
    field: Field,
    old: &[u8],
) {
    let stored = edit.result().source().records().nth(1).unwrap().payload;
    let fresh = open_index(stored, width(row), edit.result().options()).unwrap();
    let reverse = apply_index(&fresh, field, old);
    assert_eq!(reverse.result().decoded_bytes(), row.decoded);
    check_recomputed_footer(reverse.result());
    state_carrier::check_history(reverse.carrier_edit());
}
pub(super) fn check_recomputed_footer(view: &FlowTerminalIndex) {
    let stored = view.source().records().nth(1).unwrap().payload;
    let (start, end) = view.footer_interval();
    assert_eq!(end, stored.len());
    assert_eq!(end - start, view.length_width().byte_width() + 4);
    assert_eq!(
        view.backward_distance(),
        (start + view.length_width().byte_width() + 4) as u64
    );
    let mut distance = [0_u8; 8];
    let width = view.length_width().byte_width();
    distance[..width].copy_from_slice(&stored[start..start + width]);
    assert_eq!(u64::from_le_bytes(distance), stored.len() as u64);
    assert_eq!(&stored[start + width..], &INDEX.to_le_bytes());
    assert_eq!(
        streams::inflate_exact(&stored[8..start]),
        view.decoded_bytes()
    );
}
#[test]
fn all_small_entries_and_selected_large_fields_have_literal_patches_noops_history_and_fresh_decoded_reversals()
 {
    for row in cases() {
        if row.identifier == SAVED {
            check_saved_case_edits(&row);
        } else {
            check_index_case_edits(&row);
        }
    }
}
fn check_saved_case_edits(row: &ControlCase) {
    let view = open_saved(&row.stored, options()).unwrap();
    for field in fields(row) {
        check_saved_edit(&view, row, field);
    }
}
fn check_index_case_edits(row: &ControlCase) {
    let view = open_index(&row.stored, width(row), options()).unwrap();
    for field in fields(row) {
        check_index_edit(&view, row, field);
    }
}
