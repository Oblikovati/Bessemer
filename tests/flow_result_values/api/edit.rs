use super::*;
#[derive(Clone, Copy)]
pub(super) enum Field {
    Flag(usize, FlowImsValueFlag),
    Shared(usize, FlowImsValueArray, usize),
    Bl(usize, usize, usize),
}
pub(super) fn fields(row: &ValueCase) -> Vec<Field> {
    let mut result = Vec::new();
    for (ordinal, object) in row.frame.objects.iter().enumerate() {
        result.extend(FLAGS.into_iter().map(|flag| Field::Flag(ordinal, flag)));
        result.extend(shared_fields(object, ordinal));
        if let Some(bl) = &object.bl {
            result.extend(bl_fields(bl, ordinal));
        }
    }
    result
}
fn indices(count: usize, all_through: usize) -> Vec<usize> {
    if count <= all_through {
        return (0..count).collect();
    }
    vec![0, count / 2, count - 1]
}
fn shared_fields(object: &ImsValueObject, ordinal: usize) -> Vec<Field> {
    let mut result = Vec::new();
    for (kind, array) in ARRAYS.into_iter().zip([
        object.first.as_ref(),
        Some(&object.main),
        object.last.as_ref(),
    ]) {
        if let Some(array) = array {
            result.extend(
                indices(array.count as usize, 257)
                    .into_iter()
                    .map(|index| Field::Shared(ordinal, kind, index)),
            );
        }
    }
    result
}
fn bl_fields(bl: &BlRows, ordinal: usize) -> Vec<Field> {
    indices(bl.count as usize, 16)
        .into_iter()
        .flat_map(|row| (0..18).map(move |column| Field::Bl(ordinal, row, column)))
        .collect()
}
fn original(row: &ValueCase, field: Field) -> (usize, Vec<u8>) {
    match field {
        Field::Flag(ordinal, flag) => flag_original(&row.frame.objects[ordinal], flag),
        Field::Shared(ordinal, kind, index) => {
            let obj = &row.frame.objects[ordinal];
            let array =
                [obj.first.as_ref(), Some(&obj.main), obj.last.as_ref()][kind as usize].unwrap();
            (
                array.start as usize + 12 + index * 8,
                array.raw[index * 8..index * 8 + 8].to_vec(),
            )
        }
        Field::Bl(ordinal, row_index, column) => {
            let bl = row.frame.objects[ordinal].bl.as_ref().unwrap();
            let start = row_index * 144 + column * 8;
            (
                bl.start as usize + 12 + start,
                bl.raw[start..start + 8].to_vec(),
            )
        }
    }
}
fn flag_original(object: &ImsValueObject, flag: FlowImsValueFlag) -> (usize, Vec<u8>) {
    let main_end = object.main.end as usize;
    match flag {
        FlowImsValueFlag::First => (object.start as usize + 4, vec![object.first_flag]),
        FlowImsValueFlag::Bl => (main_end, vec![object.bl_flag]),
        FlowImsValueFlag::Last => (
            object
                .bl
                .as_ref()
                .map_or(main_end + 1, |bl| bl.end as usize),
            vec![object.last_flag],
        ),
    }
}
fn apply(view: &FlowImsValues, field: Field, raw: &[u8]) -> FlowSimpleTableEdit<FlowImsValues> {
    match field {
        Field::Flag(ordinal, flag) => view.replace_flag(ordinal, flag, raw[0]).unwrap(),
        Field::Shared(ordinal, kind, index) => view
            .replace_shared_pattern(
                ordinal,
                kind,
                index,
                u64::from_le_bytes(raw.try_into().unwrap()),
            )
            .unwrap(),
        Field::Bl(ordinal, row, column) => view
            .replace_bl_pattern(
                ordinal,
                row,
                column,
                u64::from_le_bytes(raw.try_into().unwrap()),
            )
            .unwrap(),
    }
}
fn replacement(field: Field, original: &[u8]) -> Vec<u8> {
    if let Field::Flag(_, _) = field {
        return vec![if original[0] == 0 {
            0
        } else {
            original[0] % 255 + 1
        }];
    }
    original.iter().map(|byte| !byte).collect()
}
pub(super) fn check_edit(view: &FlowImsValues, row: &ValueCase, field: Field) -> Vec<u8> {
    let (offset, old) = original(row, field);
    let no_op = apply(view, field, &old);
    assert_eq!(no_op.result().source().as_bytes(), view.source().as_bytes());
    assert_eq!(no_op.result().record(), view.record());
    state_carrier::check_history(no_op.carrier_edit());
    let changed = replacement(field, &old);
    let edit = apply(view, field, &changed);
    check_expected(view, row, &edit, offset, &changed);
    check_reverse(view, row, &edit, field, &old);
    check_presence_failure(view, field, &old);
    edit.result().source().as_bytes().to_vec()
}
fn check_expected(
    view: &FlowImsValues,
    row: &ValueCase,
    edit: &FlowSimpleTableEdit<FlowImsValues>,
    offset: usize,
    changed: &[u8],
) {
    let mut expected = row.frame.decoded.clone();
    expected[offset..offset + changed.len()].copy_from_slice(changed);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().decoded_bytes(), row.frame.decoded);
    assert_eq!(view.decoded_bytes(), row.frame.decoded);
    assert_eq!(edit.result().context(), view.context());
    assert_eq!(edit.result().options(), options());
    check_diagnostics(edit.result());
    state_carrier::check_history(edit.carrier_edit());
}
fn check_reverse(
    view: &FlowImsValues,
    row: &ValueCase,
    edit: &FlowSimpleTableEdit<FlowImsValues>,
    field: Field,
    old: &[u8],
) {
    let stored = edit.result().source().records().nth(1).unwrap().payload;
    let fresh = open(stored, view.context(), options()).unwrap();
    let reverse = apply(&fresh, field, old);
    assert_eq!(reverse.result().decoded_bytes(), row.frame.decoded);
    assert_eq!(reverse.result().context(), view.context());
    state_carrier::check_history(reverse.carrier_edit());
}
fn check_presence_failure(view: &FlowImsValues, field: Field, old: &[u8]) {
    let Field::Flag(ordinal, flag) = field else {
        return;
    };
    let opposite = if old[0] == 0 { 255 } else { 0 };
    assert!(view.replace_flag(ordinal, flag, opposite).is_err());
}
#[test]
fn all_flags_and_selected_shared_and_bl_cells_have_independent_patches_noops_history_and_decoded_reversals()
 {
    for row in cases() {
        let view = open(&row.frame.stored, &context(&row), options()).unwrap();
        for field in fields(&row) {
            check_edit(&view, &row, field);
        }
    }
}
