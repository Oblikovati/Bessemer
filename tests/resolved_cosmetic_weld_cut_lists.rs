//! Complete cosmetic cut lists with independently preserved native member order (#54).
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
#[path = "common/resolved_identifier_contract.rs"]
mod contract;

const CASES: [&[u8]; 14] = [
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-0-0.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-0-1.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-0-2.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-0-3.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-0-4.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-0-5.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-1-0.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-1-1.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-1-2.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-1-3.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-1-4.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-1-5.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-0-2-edited.bin"),
    include_bytes!("fixtures/cosmetic-weld-cut-lists/owned-1-2-edited.bin"),
];
const SLOTS: [usize; 14] = [
    101, 101, 107, 105, 101, 113, 101, 101, 107, 105, 101, 113, 107, 107,
];
const PATH: &str = "root.features[0].items[0].raw_888";

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
fn owned_cosmetic_cut_list_documents_reconstruct_complete_original_storage() {
    for (index, (bytes, slots)) in CASES.into_iter().zip(SLOTS).enumerate() {
        if index < 6 || index == 12 {
            contract::check_document(bytes, slots);
        } else {
            contract::check_document_with_kernel(bytes, slots, Some(kernel()));
        }
    }
}

#[test]
fn independent_cosmetic_cut_list_edits_preserve_other_roots_shared_attributes_and_inputs() {
    contract::check_word_edit(CASES[2], CASES[12], PATH);
    contract::check_word_edit_with_kernel(CASES[8], CASES[13], PATH, Some(kernel()));
}

#[test]
fn rejected_cosmetic_cut_list_scalar_types_preserve_archives_and_documents() {
    contract::check_rejected_edit(CASES[2], PATH);
    contract::check_rejected_edit_with_kernel(CASES[8], PATH, Some(kernel()));
}
