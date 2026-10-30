//! Exact counted-i32 and UniqueIds-i64 framing, edits and ownership (#129).
use SimulationPersistScalarValue::{I32, I64};
use bessemer::simulation::*;
#[path = "simulation_persistent_scalars/corpus.rs"]
mod corpus;
#[path = "simulation_persistent_scalars/projections.rs"]
mod projections;

#[test]
fn integer_array_signed_limits_duplicates_headers_and_construction_match_literal_bytes() {
    let values = [i32::MIN, i32::MAX, -1, 0, -1];
    let record = SimulationPersistScalarArchive::from_integer_values(&values, 5).unwrap();
    let expected = [
        5, 0, 0, 0, 0, 0, 0, 128, 255, 255, 255, 127, 255, 255, 255, 255, 0, 0, 0, 0, 255, 255,
        255, 255,
    ];
    assert_eq!(record.to_bytes(), expected);
    assert_eq!(record.profile(), SimulationPersistScalarProfile::CountedI32);
    assert_eq!(record.used_count(), 5);
    assert_eq!(record.allocation_count(), None);
    assert_eq!(record.header(), [5, 0, 0, 0]);
    assert_eq!(record.payload(), &expected[4..]);
    assert_entries(&record, &values.map(I32));
    assert_eq!(
        record.fields()[0].role,
        SimulationPersistScalarRole::UsedCount
    );
    assert_eq!(record.fields()[0].value, I32(5));
    assert!(SimulationPersistScalarArchive::from_integer_values(&values, 4).is_err());
}

fn assert_entries(
    record: &SimulationPersistScalarArchive,
    values: &[SimulationPersistScalarValue],
) {
    for (index, expected) in values.iter().enumerate() {
        let id = record.entry_id(index).unwrap();
        let field = record.field(&id).unwrap();
        assert_eq!(field.role, SimulationPersistScalarRole::Entry(index));
        assert_eq!(&field.value, expected);
    }
    assert!(record.entry_id(values.len()).is_err());
    assert!(record.entry_id(usize::MAX).is_err());
    assert_eq!(record.coverage().checked_entries, values.len());
    assert_eq!(record.coverage().requested_pattern_error_bits, 0);
}

#[test]
fn unique_table_preserves_signed_high_words_duplicates_and_capacity_metadata() {
    let keys = [i64::MIN, i64::MAX, 1 << 40, -1, -1];
    let record = SimulationPersistScalarArchive::from_unique_keys(&keys, i32::MAX, 5).unwrap();
    let expected = [
        5, 0, 0, 0, 255, 255, 255, 127, 0, 0, 0, 0, 0, 0, 0, 128, 255, 255, 255, 255, 255, 255,
        255, 127, 0, 0, 0, 0, 0, 1, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
        255, 255, 255, 255, 255,
    ];
    assert_eq!(record.to_bytes(), expected);
    assert_eq!(record.allocation_count(), Some(i32::MAX));
    assert_eq!(
        record.fields()[1].role,
        SimulationPersistScalarRole::AllocationCapacity
    );
    assert_eq!(record.fields()[1].stream_offset, 4);
    assert_entries(&record, &keys.map(I64));
    assert_eq!(record.coverage().checked_header_bytes, 8);
    assert_eq!(record.coverage().maximum_entries, 5);
    assert!(SimulationPersistScalarArchive::from_unique_keys(&keys, 4, 5).is_err());
    assert!(SimulationPersistScalarArchive::from_unique_keys(&keys, -1, 5).is_err());
    assert!(SimulationPersistScalarArchive::from_unique_keys(&keys, 5, 4).is_err());
}

#[test]
fn completed_empty_profiles_and_maximum_capacity_do_not_consume_capacity_entries() {
    let integer = SimulationPersistScalarArchive::from_integer_values(&[], 0).unwrap();
    let unique = SimulationPersistScalarArchive::from_unique_keys(&[], i32::MAX, 0).unwrap();
    assert_eq!(integer.to_bytes(), [0; 4]);
    assert_eq!(unique.to_bytes(), [0, 0, 0, 0, 255, 255, 255, 127]);
    assert_eq!(unique.fields().len(), 2);
    for record in [integer, unique] {
        assert_eq!(
            record.coverage().completion,
            SimulationPersistScalarCompletion::CompletedEmpty
        );
        assert_eq!(record.coverage().checked_entries, 0);
        assert!(record.payload().is_empty());
        assert!(record.entry_id(0).is_err());
    }
}

