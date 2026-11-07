//! Complete isocurve data, external entities and isolated signed-ID edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [&[u8]; 63] = [
    include_bytes!("fixtures/sketch-isocurve-information/owned-0-0.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-0-1.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-0-2.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-0-3.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-0-4.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-0-5.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-1-0.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-1-1.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-1-2.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-1-3.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-1-4.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-1-5.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-2-0.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-2-1.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-2-2.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-2-3.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-2-4.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-2-5.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-3-0.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-3-1.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-3-2.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-3-3.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-3-4.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-3-5.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-4-0.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-4-1.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-4-2.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-4-3.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-4-4.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-4-5.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-5-0.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-5-1.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-5-2.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-5-3.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-5-4.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-5-5.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-6-0.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-6-1.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-6-2.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-6-3.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-6-4.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-6-5.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-7-0.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-7-1.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-7-2.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-7-3.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-7-4.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-7-5.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-8-0.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-8-1.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-8-2.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-8-3.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-8-4.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-8-5.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-0-2-edited.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-1-2-edited.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-2-2-edited.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-3-2-edited.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-4-2-edited.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-5-2-edited.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-6-2-edited.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-7-2-edited.bin"),
    include_bytes!("fixtures/sketch-isocurve-information/owned-8-2-edited.bin"),
];
const SLOTS: [usize; 63] = [
    99, 101, 103, 103, 99, 107, 99, 101, 103, 103, 99, 107, 99, 101, 103, 103, 99, 107, 99, 101,
    103, 103, 99, 107, 99, 101, 103, 103, 99, 107, 99, 101, 103, 103, 99, 107, 99, 101, 103, 103,
    99, 107, 99, 101, 103, 103, 99, 107, 99, 101, 103, 103, 99, 107, 103, 103, 103, 103, 103, 103,
    103, 103, 103,
];
const PATH: &str = "root.features[0].items[0].dimension_id_20";

#[test]
fn isocurve_documents_reconstruct_complete_original_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn signed_id_edits_match_independent_goldens_and_preserve_neighbors_and_inputs() {
    for kind in 0..9 {
        contract::check_word_edit(CASES[kind * 6 + 2], CASES[54 + kind], PATH);
    }
}
#[test]
fn rejected_id_types_preserve_original_archives_and_documents() {
    for kind in 0..9 {
        contract::check_rejected_edit(CASES[kind * 6 + 2], PATH);
    }
}
