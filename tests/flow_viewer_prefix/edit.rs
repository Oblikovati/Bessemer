use super::*;
#[derive(Clone, Copy)]
pub(super) enum Field {
    Signed(FlowViewerSignedField),
    Array(usize, usize),
    Discarded(usize),
}
pub(super) fn fields(row: &ViewerCase) -> Vec<Field> {
    use FlowViewerSignedField::*;
    let mut result = vec![
        Field::Signed(IgnoredInitial),
        Field::Signed(StoredSecond),
        Field::Signed(FinalWord),
    ];
    result.extend((0..3).map(Field::Discarded));
    for (ordinal, array) in row.arrays.iter().enumerate() {
        let n = array.count as usize;
        let indices = selected_indices(n, row.frame.native_version.is_some());
        result.extend(
            indices
                .into_iter()
                .map(|index| Field::Array(ordinal, index)),
        );
    }
    result
}
fn selected_indices(count: usize, native: bool) -> Vec<usize> {
    if count <= 64 || native {
        return (0..count).collect();
    }
    vec![0, count / 2, count - 1]
}

fn original(row: &ViewerCase, field: Field) -> (usize, Vec<u8>) {
    match field {
        Field::Signed(FlowViewerSignedField::IgnoredInitial) => {
            (0, row.frame.initial.to_le_bytes().to_vec())
        }
        Field::Signed(FlowViewerSignedField::StoredSecond) => {
            (4, row.frame.second.to_le_bytes().to_vec())
        }
        Field::Signed(FlowViewerSignedField::FinalWord) => (
            row.extent.prefix as usize - 4,
            row.final_word.to_le_bytes().to_vec(),
        ),
        Field::Array(ordinal, index) => (
            row.arrays[ordinal].start as usize + index * 8,
            row.arrays[ordinal].raw[index * 8..index * 8 + 8].to_vec(),
        ),
        Field::Discarded(index) => (
            row.extent.prefix as usize - 28 + index * 8,
            row.triplet[index * 8..index * 8 + 8].to_vec(),
        ),
    }
}
fn apply(view: &FlowViewerPrefix, field: Field, raw: &[u8]) -> FlowViewerPrefixEdit {
    match field {
        Field::Signed(field) => view
            .replace_signed(field, i32::from_le_bytes(raw.try_into().unwrap()))
            .unwrap(),
        Field::Array(ordinal, index) => view
            .replace_array_pattern(ordinal, index, u64::from_le_bytes(raw.try_into().unwrap()))
            .unwrap(),
        Field::Discarded(index) => view
            .replace_discarded_pattern(index, u64::from_le_bytes(raw.try_into().unwrap()))
            .unwrap(),
    }
}
pub(super) fn check_edit(view: &FlowViewerPrefix, row: &ViewerCase, field: Field) -> Vec<u8> {
    let (offset, old) = original(row, field);
    let before = view.source().as_bytes().to_vec();
    let no_op = apply(view, field, &old);
    assert_eq!(no_op.result().source().as_bytes(), before);
    state_carrier::check_history(no_op.carrier_edit());
    let changed: Vec<_> = old.iter().map(|value| !value).collect();
    let edit = apply(view, field, &changed);
    check_expected(view, row, &edit, offset, &changed);
    check_reverse(row, &edit, field, &old);
    assert_eq!(view.source().as_bytes(), before);
    edit.result().source().as_bytes().to_vec()
}
fn check_expected(
    view: &FlowViewerPrefix,
    row: &ViewerCase,
    edit: &FlowViewerPrefixEdit,
    offset: usize,
    changed: &[u8],
) {
    let mut expected = row.frame.decoded.clone();
    expected[offset..offset + changed.len()].copy_from_slice(changed);
    assert_eq!(edit.result().decoded_body(), expected);
    assert_eq!(edit.source().decoded_body(), row.frame.decoded);
    assert_eq!(edit.result().opaque_tail(), row.tail);
    assert_eq!(edit.result().outer_version(), view.outer_version());
    assert_eq!(edit.result().options(), view.options());
    assert_eq!(edit.coverage(), edit.result().coverage());
    assert_eq!(
        edit.coverage().body_validation,
        FlowViewerBodyValidation::Incomplete
    );
    assert_eq!(edit.diagnostics(), view.diagnostics());
    check_metadata(edit.result(), &expected, row.extent.prefix as usize);
    state_carrier::check_history(edit.carrier_edit());
}
fn check_metadata(view: &FlowViewerPrefix, raw: &[u8], end: usize) {
    let second = i32::from_le_bytes(raw[4..8].try_into().unwrap());
    let first = i32::from_le_bytes(raw[end - 4..end].try_into().unwrap());
    assert_eq!(
        view.native_metadata(),
        FlowViewerMetadata {
            first_i32: first,
            second_i32: if second == 1 { 2 } else { second }
        }
    );
}
fn check_reverse(row: &ViewerCase, edit: &FlowViewerPrefixEdit, field: Field, old: &[u8]) {
    let stored = edit.result().source().records().nth(1).unwrap().payload;
    let fresh = open(stored, edit.result().options()).unwrap();
    let restored = apply(&fresh, field, old);
    assert_eq!(restored.result().decoded_body(), row.frame.decoded);
    assert_eq!(restored.result().opaque_tail(), row.tail);
    assert_eq!(restored.result().outer_version(), row.frame.version);
    assert_eq!(
        restored.coverage().body_validation,
        FlowViewerBodyValidation::Incomplete
    );
    state_carrier::check_history(restored.carrier_edit());
}
#[test]
fn all_signed_discarded_and_selected_array_edits_have_literal_patches_noops_full_history_and_decoded_reversals()
 {
    for row in cases() {
        let view = open(&row.frame.stored, options()).unwrap();
        for field in fields(&row) {
            check_edit(&view, &row, field);
        }
    }
}
#[test]
fn stored_second_one_and_two_have_equal_native_output_but_distinct_serialized_values() {
    let row = cases()
        .into_iter()
        .find(|row| row.name == "opaque-0")
        .unwrap();
    let view = check_case(&row);
    let one = view
        .replace_signed(FlowViewerSignedField::StoredSecond, 1)
        .unwrap();
    let two = view
        .replace_signed(FlowViewerSignedField::StoredSecond, 2)
        .unwrap();
    assert_eq!(
        one.result().native_metadata(),
        two.result().native_metadata()
    );
    assert_eq!(one.result().stored_second(), 1);
    assert_eq!(two.result().stored_second(), 2);
    assert_ne!(one.result().decoded_body(), two.result().decoded_body());
}
