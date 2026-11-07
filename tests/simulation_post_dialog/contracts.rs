use super::*;

fn assert_noop(body: &SimulationPostDialog, ordinal: usize) {
    let id = body.field_id(ordinal).unwrap();
    let value = body.field(&id).unwrap().value.clone();
    let edit = body.edit_field(&id, value.clone()).unwrap();
    assert_eq!(edit.result(), id);
    assert_eq!(edit.parent(), id);
    assert_eq!(edit.replacement(), &value);
    assert_eq!(
        edit.coverage().completion,
        SimulationPostDialogCompletion::Unchanged
    );
    assert_eq!(edit.source().to_bytes(), body.to_bytes());
    assert_eq!(edit.archive().to_bytes(), body.to_bytes());
    assert_eq!(edit.archive().field_offsets(), body.field_offsets());
    assert_noop_history(body, &edit);
}

#[test]
fn every_owned_field_noop_preserves_input_revision_bytes_spans_and_provenance() {
    for body in owned_dialogs() {
        for ordinal in 0..body.fields().len() {
            assert_noop(&body, ordinal);
        }
    }
}

#[test]
fn foreign_stale_wrong_kind_and_wrong_path_requests_preserve_the_source() {
    let body = owned_dialogs().remove(0);
    let original = body.to_bytes();
    let copy = SimulationPostDialog::from_bytes(&original, body.version(), LIMITS).unwrap();
    let id = body.field_id(0).unwrap();
    assert!(copy.field(&id).is_err());
    assert!(body.field_id(body.fields().len()).is_err());
    assert!(
        body.edit_field(&id, SimulationPostDialogValue::Float32Bits(0))
            .is_err()
    );
    let edit = body
        .edit_field(&id, SimulationPostDialogValue::RawWordBits(0x12345678))
        .unwrap();
    assert!(edit.archive().field(&id).is_err());
    let mut fields = body.fields().to_vec();
    fields[0].path = "wrong.parent.path".into();
    assert!(SimulationPostDialog::from_fields(fields, body.version(), LIMITS).is_err());
    assert_eq!(body.to_bytes(), original);
}

#[test]
fn caller_byte_and_frame_mutation_cannot_change_a_published_parent() {
    let body = owned_dialogs().remove(0);
    let original = body.to_bytes();
    let mut supplied = original.clone();
    let copied = SimulationPostDialog::from_bytes(&supplied, body.version(), LIMITS).unwrap();
    supplied.fill(255);
    assert_eq!(copied.to_bytes(), original);
    let mut payload = vec![1, 0, 255];
    let frame = SimulationPostDialogBytes::from_payload(&payload, 3).unwrap();
    payload.fill(0);
    assert_eq!(frame.payload(), &[1, 0, 255]);
    assert_eq!(frame.to_bytes(), &[3, 0, 0, 0, 1, 0, 255]);
    assert!(SimulationPostDialogBytes::from_payload(&[1], 0).is_err());
    assert_eq!(body.to_bytes(), original);
}

fn assert_noop_history(body: &SimulationPostDialog, edit: &SimulationPostDialogEdit) {
    typed_history::assert_outer(
        body.fields().len(),
        |i| body.field_id(i).unwrap(),
        |i| edit.archive().field_id(i).unwrap(),
        edit.history(),
        usize::MAX,
    );
    assert!(
        edit.nested_history()
            .iter()
            .all(|row| row.relation == SimulationPostDialogRelation::Copied)
    );
}
