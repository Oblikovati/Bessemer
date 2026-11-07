use super::*;
#[test]
fn every_selected_existing_field_edits_independently_reopens_and_reverses_with_complete_byte_history()
 {
    let mut totals = [0usize; 3];
    for row in cases() {
        let view = component(&row);
        for field in read::fields(&row, &view) {
            let replacement = replacement(&field);
            check_edit(&view, &field, replacement);
            check_noop(&view, &field);
            totals[0] += 1;
            totals[1] += usize::from(replacement != field.value);
            totals[2] += usize::from(matches!(
                field.class,
                EditClass::Presence | EditClass::SlotTag
            ));
        }
    }
    assert!(totals[0] > 10_000 && totals[1] > 10_000);
    println!(
        "{{\"selected_field_edits\":{},\"changed_fields\":{},\"protected_byte_classes\":{}}}",
        totals[0], totals[1], totals[2]
    );
}
pub(super) fn replacement(field: &EditableField) -> FlowCheckpointValue {
    use FlowCheckpointValue::*;
    match (field.class, field.value) {
        (EditClass::Presence, Byte(0)) => Byte(0),
        (EditClass::Presence, Byte(value)) => Byte(if value == 1 { 255 } else { 1 }),
        (EditClass::SlotTag, Byte(value)) => Byte(if value <= 1 {
            1 - value
        } else if value == 2 {
            255
        } else {
            2
        }),
        (_, Signed32(value)) => Signed32(!value),
        (_, Pattern32(value)) => Pattern32(!value),
        (_, Pattern64(value)) => Pattern64(!value),
        (_, Byte(value)) => Byte(value ^ 0xa5),
    }
}
pub(super) fn check_edit(
    view: &FlowCheckpointComponent,
    field: &EditableField,
    value: FlowCheckpointValue,
) -> FlowCheckpointComponentEdit {
    let mut expected = view.as_bytes().to_vec();
    expected[field.interval.0..field.interval.1].copy_from_slice(&encoded(value));
    let edit = view.replace_field(&field.id, value).unwrap();
    assert_eq!(edit.source().as_bytes(), view.as_bytes());
    assert_eq!(edit.result().as_bytes(), expected);
    check_correspondence(&edit, field, value);
    check_history(&edit, field.interval, value != field.value);
    check_reopen_and_reverse(&edit, field.value);
    assert_eq!(field.id.value().unwrap(), field.value);
    edit
}
fn check_correspondence(
    edit: &FlowCheckpointComponentEdit,
    field: &EditableField,
    value: FlowCheckpointValue,
) {
    assert_eq!(edit.source_field(), &field.id);
    assert_eq!(edit.source_field().value().unwrap(), field.value);
    assert_eq!(edit.result_field().value().unwrap(), value);
    assert_eq!(edit.result_field().payload_interval(), field.interval);
    assert_eq!(edit.result().context(), edit.source().context());
    assert_eq!(edit.result().kind(), edit.source().kind());
    assert_eq!(edit.result().options(), edit.source().options());
    let mut expected = edit.source().coverage().counts;
    if let (EditClass::SlotTag, FlowCheckpointValue::Byte(old), FlowCheckpointValue::Byte(new)) =
        (field.class, field.value, value)
    {
        expected.noncanonical_tags =
            expected.noncanonical_tags + usize::from(new > 2) - usize::from(old > 2);
    }
    assert_eq!(edit.result().coverage().counts, expected);
    assert!(!edit.result().coverage().complete_parent_record);
}
fn check_reopen_and_reverse(edit: &FlowCheckpointComponentEdit, old: FlowCheckpointValue) {
    let result = edit.result();
    let reopened = reopen(result);
    assert_eq!(reopened.as_bytes(), result.as_bytes());
    assert_eq!(reopened.coverage(), result.coverage());
    assert_eq!(reopened.diagnostics(), result.diagnostics());
    let reverse = result.replace_field(edit.result_field(), old).unwrap();
    assert_eq!(reverse.result().as_bytes(), edit.source().as_bytes());
    assert_eq!(reverse.result().coverage(), edit.source().coverage());
}
fn check_noop(view: &FlowCheckpointComponent, field: &EditableField) {
    let edit = view.replace_field(&field.id, field.value).unwrap();
    assert_eq!(edit.result().as_bytes(), view.as_bytes());
    assert_eq!(edit.source_field(), edit.result_field());
    assert_eq!(edit.source_field(), &field.id.clone());
    assert_eq!(edit.result().coverage(), view.coverage());
    assert!(
        edit.history()
            .iter()
            .all(|row| row.relation == FlowPayloadRelation::Copied)
    );
    check_history(&edit, field.interval, false);
}
fn check_history(edit: &FlowCheckpointComponentEdit, field: (usize, usize), changed: bool) {
    let mut position = 0;
    let mut derived = 0;
    for row in edit.history() {
        assert_eq!(row.source_interval, row.result_interval);
        let (start, end) = row.source_interval;
        assert_eq!(start, position);
        assert!(start < end);
        if row.relation == FlowPayloadRelation::Copied {
            assert_eq!(
                edit.source().as_bytes()[start..end],
                edit.result().as_bytes()[start..end]
            );
        } else {
            assert_eq!((start, end), field);
            derived += 1;
        }
        position = end;
    }
    assert_eq!(position, edit.source().as_bytes().len());
    assert_eq!(derived, usize::from(changed));
}
fn encoded(value: FlowCheckpointValue) -> Vec<u8> {
    match value {
        FlowCheckpointValue::Byte(value) => vec![value],
        FlowCheckpointValue::Signed32(value) => value.to_le_bytes().to_vec(),
        FlowCheckpointValue::Pattern32(value) => value.to_le_bytes().to_vec(),
        FlowCheckpointValue::Pattern64(value) => value.to_le_bytes().to_vec(),
    }
}
