//! Complete PLine silhouette representations with direct source IDs and isolated edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [&[u8]; 7] = [
    include_bytes!("fixtures/silhouette-edge-identifiers/owned-0-0.bin"),
    include_bytes!("fixtures/silhouette-edge-identifiers/owned-0-1.bin"),
    include_bytes!("fixtures/silhouette-edge-identifiers/owned-0-2.bin"),
    include_bytes!("fixtures/silhouette-edge-identifiers/owned-0-3.bin"),
    include_bytes!("fixtures/silhouette-edge-identifiers/owned-0-4.bin"),
    include_bytes!("fixtures/silhouette-edge-identifiers/owned-0-5.bin"),
    include_bytes!("fixtures/silhouette-edge-identifiers/owned-0-2-edited.bin"),
];
const SLOTS: [usize; 7] = [99, 101, 111, 123, 109, 131, 111];
const PATH: &str = "root.features[0].items[0].unknown_48";

#[test]
fn silhouette_documents_rebuild_exactly_with_shared_sources_and_unchanged_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn silhouette_signed_word_edit_preserves_second_word_equal_root_and_source_inputs() {
    contract::check_word_edit(CASES[2], CASES[6], PATH);
}

#[test]
fn silhouette_word_type_rejection_preserves_complete_archive_and_document() {
    contract::check_rejected_edit(CASES[2], PATH);
}
