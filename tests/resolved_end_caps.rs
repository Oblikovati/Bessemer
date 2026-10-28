//! Complete end-cap documents with independent scalar edit goldens (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [(&[u8], usize); 21] = [
    (include_bytes!("fixtures/end-caps/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/end-caps/owned-0-1.bin"), 107),
    (include_bytes!("fixtures/end-caps/owned-0-2.bin"), 113),
    (include_bytes!("fixtures/end-caps/owned-0-3.bin"), 111),
    (include_bytes!("fixtures/end-caps/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/end-caps/owned-0-5.bin"), 117),
    (include_bytes!("fixtures/end-caps/owned-1-0.bin"), 101),
    (include_bytes!("fixtures/end-caps/owned-1-1.bin"), 107),
    (include_bytes!("fixtures/end-caps/owned-1-2.bin"), 113),
    (include_bytes!("fixtures/end-caps/owned-1-3.bin"), 111),
    (include_bytes!("fixtures/end-caps/owned-1-4.bin"), 101),
    (include_bytes!("fixtures/end-caps/owned-1-5.bin"), 117),
    (include_bytes!("fixtures/end-caps/owned-2-0.bin"), 101),
    (include_bytes!("fixtures/end-caps/owned-2-1.bin"), 107),
    (include_bytes!("fixtures/end-caps/owned-2-2.bin"), 113),
    (include_bytes!("fixtures/end-caps/owned-2-3.bin"), 111),
    (include_bytes!("fixtures/end-caps/owned-2-4.bin"), 101),
    (include_bytes!("fixtures/end-caps/owned-2-5.bin"), 117),
    (
        include_bytes!("fixtures/end-caps/owned-0-2-edited.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/end-caps/owned-1-2-edited.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/end-caps/owned-2-2-edited.bin"),
        113,
    ),
];
const PATH: &str = "root.features[0].items[0].payload.raw_808";

#[test]
fn complete_end_cap_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn independent_end_cap_edits_preserve_shared_dimensions_and_inputs() {
    for kind in 0..3 {
        contract::check_word_edit(CASES[kind * 6 + 2].0, CASES[18 + kind].0, PATH);
    }
}

#[test]
fn rejected_end_cap_scalar_types_preserve_original_documents() {
    for kind in 0..3 {
        contract::check_rejected_edit(CASES[kind * 6 + 2].0, PATH);
    }
}
