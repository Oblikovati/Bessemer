//! Complete deleted-face documents and independent signed-word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [&[u8]; 14] = [
    include_bytes!("fixtures/deleted-faces/owned-0-0.bin"),
    include_bytes!("fixtures/deleted-faces/owned-0-1.bin"),
    include_bytes!("fixtures/deleted-faces/owned-0-2.bin"),
    include_bytes!("fixtures/deleted-faces/owned-0-3.bin"),
    include_bytes!("fixtures/deleted-faces/owned-0-4.bin"),
    include_bytes!("fixtures/deleted-faces/owned-0-5.bin"),
    include_bytes!("fixtures/deleted-faces/owned-1-0.bin"),
    include_bytes!("fixtures/deleted-faces/owned-1-1.bin"),
    include_bytes!("fixtures/deleted-faces/owned-1-2.bin"),
    include_bytes!("fixtures/deleted-faces/owned-1-3.bin"),
    include_bytes!("fixtures/deleted-faces/owned-1-4.bin"),
    include_bytes!("fixtures/deleted-faces/owned-1-5.bin"),
    include_bytes!("fixtures/deleted-faces/owned-0-2-edited.bin"),
    include_bytes!("fixtures/deleted-faces/owned-1-2-edited.bin"),
];
const SLOTS: [usize; 14] = [
    101, 105, 111, 107, 105, 115, 99, 101, 105, 101, 101, 105, 111, 105,
];
const PATHS: [&str; 2] = [
    "root.features[0].items[0].payload.raw_840",
    "root.features[0].items[0].raw_20",
];

#[test]
fn owned_deleted_face_documents_reconstruct_complete_original_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn independent_deleted_face_edits_preserve_shared_targets_other_roots_and_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_word_edit(CASES[kind * 6 + 2], CASES[12 + kind], path);
    }
}

#[test]
fn rejected_deleted_face_scalar_types_preserve_archives_and_documents() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(CASES[kind * 6 + 2], path);
    }
}
