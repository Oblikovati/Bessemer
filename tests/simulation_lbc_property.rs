//! Explicit LBC Property profiles, exact pattern edits and ownership checks (#128).
use bessemer::simulation::*;
#[path = "simulation_lbc_property/corpus.rs"]
mod corpus;
#[path = "simulation_lbc_property/projections.rs"]
mod projections;

const BOUNDARIES: [(i32, usize); 18] = [
    (0, 52),
    (800027, 52),
    (800028, 56),
    (800059, 56),
    (800060, 60),
    (820009, 60),
    (820010, 64),
    (840003, 64),
    (840004, 192),
    (840012, 192),
    (840013, 204),
    (890619, 204),
    (890620, 208),
    (890999, 208),
    (891000, 212),
    (891100, 212),
    (891101, 216),
    (i32::MAX, 216),
];
const WRITER_ORDER: [u16; 54] = [
    0x438, 0x43c, 0x440, 0x46c, 0x478, 0x47c, 0x480, 0x494, 0x484, 0x488, 0x48c, 0x490, 0x508,
    0x4f8, 0x550, 0x498, 0x470, 0x474, 0x444, 0x448, 0x44c, 0x450, 0x454, 0x458, 0x45c, 0x460,
    0x464, 0x468, 0x49c, 0x4a0, 0x4a4, 0x4a8, 0x4ac, 0x4b0, 0x4b4, 0x4b8, 0x4bc, 0x4c0, 0x4c4,
    0x4c8, 0x4cc, 0x4d0, 0x4d4, 0x4d8, 0x4dc, 0x4e0, 0x4e4, 0x4e8, 0x4ec, 0x4f0, 0x4f4, 0x520,
    0x524, 0x528,
];

fn reader_profile(version: i32) -> SimulationLbcPropertyProfile {
    SimulationLbcPropertyProfile::VersionSelected(SimulationPersistVersion::new(version).unwrap())
}

fn property(version: i32, bytes: &[u8]) -> SimulationLbcProperty {
    SimulationLbcProperty::from_bytes(bytes, reader_profile(version)).unwrap()
}

#[test]
fn all_eight_thresholds_match_literal_sizes_and_complete_signed_patterns() {
    for (version, size) in BOUNDARIES {
        for pattern in [0, -1, i32::MIN, i32::MAX] {
            assert_profile_case(version, size, pattern);
        }
    }
}

fn assert_profile_case(version: i32, size: usize, pattern: i32) {
    let bytes = pattern.to_le_bytes().repeat(size / 4);
    let record = property(version, &bytes);
    assert_eq!(record.to_bytes(), bytes);
    assert_eq!(record.profile(), reader_profile(version));
    assert_eq!(record.fields().len(), size / 4);
    let values: Vec<_> = record.fields().iter().map(|field| field.value).collect();
    assert_eq!(values, vec![pattern; size / 4]);
    assert_eq!(
        SimulationLbcProperty::from_values(&values, record.profile())
            .unwrap()
            .to_bytes(),
        bytes
    );
    verify_profile_fields(&record, size, pattern);
}

fn verify_profile_fields(record: &SimulationLbcProperty, size: usize, pattern: i32) {
    for (index, field) in record.fields().iter().enumerate() {
        assert_eq!(field.stream_offset, index * 4);
        assert_eq!(
            record
                .field(&record.field_id(index).unwrap())
                .unwrap()
                .value,
            pattern
        );
    }
    assert_eq!(record.fields()[7].member.runtime_offset(), 0x494);
    assert_eq!(record.fields()[8].member.runtime_offset(), 0x484);
    assert_eq!(record.coverage().checked_bytes, size);
    assert_eq!(record.coverage().checked_fields, size / 4);
    assert_eq!(
        record.coverage().completion,
        SimulationLbcReadCompletion::CompletedFraming
    );
}