#[test]
fn all_populated_prefixes_extra_bytes_bad_counts_capacities_and_limits_fail() {
    let integer = SimulationPersistScalarArchive::from_integer_values(&[-1, 0], 2).unwrap();
    let unique = SimulationPersistScalarArchive::from_unique_keys(&[-1, 1 << 40], 2, 2).unwrap();
    for record in [integer, unique] {
        let bytes = record.to_bytes();
        for end in 0..bytes.len() {
            assert!(
                SimulationPersistScalarArchive::from_bytes(&bytes[..end], record.profile(), 2)
                    .is_err()
            );
        }
        let mut extra = bytes.clone();
        extra.push(0);
        assert!(SimulationPersistScalarArchive::from_bytes(&extra, record.profile(), 2).is_err());
        assert!(SimulationPersistScalarArchive::from_bytes(&bytes, record.profile(), 1).is_err());
        let mut negative = bytes;
        negative[..4].copy_from_slice(&(-1i32).to_le_bytes());
        assert!(
            SimulationPersistScalarArchive::from_bytes(&negative, record.profile(), 2).is_err()
        );
    }
}

#[test]
fn invalid_unique_used_and_capacity_headers_fail_before_entries() {
    for (used, capacity) in [(-1i32, 0i32), (0, -1), (1, 0), (i32::MAX, i32::MAX)] {
        let bytes: Vec<_> = [used, capacity]
            .iter()
            .flat_map(|value| value.to_le_bytes())
            .collect();
        assert!(
            SimulationPersistScalarArchive::from_bytes(
                &bytes,
                SimulationPersistScalarProfile::UniqueIdsI64,
                usize::MAX
            )
            .is_err()
        );
    }
}

#[test]
fn entry_and_capacity_edits_match_complete_literal_splices_and_all_header_history() {
    let records = [
        SimulationPersistScalarArchive::from_integer_values(&[-1, 0], 2).unwrap(),
        SimulationPersistScalarArchive::from_unique_keys(&[-1, 1 << 40], 2, 2).unwrap(),
    ];
    for record in records {
        for field in record.fields() {
            assert_scalar_edit(&record, &field.id, field.value);
            match field.role {
                SimulationPersistScalarRole::Entry(_) => {
                    assert_scalar_edit(&record, &field.id, replacement(field.value))
                }
                SimulationPersistScalarRole::AllocationCapacity => {
                    assert_scalar_edit(&record, &field.id, I32(i32::MAX))
                }
                SimulationPersistScalarRole::UsedCount => {
                    assert!(record.edit_field(&field.id, I32(3)).is_err())
                }
            }
        }
    }
}

fn replacement(value: SimulationPersistScalarValue) -> SimulationPersistScalarValue {
    match value {
        I32(_) => I32(i32::MIN),
        I64(_) => I64(i64::MIN),
    }
}

fn pattern(value: SimulationPersistScalarValue) -> Vec<u8> {
    match value {
        I32(value) => value.to_le_bytes().to_vec(),
        I64(value) => value.to_le_bytes().to_vec(),
    }
}

fn assert_scalar_edit(
    source: &SimulationPersistScalarArchive,
    id: &SimulationPersistScalarId,
    replacement: SimulationPersistScalarValue,
) {
    let original = source.field(id).unwrap();
    let edit = source.edit_field(id, replacement).unwrap();
    let mut expected = source.to_bytes();
    let bytes = pattern(replacement);
    expected[original.stream_offset..original.stream_offset + bytes.len()].copy_from_slice(&bytes);
    assert_eq!(edit.archive.to_bytes(), expected);
    assert_eq!(edit.source.to_bytes(), source.to_bytes());
    assert_eq!(edit.archive.profile(), source.profile());
    assert_eq!(edit.archive.field(&edit.result).unwrap().value, replacement);
    assert_eq!(edit.parent, *id);
    assert_eq!(edit.replacement, replacement);
    assert_eq!(
        edit.coverage.check_mode,
        SimulationPersistScalarCheckMode::ExactCompleteFrameAndPreservation
    );
    assert_scalar_history(&edit);
}

fn assert_scalar_history(edit: &SimulationPersistScalarEdit) {
    let changed = edit.archive.to_bytes() != edit.source.to_bytes();
    assert_eq!(
        edit.completion == SimulationPersistScalarEditCompletion::Changed,
        changed
    );
    assert_eq!(edit.archive.revision() != edit.source.revision(), changed);
    assert_eq!(edit.history.len(), edit.source.fields().len());
    for (index, relation) in edit.history.iter().enumerate() {
        assert_scalar_relation(edit, relation, index, changed);
    }
}

fn assert_scalar_relation(
    edit: &SimulationPersistScalarEdit,
    relation: &SimulationPersistScalarHistory,
    index: usize,
    changed: bool,
) {
    let before = edit.source.field(&relation.parent).unwrap();
    let after = edit.archive.field(&relation.result).unwrap();
    assert_eq!(relation.parent, edit.source.fields()[index].id);
    assert_eq!(relation.result, edit.archive.fields()[index].id);
    assert_eq!(before.role, after.role);
    assert_eq!(before.stream_offset, after.stream_offset);
    let derived = changed && relation.parent == edit.parent;
    assert_eq!(
        relation.relation == SimulationPersistScalarRelation::Derived,
        derived
    );
    let expected = if derived {
        edit.replacement
    } else {
        before.value
    };
    assert_eq!(after.value, expected);
    if !changed {
        assert_eq!(relation.parent, relation.result);
    }
}

