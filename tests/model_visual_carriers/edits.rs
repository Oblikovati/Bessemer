use super::*;

pub(super) fn verify_visual_rgb_writes(
    archive: &PartResolvedFeaturesArchive,
    raw: &[u8],
    columns: &[&str],
    suffix: &str,
) {
    for (ordinal, index) in visual_group_positions(archive.fields(), suffix)
        .into_iter()
        .enumerate()
    {
        let start = columns[8 + ordinal].parse::<usize>().unwrap();
        verify_rgb_write(archive, raw, start, index + 3);
    }
}

fn verify_rgb_write(archive: &PartResolvedFeaturesArchive, raw: &[u8], start: usize, index: usize) {
    let original = u32::from_le_bytes(raw[start + 5..start + 9].try_into().unwrap());
    let color = original ^ 0x00a53cf0;
    let mut expected = raw.to_vec();
    expected[start + 5..start + 9].copy_from_slice(&color.to_le_bytes());
    let mut fields = archive.fields().to_vec();
    fields[index].value = ModelFieldValue::U32(color);
    let result = archive.edit_fields(fields).unwrap();
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(result.archive().to_bytes().unwrap(), expected);
    assert_eq!(archive.to_bytes().unwrap(), raw);
    assert_eq!(result.changes().len(), 1);
    assert_eq!(
        result.changes()[0].parents(),
        [archive.field_handle(index).unwrap()]
    );
    verify_unselected_visual_fields(archive.fields(), result.archive().fields(), index);
    assert_eq!(color >> 24, original >> 24);
}

fn verify_unselected_visual_fields(before: &[ModelField], after: &[ModelField], selected: usize) {
    assert_eq!(before.len(), after.len());
    for (index, (original, result)) in before.iter().zip(after).enumerate() {
        assert_eq!(original.path, result.path);
        if index != selected {
            assert_eq!(original.value, result.value, "{}", original.path);
        }
    }
}
