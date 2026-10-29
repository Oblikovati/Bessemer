//! Complete sketch bitmap documents, raw planes and isolated edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/sketch-bitmaps/owned-0-0.bin"), 103),
    (include_bytes!("fixtures/sketch-bitmaps/owned-0-1.bin"), 107),
    (include_bytes!("fixtures/sketch-bitmaps/owned-0-2.bin"), 107),
    (include_bytes!("fixtures/sketch-bitmaps/owned-0-3.bin"), 107),
    (include_bytes!("fixtures/sketch-bitmaps/owned-0-4.bin"), 103),
    (include_bytes!("fixtures/sketch-bitmaps/owned-0-5.bin"), 107),
    (include_bytes!("fixtures/sketch-bitmaps/owned-1-0.bin"), 99),
    (include_bytes!("fixtures/sketch-bitmaps/owned-1-1.bin"), 103),
    (include_bytes!("fixtures/sketch-bitmaps/owned-1-2.bin"), 103),
    (include_bytes!("fixtures/sketch-bitmaps/owned-1-3.bin"), 103),
    (include_bytes!("fixtures/sketch-bitmaps/owned-1-4.bin"), 99),
    (include_bytes!("fixtures/sketch-bitmaps/owned-1-5.bin"), 103),
    (
        include_bytes!("fixtures/sketch-bitmaps/owned-0-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/sketch-bitmaps/owned-1-2-edited.bin"),
        103,
    ),
];
#[test]
fn complete_bitmap_documents_reconstruct_reopen_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, *slots);
    }
}
#[test]
fn independent_bitmap_word_edits_reconstruct_exactly() {
    for (original, edited, path) in [
        (2, 12, "root.features[0].data_360.raw_50"),
        (8, 13, "root.features[0].raw_50"),
    ] {
        contract::check_word_edit(CASES[original].0, CASES[edited].0, path);
    }
}
#[test]
fn rejected_bitmap_edits_preserve_the_original() {
    for (index, path) in [
        (2, "root.features[0].data_360.raw_50"),
        (8, "root.features[0].raw_50"),
    ] {
        contract::check_rejected_edit(CASES[index].0, path);
    }
}
