use super::*;
pub(super) fn grid(
    actual: FlowCheckpointGrid<'_>,
    expected: &GridProjection,
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) {
    assert_eq!(actual.dimensions(), expected.dimensions);
    members(actual, &expected.members, edits);
    let mut position = 24;
    for (axis, expected) in expected.groups.iter().enumerate() {
        position = group(
            actual.group(axis).unwrap(),
            expected,
            position,
            edits,
            counts,
        );
    }
    assert_eq!(position, expected.end);
}
fn members(actual: FlowCheckpointGrid<'_>, expected: &[i32; 3], edits: &mut Vec<EditableField>) {
    for (axis, member) in expected.iter().enumerate() {
        let value = FlowCheckpointValue::Signed32(*member);
        check_field(
            actual.member(axis).unwrap(),
            value,
            (12 + axis * 4, 16 + axis * 4),
            EditClass::Plain,
            true,
            edits,
        );
    }
}

fn group(
    actual: FlowCheckpointGroup<'_>,
    expected: &GroupProjection,
    start: usize,
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) -> usize {
    assert_eq!(actual.axis(), expected.ordinal);
    assert_eq!(actual.len(), expected.length);
    assert_eq!(actual.is_empty(), expected.length == 0);
    assert_eq!(actual.presence_interval(), (start, start + expected.length));
    assert_eq!(actual.presence_bytes(), expected.presence);
    counts.row_positions += expected.length;
    let mut position = start + expected.length;
    for (index, presence) in expected.rows.iter().enumerate() {
        presence_value(actual, presence, index, start, edits);
        position = present_row(
            actual.row(index).unwrap(),
            presence,
            position,
            edits,
            counts,
        );
    }
    position
}
fn presence_value(
    actual: FlowCheckpointGroup<'_>,
    presence: &PresenceProjection,
    index: usize,
    start: usize,
    edits: &mut Vec<EditableField>,
) {
    let value = FlowCheckpointValue::Byte(presence.flag);
    check_field(
        actual.presence(index).unwrap(),
        value,
        (start + index, start + index + 1),
        EditClass::Presence,
        selected(index, actual.len()),
        edits,
    );
}

fn present_row(
    actual: Option<FlowCheckpointRow<'_>>,
    expected: &PresenceProjection,
    position: usize,
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) -> usize {
    assert_eq!(actual.is_some(), expected.row.is_some());
    let Some(expected_row) = &expected.row else {
        return position;
    };
    let actual = actual.unwrap();
    assert_eq!(actual.ordinal(), expected.ordinal);
    assert_eq!(expected_row.start, position);
    row(actual, expected_row, edits, counts);
    counts.present_rows += 1;
    expected_row.end
}
fn row(
    actual: FlowCheckpointRow<'_>,
    expected: &RowProjection,
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) {
    assert_eq!(actual.payload_interval(), (expected.start, expected.end));
    assert_eq!(actual.stored_count(), expected.count);
    arrays::words(
        actual.indices(),
        expected.start + 4,
        &expected.indices,
        edits,
    );
    counts.indices += expected.count;
    assert_eq!(actual.slots().is_some(), expected.slots.is_some());
    if let Some(expected_slots) = &expected.slots {
        let actual_slots = actual.slots().unwrap();
        assert_eq!(actual_slots.len(), expected.count);
        assert_eq!(actual_slots.is_empty(), expected.count == 0);
        assert_eq!(
            actual_slots.payload_interval(),
            (expected.start + 4 + expected.count * 4, expected.end)
        );
        slots(actual_slots, expected_slots, edits, counts);
    }
}
fn slots(
    actual: FlowCheckpointSlots<'_>,
    expected: &[SlotProjection],
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) {
    let mut position = actual.payload_interval().0;
    counts.slots += expected.len();
    for (index, expected) in expected.iter().enumerate() {
        position = check_slot(
            actual.slot(index).unwrap(),
            expected,
            position,
            selected(index, actual.len()),
            edits,
            counts,
        );
    }
    assert_eq!(position, actual.payload_interval().1);
}
fn check_slot(
    slot: FlowCheckpointSlot<'_>,
    expected: &SlotProjection,
    position: usize,
    select: bool,
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) -> usize {
    assert_eq!(slot.ordinal(), expected.ordinal);
    assert_eq!(slot.canonical_writer_tag().unwrap(), expected.canonical);
    let end = expected
        .container
        .as_ref()
        .map_or(position + 1, |value| value.end);
    assert_eq!(slot.payload_interval(), (position, end));
    check_field(
        slot.tag(),
        FlowCheckpointValue::Byte(expected.tag),
        (position, position + 1),
        EditClass::SlotTag,
        select,
        edits,
    );
    slot_container(slot, expected, edits, counts);
    counts.noncanonical_tags += usize::from(expected.tag > 2);
    end
}

fn slot_container(
    actual: FlowCheckpointSlot<'_>,
    expected: &SlotProjection,
    edits: &mut Vec<EditableField>,
    counts: &mut FlowCheckpointCounts,
) {
    assert_eq!(actual.container().is_some(), expected.container.is_some());
    if let Some(expected) = &expected.container {
        arrays::container(actual.container().unwrap(), expected, edits, counts);
    }
}
