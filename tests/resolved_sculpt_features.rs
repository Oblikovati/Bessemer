//! Complete sculpt-feature documents through the public archive contract (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/sculpt-features/owned-0-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/sculpt-features/owned-0-1.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/sculpt-features/owned-0-2.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/sculpt-features/owned-0-3.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/sculpt-features/owned-0-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/sculpt-features/owned-0-5.bin"),
        111,
    ),
    (include_bytes!("fixtures/sculpt-features/owned-1-0.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-1-1.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-1-2.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-1-3.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-1-4.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-1-5.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-2-0.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-2-1.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-2-2.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-2-3.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-2-4.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-2-5.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-3-0.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-3-1.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-3-2.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-3-3.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-3-4.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-3-5.bin"), 99),
    (include_bytes!("fixtures/sculpt-features/owned-4-0.bin"), 99),
    (
        include_bytes!("fixtures/sculpt-features/owned-4-1.bin"),
        129,
    ),
    (include_bytes!("fixtures/sculpt-features/owned-4-2.bin"), 99),
    (
        include_bytes!("fixtures/sculpt-features/owned-4-3.bin"),
        129,
    ),
    (include_bytes!("fixtures/sculpt-features/owned-4-4.bin"), 99),
    (
        include_bytes!("fixtures/sculpt-features/owned-4-5.bin"),
        129,
    ),
    (
        include_bytes!("fixtures/sculpt-features/owned-0-2-edited.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/sculpt-features/owned-1-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/sculpt-features/owned-2-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/sculpt-features/owned-3-2-edited.bin"),
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
fn sculpt_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[18].0, CASES[18].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn independent_sculpt_edits_match_owned_goldens() {
    for (index, edited, path) in [
        (2, 30, "payload.raw_c0"),
        (8, 31, "unknown_48"),
        (14, 32, "unknown_48"),
        (20, 33, "raw_08"),
    ] {
        contract::check_word_edit_with_kernel(
            CASES[index].0,
            CASES[edited].0,
            &format!("root.features[0].{path}"),
            Some(kernel()),
        );
    }
}
#[test]
fn sculpt_wrong_typed_edits_preserve_inputs() {
    contract::check_rejected_edit(CASES[20].0, "root.features[0].raw_08");
    contract::check_rejected_edit_with_kernel(
        CASES[2].0,
        "root.features[0].payload.raw_c0",
        Some(kernel()),
    );
}
