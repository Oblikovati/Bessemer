//! Independent direct manager-header framing, edit, isolation and concurrency checks.
use bessemer::simulation::*;
#[path = "simulation_manager_property/corpus.rs"]
mod corpus;

fn beam(version: i32) -> SimulationManagerPropertyProfile {
    SimulationManagerPropertyProfile::Beam(SimulationPersistVersion::new(version).unwrap())
}

fn modern() -> SimulationManagerProperty {
    SimulationManagerProperty::from_values(&[i32::MIN, -1, i32::MAX], beam(820006)).unwrap()
}

#[test]
fn exact_beam_thresholds_preserve_nonascending_runtime_order_and_signed_words() {
    for (version, offsets) in [
        (0, vec![0x14c]),
        (810023, vec![0x14c]),
        (810024, vec![0x14c, 0x168]),
        (810025, vec![0x14c, 0x168]),
        (820005, vec![0x14c, 0x168]),
        (820006, vec![0x14c, 0x168, 0x148]),
        (820007, vec![0x14c, 0x168, 0x148]),
        (i32::MAX, vec![0x14c, 0x168, 0x148]),
    ] {
        verify_profile(beam(version), &offsets);
    }
}

fn verify_profile(profile: SimulationManagerPropertyProfile, offsets: &[usize]) {
    let values = [i32::MIN, -1, i32::MAX];
    let source = SimulationManagerProperty::from_values(&values[..offsets.len()], profile).unwrap();
    assert_eq!(source.profile(), profile);
    for (ordinal, field) in source.fields().iter().enumerate() {
        assert_eq!(field.member.offset(), offsets[ordinal]);
        assert_eq!(field.stream_offset, ordinal * 4);
        assert_eq!(field.value, values[ordinal]);
        assert_eq!(source.field_id(ordinal).unwrap(), field.id);
        assert_eq!(source.field(&field.id).unwrap().value, values[ordinal]);
    }
    let bytes: Vec<u8> = values[..offsets.len()]
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect();
    assert_eq!(source.to_bytes(), bytes);
    verify_read_coverage(&source, offsets.len());
}

fn verify_read_coverage(source: &SimulationManagerProperty, count: usize) {
    assert_eq!(source.coverage().checked_bytes, count * 4);
    assert_eq!(source.coverage().checked_fields, count);
    assert_eq!(
        source.coverage().completion,
        SimulationManagerPropertyCompletion::CompletedFraming
    );
    assert_eq!(source.coverage().requested_pattern_error_bits, 0);
}

#[test]
fn mass_retains_absent_or_supplied_version_context_without_a_version_gate() {
    for version in [None, Some(0), Some(810024), Some(820006), Some(i32::MAX)] {
        let profile = SimulationManagerPropertyProfile::Mass(
            version.map(|value| SimulationPersistVersion::new(value).unwrap()),
        );
        verify_profile(profile, &[0x28]);
        let source = SimulationManagerProperty::from_bytes(&[0; 4], profile).unwrap();
        assert_eq!(source.fields()[0].value, 0);
        assert_eq!(
            source.coverage().check_mode,
            SimulationManagerPropertyCheckMode::ExactProfile
        );
    }
}

#[test]
fn all_missing_surplus_and_wrong_constructor_word_counts_are_typed_failures() {
    for (profile, count) in [
        (beam(0), 1),
        (beam(810024), 2),
        (beam(820006), 3),
        (SimulationManagerPropertyProfile::Mass(None), 1),
    ] {
        verify_invalid_extents(profile, count);
    }
    assert!(SimulationPersistVersion::new(-1).is_err());
}

fn verify_invalid_extents(profile: SimulationManagerPropertyProfile, count: usize) {
    for size in (0..=16).filter(|size| *size != count * 4) {
        let error = SimulationManagerProperty::from_bytes(&vec![0; size], profile).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains("expected") && error.cause.contains(&size.to_string()));
    }
    for size in (0..=4).filter(|size| *size != count) {
        assert!(SimulationManagerProperty::from_values(&vec![0; size], profile).is_err());
    }
}

#[test]
fn every_changed_and_authentic_noop_word_keeps_complete_bytes_and_ordered_history() {
    let source = modern();
    for field in source.fields() {
        for replacement in [field.value, i32::MIN, -1, 0, 1, i32::MAX] {
            let edit = source.edit_field(&field.id, replacement).unwrap();
            verify_edit(&edit);
        }
    }
}

