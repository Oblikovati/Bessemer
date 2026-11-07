use super::*;
pub(super) fn wides(
    actual: FlowCheckpointPatternArray<'_>,
    start: usize,
    values: &[u64],
    edits: &mut Vec<EditableField>,
) {
    check_shape(actual, start, values.len(), 8);
    for (index, value) in values.iter().enumerate() {
        let value = FlowCheckpointValue::Pattern64(*value);
        assert_eq!(actual.value(index).unwrap(), value);
        check_field(
            actual.field(index).unwrap(),
            value,
            (start + index * 8, start + index * 8 + 8),
            EditClass::Plain,
            selected(index, values.len()),
            edits,
        );
    }
}
pub(super) fn words(
    actual: FlowCheckpointPatternArray<'_>,
    start: usize,
    values: &[u32],
    edits: &mut Vec<EditableField>,
) {
    check_shape(actual, start, values.len(), 4);
    for (index, value) in values.iter().enumerate() {
        let value = FlowCheckpointValue::Pattern32(*value);
        assert_eq!(actual.value(index).unwrap(), value);
        check_field(
            actual.field(index).unwrap(),
            value,
            (start + index * 4, start + index * 4 + 4),
            EditClass::Plain,
            selected(index, values.len()),
            edits,
        );
    }
}
pub(super) fn bytes(
    actual: FlowCheckpointPatternArray<'_>,
    start: usize,
    values: &[u8],
    edits: &mut Vec<EditableField>,
) {
    check_shape(actual, start, values.len(), 1);
    for (index, value) in values.iter().enumerate() {
        let value = FlowCheckpointValue::Byte(*value);
        assert_eq!(actual.value(index).unwrap(), value);
        check_field(
            actual.field(index).unwrap(),
            value,
            (start + index, start + index + 1),
            EditClass::Plain,
            selected(index, values.len()),
            edits,
        );
    }
}
fn check_shape(actual: FlowCheckpointPatternArray<'_>, start: usize, count: usize, width: usize) {
    assert_eq!(actual.len(), count);
    assert_eq!(actual.is_empty(), count == 0);
    assert_eq!(actual.element_width(), width);
    assert_eq!(actual.payload_interval(), (start, start + count * width));
}
pub(super) fn container(
    actual: FlowCheckpointContainer<'_>,
    expected: &ContainerProjection,
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) {
    assert_eq!(actual.stored_count(), expected.count);
    assert_eq!(actual.payload_interval(), (expected.start, expected.end));
    let first = expected.start + 4;
    let indices = first + expected.count * 8;
    let second = indices + expected.count * 4;
    wides(actual.first_patterns(), first, &expected.first, edits);
    words(actual.indices(), indices, &expected.indices, edits);
    wides(actual.second_patterns(), second, &expected.second, edits);
    counts.containers += 1;
    counts.wide_patterns += expected.count * 3;
    counts.indices += expected.count;
}
