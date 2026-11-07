use super::*;

pub(super) fn changed_word(raw: &[u8], offset: usize, mask: u32, enabled: bool) -> Vec<u8> {
    let before = u32::from_le_bytes(raw[offset..offset + 4].try_into().unwrap());
    let after = if enabled {
        before | mask
    } else {
        before & !mask
    };
    let mut result = raw.to_vec();
    result[offset..offset + 4].copy_from_slice(&after.to_le_bytes());
    result
}

pub(super) fn assert_edit(
    archive: &PartResolvedFeaturesArchive,
    node: &ModelNodeStoredState,
    case: &NodeCase<'_>,
    ordinal: usize,
    option: (ModelNodeStoredFlag, usize, u32, bool),
) {
    let (flag, relative, mask, _) = option;
    for enabled in [false, true] {
        let expected = changed_word(case.raw, case.offsets[ordinal] + relative, mask, enabled);
        let edit = archive.edit_node_flag(node, flag, enabled).unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), expected);
        assert_preservation(
            archive,
            &edit,
            node_handle(archive, ordinal).index() + relative / 4,
        );
        assert_eq!(edit.archive().coverage().wire_bytes, case.raw.len());
        assert_eq!(edit.fidelity(), ModelEditFidelity::ExactStoredFields);
        let result = edit
            .archive()
            .node_state(&node_handle(edit.archive(), ordinal))
            .unwrap();
        assert_eq!(result.stored_flag(flag).unwrap(), enabled);
        assert_eq!(archive.to_bytes().unwrap(), case.raw);
    }
}

pub(super) fn assert_preservation(
    source: &PartResolvedFeaturesArchive,
    edit: &PartResolvedFeaturesEdit,
    selected: usize,
) {
    assert_unselected_fields(source.fields(), edit.archive().fields(), selected);
    if source.fields() == edit.archive().fields() {
        assert_eq!(edit.completion(), ModelEditCompletion::Unchanged);
        assert!(edit.changes().is_empty());
        assert!(
            edit.archive()
                .resolve_field(&source.field_handle(selected).unwrap())
                .is_ok()
        );
        return;
    }
    assert_changed_history(source, edit, selected);
}

pub(super) fn assert_unselected_fields(
    before: &[ModelField],
    after: &[ModelField],
    selected: usize,
) {
    assert_eq!(before.len(), after.len());
    for (index, (left, right)) in before.iter().zip(after).enumerate() {
        assert_eq!(left.path, right.path);
        if index != selected {
            assert_eq!(left, right);
        }
    }
}

fn assert_changed_history(
    source: &PartResolvedFeaturesArchive,
    edit: &PartResolvedFeaturesEdit,
    selected: usize,
) {
    assert_eq!(edit.completion(), ModelEditCompletion::Changed);
    assert_eq!(edit.unchanged_fields(), source.fields().len() - 1);
    assert_eq!(edit.changes().len(), 1);
    let change = &edit.changes()[0];
    assert_eq!(change.relation(), ModelFieldRelation::Derived);
    assert_eq!(change.parents(), &[source.field_handle(selected).unwrap()]);
    assert_eq!(
        change.result(),
        Some(&edit.archive().field_handle(selected).unwrap())
    );
    assert!(
        edit.archive()
            .resolve_field(&source.field_handle(selected).unwrap())
            .is_err()
    );
    assert_eq!(edit.diagnostics(), source.diagnostics());
}

#[test]
fn every_owned_selected_bit_matches_closed_form_bytes_complete_fields_and_history() {
    let (source, manager) = configuration();
    for case in cases() {
        let archive = feature_archive(&case, &source, &manager);
        for ordinal in 0..2 {
            let node = archive.node_state(&node_handle(&archive, ordinal)).unwrap();
            for &option in FLAGS {
                if option.3 && !is_parameter(&case) {
                    continue;
                }
                assert_edit(&archive, &node, &case, ordinal, option);
            }
        }
    }
}
