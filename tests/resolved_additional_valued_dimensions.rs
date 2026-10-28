//! Complete native-derived valued dimensions and isolated common-word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [&[u8]; 21] = [
    include_bytes!("fixtures/additional-valued-dimensions/owned-0-0.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-0-1.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-0-2.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-0-3.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-0-4.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-0-5.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-1-0.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-1-1.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-1-2.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-1-3.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-1-4.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-1-5.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-2-0.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-2-1.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-2-2.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-2-3.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-2-4.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-2-5.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-0-2-edited.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-1-2-edited.bin"),
    include_bytes!("fixtures/additional-valued-dimensions/owned-2-2-edited.bin"),
];
const SLOTS: [usize; 21] = [
    99, 101, 107, 103, 107, 105, 99, 101, 107, 103, 107, 105, 99, 103, 111, 103, 111, 107, 107,
    107, 111,
];
const PATH: &str = "root.features[0].items[0].dcm_type";

#[test]
fn additional_dimension_documents_reconstruct_original_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn dimension_edits_match_independent_goldens_and_preserve_other_roots_and_inputs() {
    for kind in 0..3 {
        contract::check_word_edit(CASES[kind * 6 + 2], CASES[18 + kind], PATH);
    }
}
#[test]
fn rejected_dimension_word_types_preserve_original_archives_and_documents() {
    for kind in 0..3 {
        contract::check_rejected_edit(CASES[kind * 6 + 2], PATH);
    }
}
