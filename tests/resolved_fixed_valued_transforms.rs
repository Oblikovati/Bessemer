//! Complete fixed valued transforms and isolated raw-word edits (#86).
use bessemer::model_archives::ModelFieldValue;
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [&[u8]; 28] = [
    include_bytes!("fixtures/fixed-valued-transforms/owned-0-0.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-0-1.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-0-2.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-0-3.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-0-4.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-0-5.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-1-0.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-1-1.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-1-2.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-1-3.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-1-4.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-1-5.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-2-0.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-2-1.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-2-2.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-2-3.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-2-4.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-2-5.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-3-0.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-3-1.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-3-2.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-3-3.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-3-4.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-3-5.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-0-2-edited.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-1-2-edited.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-2-2-edited.bin"),
    include_bytes!("fixtures/fixed-valued-transforms/owned-3-2-edited.bin"),
];
const SLOTS: [usize; 28] = [
    99, 101, 107, 101, 101, 107, 99, 103, 111, 103, 103, 111, 99, 101, 107, 101, 101, 107, 99, 99,
    103, 99, 99, 103, 107, 111, 107, 103,
];
const PATH: &str = "root.features[0].items[0].plane_transform.words[0]";

#[test]
fn fixed_valued_documents_reconstruct_original_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn fixed_word_edits_match_independent_goldens_and_preserve_other_roots_and_inputs() {
    for kind in 0..4 {
        contract::check_value_edit(
            CASES[kind * 6 + 2],
            CASES[24 + kind],
            PATH,
            ModelFieldValue::U32(12345),
            None,
        );
    }
}
#[test]
fn rejected_fixed_word_types_preserve_original_archives_and_documents() {
    for kind in 0..4 {
        contract::check_rejected_edit(CASES[kind * 6 + 2], PATH);
    }
}
