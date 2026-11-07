//! Exact surface identifier documents and isolated signed-word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const PATH: &str = "root.features[0].items[0].unknown_38";
const CASES: [(&[u8], usize); 14] = [
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-0-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-0-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-0-2.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-0-3.bin"),
        123,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-0-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-0-5.bin"),
        127,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-1-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-1-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-1-2.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-1-3.bin"),
        123,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-1-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-1-5.bin"),
        127,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-0-2-edited.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/surface-identifier-variants/owned-1-2-edited.bin"),
        109,
    ),
];
#[test]
fn complete_surface_identifier_documents_reconstruct_and_preserve_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn signed_identifier_edits_match_independent_goldens_and_preserve_neighbors() {
    for kind in 0..2 {
        contract::check_word_edit(CASES[kind * 6 + 2].0, CASES[12 + kind].0, PATH);
    }
}
#[test]
fn rejected_identifier_types_preserve_archives_and_documents() {
    for kind in 0..2 {
        contract::check_rejected_edit(CASES[kind * 6 + 2].0, PATH);
    }
}
