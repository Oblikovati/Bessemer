//! Independent attachment-data and parametric-point document reconstruction (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::header::StoredDouble;
use bessemer::model_archives::ModelFieldValue;
const CASES: [(&[u8], usize); 14] = [
    (
        include_bytes!("fixtures/attachment-records/owned-0-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-0-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-0-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-0-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-0-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-0-5.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-1-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-1-1.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-1-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-1-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-1-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-1-5.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-0-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/attachment-records/owned-1-2-edited.bin"),
        105,
    ),
];
#[test]
fn complete_attachment_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn attachment_scalar_edits_match_independent_goldens_and_preserve_inputs() {
    contract::check_value_edit(
        CASES[2].0,
        CASES[12].0,
        "root.features[0].items[0].raw_40",
        ModelFieldValue::F64(StoredDouble::from_bits(42f64.to_bits())),
        None,
    );
    contract::check_word_edit(
        CASES[8].0,
        CASES[13].0,
        "root.features[0].items[0].word_130",
    );
}
#[test]
fn rejected_attachment_scalar_types_preserve_original_documents() {
    contract::check_rejected_edit(CASES[2].0, "root.features[0].items[0].raw_40");
    contract::check_rejected_edit(CASES[8].0, "root.features[0].items[0].word_130");
}
