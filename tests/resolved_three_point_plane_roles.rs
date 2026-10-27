//! Three-point planes accept the reviewed complete point family (#46).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::{ModelFieldValue, StoredDouble};
const CASES: [&[u8]; 7] = [
    include_bytes!("fixtures/three-point-plane-roles/owned-0-0.bin"),
    include_bytes!("fixtures/three-point-plane-roles/owned-0-1.bin"),
    include_bytes!("fixtures/three-point-plane-roles/owned-0-2.bin"),
    include_bytes!("fixtures/three-point-plane-roles/owned-0-3.bin"),
    include_bytes!("fixtures/three-point-plane-roles/owned-0-4.bin"),
    include_bytes!("fixtures/three-point-plane-roles/owned-0-5.bin"),
    include_bytes!("fixtures/three-point-plane-roles/owned-0-2-edited.bin"),
];
const SLOTS: [usize; 7] = [99, 101, 103, 103, 109, 109, 103];
const POINT_X: &str = "root.features[0].items[0].point_a8[0]";

#[test]
fn owned_three_point_documents_reconstruct_complete_original_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn endpoint_coordinate_edits_retain_all_three_shared_occurrences_and_source_inputs() {
    contract::check_value_edit(
        CASES[2],
        CASES[6],
        POINT_X,
        ModelFieldValue::F64(StoredDouble::from_bits(123.5f64.to_bits())),
        None,
    );
}

#[test]
fn rejected_coordinate_types_preserve_archives_and_original_documents() {
    contract::check_rejected_edit(CASES[2], POINT_X);
}
