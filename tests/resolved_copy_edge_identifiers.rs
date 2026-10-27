//! Complete native-bound copy, trim/extend and reference edge identifiers (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod identifier_contract;
use identifier_contract::{check_document, check_rejected_edit, check_word_edit};

const CASES: [&[u8]; 21] = [
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-0-0.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-0-1.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-0-2.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-0-3.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-0-4.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-0-5.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-1-0.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-1-1.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-1-2.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-1-3.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-1-4.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-1-5.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-2-0.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-2-1.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-2-2.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-2-3.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-2-4.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-2-5.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-0-2-edited.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-1-2-edited.bin"),
    include_bytes!("fixtures/copy-trim-edge-identifiers/owned-2-2-edited.bin"),
];
const SLOTS: [usize; 21] = [
    99, 101, 111, 123, 109, 131, 99, 101, 111, 123, 109, 131, 99, 101, 109, 119, 107, 127, 111,
    111, 109,
];
const PATHS: [&str; 3] = [
    "root.features[0].items[0].unknown_48",
    "root.features[0].items[0].unknown_48",
    "root.features[0].items[0].unknown_38",
];

#[test]
fn copy_edges_rebuild_all_owned_documents_without_changing_untouched_storage() {
    for (features, slots) in CASES.into_iter().zip(SLOTS) {
        check_document(features, slots);
    }
}

#[test]
fn copy_edges_edit_one_signed_word_preserving_equal_features_and_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        check_word_edit(CASES[kind * 6 + 2], CASES[18 + kind], path);
    }
}

#[test]
fn copy_edges_reject_word_type_changes_without_mutating_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        check_rejected_edit(CASES[kind * 6 + 2], path);
    }
}
