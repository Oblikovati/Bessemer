use super::*;

pub(super) fn verify_ellipse_result(
    source: &PartResolvedFeaturesArchive,
    state: &ModelEllipseBackupRotationState,
    edit: &PartResolvedFeaturesEdit,
    snapshot: i32,
) {
    let changed = usize::from(snapshot != state.rotation_direction_snapshot());
    assert_eq!(edit.changes().len(), changed);
    assert_eq!(edit.unchanged_fields(), source.fields().len() - changed);
    let completion = if changed == 0 {
        ModelEditCompletion::Unchanged
    } else {
        ModelEditCompletion::Changed
    };
    assert_eq!(edit.completion(), completion);
    assert_eq!(edit.coverage().mode, ModelEditCheckMode::ExactStoredArchive);
    assert_eq!(edit.coverage().typed_fields, source.fields().len());
    assert_eq!(edit.coverage().wire_bytes, source.to_bytes().unwrap().len());
    assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(edit.diagnostics(), source.diagnostics());
    for change in edit.changes() {
        verify_ellipse_relation(source, state, edit, change, snapshot);
    }
}

fn verify_ellipse_relation(
    source: &PartResolvedFeaturesArchive,
    state: &ModelEllipseBackupRotationState,
    edit: &PartResolvedFeaturesEdit,
    change: &ModelFieldChange,
    snapshot: i32,
) {
    assert_eq!(change.parents(), std::slice::from_ref(state.source_field()));
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(
        change.path(),
        source
            .resolve_field(state.source_field())
            .unwrap()
            .path
            .as_ref()
    );
    let result = edit
        .archive()
        .resolve_field(change.result().unwrap())
        .unwrap();
    assert_eq!(result.value, ModelFieldValue::I32(snapshot));
}

pub(super) fn verify_non_ellipse_handles(source: &PartResolvedFeaturesArchive) {
    let owners: Vec<_> = source
        .ellipse_backup_rotation_states()
        .unwrap()
        .iter()
        .map(|state| state.owner_field().index())
        .collect();
    for (index, field) in source.fields().iter().enumerate() {
        if owners.contains(&index)
            || (!matches!(field.value, ModelFieldValue::ObjectTag(_))
                && !field.path.ends_with(".word_20"))
        {
            continue;
        }
        let error = source
            .ellipse_backup_rotation_state(&source.field_handle(index).unwrap())
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Invalid);
        assert!(error.cause.contains("schema1 moEllipseBackedUpData_c"));
    }
}

pub(super) fn verify_ellipse_revision_guards(
    source: &PartResolvedFeaturesArchive,
    foreign: &PartResolvedFeaturesArchive,
    state: &ModelEllipseBackupRotationState,
) {
    verify_rejected_ellipse_revision(foreign, state);
    let changed = source
        .edit_ellipse_backup_rotation(state, !state.rotation_direction_snapshot())
        .unwrap();
    verify_rejected_ellipse_revision(changed.archive(), state);
    let unchanged = source
        .edit_ellipse_backup_rotation(state, state.rotation_direction_snapshot())
        .unwrap();
    assert_eq!(
        unchanged
            .archive()
            .ellipse_backup_rotation_state(state.owner_field())
            .unwrap(),
        *state
    );
}

fn verify_rejected_ellipse_revision(
    foreign: &PartResolvedFeaturesArchive,
    state: &ModelEllipseBackupRotationState,
) {
    let expected = foreign.resolve_field(state.owner_field()).unwrap_err();
    assert_eq!(
        foreign.edit_ellipse_backup_rotation(state, 0).unwrap_err(),
        expected
    );
    assert_eq!(
        foreign
            .ellipse_backup_rotation_state(state.owner_field())
            .unwrap_err(),
        expected
    );
}
