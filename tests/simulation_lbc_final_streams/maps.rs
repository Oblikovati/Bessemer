use bessemer::simulation::*;

fn groups() -> [Vec<[i32; 4]>; 3] {
    [
        vec![[i32::MIN, -1, 0, i32::MAX], [1, 2, 3, 4]],
        vec![],
        vec![[7, 8, -9, 10]],
    ]
}

fn populated() -> SimulationLegacyMidsurfMap {
    SimulationLegacyMidsurfMap::from_groups(&groups(), 2).unwrap()
}

#[test]
fn legacy_three_group_layout_keeps_signed_words_counts_positions_and_exact_eof() {
    let map = populated();
    assert_eq!(map.to_bytes().len(), 60);
    assert_eq!(map.count_offsets(), [0, 36, 40]);
    assert_eq!(map.count_header(0).unwrap(), [2, 0, 0, 0]);
    assert_eq!(map.count_header(1).unwrap(), [0; 4]);
    assert_eq!(map.count_header(2).unwrap(), [1, 0, 0, 0]);
    for (actual, expected) in map.groups().iter().zip(groups()) {
        assert_eq!(actual.as_ref(), expected);
    }
    let id = map.cell_id(2, 0, 3).unwrap();
    assert_eq!(id.stream_offset(), 56);
    assert_eq!(id.coordinates(), (2, 0, 3));
    assert_eq!(map.cell(&id).unwrap(), 10);
    assert_eq!(map.coverage().checked_map_groups, 3);
    assert_eq!(map.coverage().checked_map_records, 3);
    assert_eq!(map.coverage().checked_text_frames, 0);
    assert_eq!(
        map.coverage().completion,
        SimulationLbcFinalCompletion::CompletedRecords
    );
}

#[test]
fn complete_empty_legacy_map_has_three_counts_and_no_usable_cell_id() {
    let map = SimulationLegacyMidsurfMap::from_bytes(&[0; 12], 0).unwrap();
    assert_eq!(map.to_bytes(), [0; 12]);
    assert!(map.groups().iter().all(|group| group.is_empty()));
    assert_eq!(
        map.coverage().completion,
        SimulationLbcFinalCompletion::CompletedEmpty
    );
    assert_eq!(map.coverage().checked_count_words, 3);
    assert!(map.cell_id(0, 0, 0).is_err());
    assert!(map.count_header(3).is_err());
    assert!(SimulationLegacyMidsurfMap::from_bytes(&[], 0).is_err());
}

#[test]
fn per_group_limits_all_prefixes_trailing_bytes_and_signed_bad_counts_fail() {
    let bytes = populated().to_bytes();
    for end in 0..bytes.len() {
        assert!(SimulationLegacyMidsurfMap::from_bytes(&bytes[..end], 2).is_err());
    }
    let mut extra = bytes.clone();
    extra.push(0);
    assert!(SimulationLegacyMidsurfMap::from_bytes(&extra, 2).is_err());
    assert!(SimulationLegacyMidsurfMap::from_bytes(&bytes, 1).is_err());
    assert!(SimulationLegacyMidsurfMap::from_groups(&groups(), 1).is_err());
    let mut negative = bytes;
    negative[..4].copy_from_slice(&(-1i32).to_le_bytes());
    assert!(SimulationLegacyMidsurfMap::from_bytes(&negative, usize::MAX).is_err());
    let one_each = [vec![[0; 4]], vec![[0; 4]], vec![[0; 4]]];
    assert_eq!(
        SimulationLegacyMidsurfMap::from_groups(&one_each, 1)
            .unwrap()
            .coverage()
            .checked_map_records,
        3
    );
}

#[test]
fn every_cell_changed_and_noop_edit_preserves_full_bytes_all_counts_and_word_history() {
    let source = populated();
    for (group_index, group) in source.groups().iter().enumerate() {
        for (record_index, record) in group.iter().enumerate() {
            verify_record_edits(&source, group_index, record_index, record);
        }
    }
}

fn verify_record_edits(
    source: &SimulationLegacyMidsurfMap,
    group: usize,
    record: usize,
    values: &[i32; 4],
) {
    for (component, value) in values.iter().enumerate() {
        let id = source.cell_id(group, record, component).unwrap();
        verify_cell_edit(source, &id, *value);
        verify_cell_edit(source, &id, *value ^ i32::MIN);
    }
}

fn verify_cell_edit(
    source: &SimulationLegacyMidsurfMap,
    id: &SimulationLegacyMapCellId,
    replacement: i32,
) {
    let edit = source.edit_cell(id, replacement).unwrap();
    let mut expected = source.to_bytes();
    expected[id.stream_offset()..id.stream_offset() + 4]
        .copy_from_slice(&replacement.to_le_bytes());
    assert_eq!(edit.archive.to_bytes(), expected);
    assert_eq!(edit.source.to_bytes(), source.to_bytes());
    assert_eq!(edit.archive.cell(&edit.result).unwrap(), replacement);
    assert_eq!(edit.replacement, replacement);
    assert_eq!(&edit.parent, id);
    verify_map_history(&edit);
}

