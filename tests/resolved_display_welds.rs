//! Complete display-weld documents and isolated signed-word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const PATH: &str = "root.features[0].items[0].payload.raw_5d8";
const CASES: [(&[u8], usize); 21] = [
    (include_bytes!("fixtures/display-welds/owned-0-0.bin"), 99),
    (include_bytes!("fixtures/display-welds/owned-0-1.bin"), 109),
    (include_bytes!("fixtures/display-welds/owned-0-2.bin"), 105),
    (include_bytes!("fixtures/display-welds/owned-0-3.bin"), 113),
    (include_bytes!("fixtures/display-welds/owned-0-4.bin"), 99),
    (include_bytes!("fixtures/display-welds/owned-0-5.bin"), 125),
    (include_bytes!("fixtures/display-welds/owned-1-0.bin"), 99),
    (include_bytes!("fixtures/display-welds/owned-1-1.bin"), 109),
    (include_bytes!("fixtures/display-welds/owned-1-2.bin"), 105),
    (include_bytes!("fixtures/display-welds/owned-1-3.bin"), 113),
    (include_bytes!("fixtures/display-welds/owned-1-4.bin"), 99),
    (include_bytes!("fixtures/display-welds/owned-1-5.bin"), 125),
    (include_bytes!("fixtures/display-welds/owned-2-0.bin"), 99),
    (include_bytes!("fixtures/display-welds/owned-2-1.bin"), 109),
    (include_bytes!("fixtures/display-welds/owned-2-2.bin"), 105),
    (include_bytes!("fixtures/display-welds/owned-2-3.bin"), 113),
    (include_bytes!("fixtures/display-welds/owned-2-4.bin"), 99),
    (include_bytes!("fixtures/display-welds/owned-2-5.bin"), 125),
    (
        include_bytes!("fixtures/display-welds/owned-0-2-edited.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/display-welds/owned-1-2-edited.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/display-welds/owned-2-2-edited.bin"),
        105,
    ),
];
#[test]
fn complete_display_weld_documents_reconstruct_and_preserve_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn weld_signed_word_edits_match_independent_goldens_and_preserve_inputs() {
    for kind in 0..3 {
        contract::check_word_edit(CASES[kind * 6 + 2].0, CASES[18 + kind].0, PATH);
    }
}
#[test]
fn rejected_weld_scalar_types_preserve_archives_and_documents() {
    for kind in 0..3 {
        contract::check_rejected_edit(CASES[kind * 6 + 2].0, PATH);
    }
}
