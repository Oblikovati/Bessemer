use super::*;
use bessemer::container_timestamps::*;
use bessemer::display_lists::DisplayEditCompletion;
mod replay;

fn packed_request(packed_override: u32) -> ZipTimestampRequest {
    ZipTimestampRequest {
        calendar_words: None,
        packed_override,
    }
}
fn source_for(row: &NativeTimestamp) -> ZipEntryTimestamps {
    ZipEntryTimestamps::from_bytes(NATIVE[row.fixture], NATIVE[row.fixture].len()).unwrap()
}
#[test]
fn packing_api_matches_all_22398_frozen_explicit_native_requests() {
    for row in requests() {
        let request = ZipTimestampRequest {
            calendar_words: row.calendar,
            packed_override: row.override_word,
        };
        assert_eq!(request.packed_word(), row.expected);
        assert_eq!(request.packed_word(), arithmetic_oracle(&row));
    }
}
#[test]
fn every_native_entry_view_matches_exact_frozen_header_intervals_and_limits() {
    for row in natives() {
        let source = source_for(&row);
        check_native_entry(&source, &row);
        assert_eq!(source.maximum_file_bytes(), NATIVE[row.fixture].len());
        assert!(
            ZipEntryTimestamps::from_bytes(source.as_bytes(), source.as_bytes().len() - 1).is_err()
        );
        assert!(source.entry_named("missing-stream").is_err());
        assert!(format!("{source:?}").len() < 150);
    }
}
fn check_native_entry(source: &ZipEntryTimestamps, row: &NativeTimestamp) {
    let entry = source.entry_named(&row.name).unwrap();
    assert_eq!(entry.name(), row.name);
    assert_eq!(entry.packed_word(), row.packed);
    assert_eq!(
        entry.timestamp_intervals(),
        ((row.local, row.local + 4), (row.central, row.central + 4))
    );
    assert_eq!(source.entry(&entry.id()).unwrap().id(), entry.id());
    assert_eq!(
        source.entries().nth(entry.id().ordinal()).unwrap().name(),
        row.name
    );
    let proof = source.coverage();
    assert_eq!(proof.mode, ZipTimestampCheckMode::ExactContainerHeaders);
    assert_eq!(proof.file_bytes, NATIVE[row.fixture].len());
    assert_eq!(proof.entries, source.entries().len());
    assert_eq!(proof.timestamp_copies, source.entries().len() * 2);
    assert!(!proof.payload_crc_checked);
    assert_eq!(proof.achieved_error, 0);
    assert_eq!(source.diagnostics().len(), 3);
}
#[test]
fn every_native_timestamp_pattern_edit_matches_literal_offsets_history_noops_and_fresh_inverses() {
    for row in natives() {
        let source = source_for(&row);
        for packed in raw_patterns().into_iter().chain([row.packed]) {
            check_native_edit(&source, &row, packed);
        }
        assert_eq!(source.as_bytes(), NATIVE[row.fixture]);
    }
}
fn check_native_edit(source: &ZipEntryTimestamps, row: &NativeTimestamp, packed: u32) {
    let entry = source.entry_named(&row.name).unwrap();
    let edit = source
        .replace_timestamp(&entry.id(), packed_request(packed))
        .unwrap();
    let expected = expected_wire(row, packed);
    assert_eq!(edit.result().as_bytes(), expected);
    assert_eq!(edit.source().as_bytes(), NATIVE[row.fixture]);
    assert_eq!(edit.source_entry(), entry.id());
    check_edit_metadata(&edit, packed);
    check_completion(&edit, packed != row.packed);
    check_history(&edit);
    check_inverse(&edit, row);
}
fn expected_wire(row: &NativeTimestamp, packed: u32) -> Vec<u8> {
    let mut expected = NATIVE[row.fixture].to_vec();
    expected[row.local..row.local + 4].copy_from_slice(&packed.to_le_bytes());
    expected[row.central..row.central + 4].copy_from_slice(&packed.to_le_bytes());
    expected
}
#[test]
fn every_frozen_request_reaches_both_actual_stored_timestamp_fields() {
    let row = natives().into_iter().find(|row| row.fixture == 2).unwrap();
    let source = source_for(&row);
    let id = source.entry_named(&row.name).unwrap().id();
    for frozen in requests() {
        let request = ZipTimestampRequest {
            calendar_words: frozen.calendar,
            packed_override: frozen.override_word,
        };
        let edit = source.replace_timestamp(&id, request).unwrap();
        assert_eq!(
            edit.result().as_bytes(),
            expected_wire(&row, frozen.expected)
        );
        check_history(&edit);
    }
}
fn check_edit_metadata(edit: &ZipTimestampEdit, packed: u32) {
    assert_eq!(
        edit.result()
            .entry(&edit.result_entry())
            .unwrap()
            .packed_word(),
        packed
    );
    assert_eq!(edit.coverage(), edit.result().coverage());
    assert_eq!(edit.diagnostics(), edit.result().diagnostics());
}
fn check_completion(edit: &ZipTimestampEdit, changed: bool) {
    assert_eq!(
        edit.completion(),
        if changed {
            DisplayEditCompletion::Changed
        } else {
            DisplayEditCompletion::Unchanged
        }
    );
    assert_eq!(edit.source_entry() == edit.result_entry(), !changed);
    assert_eq!(edit.result().entry(&edit.source_entry()).is_err(), changed);
    assert_eq!(edit.source().entry(&edit.result_entry()).is_err(), changed);
}
fn check_history(edit: &ZipTimestampEdit) {
    let mut position = 0;
    for span in edit.history() {
        assert_eq!(span.source_interval, span.result_interval);
        assert_eq!(span.source_interval.0, position);
        assert!(span.source_interval.1 > position);
        position = span.source_interval.1;
        let (start, end) = span.source_interval;
        let copied = edit.source().as_bytes()[start..end] == edit.result().as_bytes()[start..end];
        assert_eq!(span.relation == ZipTimestampByteRelation::Copied, copied);
    }
    assert_eq!(position, edit.source().as_bytes().len());
}
fn check_inverse(edit: &ZipTimestampEdit, row: &NativeTimestamp) {
    let fresh = ZipEntryTimestamps::from_bytes(
        edit.result().as_bytes(),
        edit.result().maximum_file_bytes(),
    )
    .unwrap();
    let id = fresh.entry_named(&row.name).unwrap().id();
    let inverse = fresh
        .replace_timestamp(&id, packed_request(row.packed))
        .unwrap();
    assert_eq!(inverse.result().as_bytes(), NATIVE[row.fixture]);
    assert!(Document::from_bytes(edit.result().as_bytes()).is_ok());
}
#[test]
fn timestamp_views_reject_foreign_ids_malformed_files_and_one_sided_timestamp_changes() {
    for row in natives() {
        let source = source_for(&row);
        let foreign = source_for(&row).entry_named(&row.name).unwrap().id();
        assert!(source.entry(&foreign).is_err());
        assert!(
            source
                .replace_timestamp(&foreign, packed_request(7))
                .is_err()
        );
        let mut raw = NATIVE[row.fixture].to_vec();
        raw[row.local] ^= 1;
        assert!(ZipEntryTimestamps::from_bytes(&raw, raw.len()).is_err());
        assert_eq!(source.as_bytes(), NATIVE[row.fixture]);
    }
    for length in 0..128 {
        assert!(ZipEntryTimestamps::from_bytes(&NATIVE[0][..length], NATIVE[0].len()).is_err());
    }
}
#[test]
fn snapshot_of_a_document_with_pending_payload_edits_preserves_its_current_checkpoint() {
    let mut document = Document::from_bytes(NATIVE[0]).unwrap();
    document
        .replace_stream("PreviewPNG", vec![1, 2, 3])
        .unwrap();
    let current = document.to_bytes().unwrap();
    let source = ZipEntryTimestamps::from_bytes(&current, current.len()).unwrap();
    let entry = source.entry_named("PreviewPNG").unwrap();
    let edit = source
        .replace_timestamp(&entry.id(), packed_request(!entry.packed_word()))
        .unwrap();
    let result = Document::from_bytes(edit.result().as_bytes()).unwrap();
    assert_eq!(result.read_stream("PreviewPNG").unwrap(), [1, 2, 3]);
    assert_eq!(edit.source().as_bytes(), current);
    check_later_document_mutation(&mut document, &source, &edit, &current);
    check_history(&edit);
}
fn check_later_document_mutation(
    document: &mut Document,
    source: &ZipEntryTimestamps,
    edit: &ZipTimestampEdit,
    current: &[u8],
) {
    assert_eq!(document.to_bytes().unwrap(), current);
    document.replace_stream("PreviewPNG", vec![9, 8]).unwrap();
    assert_ne!(document.to_bytes().unwrap(), current);
    assert_eq!(source.as_bytes(), current);
    assert_eq!(edit.source().as_bytes(), current);
    let result = Document::from_bytes(edit.result().as_bytes()).unwrap();
    assert_eq!(result.read_stream("PreviewPNG").unwrap(), [1, 2, 3]);
    let original = source.entry(&edit.source_entry()).unwrap().packed_word();
    let inverse = edit
        .result()
        .replace_timestamp(&edit.result_entry(), packed_request(original))
        .unwrap();
    assert_eq!(inverse.result().as_bytes(), current);
}
#[test]
fn published_timestamp_requests_views_identities_coverage_history_and_results_are_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<ZipTimestampRequest>();
    check::<ZipEntryTimestamps>();
    check::<ZipTimestampEntryId>();
    check::<ZipTimestampEntry<'static>>();
    check::<ZipTimestampCheckMode>();
    check::<ZipTimestampCoverage>();
    check::<ZipTimestampDiagnostic>();
    check::<ZipTimestampByteRelation>();
    check::<ZipTimestampByteHistory>();
    check::<ZipTimestampEdit>();
}
