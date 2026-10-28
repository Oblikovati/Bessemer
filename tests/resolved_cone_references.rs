//! Exact cone streams and isolated raw-double edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::header::StoredDouble;
use bessemer::model_archives::ModelFieldValue;
const PATH: &str = "root.features[0].items[0].raw_100";
const EDIT: ModelFieldValue = ModelFieldValue::F64(StoredDouble::from_bits(0x4004000000000000));
const CASES: [(&[u8], usize); 48] = [
    (include_bytes!("fixtures/cone-references/owned-0-0.bin"), 99),
    (
        include_bytes!("fixtures/cone-references/owned-0-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/cone-references/owned-0-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cone-references/owned-0-3.bin"),
        103,
    ),
    (include_bytes!("fixtures/cone-references/owned-0-4.bin"), 99),
    (
        include_bytes!("fixtures/cone-references/owned-0-5.bin"),
        107,
    ),
    (include_bytes!("fixtures/cone-references/owned-1-0.bin"), 99),
    (
        include_bytes!("fixtures/cone-references/owned-1-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/cone-references/owned-1-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cone-references/owned-1-3.bin"),
        103,
    ),
    (include_bytes!("fixtures/cone-references/owned-1-4.bin"), 99),
    (
        include_bytes!("fixtures/cone-references/owned-1-5.bin"),
        107,
    ),
    (include_bytes!("fixtures/cone-references/owned-2-0.bin"), 99),
    (
        include_bytes!("fixtures/cone-references/owned-2-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/cone-references/owned-2-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cone-references/owned-2-3.bin"),
        103,
    ),
    (include_bytes!("fixtures/cone-references/owned-2-4.bin"), 99),
    (
        include_bytes!("fixtures/cone-references/owned-2-5.bin"),
        107,
    ),
    (include_bytes!("fixtures/cone-wrappers/owned-0-0.bin"), 105),
    (include_bytes!("fixtures/cone-wrappers/owned-0-1.bin"), 115),
    (include_bytes!("fixtures/cone-wrappers/owned-0-2.bin"), 105),
    (include_bytes!("fixtures/cone-wrappers/owned-0-3.bin"), 115),
    (include_bytes!("fixtures/cone-wrappers/owned-0-4.bin"), 105),
    (include_bytes!("fixtures/cone-wrappers/owned-0-5.bin"), 115),
    (include_bytes!("fixtures/cone-wrappers/owned-1-0.bin"), 107),
    (include_bytes!("fixtures/cone-wrappers/owned-1-1.bin"), 119),
    (include_bytes!("fixtures/cone-wrappers/owned-1-2.bin"), 107),
    (include_bytes!("fixtures/cone-wrappers/owned-1-3.bin"), 119),
    (include_bytes!("fixtures/cone-wrappers/owned-1-4.bin"), 107),
    (include_bytes!("fixtures/cone-wrappers/owned-1-5.bin"), 119),
    (include_bytes!("fixtures/cone-wrappers/owned-2-0.bin"), 105),
    (include_bytes!("fixtures/cone-wrappers/owned-2-1.bin"), 115),
    (include_bytes!("fixtures/cone-wrappers/owned-2-2.bin"), 105),
    (include_bytes!("fixtures/cone-wrappers/owned-2-3.bin"), 115),
    (include_bytes!("fixtures/cone-wrappers/owned-2-4.bin"), 105),
    (include_bytes!("fixtures/cone-wrappers/owned-2-5.bin"), 115),
    (include_bytes!("fixtures/cone-wrappers/owned-3-0.bin"), 107),
    (include_bytes!("fixtures/cone-wrappers/owned-3-1.bin"), 109),
    (include_bytes!("fixtures/cone-wrappers/owned-3-2.bin"), 119),
    (include_bytes!("fixtures/cone-wrappers/owned-3-3.bin"), 135),
    (include_bytes!("fixtures/cone-wrappers/owned-3-4.bin"), 103),
    (include_bytes!("fixtures/cone-wrappers/owned-3-5.bin"), 175),
    (include_bytes!("fixtures/cone-wrappers/owned-4-0.bin"), 101),
    (include_bytes!("fixtures/cone-wrappers/owned-4-1.bin"), 103),
    (include_bytes!("fixtures/cone-wrappers/owned-4-2.bin"), 101),
    (include_bytes!("fixtures/cone-wrappers/owned-4-3.bin"), 103),
    (include_bytes!("fixtures/cone-wrappers/owned-4-4.bin"), 101),
    (include_bytes!("fixtures/cone-wrappers/owned-4-5.bin"), 103),
];
const EDITED: [&[u8]; 3] = [
    include_bytes!("fixtures/cone-references/owned-0-2-edited.bin"),
    include_bytes!("fixtures/cone-references/owned-1-2-edited.bin"),
    include_bytes!("fixtures/cone-references/owned-2-2-edited.bin"),
];
#[test]
fn complete_cone_documents_and_generic_callers_reconstruct_and_remain_isolated() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn cone_double_edits_match_independent_goldens_and_preserve_inputs() {
    for (kind, edited) in EDITED.into_iter().enumerate() {
        contract::check_value_edit(CASES[kind * 6 + 2].0, edited, PATH, EDIT, None);
    }
}
#[test]
fn cone_rejected_double_edits_preserve_inputs() {
    contract::check_rejected_edit(CASES[2].0, PATH);
}
