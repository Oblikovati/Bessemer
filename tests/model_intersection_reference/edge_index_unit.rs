use super::super::projection_tests::reference_archive;
use super::*;

fn index_projection_control() -> (Vec<ModelField>, ModelFieldHandle) {
    let archive = reference_archive();
    let index = archive
        .fields()
        .iter()
        .position(|field| index_candidate(&field.path))
        .unwrap();
    (
        archive.fields().to_vec(),
        archive.field_handle(index).unwrap(),
    )
}

#[test]
fn edge_index_projection_retains_signed_words_and_revision_identity() {
    let (mut fields, handle) = index_projection_control();
    for value in [i32::MIN, -2, -1, 0, 1, i32::MAX] {
        fields[handle.index()].value = ModelFieldValue::I32(value);
        let state = project_edge_index(&fields, &handle).unwrap();
        assert_eq!(state.value(), value);
        assert_eq!(state.source_field(), &handle);
        assert_eq!(state.clone(), state);
    }
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ModelIntersectionEdgeStoredIndex>();
}

#[test]
fn edge_index_projection_rejects_each_wrong_header_type_and_scope() {
    let (fields, handle) = index_projection_control();
    for relative in 0..5 {
        let mut wrong = fields.clone();
        wrong[handle.index() - relative].value = ModelFieldValue::U16(0);
        assert!(project_edge_index(&wrong, &handle).is_err());
        let mut wrong = fields.clone();
        wrong[handle.index() - relative].path = "another.edge.raw_168".into();
        assert!(project_edge_index(&wrong, &handle).is_err());
    }
}

#[test]
fn edge_index_projection_rejects_noncanonical_paths_and_missing_fields() {
    let (fields, handle) = index_projection_control();
    for replacement in [
        ".edges_48[00].raw_168",
        ".edges_48[-1].raw_168",
        ".edges_48[0].raw_164",
    ] {
        let mut wrong = fields.clone();
        wrong[handle.index()].path = wrong[handle.index()]
            .path
            .replace(".edges_48[0].raw_168", replacement)
            .into();
        assert!(project_edge_index(&wrong, &handle).is_err());
    }
    assert!(project_edge_index(&[], &handle).is_err());
    assert!(!index_candidate("other.raw_168"));
    assert!(index_candidate(&fields[handle.index()].path));
}

#[test]
fn edge_index_projection_requires_ordinary_marker_and_bounded_edge() {
    let (fields, handle) = index_projection_control();
    for (suffix, value) in [
        (".marker", ModelFieldValue::U16(1)),
        (".handle_count", ModelFieldValue::I32(0)),
    ] {
        verify_rejected_index_owner(&fields, &handle, suffix, value);
    }
}

fn verify_rejected_index_owner(
    fields: &[ModelField],
    handle: &ModelFieldHandle,
    suffix: &str,
    value: ModelFieldValue,
) {
    let prefix = edge_index_scope(&fields[handle.index()].path)
        .unwrap()
        .group
        .to_owned();
    let intersection = prefix.rsplit_once(".edges_48[").unwrap().0;
    let mut wrong = fields.to_vec();
    let field = wrong
        .iter_mut()
        .find(|field| field.path.strip_prefix(intersection) == Some(suffix))
        .unwrap();
    field.value = value;
    let error = project_edge_index(&wrong, handle).unwrap_err();
    assert!(
        error
            .cause
            .contains("expected an ordinary sgSketch marker0 edge")
    );
}
