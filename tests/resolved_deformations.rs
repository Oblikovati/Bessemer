//! Complete native-bound deform records through the public document contract (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/deformations/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-0-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-0-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-0-3.bin"), 119),
    (include_bytes!("fixtures/deformations/owned-0-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-0-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-1-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-1-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-1-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-1-3.bin"), 119),
    (include_bytes!("fixtures/deformations/owned-1-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-1-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-2-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-2-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-2-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-2-3.bin"), 119),
    (include_bytes!("fixtures/deformations/owned-2-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-2-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-3-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-3-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-3-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-3-3.bin"), 119),
    (include_bytes!("fixtures/deformations/owned-3-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-3-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-4-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-4-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-4-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-4-3.bin"), 119),
    (include_bytes!("fixtures/deformations/owned-4-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-4-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-5-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-5-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-5-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-5-3.bin"), 119),
    (include_bytes!("fixtures/deformations/owned-5-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-5-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-6-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-6-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-6-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-6-3.bin"), 119),
    (include_bytes!("fixtures/deformations/owned-6-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-6-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-7-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-7-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-7-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-7-3.bin"), 119),
    (include_bytes!("fixtures/deformations/owned-7-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-7-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-8-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-8-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-8-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-8-3.bin"), 119),
    (include_bytes!("fixtures/deformations/owned-8-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-8-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-9-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-9-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-9-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-9-3.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-9-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-9-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-10-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-10-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-10-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-10-3.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-10-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-10-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-11-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-11-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-11-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-11-3.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-11-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-11-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-12-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-12-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-12-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-12-3.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-12-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-12-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-13-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-13-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-13-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-13-3.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-13-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-13-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-14-0.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-14-1.bin"), 111),
    (include_bytes!("fixtures/deformations/owned-14-2.bin"), 115),
    (include_bytes!("fixtures/deformations/owned-14-3.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-14-4.bin"), 107),
    (include_bytes!("fixtures/deformations/owned-14-5.bin"), 121),
    (include_bytes!("fixtures/deformations/owned-15-0.bin"), 99),
    (include_bytes!("fixtures/deformations/owned-15-1.bin"), 99),
    (include_bytes!("fixtures/deformations/owned-15-2.bin"), 99),
    (include_bytes!("fixtures/deformations/owned-15-3.bin"), 101),
    (include_bytes!("fixtures/deformations/owned-15-4.bin"), 99),
    (include_bytes!("fixtures/deformations/owned-15-5.bin"), 101),
    (
        include_bytes!("fixtures/deformations/owned-13-2-edited.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/deformations/owned-15-2-edited.bin"),
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
fn complete_documents_save_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn deform_edits_match_both_independent_goldens() {
    for (index, golden, path) in [(80, 96, "payload.raw_810"), (92, 97, "raw_f0")] {
        contract::check_word_edit_with_kernel(
            CASES[index].0,
            CASES[golden].0,
            &format!("root.features[0].{path}"),
            Some(kernel()),
        );
    }
}
#[test]
fn deform_wrong_typed_edits_preserve_the_source_document() {
    contract::check_rejected_edit(CASES[92].0, "root.features[0].raw_f0");
    contract::check_rejected_edit_with_kernel(
        CASES[80].0,
        "root.features[0].payload.raw_810",
        Some(kernel()),
    );
}
