//! Separate final-LBC name list and explicit legacy-map framing contracts (#130).
use bessemer::simulation::*;
#[path = "simulation_lbc_final_streams/maps.rs"]
mod maps;

fn names() -> SimulationLbcNameOrder {
    SimulationLbcNameOrder::from_bytes(
        &[
            3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0x81, 0, 0,
        ],
        3,
        8,
    )
    .unwrap()
}

#[test]
fn no_leading_string_is_invented_and_null_empty_odd_names_keep_exact_positions() {
    let order = names();
    assert_eq!(order.count_header(), [3, 0, 0, 0]);
    assert_eq!(order.items().len(), 3);
    assert_eq!(order.items()[0].source_offsets(), (4, 8, 8));
    assert!(order.items()[0].is_null());
    assert_eq!(order.items()[1].utf16_units(), Some(vec![]));
    assert_eq!(order.items()[2].source_offsets(), (14, 18, 21));
    assert_eq!(order.items()[2].payload(), [0x81, 0, 0]);
    assert_eq!(order.items()[2].utf16_units(), None);
    assert_eq!(order.coverage().checked_text_frames, 3);
    let rebuilt = SimulationLbcNameOrder::from_items(order.items(), 3, 8).unwrap();
    assert_eq!(rebuilt.to_bytes(), order.to_bytes());
}

#[test]
fn duplicates_surrogates_and_complete_empty_list_keep_stored_bits() {
    let text = SimulationPersistText::from_units(Some(&[0xd800, 0, 0xdfff]), 8).unwrap();
    let order = SimulationLbcNameOrder::from_items(&[text.clone(), text], 2, 8).unwrap();
    assert_eq!(order.items()[0].to_bytes(), order.items()[1].to_bytes());
    assert_eq!(
        order.items()[0].utf16_units(),
        Some(vec![0xd800, 0, 0xdfff])
    );
    assert_ne!(order.item_id(0).unwrap(), order.item_id(1).unwrap());
    let empty = SimulationLbcNameOrder::from_bytes(&[0; 4], 0, 0).unwrap();
    assert!(empty.items().is_empty());
    assert_eq!(
        empty.coverage().completion,
        SimulationLbcFinalCompletion::CompletedEmpty
    );
    assert!(empty.item_id(0).is_err());
}

#[test]
fn negative_counts_all_prefixes_trailing_bytes_and_limits_fail_before_publication() {
    let bytes = names().to_bytes();
    for end in 0..bytes.len() {
        assert!(SimulationLbcNameOrder::from_bytes(&bytes[..end], 3, 8).is_err());
    }
    let mut extra = bytes.clone();
    extra.push(0);
    assert!(SimulationLbcNameOrder::from_bytes(&extra, 3, 8).is_err());
    assert!(SimulationLbcNameOrder::from_bytes(&(-1i32).to_le_bytes(), usize::MAX, 8).is_err());
    assert!(SimulationLbcNameOrder::from_bytes(&bytes, 2, 8).is_err());
    assert!(SimulationLbcNameOrder::from_bytes(&bytes, 3, 2).is_err());
    let order = names();
    assert!(SimulationLbcNameOrder::from_items(order.items(), 2, 8).is_err());
    assert!(SimulationLbcNameOrder::from_items(order.items(), 3, 2).is_err());
}

#[test]
fn name_edit_matches_literal_complete_image_and_all_original_parent_relations() {
    let source = names();
    let text = SimulationPersistText::from_units(Some(&[65]), 4).unwrap();
    let edit = source
        .edit_item(&source.item_id(0).unwrap(), &text)
        .unwrap();
    assert_eq!(
        edit.archive.to_bytes(),
        [
            3, 0, 0, 0, 4, 0, 0, 0, 65, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0x81, 0, 0
        ]
    );
    assert_eq!(edit.completion, SimulationLbcNameEditCompletion::Changed);
    assert_ne!(edit.archive.revision(), source.revision());
    assert_eq!(edit.source.to_bytes(), source.to_bytes());
    assert_eq!(edit.replacement.to_bytes(), text.to_bytes());
    assert_eq!(edit.archive.count_header(), source.count_header());
    verify_name_history(&edit);
}

