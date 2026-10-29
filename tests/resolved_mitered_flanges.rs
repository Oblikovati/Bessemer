//! Complete mitered-flange documents and isolated word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/mitered-flanges/owned-0-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/mitered-flanges/owned-0-1.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/mitered-flanges/owned-0-2.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/mitered-flanges/owned-0-3.bin"),
        117,
    ),
    (
        include_bytes!("fixtures/mitered-flanges/owned-0-4.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/mitered-flanges/owned-0-5.bin"),
        117,
    ),
    (
        include_bytes!("fixtures/mitered-flanges/owned-0-2-edited.bin"),
        113,
    ),
];
#[test]
fn complete_mitered_documents_reconstruct_reopen_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, *slots);
    }
}
#[test]
fn independent_mitered_word_edit_reconstructs_exactly() {
    contract::check_word_edit(CASES[2].0, CASES[6].0, "root.features[0].payload.raw_978");
}
#[test]
fn rejected_mitered_edit_preserves_the_original() {
    contract::check_rejected_edit(CASES[2].0, "root.features[0].payload.raw_978");
}
