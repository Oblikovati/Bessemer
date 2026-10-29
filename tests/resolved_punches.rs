//! Complete punch revision documents and isolated terminal-tag edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/punches/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-0-1.bin"), 109),
    (include_bytes!("fixtures/punches/owned-0-2.bin"), 109),
    (include_bytes!("fixtures/punches/owned-0-3.bin"), 111),
    (include_bytes!("fixtures/punches/owned-0-4.bin"), 105),
    (include_bytes!("fixtures/punches/owned-0-5.bin"), 111),
    (include_bytes!("fixtures/punches/owned-1-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-1-1.bin"), 109),
    (include_bytes!("fixtures/punches/owned-1-2.bin"), 109),
    (include_bytes!("fixtures/punches/owned-1-3.bin"), 111),
    (include_bytes!("fixtures/punches/owned-1-4.bin"), 105),
    (include_bytes!("fixtures/punches/owned-1-5.bin"), 111),
    (include_bytes!("fixtures/punches/owned-2-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-2-1.bin"), 109),
    (include_bytes!("fixtures/punches/owned-2-2.bin"), 109),
    (include_bytes!("fixtures/punches/owned-2-3.bin"), 111),
    (include_bytes!("fixtures/punches/owned-2-4.bin"), 105),
    (include_bytes!("fixtures/punches/owned-2-5.bin"), 111),
    (include_bytes!("fixtures/punches/owned-3-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-3-1.bin"), 105),
    (include_bytes!("fixtures/punches/owned-3-2.bin"), 105),
    (include_bytes!("fixtures/punches/owned-3-3.bin"), 107),
    (include_bytes!("fixtures/punches/owned-3-4.bin"), 101),
    (include_bytes!("fixtures/punches/owned-3-5.bin"), 107),
    (include_bytes!("fixtures/punches/owned-4-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-4-1.bin"), 105),
    (include_bytes!("fixtures/punches/owned-4-2.bin"), 105),
    (include_bytes!("fixtures/punches/owned-4-3.bin"), 107),
    (include_bytes!("fixtures/punches/owned-4-4.bin"), 101),
    (include_bytes!("fixtures/punches/owned-4-5.bin"), 107),
    (include_bytes!("fixtures/punches/owned-5-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-5-1.bin"), 105),
    (include_bytes!("fixtures/punches/owned-5-2.bin"), 105),
    (include_bytes!("fixtures/punches/owned-5-3.bin"), 107),
    (include_bytes!("fixtures/punches/owned-5-4.bin"), 101),
    (include_bytes!("fixtures/punches/owned-5-5.bin"), 107),
    (include_bytes!("fixtures/punches/owned-6-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-6-1.bin"), 105),
    (include_bytes!("fixtures/punches/owned-6-2.bin"), 105),
    (include_bytes!("fixtures/punches/owned-6-3.bin"), 107),
    (include_bytes!("fixtures/punches/owned-6-4.bin"), 101),
    (include_bytes!("fixtures/punches/owned-6-5.bin"), 107),
    (include_bytes!("fixtures/punches/owned-7-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-7-1.bin"), 105),
    (include_bytes!("fixtures/punches/owned-7-2.bin"), 105),
    (include_bytes!("fixtures/punches/owned-7-3.bin"), 107),
    (include_bytes!("fixtures/punches/owned-7-4.bin"), 101),
    (include_bytes!("fixtures/punches/owned-7-5.bin"), 107),
    (include_bytes!("fixtures/punches/owned-8-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-8-1.bin"), 107),
    (include_bytes!("fixtures/punches/owned-8-2.bin"), 107),
    (include_bytes!("fixtures/punches/owned-8-3.bin"), 109),
    (include_bytes!("fixtures/punches/owned-8-4.bin"), 101),
    (include_bytes!("fixtures/punches/owned-8-5.bin"), 109),
    (include_bytes!("fixtures/punches/owned-9-0.bin"), 101),
    (include_bytes!("fixtures/punches/owned-9-1.bin"), 107),
    (include_bytes!("fixtures/punches/owned-9-2.bin"), 107),
    (include_bytes!("fixtures/punches/owned-9-3.bin"), 109),
    (include_bytes!("fixtures/punches/owned-9-4.bin"), 101),
    (include_bytes!("fixtures/punches/owned-9-5.bin"), 109),
    (include_bytes!("fixtures/punches/owned-0-2-edited.bin"), 109),
    (include_bytes!("fixtures/punches/owned-3-2-edited.bin"), 105),
    (include_bytes!("fixtures/punches/owned-9-2-edited.bin"), 107),
];
#[test]
fn complete_punch_documents_reconstruct_reopen_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, *slots);
    }
}
#[test]
fn independent_punch_terminal_edits_reconstruct_exactly() {
    for (original, edited) in [(2, 60), (20, 61), (56, 62)] {
        contract::check_word_edit(
            CASES[original].0,
            CASES[edited].0,
            "root.features[0].payload.terminal_tag",
        );
    }
}
#[test]
fn rejected_punch_edits_preserve_the_original() {
    for index in [2, 20, 56] {
        contract::check_rejected_edit(CASES[index].0, "root.features[0].payload.terminal_tag");
    }
}
