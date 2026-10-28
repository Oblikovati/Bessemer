//! Complete thin extrusion/revolution documents and immutable owned edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [&[u8]; 35] = [
    include_bytes!("fixtures/thin-extrusions/owned-0-0.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-0-1.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-0-2.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-0-3.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-0-4.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-0-5.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-1-0.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-1-1.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-1-2.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-1-3.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-1-4.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-1-5.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-2-0.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-2-1.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-2-2.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-2-3.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-2-4.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-2-5.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-3-0.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-3-1.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-3-2.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-3-3.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-3-4.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-3-5.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-4-0.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-4-1.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-4-2.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-4-3.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-4-4.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-4-5.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-0-2-edited.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-1-2-edited.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-2-2-edited.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-3-2-edited.bin"),
    include_bytes!("fixtures/thin-extrusions/owned-4-2-edited.bin"),
];
const SLOTS: [usize; 35] = [
    99, 103, 107, 101, 105, 105, 103, 107, 113, 117, 121, 135, 103, 107, 113, 117, 121, 135, 103,
    107, 113, 117, 121, 135, 103, 107, 113, 115, 119, 131, 107, 113, 113, 113, 113,
];
const PATHS: [&str; 5] = [
    "root.features[0].items[0].raw_2c",
    "root.features[0].items[0].thin_specification_860.raw_2c",
    "root.features[0].items[0].thin_specification_860.raw_2c",
    "root.features[0].items[0].thin_specification_860.raw_2c",
    "root.features[0].items[0].thin_specification_8e8.raw_2c",
];

#[test]
fn all_owned_thin_documents_reconstruct_complete_original_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn independent_thin_edits_retain_shared_specifications_other_roots_and_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_word_edit(CASES[kind * 6 + 2], CASES[30 + kind], path);
    }
}

#[test]
fn rejected_thin_scalar_types_preserve_complete_archives_and_documents() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(CASES[kind * 6 + 2], path);
    }
}
