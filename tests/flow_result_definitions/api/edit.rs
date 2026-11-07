use super::*;
#[derive(Clone, Copy)]
pub(super) enum Field {
    Identifier(usize),
    Allocation(usize),
    Pattern(usize, FlowImsArrayKind, usize),
}
pub(super) fn fields(row: &DefinitionCase) -> Vec<Field> {
    let mut result = Vec::new();
    for (ordinal, object) in row.groups[0].objects.iter().enumerate() {
        result.push(Field::Identifier(ordinal));
        if object.body.allocation.is_some() {
            result.push(Field::Allocation(ordinal));
        }
        result.extend(array_fields(row, object, ordinal));
    }
    result
}
fn array_fields(row: &DefinitionCase, object: &DefinitionObject, ordinal: usize) -> Vec<Field> {
    let mut result = Vec::new();
    for (kind, array) in KINDS.into_iter().zip(&object.body.arrays) {
        if let Some(array) = array {
            result.extend(
                selected_indices(array.count as usize, row.native)
                    .into_iter()
                    .map(|index| Field::Pattern(ordinal, kind, index)),
            );
        }
    }
    result
}
fn selected_indices(count: usize, native: bool) -> Vec<usize> {
    if count <= 64 || native {
        return (0..count).collect();
    }
    vec![0, count / 2, count - 1]
}
fn original(row: &DefinitionCase, field: Field) -> (usize, Vec<u8>) {
    match field {
        Field::Identifier(ordinal) => (
            5 + 4 * ordinal,
            row.groups[0].objects[ordinal]
                .identifier
                .to_le_bytes()
                .to_vec(),
        ),
        Field::Allocation(ordinal) => {
            let body = &row.groups[0].objects[ordinal].body;
            (
                body.arrays[1].as_ref().unwrap().end as usize,
                vec![body.allocation.unwrap()],
            )
        }
        Field::Pattern(ordinal, kind, index) => original_pattern(row, ordinal, kind, index),
    }
}
fn apply(
    view: &FlowResultDefinitions,
    field: Field,
    raw: &[u8],
) -> FlowSimpleTableEdit<FlowResultDefinitions> {
    match field {
        Field::Identifier(ordinal) => view
            .replace_identifier(ordinal, i32::from_le_bytes(raw.try_into().unwrap()))
            .unwrap(),
        Field::Allocation(ordinal) => view.replace_allocation_byte(ordinal, raw[0]).unwrap(),
        Field::Pattern(ordinal, kind, index) => view
            .replace_pattern(
                ordinal,
                kind,
                index,
                u32::from_le_bytes(raw.try_into().unwrap()),
            )
            .unwrap(),
    }
}
pub(super) fn check_edit(
    view: &FlowResultDefinitions,
    row: &DefinitionCase,
    field: Field,
) -> Vec<u8> {
    let (offset, old) = original(row, field);
    let before = view.source().as_bytes().to_vec();
    let no_op = apply(view, field, &old);
    assert_eq!(no_op.result().source().as_bytes(), before);
    assert_eq!(no_op.result().record(), view.record());
    state_carrier::check_history(no_op.carrier_edit());
    let changed: Vec<_> = old.iter().map(|value| !value).collect();
    let edit = apply(view, field, &changed);
    check_expected(row, &edit, offset, &changed);
    check_reverse(row, &edit, field, &old);
    assert_eq!(view.source().as_bytes(), before);
    edit.result().source().as_bytes().to_vec()
}
fn check_expected(
    row: &DefinitionCase,
    edit: &FlowSimpleTableEdit<FlowResultDefinitions>,
    offset: usize,
    changed: &[u8],
) {
    let mut expected = row.decoded.clone();
    expected[offset..offset + changed.len()].copy_from_slice(changed);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().decoded_bytes(), row.decoded);
    assert_eq!(edit.result().stored_count(), row.groups[0].objects.len());
    assert_eq!(edit.result().coverage().raw_words, raw_count(row));
    assert_eq!(edit.result().options(), options());
    check_diagnostics(edit.result());
    state_carrier::check_history(edit.carrier_edit());
}
fn check_reverse(
    row: &DefinitionCase,
    edit: &FlowSimpleTableEdit<FlowResultDefinitions>,
    field: Field,
    old: &[u8],
) {
    let stored = edit.result().source().records().nth(1).unwrap().payload;
    let fresh = open(stored, options()).unwrap();
    let reverse = apply(&fresh, field, old);
    assert_eq!(reverse.result().decoded_bytes(), row.decoded);
    assert_eq!(reverse.result().coverage().raw_words, raw_count(row));
    state_carrier::check_history(reverse.carrier_edit());
}
#[test]
fn all_identifiers_allocation_bytes_and_selected_patterns_have_literal_patches_noops_and_fresh_decoded_reversals()
 {
    for row in cases() {
        let view = open(&row.stored, options()).unwrap();
        for field in fields(&row) {
            check_edit(&view, &row, field);
        }
    }
}

fn original_pattern(
    row: &DefinitionCase,
    ordinal: usize,
    kind: FlowImsArrayKind,
    index: usize,
) -> (usize, Vec<u8>) {
    let array = row.groups[0].objects[ordinal].body.arrays[kind as usize]
        .as_ref()
        .unwrap();
    (
        array.start as usize + 12 + index * 4,
        array.raw[index * 4..index * 4 + 4].to_vec(),
    )
}
