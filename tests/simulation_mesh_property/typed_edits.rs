use super::*;

pub(super) fn replace(
    body: &SimulationMeshProperty,
    index: usize,
    replacement: SimulationMeshPropertyValue,
) -> SimulationMeshPropertyEdit {
    let original = body.to_bytes();
    let (start, end) = body.field_offsets()[index];
    let wire = typed_load_values::wire(&replacement);
    let expected = [&original[..start], &wire, &original[end..]].concat();
    let parent = body.field_id(index).unwrap();
    let edit = body.edit_field(&parent, replacement.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), expected);
    assert_eq!(body.to_bytes(), original);
    assert_eq!(edit.source().to_bytes(), original);
    assert_eq!(edit.parent(), parent);
    assert_eq!(edit.replacement(), &replacement);
    assert_ne!(edit.result(), parent);
    assert_preservation(body, &edit, index, wire.len());
    assert_history(body, &edit, index);
    edit
}

fn assert_history(
    body: &SimulationMeshProperty,
    edit: &SimulationMeshPropertyEdit,
    changed: usize,
) {
    typed_load_history::assert_outer(
        body.fields().len(),
        |index| body.field_id(index).unwrap(),
        |index| edit.archive().field_id(index).unwrap(),
        edit.history(),
        changed,
    );
    typed_load_history::assert_nested(
        body.fields(),
        edit.archive().fields(),
        |index| body.field_id(index).unwrap(),
        |index| edit.archive().field_id(index).unwrap(),
        edit.replacement(),
        edit.nested_history(),
        changed,
    );
}

fn assert_preservation(
    body: &SimulationMeshProperty,
    edit: &SimulationMeshPropertyEdit,
    changed: usize,
    width: usize,
) {
    assert_field_spans(body, edit, changed, width);
    assert_record_spans(body, edit, changed, width);
    assert_constructed(edit.archive());
    assert_edit_coverage(edit);
}

fn assert_field_spans(
    body: &SimulationMeshProperty,
    edit: &SimulationMeshPropertyEdit,
    changed: usize,
    width: usize,
) {
    assert_eq!(body.fields().len(), edit.archive().fields().len());
    for (index, field) in body.fields().iter().enumerate() {
        if index != changed {
            assert_eq!(field, &edit.archive().fields()[index]);
        }
        let expected = typed_load_values::shifted_span(
            body.field_offsets()[index],
            body.field_offsets()[changed],
            index,
            changed,
            width,
        );
        assert_eq!(edit.archive().field_offsets()[index], expected);
    }
}

fn assert_edit_coverage(edit: &SimulationMeshPropertyEdit) {
    assert_eq!(
        edit.coverage().checked_bytes,
        edit.archive().to_bytes().len()
    );
    assert_eq!(
        edit.coverage().checked_fields,
        edit.archive().fields().len()
    );
    assert_eq!(
        edit.coverage().record_count,
        edit.archive().record_spans().len()
    );
    assert_eq!(
        edit.coverage().completion,
        SimulationMeshPropertyCompletion::Changed
    );
    assert_eq!(
        edit.coverage().check_mode,
        SimulationMeshPropertyCheckMode::ExactSelectedProfilesAndPreservation
    );
}

fn assert_record_spans(
    body: &SimulationMeshProperty,
    edit: &SimulationMeshPropertyEdit,
    changed: usize,
    width: usize,
) {
    let (start, end) = body.field_offsets()[changed];
    let delta = width as isize - (end - start) as isize;
    let shifted = |offset| shift_boundary(offset, end, delta);
    assert_eq!(
        body.record_spans().len(),
        edit.archive().record_spans().len()
    );
    for (before, after) in body
        .record_spans()
        .iter()
        .zip(edit.archive().record_spans())
    {
        assert_eq!(after.index, before.index);
        assert_eq!(after.start, shifted(before.start));
        assert_eq!(after.end, shifted(before.end));
    }
}

fn shift_boundary(offset: usize, target_end: usize, delta: isize) -> usize {
    if offset < target_end {
        return offset;
    }
    offset.checked_add_signed(delta).unwrap()
}

#[test]
fn all_owned_properties_support_every_stored_changed_value_kind_and_complete_supplied_archive_parents()
 {
    let mut counts = [0_usize; 6];
    for body in contracts::owned_properties() {
        for (index, replacement) in typed_values::choices(&body) {
            counts[typed_load_values::value_kind(&replacement)] += 1;
            replace(&body, index, replacement);
        }
    }
    assert_eq!(counts, [588, 0, 588, 4128, 46430, 1392]);
}
