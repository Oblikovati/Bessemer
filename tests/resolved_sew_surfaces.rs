//! Complete sew surfaces, inherited empty keepers and isolated flag-word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod identifier_contract;
use identifier_contract::{check_document, check_rejected_edit, check_word_edit};

const CASES: [&[u8]; 13] = [
    include_bytes!("fixtures/sew-surfaces/owned-0-0.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-0-1.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-0-2.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-0-3.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-0-4.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-0-5.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-1-0.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-1-1.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-1-2.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-1-3.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-1-4.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-1-5.bin"),
    include_bytes!("fixtures/sew-surfaces/owned-0-2-edited.bin"),
];
const SLOTS: [usize; 13] = [
    101, 101, 111, 109, 113, 117, 99, 103, 103, 111, 109, 111, 111,
];
const WORD_PATH: &str = "root.features[0].items[0].payload.raw_flag_words[0]";

#[test]
fn all_sew_surface_and_keeper_documents_rebuild_without_changing_other_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        check_document(bytes, slots);
    }
}

#[test]
fn independent_sew_flag_edit_preserves_the_other_equal_surface() {
    check_word_edit(CASES[2], CASES[12], WORD_PATH);
}

#[test]
fn rejected_sew_flag_type_preserves_the_archive_and_document() {
    check_rejected_edit(CASES[2], WORD_PATH);
}
