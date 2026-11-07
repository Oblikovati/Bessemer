//! Complete feature-state documents with independent signed-word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [(&[u8], usize); 7] = [
    (include_bytes!("fixtures/feature-states/owned-0-0.bin"), 99),
    (include_bytes!("fixtures/feature-states/owned-0-1.bin"), 99),
    (include_bytes!("fixtures/feature-states/owned-0-2.bin"), 103),
    (include_bytes!("fixtures/feature-states/owned-0-3.bin"), 99),
    (include_bytes!("fixtures/feature-states/owned-0-4.bin"), 99),
    (include_bytes!("fixtures/feature-states/owned-0-5.bin"), 103),
    (
        include_bytes!("fixtures/feature-states/owned-0-2-edited.bin"),
        103,
    ),
];
const PATH: &str = "root.features[0].items[0].raw_08";
#[test]
fn complete_feature_state_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn independent_feature_state_edit_matches_frozen_bytes_and_preserves_inputs() {
    contract::check_word_edit(CASES[2].0, CASES[6].0, PATH);
}
#[test]
fn rejected_feature_state_scalar_types_preserve_originals() {
    contract::check_rejected_edit(CASES[2].0, PATH);
}
