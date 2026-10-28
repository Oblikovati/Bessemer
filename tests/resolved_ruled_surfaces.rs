//! Complete ruled surfaces and independent edits with retained kernel frames (#54).
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
#[path = "common/resolved_identifier_contract.rs"]
mod contract;

const CASES: [&[u8]; 14] = [
    include_bytes!("fixtures/ruled-surfaces/owned-0-0.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-0-1.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-0-2.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-0-3.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-0-4.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-0-5.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-1-0.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-1-1.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-1-2.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-1-3.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-1-4.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-1-5.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-0-2-edited.bin"),
    include_bytes!("fixtures/ruled-surfaces/owned-1-2-edited.bin"),
];
const SLOTS: [usize; 14] = [
    101, 103, 115, 111, 109, 119, 103, 105, 115, 113, 111, 117, 115, 115,
];
const PATHS: [&str; 2] = [
    "root.features[0].items[0].direct_data_7b0.raw_98",
    "root.features[0].items[0].raw_98",
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
fn owned_ruled_documents_reconstruct_complete_original_storage() {
    for (index, (bytes, slots)) in CASES.into_iter().zip(SLOTS).enumerate() {
        if index != 4 && index != 10 {
            contract::check_document(bytes, slots);
        } else {
            contract::check_document_with_kernel(bytes, slots, Some(kernel()));
        }
    }
}

#[test]
fn independent_ruled_edits_preserve_other_roots_shared_arrays_and_inputs() {
    contract::check_word_edit(CASES[2], CASES[12], PATHS[0]);
    contract::check_word_edit_with_kernel(CASES[8], CASES[13], PATHS[1], Some(kernel()));
}

#[test]
fn rejected_ruled_scalar_types_preserve_archives_and_documents() {
    contract::check_rejected_edit(CASES[2], PATHS[0]);
    contract::check_rejected_edit_with_kernel(CASES[8], PATHS[1], Some(kernel()));
}
