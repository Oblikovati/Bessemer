//! Complete native-bound deform records through the public document contract (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/deformations/callers/owned-0-0.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-0-1.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-0-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-0-3.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-0-4.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-0-5.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-1-0.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-1-1.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-1-2.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-1-3.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-1-4.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-1-5.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-2-0.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-2-1.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-2-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-2-3.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-2-4.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-2-5.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-3-0.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-3-1.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-3-2.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-3-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-3-4.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-3-5.bin"),
        175,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-4-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-4-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-4-2.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-4-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-4-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/deformations/callers/owned-4-5.bin"),
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
fn complete_documents_save_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn wrapper_wrong_typed_edits_preserve_the_source_document() {
    contract::check_rejected_edit(CASES[24].0, "root.features[0].reference.unknown_64");
    contract::check_rejected_edit_with_kernel(
        CASES[24].0,
        "root.features[0].reference.raw_f0",
        Some(kernel()),
    );
}