#[test]
fn wrong_width_invalid_capacity_changed_count_and_foreign_identity_leave_source_isolated() {
    let record = SimulationPersistScalarArchive::from_unique_keys(&[-1], 1, 1).unwrap();
    let original = record.to_bytes();
    assert!(
        record
            .edit_field(&record.entry_id(0).unwrap(), I32(0))
            .is_err()
    );
    assert!(record.edit_field(&record.fields()[1].id, I32(0)).is_err());
    assert!(record.edit_field(&record.fields()[1].id, I32(-1)).is_err());
    assert!(record.edit_field(&record.fields()[0].id, I32(2)).is_err());
    let foreign = SimulationPersistScalarArchive::from_unique_keys(&[-1], 1, 1).unwrap();
    assert!(record.field(&foreign.entry_id(0).unwrap()).is_err());
    assert!(
        record
            .edit_field(&foreign.entry_id(0).unwrap(), I64(0))
            .is_err()
    );
    assert_eq!(record.to_bytes(), original);
}

#[test]
fn explicit_profiles_and_both_width_mismatches_never_infer_a_conversion() {
    let integer = SimulationPersistScalarArchive::from_integer_values(&[-1], 1).unwrap();
    assert!(
        integer
            .edit_field(&integer.entry_id(0).unwrap(), I64(-1))
            .is_err()
    );
    let unique = SimulationPersistScalarArchive::from_unique_keys(&[], i32::MAX, 0).unwrap();
    assert!(unique.edit_field(&unique.fields()[1].id, I64(0)).is_err());
    assert_empty_profile_rejected(&[0; 4], SimulationPersistScalarProfile::UniqueIdsI64);
    assert_empty_profile_rejected(&[0; 8], SimulationPersistScalarProfile::CountedI32);
    assert_eq!(integer.used_count(), 1);
    assert_eq!(unique.allocation_count(), Some(i32::MAX));
}

fn assert_empty_profile_rejected(raw: &[u8], profile: SimulationPersistScalarProfile) {
    assert!(SimulationPersistScalarArchive::from_bytes(raw, profile, 0).is_err());
}

#[test]
fn caller_drop_returned_bytes_and_cloned_field_mutations_leave_owned_input_unchanged() {
    let mut bytes = vec![1, 0, 0, 0, 255, 255, 255, 255];
    let record = SimulationPersistScalarArchive::from_bytes(
        &bytes,
        SimulationPersistScalarProfile::CountedI32,
        1,
    )
    .unwrap();
    bytes.fill(0);
    drop(bytes);
    let mut exported = record.to_bytes();
    exported.fill(0x81);
    let mut field = record.fields()[1].clone();
    field.value = I32(0);
    assert_eq!(record.field(&field.id).unwrap().value, I32(-1));
    assert_eq!(record.to_bytes(), [1, 0, 0, 0, 255, 255, 255, 255]);
}

#[test]
fn public_scalar_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SimulationPersistScalarProfile>();
    assert_send_sync::<SimulationPersistScalarValue>();
    assert_send_sync::<SimulationPersistScalarRole>();
    assert_send_sync::<SimulationPersistScalarRevision>();
    assert_send_sync::<SimulationPersistScalarId>();
    assert_send_sync::<SimulationPersistScalarField>();
    assert_send_sync::<SimulationPersistScalarCompletion>();
    assert_send_sync::<SimulationPersistScalarCheckMode>();
    assert_send_sync::<SimulationPersistScalarDiagnostic>();
    assert_send_sync::<SimulationPersistScalarCoverage>();
    assert_send_sync::<SimulationPersistScalarArchive>();
    assert_send_sync::<SimulationPersistScalarEditCompletion>();
    assert_send_sync::<SimulationPersistScalarRelation>();
    assert_send_sync::<SimulationPersistScalarHistory>();
    assert_send_sync::<SimulationPersistScalarEdit>();
}

#[test]
fn concurrent_complete_scalar_edits_equal_serial_images_and_all_ordered_relations() {
    let source =
        SimulationPersistScalarArchive::from_unique_keys(&[-1, i64::MIN], i32::MAX, 2).unwrap();
    let serial = source
        .edit_field(&source.entry_id(0).unwrap(), I64(1 << 40))
        .unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (source, serial) = (&source, &serial);
            scope.spawn(move || repeat_scalar_edit(source, serial));
        }
    });
    assert_eq!(source.allocation_count(), Some(i32::MAX));
}

fn repeat_scalar_edit(
    source: &SimulationPersistScalarArchive,
    serial: &SimulationPersistScalarEdit,
) {
    for _ in 0..32 {
        let edit = source
            .edit_field(&source.entry_id(0).unwrap(), I64(1 << 40))
            .unwrap();
        assert_eq!(edit.archive.to_bytes(), serial.archive.to_bytes());
        assert_eq!(edit.coverage, serial.coverage);
        assert_scalar_history(&edit);
    }
}
