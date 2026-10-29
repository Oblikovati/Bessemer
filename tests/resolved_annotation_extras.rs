//! Complete annotation documents and independent edit goldens (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/annotation-extras/owned-0-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-0-1.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-0-2.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-0-3.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-0-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-0-5.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-1-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-1-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-1-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-1-3.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-1-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-1-5.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-2-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-2-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-2-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-2-3.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-2-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-2-5.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-3-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-3-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-3-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-3-3.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-3-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-3-5.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-4-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-4-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-4-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-4-3.bin"),
        129,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-4-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-4-5.bin"),
        129,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-5-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-5-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-5-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-5-3.bin"),
        129,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-5-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-5-5.bin"),
        129,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-6-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-6-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-6-2.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-6-3.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-6-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-6-5.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-7-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-7-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-7-2.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-7-3.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-7-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-7-5.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-8-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-8-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-8-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-8-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-8-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-8-5.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-9-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-9-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-9-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-9-3.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-9-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-9-5.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-0-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-1-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-2-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-3-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-4-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-5-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-6-2-edited.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-7-2-edited.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-8-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/annotation-extras/owned-9-2-edited.bin"),
        99,
    ),
];
fn kernel() -> CompressedKernelContext {
    CompressedKernelContext {
        base: BaseSchema::from_bytes(include_bytes!("fixtures/parasolid/base-schema.s_t")).unwrap(),
        maximum_expanded: 1024 * 1024,
        transmit_limits: TransmitLimits {
            maximum_records: 1000,
            maximum_field_elements: 10000,
        },
    }
}
#[test]
fn annotation_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn annotation_edits_match_all_ten_independent_goldens() {
    for (kind, path) in [
        (0, "base.raw_08"),
        (1, "base.raw_08"),
        (2, "pattern_position_130"),
        (3, "pattern_position_130"),
        (4, "dimension.word_8e0"),
        (5, "dimension.word_8e0"),
        (6, "sketch_dimension_id"),
        (7, "sketch_dimension_id"),
        (8, "array_08.items[0].raw_08"),
        (9, "raw_08"),
    ] {
        contract::check_word_edit_with_kernel(
            CASES[kind * 6 + 2].0,
            CASES[60 + kind].0,
            &format!("root.features[0].{path}"),
            Some(kernel()),
        );
    }
}
#[test]
fn annotation_wrong_typed_edits_preserve_original_documents() {
    contract::check_rejected_edit(CASES[56].0, "root.features[0].raw_08");
    contract::check_rejected_edit_with_kernel(
        CASES[2].0,
        "root.features[0].base.raw_08",
        Some(kernel()),
    );
}