#[test]
fn fixed_writer_is_explicit_and_all_54_members_match_independent_literal_order() {
    let profile = SimulationLbcPropertyProfile::InstalledFixedWriter;
    let values: Vec<_> = (0..54).map(|index| i32::MIN + index).collect();
    let record = SimulationLbcProperty::from_values(&values, profile).unwrap();
    let actual = runtime_members(&record);
    assert_eq!(actual, WRITER_ORDER);
    assert_eq!(record.profile(), profile);
    assert_eq!(record.to_bytes().len(), 216);
    assert_eq!(
        runtime_members(&property(891101, &record.to_bytes())),
        WRITER_ORDER
    );
    assert!(SimulationLbcProperty::from_bytes(&record.to_bytes(), reader_profile(890620)).is_err());
    assert!(SimulationLbcProperty::from_values(&values[..53], profile).is_err());
}

fn runtime_members(record: &SimulationLbcProperty) -> Vec<u16> {
    record
        .fields()
        .iter()
        .map(|field| field.member.runtime_offset())
        .collect()
}

#[test]
fn every_truncated_prefix_extra_byte_and_wrong_version_shape_fails() {
    for (version, size) in BOUNDARIES {
        let bytes = vec![0x81; size + 1];
        for end in 0..size {
            assert!(
                SimulationLbcProperty::from_bytes(&bytes[..end], reader_profile(version)).is_err()
            );
        }
        assert!(SimulationLbcProperty::from_bytes(&bytes, reader_profile(version)).is_err());
    }
    assert!(SimulationLbcProperty::from_bytes(&[0; 204], reader_profile(890620)).is_err());
    assert_eq!(property(890619, &[0; 204]).to_bytes().len(), 204);
    assert!(SimulationPersistVersion::new(-1).is_err());
}

#[test]
fn all_field_edits_preserve_complete_original_bytes_and_profile() {
    for (version, size) in BOUNDARIES {
        let bytes: Vec<u8> = (0..size).map(|index| (index * 37) as u8).collect();
        let source = property(version, &bytes);
        for ordinal in 0..size / 4 {
            assert_field_edit(&source, ordinal, i32::MIN);
            assert_field_edit(&source, ordinal, source.fields()[ordinal].value);
        }
        assert_eq!(source.to_bytes(), bytes);
    }
}

fn assert_field_edit(source: &SimulationLbcProperty, ordinal: usize, replacement: i32) {
    let id = source.field_id(ordinal).unwrap();
    let edit = source.edit_field(&id, replacement).unwrap();
    let mut expected = source.to_bytes();
    expected[ordinal * 4..ordinal * 4 + 4].copy_from_slice(&replacement.to_le_bytes());
    assert_eq!(edit.archive.to_bytes(), expected);
    assert_eq!(edit.archive.profile(), source.profile());
    assert_eq!(edit.source.to_bytes(), source.to_bytes());
    assert_eq!(edit.replacement_value, replacement);
    assert_eq!(edit.parent, id);
    assert_eq!(edit.archive.field(&edit.result).unwrap().value, replacement);
    assert_eq!(
        edit.coverage.check_mode,
        SimulationLbcPropertyCheckMode::ExactSelectedProfileAndPreservation
    );
    verify_field_history(&edit, ordinal);
}

fn verify_field_history(edit: &SimulationLbcFieldEdit, ordinal: usize) {
    let changed = edit.archive.to_bytes() != edit.source.to_bytes();
    assert_eq!(
        edit.completion == SimulationLbcEditCompletion::Changed,
        changed
    );
    assert_eq!(edit.archive.revision() != edit.source.revision(), changed);
    assert_eq!(edit.history.len(), edit.source.fields().len());
    for (index, entry) in edit.history.iter().enumerate() {
        verify_field_relation(edit, entry, index, changed && index == ordinal, !changed);
    }
}