fn verify_name_history(edit: &SimulationLbcNameEdit) {
    let changed = edit.completion == SimulationLbcNameEditCompletion::Changed;
    assert_eq!(edit.history.len(), edit.source.items().len());
    for (index, entry) in edit.history.iter().enumerate() {
        assert_eq!(entry.parent, edit.source.item_id(index).unwrap());
        assert_eq!(entry.result, edit.archive.item_id(index).unwrap());
        let parent = edit.source.item(&entry.parent).unwrap();
        let result = edit.archive.item(&entry.result).unwrap();
        let derived = changed && entry.parent == edit.parent;
        assert_eq!(
            entry.relation == SimulationLbcNameRelation::Derived,
            derived
        );
        let expected = if derived { &edit.replacement } else { parent };
        assert_eq!(result.to_bytes(), expected.to_bytes());
        if !changed {
            assert_eq!(entry.parent, entry.result);
        }
    }
}

#[test]
fn authentic_noop_retains_identity_and_foreign_ids_or_oversized_edits_leave_source_unchanged() {
    let source = names();
    let noop = source
        .edit_item(&source.item_id(2).unwrap(), &source.items()[2])
        .unwrap();
    assert_eq!(noop.completion, SimulationLbcNameEditCompletion::Unchanged);
    assert_eq!(noop.archive.revision(), source.revision());
    assert_eq!(noop.parent, noop.result);
    verify_name_history(&noop);
}

#[test]
fn invalid_name_edits_preserve_source_on_foreign_identity_and_limits() {
    let source = names();
    let foreign = names();
    assert!(source.item(&foreign.item_id(0).unwrap()).is_err());
    assert!(
        source
            .edit_item(&foreign.item_id(0).unwrap(), &source.items()[0])
            .is_err()
    );
    assert!(source.item_id(3).is_err());
    assert!(source.item_id(usize::MAX).is_err());
    let oversized = SimulationPersistText::from_units(Some(&[1, 2, 3, 4]), 10).unwrap();
    assert!(
        source
            .edit_item(&source.item_id(0).unwrap(), &oversized)
            .is_err()
    );
    assert_eq!(source.to_bytes(), names().to_bytes());
}

#[test]
fn all_names_share_one_owned_parent_and_survive_caller_drop_and_returned_mutation() {
    let mut bytes = names().to_bytes();
    let source = SimulationLbcNameOrder::from_bytes(&bytes, 3, 8).unwrap();
    bytes.fill(0xff);
    drop(bytes);
    for item in source.items() {
        assert_eq!(item.source_parent(), source.to_bytes());
        assert_eq!(
            item.source_parent().as_ptr(),
            source.items()[0].source_parent().as_ptr()
        );
    }
    let mut exported = source.to_bytes();
    exported.fill(0x81);
    assert_eq!(source.to_bytes(), names().to_bytes());
}

#[test]
fn public_name_types_are_send_sync_and_concurrent_complete_edits_match_serial() {
    assert_name_types();
    let source = names();
    let replacement = SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 8).unwrap();
    let serial = source
        .edit_item(&source.item_id(2).unwrap(), &replacement)
        .unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (source, replacement, serial) = (&source, &replacement, &serial);
            scope.spawn(move || repeat_name_edit(source, replacement, serial));
        }
    });
}

fn repeat_name_edit(
    source: &SimulationLbcNameOrder,
    replacement: &SimulationPersistText,
    serial: &SimulationLbcNameEdit,
) {
    for _ in 0..32 {
        let edit = source
            .edit_item(&source.item_id(2).unwrap(), replacement)
            .unwrap();
        assert_eq!(edit.archive.to_bytes(), serial.archive.to_bytes());
        assert_eq!(edit.coverage, serial.coverage);
        verify_name_history(&edit);
    }
}

fn assert_name_types() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<SimulationLbcFinalCompletion>();
    assert_send_sync::<SimulationLbcFinalCheckMode>();
    assert_send_sync::<SimulationLbcFinalDiagnostic>();
    assert_send_sync::<SimulationLbcFinalCoverage>();
    assert_send_sync::<SimulationLbcNameRevision>();
    assert_send_sync::<SimulationLbcNameId>();
    assert_send_sync::<SimulationLbcNameOrder>();
    assert_send_sync::<SimulationLbcNameEditCompletion>();
    assert_send_sync::<SimulationLbcNameRelation>();
    assert_send_sync::<SimulationLbcNameHistory>();
    assert_send_sync::<SimulationLbcNameEdit>();
}
#[path = "simulation_lbc_final_streams/corpus.rs"]
mod corpus;
