//! Complete stock documents and isolated scalar edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/stock-features/owned-0-0.bin"), 103),
    (include_bytes!("fixtures/stock-features/owned-0-1.bin"), 111),
    (include_bytes!("fixtures/stock-features/owned-0-2.bin"), 107),
    (include_bytes!("fixtures/stock-features/owned-0-3.bin"), 111),
    (include_bytes!("fixtures/stock-features/owned-0-4.bin"), 105),
    (include_bytes!("fixtures/stock-features/owned-0-5.bin"), 111),
    (include_bytes!("fixtures/stock-features/owned-1-0.bin"), 103),
    (include_bytes!("fixtures/stock-features/owned-1-1.bin"), 113),
    (include_bytes!("fixtures/stock-features/owned-1-2.bin"), 107),
    (include_bytes!("fixtures/stock-features/owned-1-3.bin"), 113),
    (include_bytes!("fixtures/stock-features/owned-1-4.bin"), 105),
    (include_bytes!("fixtures/stock-features/owned-1-5.bin"), 113),
    (include_bytes!("fixtures/stock-features/owned-2-0.bin"), 99),
    (include_bytes!("fixtures/stock-features/owned-2-1.bin"), 103),
    (include_bytes!("fixtures/stock-features/owned-2-2.bin"), 103),
    (include_bytes!("fixtures/stock-features/owned-2-3.bin"), 103),
    (include_bytes!("fixtures/stock-features/owned-2-4.bin"), 101),
    (include_bytes!("fixtures/stock-features/owned-2-5.bin"), 103),
    (include_bytes!("fixtures/stock-features/owned-3-0.bin"), 99),
    (include_bytes!("fixtures/stock-features/owned-3-1.bin"), 99),
    (include_bytes!("fixtures/stock-features/owned-3-2.bin"), 99),
    (include_bytes!("fixtures/stock-features/owned-3-3.bin"), 99),
    (include_bytes!("fixtures/stock-features/owned-3-4.bin"), 99),
    (include_bytes!("fixtures/stock-features/owned-3-5.bin"), 99),
    (
        include_bytes!("fixtures/stock-features/owned-0-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/stock-features/owned-1-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/stock-features/owned-2-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/stock-features/owned-3-2-edited.bin"),
        99,
    ),
];
const PATHS: [&str; 4] = [
    "root.features[0].raw_96c",
    "root.features[0].raw_96c",
    "root.features[0].raw_a8",
    "root.features[0].raw_f4",
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
fn complete_stock_documents_reconstruct_and_preserve_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn stock_edits_match_independent_goldens_and_preserve_inputs() {
    for (kind, path) in PATHS.iter().enumerate() {
        contract::check_word_edit_with_kernel(
            CASES[kind * 6 + 2].0,
            CASES[24 + kind].0,
            path,
            Some(kernel()),
        );
    }
}
#[test]
fn rejected_stock_edits_preserve_archives_and_documents() {
    contract::check_rejected_edit(CASES[0].0, PATHS[0]);
    for (kind, path) in PATHS.iter().enumerate() {
        contract::check_rejected_edit_with_kernel(CASES[kind * 6 + 2].0, path, Some(kernel()));
    }
}

const WRAPPER_CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-0-0.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-0-1.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-0-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-0-3.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-0-4.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-0-5.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-1-0.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-1-1.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-1-2.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-1-3.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-1-4.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-1-5.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-2-0.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-2-1.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-2-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-2-3.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-2-4.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-2-5.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-3-0.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-3-1.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-3-2.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-3-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-3-4.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-3-5.bin"),
        175,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-4-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-4-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-4-2.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-4-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-4-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/body-reference-wrappers/owned-4-5.bin"),
        103,
    ),
];
#[test]
fn complete_body_reference_wrapper_callers_reconstruct_and_preserve_inputs() {
    for (bytes, slots) in WRAPPER_CASES {
        contract::check_document(bytes, *slots);
    }
}
