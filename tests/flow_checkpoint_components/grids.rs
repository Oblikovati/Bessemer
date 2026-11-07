use super::*;
pub(super) fn grid(raw: &[u8], expected: &GridProjection, with_slots: bool) {
    assert_eq!((expected.start, expected.end), (0, raw.len()));
    for axis in 0..3 {
        assert_eq!(
            scalars::unsigned(raw, axis * 4) as i32,
            expected.dimensions[axis]
        );
        assert_eq!(
            scalars::unsigned(raw, 12 + axis * 4) as i32,
            expected.members[axis]
        );
        assert!(expected.dimensions[axis] >= 0);
    }
    assert_eq!(expected.groups.len(), 3);
    let mut position = 24;
    for (axis, group) in expected.groups.iter().enumerate() {
        assert_eq!(group.ordinal, axis);
        assert_eq!(group.length, expected.dimensions[(axis + 1) % 3] as usize);
        position = group_rows(raw, group, position, with_slots);
    }
    assert_eq!(position, raw.len());
}
fn group_rows(raw: &[u8], group: &GroupProjection, start: usize, with_slots: bool) -> usize {
    assert_eq!(group.presence.len(), group.length);
    assert_eq!(group.rows.len(), group.length);
    assert_eq!(raw[start..start + group.length], group.presence);
    let mut position = start + group.length;
    for (ordinal, presence) in group.rows.iter().enumerate() {
        assert_eq!(presence.ordinal, ordinal);
        assert_eq!(presence.flag, group.presence[ordinal]);
        assert_eq!(presence.row.is_some(), presence.flag != 0);
        if let Some(expected) = &presence.row {
            assert_eq!(expected.start, position);
            row(raw, expected, with_slots);
            position = expected.end;
        }
    }
    position
}
fn row(raw: &[u8], expected: &RowProjection, with_slots: bool) {
    assert_eq!(
        scalars::unsigned(raw, expected.start) as usize,
        expected.count
    );
    assert_eq!(expected.count, expected.indices.len());
    let mut position = scalars::words(raw, expected.start + 4, &expected.indices);
    assert_eq!(expected.slots.is_some(), with_slots);
    if let Some(slots) = &expected.slots {
        assert_eq!(slots.len(), expected.count);
        for (ordinal, slot) in slots.iter().enumerate() {
            assert_eq!(slot.ordinal, ordinal);
            position = slot_body(raw, slot, position);
        }
    }
    assert_eq!(position, expected.end);
}
fn slot_body(raw: &[u8], slot: &SlotProjection, start: usize) -> usize {
    assert_eq!(raw[start], slot.tag);
    assert_eq!(slot.canonical, slot.tag <= 2);
    assert_eq!(slot.container.is_some(), slot.tag > 1);
    let Some(container) = &slot.container else {
        return start + 1;
    };
    assert_eq!(container.start, start + 1);
    scalars::container(raw, container);
    container.end
}
