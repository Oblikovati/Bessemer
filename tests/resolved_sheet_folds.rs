//! Complete fold documents and independent scalar edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: [(&[u8], usize); 28] = [
    (include_bytes!("fixtures/sheet-folds/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/sheet-folds/owned-0-1.bin"), 113),
    (include_bytes!("fixtures/sheet-folds/owned-0-2.bin"), 109),
    (include_bytes!("fixtures/sheet-folds/owned-0-3.bin"), 115),
    (include_bytes!("fixtures/sheet-folds/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/sheet-folds/owned-0-5.bin"), 123),
    (include_bytes!("fixtures/sheet-folds/owned-1-0.bin"), 101),
    (include_bytes!("fixtures/sheet-folds/owned-1-1.bin"), 109),
    (include_bytes!("fixtures/sheet-folds/owned-1-2.bin"), 109),
    (include_bytes!("fixtures/sheet-folds/owned-1-3.bin"), 111),
    (include_bytes!("fixtures/sheet-folds/owned-1-4.bin"), 101),
    (include_bytes!("fixtures/sheet-folds/owned-1-5.bin"), 119),
    (include_bytes!("fixtures/sheet-folds/owned-2-0.bin"), 101),
    (include_bytes!("fixtures/sheet-folds/owned-2-1.bin"), 111),
    (include_bytes!("fixtures/sheet-folds/owned-2-2.bin"), 109),
    (include_bytes!("fixtures/sheet-folds/owned-2-3.bin"), 113),
    (include_bytes!("fixtures/sheet-folds/owned-2-4.bin"), 101),
    (include_bytes!("fixtures/sheet-folds/owned-2-5.bin"), 121),
    (include_bytes!("fixtures/sheet-folds/owned-3-0.bin"), 99),
    (include_bytes!("fixtures/sheet-folds/owned-3-1.bin"), 101),
    (include_bytes!("fixtures/sheet-folds/owned-3-2.bin"), 105),
    (include_bytes!("fixtures/sheet-folds/owned-3-3.bin"), 101),
    (include_bytes!("fixtures/sheet-folds/owned-3-4.bin"), 99),
    (include_bytes!("fixtures/sheet-folds/owned-3-5.bin"), 105),
    (
        include_bytes!("fixtures/sheet-folds/owned-0-2-edited.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/sheet-folds/owned-1-2-edited.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/sheet-folds/owned-2-2-edited.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/sheet-folds/owned-3-2-edited.bin"),
        105,
    ),
];
const PATHS: [&str; 4] = [
    "root.features[0].items[0].payload.raw_884",
    "root.features[0].items[0].payload.raw_838",
    "root.features[0].items[0].payload.raw_838",
    "root.features[0].items[0].entries[0].value",
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
fn complete_fold_documents_reconstruct_and_preserve_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, slots, Some(kernel()));
    }
}
#[test]
fn independent_fold_edits_match_frozen_goldens_and_preserve_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_word_edit_with_kernel(
            CASES[kind * 6 + 2].0,
            CASES[24 + kind].0,
            path,
            Some(kernel()),
        );
    }
}
#[test]
fn rejected_fold_scalar_types_preserve_original_documents() {
    contract::check_rejected_edit(CASES[2].0, PATHS[0]);
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit_with_kernel(CASES[kind * 6 + 2].0, path, Some(kernel()));
    }
}
