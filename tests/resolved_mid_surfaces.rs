//! Complete mid-surface bodies, independent edits and isolated inputs (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/mid-surfaces/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-0-1.bin"), 105),
    (include_bytes!("fixtures/mid-surfaces/owned-0-2.bin"), 105),
    (include_bytes!("fixtures/mid-surfaces/owned-0-3.bin"), 107),
    (include_bytes!("fixtures/mid-surfaces/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-0-5.bin"), 107),
    (include_bytes!("fixtures/mid-surfaces/owned-1-0.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-1-1.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-1-2.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-1-3.bin"), 107),
    (include_bytes!("fixtures/mid-surfaces/owned-1-4.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-1-5.bin"), 107),
    (include_bytes!("fixtures/mid-surfaces/owned-2-0.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-2-1.bin"), 107),
    (include_bytes!("fixtures/mid-surfaces/owned-2-2.bin"), 107),
    (include_bytes!("fixtures/mid-surfaces/owned-2-3.bin"), 113),
    (include_bytes!("fixtures/mid-surfaces/owned-2-4.bin"), 103),
    (include_bytes!("fixtures/mid-surfaces/owned-2-5.bin"), 113),
    (include_bytes!("fixtures/mid-surfaces/owned-3-0.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-3-1.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-3-2.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-3-3.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-3-4.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-3-5.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-4-0.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-4-1.bin"), 103),
    (include_bytes!("fixtures/mid-surfaces/owned-4-2.bin"), 103),
    (include_bytes!("fixtures/mid-surfaces/owned-4-3.bin"), 103),
    (include_bytes!("fixtures/mid-surfaces/owned-4-4.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-4-5.bin"), 103),
    (include_bytes!("fixtures/mid-surfaces/owned-5-0.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-5-1.bin"), 103),
    (include_bytes!("fixtures/mid-surfaces/owned-5-2.bin"), 103),
    (include_bytes!("fixtures/mid-surfaces/owned-5-3.bin"), 103),
    (include_bytes!("fixtures/mid-surfaces/owned-5-4.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-5-5.bin"), 103),
    (include_bytes!("fixtures/mid-surfaces/owned-6-0.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-6-1.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-6-2.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-6-3.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-6-4.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-6-5.bin"), 101),
    (include_bytes!("fixtures/mid-surfaces/owned-7-0.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-7-1.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-7-2.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-7-3.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-7-4.bin"), 99),
    (include_bytes!("fixtures/mid-surfaces/owned-7-5.bin"), 99),
    (
        include_bytes!("fixtures/mid-surfaces/owned-2-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/mid-surfaces/owned-3-2-edited.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/mid-surfaces/owned-7-2-edited.bin"),
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
fn complete_mid_surface_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn independent_mid_surface_edits_reconstruct_exact_wire_bytes() {
    contract::check_word_edit_with_kernel(
        CASES[14].0,
        CASES[48].0,
        "root.features[0].payload.raw_7c8",
        Some(kernel()),
    );
    contract::check_word_edit_with_kernel(
        CASES[20].0,
        CASES[49].0,
        "root.features[0].raw_10",
        Some(kernel()),
    );
    contract::check_word_edit_with_kernel(
        CASES[44].0,
        CASES[50].0,
        "root.features[0].unknown_48",
        Some(kernel()),
    );
}
#[test]
fn rejected_mid_surface_edits_preserve_the_original() {
    contract::check_rejected_edit(CASES[0].0, "root.features[0].payload.raw_800");
    contract::check_rejected_edit_with_kernel(
        CASES[14].0,
        "root.features[0].payload.raw_7c8",
        Some(kernel()),
    );
}
