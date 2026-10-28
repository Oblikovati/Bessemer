//! Complete absolute-plane data and typed plane edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::header::StoredDouble;
use bessemer::model_archives::ModelFieldValue;
const CASES: [(&[u8], usize); 14] = [
    (
        include_bytes!("fixtures/absolute-plane-data/owned-0-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-0-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-0-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-0-3.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-0-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-0-5.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-1-0.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-1-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-1-2.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-1-3.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-1-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-1-5.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-0-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/absolute-plane-data/owned-1-2-edited.bin"),
        111,
    ),
];
const PATHS: [&str; 2] = ["root_point[0]", "plane_data.root_point[0]"];

#[test]
fn absolute_plane_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn absolute_plane_point_edits_match_independent_goldens() {
    for (kind, suffix) in PATHS.into_iter().enumerate() {
        contract::check_value_edit(
            CASES[kind * 6 + 2].0,
            CASES[12 + kind].0,
            &format!("root.features[0].items[0].{suffix}"),
            ModelFieldValue::F64(StoredDouble::from_bits(42f64.to_bits())),
            None,
        );
    }
}

#[test]
fn rejected_absolute_plane_point_types_preserve_inputs() {
    for (kind, suffix) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(
            CASES[kind * 6 + 2].0,
            &format!("root.features[0].items[0].{suffix}"),
        );
    }
}
