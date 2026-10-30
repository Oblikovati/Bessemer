use bessemer::simulation::*;

fn source_order() -> SimulationStudyOrder {
    SimulationStudyOrder::from_bytes(&[0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 2, 8)
        .unwrap()
}

#[test]
fn isolated_item_edit_matches_literal_complete_file_and_all_history_parents() {
    let source = source_order();
    let replacement = SimulationPersistText::from_units(Some(&[65]), 4).unwrap();
    let edited = source
        .edit_text(&source.item_id(0).unwrap(), &replacement)
        .unwrap();
    assert_eq!(
        edited.archive.to_bytes(),
        [0, 0, 0, 0, 2, 0, 0, 0, 4, 0, 0, 0, 65, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(edited.completion, SimulationStudyEditCompletion::Changed);
    assert_ne!(source.revision(), edited.archive.revision());
    assert_eq!(edited.source.to_bytes(), source.to_bytes());
    assert_eq!(edited.replacement.to_bytes(), replacement.to_bytes());
    verify_edit_contract(&edited);
    verify_edit_identities(&edited);
}

fn verify_edit_contract(edited: &SimulationStudyTextEdit) {
    assert_eq!(
        edited.coverage.check_mode,
        SimulationPersistCheckMode::ExactStudyOrderAndPreservation
    );
    assert_eq!(edited.coverage.requested_pattern_error_bits, 0);
    assert_eq!(
        edited
            .history
            .iter()
            .map(|entry| entry.relation)
            .collect::<Vec<_>>(),
        [
            SimulationStudyTextRelation::Copied,
            SimulationStudyTextRelation::Derived,
            SimulationStudyTextRelation::Copied
        ]
    );
}

fn verify_edit_identities(edited: &SimulationStudyTextEdit) {
    verify_history(edited);
    assert_eq!(
        edited.archive.text(&edited.result).unwrap().to_bytes(),
        edited.replacement.to_bytes()
    );
    assert_eq!(
        edited.source.text(&edited.parent).unwrap().to_bytes(),
        [0, 0, 0, 0]
    );
    assert!(edited.source.text(&edited.result).is_err());
}

fn verify_history(edited: &SimulationStudyTextEdit) {
    assert_eq!(edited.history.len(), 1 + edited.source.items().len());
    for entry in edited.history.iter() {
        let parent = edited.source.text(&entry.parent).unwrap();
        let result = edited.archive.text(&entry.result).unwrap();
        let expected = if entry.relation == SimulationStudyTextRelation::Derived {
            &edited.replacement
        } else {
            parent
        };
        assert_eq!(result.to_bytes(), expected.to_bytes());
    }
}

#[test]
fn leading_edit_moves_count_but_preserves_items_and_raw_count_bytes() {
    let source = source_order();
    let replacement = SimulationPersistText::from_payload(Some(&[0x81, 0, 0]), 3).unwrap();
    let edited = source
        .edit_text(&source.leading_id(), &replacement)
        .unwrap();
    assert_eq!(
        edited.archive.to_bytes(),
        [3, 0, 0, 0, 0x81, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert_eq!(edited.archive.count_offset(), 7);
    assert_eq!(edited.archive.count_header(), source.count_header());
    assert_eq!(edited.archive.items()[0].source_offsets(), (11, 15, 15));
    verify_history(&edited);
}

#[test]
fn noop_nonzero_parent_failure_and_returned_buffer_mutations_leave_source_isolated() {
    let source = source_order();
    let original_bytes = source.to_bytes();
    let replacement = SimulationPersistText::read_from(&original_bytes, 8, 0).unwrap();
    let noop = source
        .edit_text(&source.item_id(0).unwrap(), &replacement)
        .unwrap();
    assert_eq!(noop.completion, SimulationStudyEditCompletion::Unchanged);
    assert_eq!(noop.archive.revision(), source.revision());
    assert_eq!(noop.parent, noop.result);
    assert!(noop.history.iter().all(|entry| entry.parent == entry.result
        && entry.relation == SimulationStudyTextRelation::Copied));
    let mut exported = noop.archive.to_bytes();
    exported.fill(0xff);
    drop(noop);
    assert_eq!(source.to_bytes(), original_bytes);
}

#[test]
fn invalid_edits_preserve_source_on_foreign_identity_and_payload_limit() {
    let source = source_order();
    let replacement = SimulationPersistText::from_payload(None, 0).unwrap();
    let foreign = source_order();
    assert!(
        source
            .edit_text(&foreign.leading_id(), &replacement)
            .is_err()
    );
    let oversized = SimulationPersistText::from_units(Some(&[1, 2, 3, 4]), 10).unwrap();
    assert!(source.edit_text(&source.leading_id(), &oversized).is_err());
    assert_eq!(source.to_bytes(), source_order().to_bytes());
}

#[test]
fn all_public_persistence_types_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SimulationPersistText>();
    assert_send_sync::<SimulationPersistCompletion>();
    assert_send_sync::<SimulationPersistCheckMode>();
    assert_send_sync::<SimulationPersistDiagnostic>();
    assert_send_sync::<SimulationPersistCoverage>();
    assert_send_sync::<SimulationPersistVersion>();
    assert_send_sync::<SimulationPersistReadMode>();
    assert_send_sync::<SimulationPersistOwnerSlot>();
    assert_send_sync::<SimulationPersistOwnerUpdate>();
    assert_send_sync::<SimulationPrimaryVersion>();
    assert_send_sync::<SimulationLegacyVersion>();
    assert_send_sync::<SimulationSelectedVersion>();
    assert_send_sync::<SimulationStudyOrderRevision>();
    assert_send_sync::<SimulationStudyTextId>();
    assert_send_sync::<SimulationStudyOrder>();
    assert_send_sync::<SimulationStudyEditCompletion>();
    assert_send_sync::<SimulationStudyTextRelation>();
    assert_send_sync::<SimulationStudyTextHistory>();
    assert_send_sync::<SimulationStudyTextEdit>();
}

#[test]
fn shared_source_concurrent_edits_match_complete_serial_bytes_and_relations() {
    let source = source_order();
    let replacement = SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 8).unwrap();
    let serial = source
        .edit_text(&source.item_id(1).unwrap(), &replacement)
        .unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (source, replacement, serial) = (&source, &replacement, &serial);
            scope.spawn(move || repeat_edit(source, replacement, serial));
        }
    });
    assert_eq!(source.to_bytes(), source_order().to_bytes());
}

fn repeat_edit(
    source: &SimulationStudyOrder,
    replacement: &SimulationPersistText,
    serial: &SimulationStudyTextEdit,
) {
    for _ in 0..32 {
        let edited = source
            .edit_text(&source.item_id(1).unwrap(), replacement)
            .unwrap();
        assert_eq!(edited.archive.to_bytes(), serial.archive.to_bytes());
        assert_eq!(edited.coverage, serial.coverage);
        assert_eq!(
            edited
                .history
                .iter()
                .map(|entry| entry.relation)
                .collect::<Vec<_>>(),
            serial
                .history
                .iter()
                .map(|entry| entry.relation)
                .collect::<Vec<_>>()
        );
        verify_history(&edited);
    }
}
