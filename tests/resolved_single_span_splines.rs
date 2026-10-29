//! Exact-mode single-span splines through complete public sketch documents (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: &[&[u8]] = &[
    include_bytes!("fixtures/single-span-splines/owned-0.bin"),
    include_bytes!("fixtures/single-span-splines/owned-1.bin"),
    include_bytes!("fixtures/single-span-splines/owned-2.bin"),
    include_bytes!("fixtures/single-span-splines/owned-3.bin"),
    include_bytes!("fixtures/single-span-splines/owned-4.bin"),
    include_bytes!("fixtures/single-span-splines/owned-5.bin"),
    include_bytes!("fixtures/single-span-splines/owned-2-edited.bin"),
];
#[test]
fn complete_single_span_sketches_reconstruct_reopen_and_isolate_inputs() {
    for bytes in CASES {
        contract::check_document(bytes, 99);
    }
}
#[test]
fn single_span_public_edit_matches_independent_owned_sketch() {
    contract::check_word_edit(
        CASES[2],
        CASES[6],
        "root.features[0].splines[0].unknown_190",
    );
}
#[test]
fn single_span_wrong_typed_edit_preserves_the_source() {
    contract::check_rejected_edit(CASES[2], "root.features[0].splines[0].unknown_190");
}
