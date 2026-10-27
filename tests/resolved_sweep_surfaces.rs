//! Complete sweep-reference surfaces, direct sweep data and edge helpers (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod identifier_contract;
use identifier_contract::{check_document, check_rejected_edit, check_word_edit};
const CASES: [&[u8]; 19] = [
    include_bytes!("fixtures/sweep-reference-surfaces/owned-0-0.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-0-1.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-0-2.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-0-3.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-0-4.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-0-5.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-1-0.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-1-1.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-1-2.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-1-3.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-1-4.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-1-5.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-2-0.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-2-1.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-2-2.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-2-3.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-2-4.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-2-5.bin"),
    include_bytes!("fixtures/sweep-reference-surfaces/owned-0-2-edited.bin"),
];
const SLOTS: [usize; 19] = [
    101, 105, 119, 117, 111, 131, 99, 99, 107, 107, 103, 111, 99, 99, 103, 99, 99, 103, 119,
];
const PATH: &str = "root.features[0].items[0].payload.data.raw_3b0";

#[test]
fn sweep_surface_documents_rebuild_exactly_preserving_untouched_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        check_document(bytes, slots);
    }
}

#[test]
fn isolated_sweep_data_word_edit_preserves_equal_surface_and_input_document() {
    check_word_edit(CASES[2], CASES[18], PATH);
}

#[test]
fn rejected_sweep_data_word_type_preserves_input_archive_and_document() {
    check_rejected_edit(CASES[2], PATH);
}
