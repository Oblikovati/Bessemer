//! Complete weld symbol graphs and isolated signed/raw-double edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::header::StoredDouble;
use bessemer::model_archives::ModelFieldValue;
const PATHS: [&str; 4] = [
    "root.features[0].items[0].raw_118",
    "root.features[0].items[0].weld.raw_118",
    "root.features[0].items[0].raw_8c",
    "root.features[0].items[0].raw_110",
];
const CASES: [(&[u8], usize); 28] = [
    (include_bytes!("fixtures/weld-symbols/owned-0-0.bin"), 99),
    (include_bytes!("fixtures/weld-symbols/owned-0-1.bin"), 109),
    (include_bytes!("fixtures/weld-symbols/owned-0-2.bin"), 105),
    (include_bytes!("fixtures/weld-symbols/owned-0-3.bin"), 109),
    (include_bytes!("fixtures/weld-symbols/owned-0-4.bin"), 99),
    (include_bytes!("fixtures/weld-symbols/owned-0-5.bin"), 115),
    (include_bytes!("fixtures/weld-symbols/owned-1-0.bin"), 99),
    (include_bytes!("fixtures/weld-symbols/owned-1-1.bin"), 109),
    (include_bytes!("fixtures/weld-symbols/owned-1-2.bin"), 105),
    (include_bytes!("fixtures/weld-symbols/owned-1-3.bin"), 109),
    (include_bytes!("fixtures/weld-symbols/owned-1-4.bin"), 99),
    (include_bytes!("fixtures/weld-symbols/owned-1-5.bin"), 115),
    (include_bytes!("fixtures/weld-symbols/owned-2-0.bin"), 99),
    (include_bytes!("fixtures/weld-symbols/owned-2-1.bin"), 107),
    (include_bytes!("fixtures/weld-symbols/owned-2-2.bin"), 103),
    (include_bytes!("fixtures/weld-symbols/owned-2-3.bin"), 109),
    (include_bytes!("fixtures/weld-symbols/owned-2-4.bin"), 99),
    (include_bytes!("fixtures/weld-symbols/owned-2-5.bin"), 115),
    (include_bytes!("fixtures/weld-symbols/owned-3-0.bin"), 99),
    (include_bytes!("fixtures/weld-symbols/owned-3-1.bin"), 105),
    (include_bytes!("fixtures/weld-symbols/owned-3-2.bin"), 105),
    (include_bytes!("fixtures/weld-symbols/owned-3-3.bin"), 103),
    (include_bytes!("fixtures/weld-symbols/owned-3-4.bin"), 101),
    (include_bytes!("fixtures/weld-symbols/owned-3-5.bin"), 111),
    (
        include_bytes!("fixtures/weld-symbols/owned-0-2-edited.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/weld-symbols/owned-1-2-edited.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/weld-symbols/owned-2-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/weld-symbols/owned-3-2-edited.bin"),
        105,
    ),
];
#[test]
fn complete_symbol_documents_reconstruct_and_preserve_input_ownership() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn symbol_scalar_edits_match_independent_goldens_without_mutating_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate().take(3) {
        contract::check_word_edit(CASES[kind * 6 + 2].0, CASES[24 + kind].0, path);
    }
    contract::check_value_edit(
        CASES[20].0,
        CASES[27].0,
        PATHS[3],
        ModelFieldValue::F64(StoredDouble::from_bits(42f64.to_bits())),
        None,
    );
}
#[test]
fn rejected_symbol_scalar_types_preserve_original_archives_and_documents() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(CASES[kind * 6 + 2].0, path);
    }
}
