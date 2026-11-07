//! Complete extended surfaces and independently reconstructed edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/extend-surfaces/owned-0-0.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/extend-surfaces/owned-0-1.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/extend-surfaces/owned-0-2.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/extend-surfaces/owned-0-3.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/extend-surfaces/owned-0-4.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/extend-surfaces/owned-0-5.bin"),
        111,
    ),
    (include_bytes!("fixtures/extend-surfaces/owned-1-0.bin"), 99),
    (
        include_bytes!("fixtures/extend-surfaces/owned-1-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/extend-surfaces/owned-1-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/extend-surfaces/owned-1-3.bin"),
        101,
    ),
    (include_bytes!("fixtures/extend-surfaces/owned-1-4.bin"), 99),
    (
        include_bytes!("fixtures/extend-surfaces/owned-1-5.bin"),
        103,
    ),
    (include_bytes!("fixtures/extend-surfaces/owned-2-0.bin"), 99),
    (include_bytes!("fixtures/extend-surfaces/owned-2-1.bin"), 99),
    (include_bytes!("fixtures/extend-surfaces/owned-2-2.bin"), 99),
    (include_bytes!("fixtures/extend-surfaces/owned-2-3.bin"), 99),
    (include_bytes!("fixtures/extend-surfaces/owned-2-4.bin"), 99),
    (include_bytes!("fixtures/extend-surfaces/owned-2-5.bin"), 99),
    (
        include_bytes!("fixtures/extend-surfaces/owned-0-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/extend-surfaces/owned-1-2-edited.bin"),
        103,
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
fn complete_extension_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn independent_extension_word_edits_reconstruct_exactly() {
    for (original, edited, path) in [
        (2, 18, "root.features[0].extend_data_7b0.raw_method_10"),
        (8, 19, "root.features[0].raw_method_10"),
    ] {
        contract::check_word_edit_with_kernel(
            CASES[original].0,
            CASES[edited].0,
            path,
            Some(kernel()),
        );
    }
}
#[test]
fn rejected_extension_edits_preserve_the_original() {
    contract::check_rejected_edit(CASES[8].0, "root.features[0].raw_method_10");
    contract::check_rejected_edit_with_kernel(
        CASES[2].0,
        "root.features[0].extend_data_7b0.raw_method_10",
        Some(kernel()),
    );
}
