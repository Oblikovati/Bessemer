use super::*;
pub(super) fn selected_indices(array: &VectorProjection) -> Vec<usize> {
    let count = array.length as usize;
    if count <= 257 {
        return (0..count).collect();
    }
    vec![0, count / 2, count - 1]
}
pub(super) fn check_edit(view: &FlowWideVectors, vector: usize, index: usize) -> Vec<u8> {
    let array = view.vector(vector).unwrap();
    let bits = array.raw_bits(index).unwrap();
    let before = view.source().as_bytes().to_vec();
    let no_op = view.replace_pattern(vector, index, bits).unwrap();
    assert_eq!(no_op.result().source().as_bytes(), before);
    state_carrier::check_history(no_op.carrier_edit());
    let edit = view.replace_pattern(vector, index, !bits).unwrap();
    check_expected(view, &edit, array.payload_interval().0 + index * 8, !bits);
    let fresh = open(edit.result().decoded_bytes(), view.options()).unwrap();
    let restored = fresh.replace_pattern(vector, index, bits).unwrap();
    assert_eq!(restored.result().source().as_bytes(), before);
    state_carrier::check_history(restored.carrier_edit());
    assert_eq!(view.source().as_bytes(), before);
    edit.result().source().as_bytes().to_vec()
}
fn check_expected(
    view: &FlowWideVectors,
    edit: &FlowSimpleTableEdit<FlowWideVectors>,
    offset: usize,
    bits: u64,
) {
    let mut expected = view.decoded_bytes().to_vec();
    expected[offset..offset + 8].copy_from_slice(&bits.to_le_bytes());
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(edit.source().decoded_bytes(), view.decoded_bytes());
    assert_eq!(edit.result().coverage(), view.coverage());
    assert_eq!(edit.result().options(), view.options());
    let (start, end) = view.length_table_interval();
    assert_eq!(
        &edit.result().decoded_bytes()[start..end],
        &view.decoded_bytes()[start..end]
    );
    state_carrier::check_history(edit.carrier_edit());
}
#[test]
fn selected_existing_patterns_preserve_length_table_and_have_literal_patches_noops_history_and_fresh_reversals()
 {
    for row in cases() {
        let view = open(&row.raw, options()).unwrap();
        for (ordinal, array) in row.arrays.iter().enumerate() {
            for index in selected_indices(array) {
                check_edit(&view, ordinal, index);
            }
        }
    }
}