fn verify_edit(edit: &SimulationManagerPropertyEdit) {
    let previous = edit.source.field(&edit.parent).unwrap();
    let mut expected = edit.source.to_bytes();
    expected[previous.stream_offset..previous.stream_offset + 4]
        .copy_from_slice(&edit.replacement.to_le_bytes());
    assert_eq!(edit.archive.to_bytes(), expected);
    assert_eq!(edit.archive.profile(), edit.source.profile());
    assert_eq!(
        edit.archive.field(&edit.result).unwrap().value,
        edit.replacement
    );
    let changed = previous.value != edit.replacement;
    assert_eq!(changed, edit.source.revision() != edit.archive.revision());
    verify_edit_coverage(edit, changed);
    assert_eq!(edit.history.len(), edit.source.fields().len());
    for (ordinal, entry) in edit.history.iter().enumerate() {
        verify_history(edit, ordinal, entry, changed);
    }
}

fn verify_edit_coverage(edit: &SimulationManagerPropertyEdit, changed: bool) {
    assert_eq!(
        edit.coverage.completion == SimulationManagerPropertyCompletion::Changed,
        changed
    );
    assert_eq!(
        edit.coverage.check_mode,
        SimulationManagerPropertyCheckMode::ExactProfileAndPreservation
    );
}

fn verify_history(
    edit: &SimulationManagerPropertyEdit,
    ordinal: usize,
    entry: &SimulationManagerPropertyHistory,
    changed: bool,
) {
    assert_eq!(entry.parent, edit.source.field_id(ordinal).unwrap());
    assert_eq!(entry.result, edit.archive.field_id(ordinal).unwrap());
    let parent = edit.source.field(&entry.parent).unwrap();
    let result = edit.archive.field(&entry.result).unwrap();
    assert_eq!(parent.member, result.member);
    assert_eq!(parent.stream_offset, result.stream_offset);
    let derived = changed && entry.parent == edit.parent;
    assert_eq!(
        entry.relation == SimulationManagerPropertyRelation::Derived,
        derived
    );
    assert_eq!(
        result.value,
        if derived {
            edit.replacement
        } else {
            parent.value
        }
    );
    assert!(changed || entry.parent == entry.result);
}

#[test]
fn foreign_and_out_of_range_ids_leave_complete_source_unchanged() {
    let source = modern();
    let before = source.to_bytes();
    let foreign = modern().field_id(0).unwrap();
    assert!(source.field(&foreign).is_err());
    assert!(source.edit_field(&foreign, 1).is_err());
    assert!(source.field_id(3).is_err());
    assert!(source.field_id(usize::MAX).is_err());
    assert_eq!(source.to_bytes(), before);
}

#[test]
fn caller_drop_and_returned_field_buffer_mutations_cannot_change_published_header() {
    let mut original = modern().to_bytes();
    let source = SimulationManagerProperty::from_bytes(&original, beam(820006)).unwrap();
    original.fill(0);
    drop(original);
    let mut exported = source.to_bytes();
    exported.fill(0x81);
    let mut field = source.fields()[0].clone();
    field.value = 0;
    field.member = SimulationManagerPropertyMember::Mass28;
    assert_eq!(source.field(&field.id).unwrap().value, i32::MIN);
    assert_eq!(source.to_bytes(), modern().to_bytes());
}

#[test]
fn public_manager_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SimulationManagerPropertyProfile>();
    assert_send_sync::<SimulationManagerPropertyMember>();
    assert_send_sync::<SimulationManagerPropertyRevision>();
    assert_send_sync::<SimulationManagerPropertyFieldId>();
    assert_send_sync::<SimulationManagerPropertyField>();
    assert_send_sync::<SimulationManagerPropertyCompletion>();
    assert_send_sync::<SimulationManagerPropertyCheckMode>();
    assert_send_sync::<SimulationManagerPropertyDiagnostic>();
    assert_send_sync::<SimulationManagerPropertyCoverage>();
    assert_send_sync::<SimulationManagerPropertyRelation>();
    assert_send_sync::<SimulationManagerPropertyHistory>();
    assert_send_sync::<SimulationManagerPropertyEdit>();
    assert_send_sync::<SimulationManagerProperty>();
}

#[test]
fn concurrent_complete_manager_edits_and_all_relations_equal_serial_results() {
    let source = modern();
    let serial = source.edit_field(&source.field_id(1).unwrap(), 0).unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (source, serial) = (&source, &serial);
            scope.spawn(move || repeat_edit(source, serial));
        }
    });
}

fn repeat_edit(source: &SimulationManagerProperty, serial: &SimulationManagerPropertyEdit) {
    for _ in 0..32 {
        let edit = source.edit_field(&source.field_id(1).unwrap(), 0).unwrap();
        assert_eq!(edit.archive.to_bytes(), serial.archive.to_bytes());
        assert_eq!(edit.coverage, serial.coverage);
        verify_edit(&edit);
        assert_eq!(source.to_bytes(), modern().to_bytes());
    }
}
