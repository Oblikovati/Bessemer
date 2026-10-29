//! Complete sketch-bend documents and isolated word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/sketch-bends/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/sketch-bends/owned-0-1.bin"), 117),
    (include_bytes!("fixtures/sketch-bends/owned-0-2.bin"), 115),
    (include_bytes!("fixtures/sketch-bends/owned-0-3.bin"), 119),
    (include_bytes!("fixtures/sketch-bends/owned-0-4.bin"), 103),
    (include_bytes!("fixtures/sketch-bends/owned-0-5.bin"), 119),
    (
        include_bytes!("fixtures/sketch-bends/owned-0-2-edited.bin"),
        115,
    ),
];
#[test]
fn complete_sketch_bend_documents_reconstruct_reopen_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, *slots);
    }
}
#[test]
fn independent_sketch_bend_word_edit_reconstructs_exactly() {
    contract::check_word_edit(CASES[2].0, CASES[6].0, "root.features[0].payload.raw_e70");
}
#[test]
fn rejected_sketch_bend_edit_preserves_the_original() {
    contract::check_rejected_edit(CASES[2].0, "root.features[0].payload.raw_e70");
}