fn verify_field_relation(
    edit: &SimulationLbcFieldEdit,
    entry: &SimulationLbcFieldHistory,
    index: usize,
    derived: bool,
    unchanged: bool,
) {
    let parent = edit.source.field(&entry.parent).unwrap();
    let child = edit.archive.field(&entry.result).unwrap();
    assert_eq!(parent.member, child.member);
    assert_eq!(parent.stream_offset, index * 4);
    assert_eq!(parent.stream_offset, child.stream_offset);
    assert_eq!(
        entry.relation == SimulationLbcFieldRelation::Derived,
        derived
    );
    let expected = if derived {
        edit.replacement_value
    } else {
        parent.value
    };
    assert_eq!(child.value, expected);
    if unchanged {
        assert_eq!(entry.parent, entry.result);
    }
}

#[test]
fn normalized_ids_reject_foreign_images_out_of_bounds_and_fail_without_mutation() {
    let source = property(0, &[0; 52]);
    let foreign = property(0, &[0; 52]);
    let foreign_id = foreign.field_id(0).unwrap();
    assert!(source.field(&foreign_id).is_err());
    assert!(source.edit_field(&foreign_id, -1).is_err());
    assert!(source.field_id(13).is_err());
    assert!(source.field_id(usize::MAX).is_err());
    assert_eq!(source.to_bytes(), [0; 52]);
    assert_ne!(source.revision(), foreign.revision());
}

#[test]
fn caller_source_drop_and_returned_field_buffer_mutations_are_isolated() {
    let mut bytes = vec![0xff; 52];
    let source = property(0, &bytes);
    bytes.fill(0);
    drop(bytes);
    let edit = source.edit_field(&source.field_id(0).unwrap(), 0).unwrap();
    let mut exported = edit.archive.to_bytes();
    exported.fill(0x81);
    let mut copied_field = source.fields()[0].clone();
    copied_field.value = 0;
    assert_eq!(source.field(&copied_field.id).unwrap().value, -1);
    assert_eq!(source.to_bytes(), [0xff; 52]);
    assert_eq!(edit.archive.fields()[0].value, 0);
}

#[test]
fn public_lbc_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SimulationLbcPropertyProfile>();
    assert_send_sync::<SimulationLbcRuntimeMember>();
    assert_send_sync::<SimulationLbcPropertyRevision>();
    assert_send_sync::<SimulationLbcPropertyFieldId>();
    assert_send_sync::<SimulationLbcPropertyField>();
    assert_send_sync::<SimulationLbcPropertyCheckMode>();
    assert_send_sync::<SimulationLbcPropertyDiagnostic>();
    assert_send_sync::<SimulationLbcReadCompletion>();
    assert_send_sync::<SimulationLbcPropertyCoverage>();
    assert_send_sync::<SimulationLbcProperty>();
    assert_send_sync::<SimulationLbcEditCompletion>();
    assert_send_sync::<SimulationLbcFieldRelation>();
    assert_send_sync::<SimulationLbcFieldHistory>();
    assert_send_sync::<SimulationLbcFieldEdit>();
}

#[test]
fn eight_threads_repeat_complete_serial_edit_images_and_all_field_relations() {
    let source = property(891101, &[0xff; 216]);
    let serial = source
        .edit_field(&source.field_id(53).unwrap(), i32::MIN)
        .unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (source, serial) = (&source, &serial);
            scope.spawn(move || repeat_parallel_edit(source, serial));
        }
    });
    assert_eq!(source.to_bytes(), [0xff; 216]);
}

fn repeat_parallel_edit(source: &SimulationLbcProperty, serial: &SimulationLbcFieldEdit) {
    for _ in 0..32 {
        let edit = source
            .edit_field(&source.field_id(53).unwrap(), i32::MIN)
            .unwrap();
        assert_eq!(edit.archive.to_bytes(), serial.archive.to_bytes());
        assert_eq!(edit.coverage, serial.coverage);
        verify_field_history(&edit, 53);
    }
}
