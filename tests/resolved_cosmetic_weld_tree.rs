//! Complete cosmetic weld tree reconstruction and isolated scalar edits (#54).
use bessemer::model_archives::{CompressedKernelContext, ModelFieldValue};
use bessemer::parasolid::{BaseSchema, TransmitLimits};
#[path = "common/resolved_identifier_contract.rs"]
mod contract;

const CASES: [(&[u8], usize); 42] = [
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-0-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-0-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-0-2.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-0-3.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-0-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-0-5.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-1-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-1-1.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-1-2.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-1-3.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-1-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-1-5.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-2-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-2-1.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-2-2.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-2-3.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-2-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-2-5.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-3-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-3-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-3-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-3-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-3-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-3-5.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-4-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-4-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-4-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-4-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-4-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-4-5.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-5-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-5-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-5-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-5-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-5-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-5-5.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-0-2-edited.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-1-2-edited.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-2-2-edited.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-3-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-4-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/cosmetic-weld-tree/owned-5-2-edited.bin"),
        103,
    ),
];
const EDIT_FIELDS: [&str; 6] = [
    "raw_358", "raw_6c8", "raw_1b8", "raw_08", "raw_08", "raw_08",
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
fn owned_weld_tree_documents_reconstruct_and_isolate_inputs() {
    for (index, (bytes, slots)) in CASES.into_iter().enumerate() {
        if (6..18).contains(&index) {
            contract::check_document_with_kernel(bytes, slots, Some(kernel()));
        } else {
            contract::check_document(bytes, slots);
        }
    }
}

#[test]
fn independent_tree_edits_preserve_shared_children_and_inputs() {
    for (kind, label) in EDIT_FIELDS.into_iter().enumerate() {
        let value = if kind == 5 {
            ModelFieldValue::U32(12345)
        } else {
            ModelFieldValue::I32(12345)
        };
        contract::check_value_edit(
            CASES[kind * 6 + 2].0,
            CASES[36 + kind].0,
            &format!("root.features[0].items[0].{label}"),
            value,
            Some(kernel()),
        );
    }
}

#[test]
fn rejected_tree_scalar_types_preserve_original_documents() {
    for (kind, label) in EDIT_FIELDS.into_iter().enumerate() {
        let path = format!("root.features[0].items[0].{label}");
        if matches!(kind, 1 | 2) {
            contract::check_rejected_edit_with_kernel(CASES[kind * 6 + 2].0, &path, Some(kernel()));
        } else {
            contract::check_rejected_edit(CASES[kind * 6 + 2].0, &path);
        }
    }
}
