use super::reference::LIMITS;
use bessemer::simulation::*;
use std::sync::Arc;

fn guid() -> SimulationGuidArchive {
    let parts = SimulationGuidParts {
        fields: [None, Some([].into()), Some([65].into()), Some([66].into())],
        tail: [].into(),
    };
    SimulationGuidArchive::from_parts(parts, LIMITS.maximum_guid_stream_bytes).unwrap()
}

#[test]
fn guid_profiles_reject_every_short_prefix_bad_counts_and_missing_terminators() {
    let raw = guid().to_bytes();
    assert_eq!(raw.len(), 26);
    for length in 0..raw.len() {
        assert!(
            SimulationGuidArchive::from_bytes(&raw[..length], LIMITS.maximum_guid_stream_bytes)
                .is_err()
        );
    }
    for length in [1_u32, 3, 0x100002, u32::MAX] {
        let mut changed = raw.clone();
        changed[..4].copy_from_slice(&length.to_le_bytes());
        assert!(
            SimulationGuidArchive::from_bytes(&changed, LIMITS.maximum_guid_stream_bytes).is_err()
        );
    }
    let mut bad_terminator = raw.clone();
    bad_terminator[9] = 1;
    assert!(
        SimulationGuidArchive::from_bytes(&bad_terminator, LIMITS.maximum_guid_stream_bytes)
            .is_err()
    );
    assert!(SimulationGuidArchive::from_bytes(&raw, raw.len() - 1).is_err());
}

#[test]
fn caller_nul_validation_precedes_gate_and_oversized_created_fields_fail() {
    assert!(
        SimulationGuidWriterProjection::from_caller_strings(
            [Some(&[0]), None, None, None],
            LIMITS.maximum_guid_stream_bytes
        )
        .is_err()
    );
    let oversize = vec![65; 0x80000];
    assert!(
        SimulationGuidWriterProjection::from_caller_strings(
            [Some(&oversize), None, Some(&[65]), Some(&[66])],
            LIMITS.maximum_guid_stream_bytes
        )
        .is_err()
    );
    let suppressed = SimulationGuidWriterProjection::from_caller_strings(
        [Some(&oversize), None, Some(&[]), Some(&[66])],
        0,
    )
    .unwrap();
    assert!(suppressed.stream().is_none());
    assert!(
        SimulationGuidWriterProjection::from_caller_strings(
            [None, None, Some(&[65]), Some(&[66])],
            16
        )
        .is_err()
    );
    assert!(
        guid()
            .edit_field(&guid().fields()[0].id, Some(&[65]))
            .is_err()
    );
}

#[test]
fn unknown_tail_stays_incomplete_and_stored_nuls_do_not_collapse_null_or_empty() {
    let mut parts = guid().parts();
    parts.fields[2] = Some([65, 0, 66, 0xd800].into());
    parts.tail = [1, 2, 3].into();
    let archive =
        SimulationGuidArchive::from_parts(parts, LIMITS.maximum_guid_stream_bytes).unwrap();
    assert_eq!(archive.fields()[0].stored_units(), None);
    assert_eq!(archive.fields()[1].stored_units(), Some([].as_slice()));
    assert_eq!(
        archive.fields()[2].caller_visible_units(),
        Some([65].as_slice())
    );
    assert_eq!(
        archive.fields()[2].stored_units(),
        Some([65, 0, 66, 0xd800].as_slice())
    );
    let edit = archive
        .edit_field(&archive.fields()[0].id, Some(&[]))
        .unwrap();
    assert_eq!(
        edit.coverage.completion,
        SimulationGuidStreamCompletion::IncompleteUnassignedTail
    );
    assert_eq!(edit.archive.tail(), [1, 2, 3]);
}

#[test]
fn stored_parts_and_edits_enforce_field_and_total_budgets_before_publication() {
    let original = guid();
    let oversize: Arc<[u16]> = vec![65; 0x80000].into();
    let mut parts = original.parts();
    parts.fields[0] = Some(oversize.clone());
    assert!(SimulationGuidArchive::from_parts(parts, LIMITS.maximum_guid_stream_bytes).is_err());
    assert!(
        original
            .edit_field(&original.fields()[0].id, Some(&oversize))
            .is_err()
    );
    let bounded =
        SimulationGuidArchive::from_bytes(&original.to_bytes(), original.to_bytes().len()).unwrap();
    assert!(
        bounded
            .edit_field(&bounded.fields()[0].id, Some(&[65]))
            .is_err()
    );
}

