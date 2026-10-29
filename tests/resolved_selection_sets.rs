//! Complete selection-set documents and independent edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/selection-sets/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/selection-sets/owned-0-1.bin"), 111),
    (include_bytes!("fixtures/selection-sets/owned-0-2.bin"), 111),
    (include_bytes!("fixtures/selection-sets/owned-0-3.bin"), 111),
    (include_bytes!("fixtures/selection-sets/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/selection-sets/owned-0-5.bin"), 111),
    (include_bytes!("fixtures/selection-sets/owned-1-0.bin"), 101),
    (include_bytes!("fixtures/selection-sets/owned-1-1.bin"), 103),
    (include_bytes!("fixtures/selection-sets/owned-1-2.bin"), 103),
    (include_bytes!("fixtures/selection-sets/owned-1-3.bin"), 103),
    (include_bytes!("fixtures/selection-sets/owned-1-4.bin"), 101),
    (include_bytes!("fixtures/selection-sets/owned-1-5.bin"), 103),
    (
        include_bytes!("fixtures/selection-sets/owned-0-2-edited.bin"),
        111,
    ),
];
#[test]
fn complete_selection_documents_reconstruct_reopen_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, *slots);
    }
}
#[test]
fn independent_selection_word_edit_reconstructs_exactly() {
    contract::check_word_edit(CASES[2].0, CASES[12].0, "root.features[0].raw_40");
}
#[test]
fn rejected_selection_edit_preserves_the_original() {
    contract::check_rejected_edit(CASES[2].0, "root.features[0].raw_40");
}
