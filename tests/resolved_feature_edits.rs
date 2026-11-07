//! Complete move-face and bending bodies, independent edits and isolated inputs (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/feature-edits/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/feature-edits/owned-0-1.bin"), 115),
    (include_bytes!("fixtures/feature-edits/owned-0-2.bin"), 121),
    (include_bytes!("fixtures/feature-edits/owned-0-3.bin"), 117),
    (include_bytes!("fixtures/feature-edits/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/feature-edits/owned-0-5.bin"), 125),
    (include_bytes!("fixtures/feature-edits/owned-1-0.bin"), 103),
    (include_bytes!("fixtures/feature-edits/owned-1-1.bin"), 127),
    (include_bytes!("fixtures/feature-edits/owned-1-2.bin"), 129),
    (include_bytes!("fixtures/feature-edits/owned-1-3.bin"), 131),
    (include_bytes!("fixtures/feature-edits/owned-1-4.bin"), 103),
    (include_bytes!("fixtures/feature-edits/owned-1-5.bin"), 137),
    (include_bytes!("fixtures/feature-edits/owned-2-0.bin"), 99),
    (include_bytes!("fixtures/feature-edits/owned-2-1.bin"), 119),
    (include_bytes!("fixtures/feature-edits/owned-2-2.bin"), 117),
    (include_bytes!("fixtures/feature-edits/owned-2-3.bin"), 121),
    (include_bytes!("fixtures/feature-edits/owned-2-4.bin"), 99),
    (include_bytes!("fixtures/feature-edits/owned-2-5.bin"), 121),
    (
        include_bytes!("fixtures/feature-edits/owned-0-2-edited.bin"),
        121,
    ),
    (
        include_bytes!("fixtures/feature-edits/owned-1-2-edited.bin"),
        129,
    ),
    (
        include_bytes!("fixtures/feature-edits/owned-2-2-edited.bin"),
        117,
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
fn complete_feature_edit_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn independent_feature_edit_edits_reconstruct_exact_wire_bytes() {
    contract::check_word_edit_with_kernel(
        CASES[2].0,
        CASES[18].0,
        "root.features[0].items[0].payload.raw_800",
        Some(kernel()),
    );
    contract::check_word_edit_with_kernel(
        CASES[8].0,
        CASES[19].0,
        "root.features[0].items[0].data_810.packed_80_e8_ec",
        Some(kernel()),
    );
    contract::check_word_edit_with_kernel(
        CASES[14].0,
        CASES[20].0,
        "root.features[0].items[0].packed_80_e8_ec",
        Some(kernel()),
    );
}
#[test]
fn rejected_feature_edit_edits_preserve_the_original() {
    contract::check_rejected_edit(CASES[0].0, "root.features[0].payload.raw_800");
    contract::check_rejected_edit_with_kernel(
        CASES[2].0,
        "root.features[0].items[0].payload.raw_800",
        Some(kernel()),
    );
}