#[test]
fn scalars_reject_invented_envelopes_and_unbound_names_without_assigning_state_enums() {
    for name in ["TimeStampGUIDs", "Timestamp", "Other", ""] {
        assert!(SimulationScalarKind::from_stream_name(name).is_err());
    }
    for size in [0, 3, 5, 8] {
        assert!(
            SimulationScalarArchive::from_bytes(SimulationScalarKind::TimeStamp, &vec![0; size])
                .is_err()
        );
    }
    let state = SimulationScalarArchive::from_signed(SimulationScalarKind::RunState, -1);
    assert_eq!(state.pattern(), u32::MAX);
    assert_eq!(state.meaning(), SimulationScalarMeaning::Unassigned);
    assert!(!state.coverage().semantic_binding_complete);
    assert!(
        state
            .diagnostics()
            .contains(&SimulationScalarDiagnostic::RunStateEnumUnassigned)
    );
}

#[test]
fn stored_guid_snapshots_own_input_units_and_bytes_after_source_mutation() {
    let original = guid();
    let mut raw = original.to_bytes();
    let copied = SimulationGuidArchive::from_bytes(&raw, LIMITS.maximum_guid_stream_bytes).unwrap();
    raw.fill(0);
    assert_eq!(copied.to_bytes(), original.to_bytes());
    let mut units: Arc<[u16]> = [0xd800, 65].into();
    let parts = SimulationGuidParts {
        fields: [Some(units.clone()), None, None, None],
        tail: [].into(),
    };
    let archive =
        SimulationGuidArchive::from_parts(parts, LIMITS.maximum_guid_stream_bytes).unwrap();
    Arc::make_mut(&mut units).fill(0);
    assert_eq!(
        archive.fields()[0].stored_units(),
        Some([0xd800, 65].as_slice())
    );
}

#[test]
fn all_metadata_results_are_send_sync_and_concurrent_field_edits_are_deterministic() {
    fn check<T: Send + Sync>() {}
    check::<SimulationScalarKind>();
    check::<SimulationScalarMeaning>();
    check::<SimulationScalarId>();
    check::<SimulationScalarArchive>();
    check::<SimulationScalarHelperProjection>();
    check::<SimulationScalarCheckMode>();
    check::<SimulationScalarCoverage>();
    check::<SimulationScalarDiagnostic>();
    check::<SimulationScalarFidelity>();
    check::<SimulationScalarEdit>();
    check::<SimulationMetadataLimits>();
    check::<SimulationMetadataArchive>();
    check::<SimulationMetadataScalarEdit>();
    check::<SimulationMetadataHelperEdit>();
    check::<SimulationGuidParts>();
    check::<SimulationGuidRevision>();
    check::<SimulationGuidFieldId>();
    check::<SimulationGuidField>();
    check::<SimulationGuidStreamCompletion>();
    check::<SimulationGuidCheckMode>();
    check::<SimulationGuidCoverage>();
    check::<SimulationGuidArchive>();
    check::<SimulationGuidFieldHistory>();
    check::<SimulationGuidDiagnostic>();
    check::<SimulationGuidFidelity>();
    check::<SimulationGuidEdit>();
    check::<SimulationGuidWriterProjection>();
    check::<SimulationMetadataGuidEdit>();
    let archive = guid();
    std::thread::scope(|scope| {
        let tasks: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat_edit(&archive)))
            .collect();
        for task in tasks {
            task.join().unwrap();
        }
    });
}

fn repeat_edit(archive: &SimulationGuidArchive) {
    for _ in 0..32 {
        let edit = archive
            .edit_field(&archive.fields()[0].id, Some(&[0xd800, 0, 65]))
            .unwrap();
        let reverted = edit
            .archive
            .edit_field(&edit.history[0].result, None)
            .unwrap();
        assert_eq!(reverted.archive.to_bytes(), archive.to_bytes());
    }
}