pub(super) fn verify_map_history(edit: &SimulationLegacyMapCellEdit) {
    let changed = edit.completion == SimulationLegacyMapEditCompletion::Changed;
    assert_eq!(changed, edit.source.revision() != edit.archive.revision());
    assert_eq!(
        edit.cell_history.len(),
        edit.source.coverage().checked_map_records * 4
    );
    verify_count_history(edit);
    for entry in edit.cell_history.iter() {
        verify_cell_history(edit, entry, changed);
    }
    assert_eq!(
        edit.coverage.check_mode,
        SimulationLbcFinalCheckMode::ExactLegacyMapAndPreservation
    );
}

fn verify_count_history(edit: &SimulationLegacyMapCellEdit) {
    for (index, entry) in edit.count_history.iter().enumerate() {
        assert_eq!(entry.group, index);
        assert_eq!(entry.parent_offset, entry.result_offset);
        assert_eq!(entry.parent_bytes, entry.result_bytes);
        assert_eq!(
            entry.parent_bytes.as_slice(),
            edit.source.count_header(index).unwrap()
        );
        assert_eq!(entry.relation, SimulationLegacyMapRelation::Copied);
    }
}

fn verify_cell_history(
    edit: &SimulationLegacyMapCellEdit,
    entry: &SimulationLegacyMapCellHistory,
    changed: bool,
) {
    assert_eq!(entry.parent.coordinates(), entry.result.coordinates());
    assert_eq!(entry.parent.stream_offset(), entry.result.stream_offset());
    let derived = changed && entry.parent == edit.parent;
    assert_eq!(
        entry.relation == SimulationLegacyMapRelation::Derived,
        derived
    );
    let expected = if derived {
        edit.replacement
    } else {
        edit.source.cell(&entry.parent).unwrap()
    };
    assert_eq!(edit.archive.cell(&entry.result).unwrap(), expected);
    if !changed {
        assert_eq!(entry.parent, entry.result);
    }
}

#[test]
fn invalid_coordinates_and_foreign_revisions_do_not_mutate_source() {
    let source = populated();
    for position in [
        (3, 0, 0),
        (0, 2, 0),
        (0, 0, 4),
        (1, 0, 0),
        (usize::MAX, 0, 0),
    ] {
        assert!(source.cell_id(position.0, position.1, position.2).is_err());
    }
    let foreign = populated().cell_id(0, 0, 0).unwrap();
    assert!(source.cell(&foreign).is_err());
    assert!(source.edit_cell(&foreign, 0).is_err());
    assert_eq!(source.to_bytes(), populated().to_bytes());
}

#[test]
fn map_owns_caller_groups_bytes_and_returned_arc_mutations() {
    let mut original = groups();
    let source = SimulationLegacyMidsurfMap::from_groups(&original, 2).unwrap();
    original[0][0].fill(0);
    drop(original);
    let mut exported = source.to_bytes();
    exported.fill(0x81);
    let mut group_copies = source.groups().clone();
    std::sync::Arc::make_mut(&mut group_copies[0])[0].fill(0);
    assert_eq!(source.groups()[0][0][0], i32::MIN);
    assert_eq!(source.to_bytes(), populated().to_bytes());
}

#[test]
fn map_public_types_are_send_sync_and_parallel_complete_edit_histories_equal_serial() {
    assert_map_types();
    let source = populated();
    let serial = source
        .edit_cell(&source.cell_id(2, 0, 3).unwrap(), i32::MIN)
        .unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (source, serial) = (&source, &serial);
            scope.spawn(move || repeat_map_edit(source, serial));
        }
    });
}

fn repeat_map_edit(source: &SimulationLegacyMidsurfMap, serial: &SimulationLegacyMapCellEdit) {
    for _ in 0..32 {
        let edit = source
            .edit_cell(&source.cell_id(2, 0, 3).unwrap(), i32::MIN)
            .unwrap();
        assert_eq!(edit.archive.to_bytes(), serial.archive.to_bytes());
        assert_eq!(edit.coverage, serial.coverage);
        verify_map_history(&edit);
    }
}

fn assert_map_types() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SimulationLegacyMapRevision>();
    assert_send_sync::<SimulationLegacyMapCellId>();
    assert_send_sync::<SimulationLegacyMidsurfMap>();
    assert_send_sync::<SimulationLegacyMapEditCompletion>();
    assert_send_sync::<SimulationLegacyMapRelation>();
    assert_send_sync::<SimulationLegacyMapCellHistory>();
    assert_send_sync::<SimulationLegacyMapCountHistory>();
    assert_send_sync::<SimulationLegacyMapCellEdit>();
}
