use super::*;

pub(super) fn verify_visual_word_writes(
    archive: &PartResolvedFeaturesArchive,
    raw: &[u8],
    columns: &[&str],
    suffix: &str,
) {
    let positions = visual_group_positions(archive.fields(), suffix);
    let starts = [
        columns[8].parse::<usize>().unwrap(),
        columns[9].parse().unwrap(),
    ];
    for ordinal in 0..2 {
        let replacement = &raw[starts[1 - ordinal]..starts[1 - ordinal] + 17];
        verify_complete_word_write(
            archive,
            raw,
            positions[ordinal],
            starts[ordinal],
            replacement,
        );
    }
}

fn verify_complete_word_write(
    source: &PartResolvedFeaturesArchive,
    raw: &[u8],
    index: usize,
    start: usize,
    replacement: &[u8],
) {
    let mut fields = source.fields().to_vec();
    for (field, value) in fields[index..index + 7]
        .iter_mut()
        .zip(visual_literal_values(replacement))
    {
        field.value = value;
    }
    let result = source.edit_fields(fields).unwrap();
    let mut expected = raw.to_vec();
    expected[start..start + 17].copy_from_slice(replacement);
    assert_eq!(result.archive().to_bytes().unwrap(), expected);
    assert_eq!(source.to_bytes().unwrap(), raw);
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    verify_complete_word_history(source, &result, index);
    verify_unselected_word_fields(source.fields(), result.archive().fields(), index);
}

fn verify_complete_word_history(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    index: usize,
) {
    assert_eq!(result.changes().len(), 7);
    for (offset, change) in [4, 6, 5, 0, 1, 2, 3].into_iter().zip(result.changes()) {
        assert_eq!(
            change.parents(),
            [source.field_handle(index + offset).unwrap()]
        );
        assert_eq!(change.path(), source.fields()[index + offset].path.as_ref());
    }
}

fn verify_unselected_word_fields(before: &[ModelField], after: &[ModelField], start: usize) {
    assert_eq!(before.len(), after.len());
    for (index, (original, result)) in before.iter().zip(after).enumerate() {
        assert_eq!(original.path, result.path);
        if !(start..start + 7).contains(&index) {
            assert_eq!(original.value, result.value, "{}", original.path);
        }
    }
}
