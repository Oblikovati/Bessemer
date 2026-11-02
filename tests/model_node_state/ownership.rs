use super::*;

#[test]
fn views_and_flags_are_send_sync_and_preserve_owned_words_after_buffers_change() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ModelNodeStoredState>();
    send_sync::<ModelNodeStoredFlag>();
    let (source, manager) = configuration();
    let case = cases().remove(0);
    let mut raw = case.raw.to_vec();
    let archive = PartResolvedFeaturesArchive::from_bytes(&raw, source, &manager, LIMITS).unwrap();
    let node = archive.node_state(&node_handle(&archive, 0)).unwrap();
    raw.fill(0xa5);
    assert_projection(&node, &case, 0);
    assert_eq!(archive.to_bytes().unwrap(), case.raw);
}

#[test]
fn foreign_stale_wrong_type_and_parameter_scope_requests_reject_without_mutating_sources() {
    let (source, manager) = configuration();
    let case = cases().remove(0);
    let archive = feature_archive(&case, &source, &manager);
    let foreign = feature_archive(&case, &source, &manager);
    let handle = node_handle(&archive, 0);
    let node = archive.node_state(&handle).unwrap();
    assert_foreign_view(&foreign, &handle, &node);
    assert!(
        archive
            .node_state(&archive.field_handle(0).unwrap())
            .is_err()
    );
    assert!(archive.field_handle(usize::MAX).is_err());
    assert_scope_and_stale(&archive, &node);
    assert_eq!(archive.to_bytes().unwrap(), case.raw);
    assert!(archive.resolve_field(&handle).is_ok());
}

fn assert_foreign_view(
    foreign: &PartResolvedFeaturesArchive,
    handle: &ModelFieldHandle,
    node: &ModelNodeStoredState,
) {
    assert!(foreign.node_state(handle).is_err());
    assert!(
        foreign
            .edit_node_flag(node, ModelNodeStoredFlag::NeedsUpdate, true)
            .is_err()
    );
}

fn assert_scope_and_stale(archive: &PartResolvedFeaturesArchive, node: &ModelNodeStoredState) {
    for &(flag, _, _, parameter) in FLAGS {
        if parameter {
            assert_eq!(
                archive.edit_node_flag(node, flag, true).unwrap_err().class,
                FailureClass::Unsupported
            );
        }
    }
    assert_stale_view(archive, node);
}

fn assert_stale_view(archive: &PartResolvedFeaturesArchive, node: &ModelNodeStoredState) {
    let changed = archive
        .edit_node_flag(
            node,
            ModelNodeStoredFlag::NeedsUpdate,
            !node.stored_flag(ModelNodeStoredFlag::NeedsUpdate).unwrap(),
        )
        .unwrap();
    let error = changed
        .archive()
        .edit_node_flag(node, ModelNodeStoredFlag::NeedsUpdate, false)
        .unwrap_err();
    assert!(error.entity.contains("ResolvedFeatures"));
}
