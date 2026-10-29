//! Complete smart-component graphs through the public document contract (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/smart-components/owned-0-0.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-0-1.bin"),
        133,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-0-2.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-0-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-0-4.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-0-5.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-1-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-1-1.bin"),
        133,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-1-2.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-1-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-1-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-1-5.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-2-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-2-1.bin"),
        133,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-2-2.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-2-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-2-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-2-5.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-3-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-3-1.bin"),
        133,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-3-2.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-3-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-3-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-3-5.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-4-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-4-1.bin"),
        133,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-4-2.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-4-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-4-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-4-5.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-5-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-5-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-5-2.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-5-3.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-5-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-5-5.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-6-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-6-1.bin"),
        133,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-6-2.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-6-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-6-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-6-5.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-7-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-7-1.bin"),
        133,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-7-2.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-7-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-7-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-7-5.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-8-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-8-1.bin"),
        133,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-8-2.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-8-3.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-8-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-8-5.bin"),
        135,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-9-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-9-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-9-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-9-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-9-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-9-5.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-0-2-edited.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-1-2-edited.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-2-2-edited.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-3-2-edited.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-4-2-edited.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-5-2-edited.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-6-2-edited.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-7-2-edited.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-8-2-edited.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/smart-components/owned-9-2-edited.bin"),
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
fn smart_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn smart_edits_match_all_ten_independent_owned_goldens() {
    for (kind, path) in [
        (0, "model_feature.word_354"),
        (1, "raw_130"),
        (2, "folder.unknown_368"),
        (3, "folder.unknown_368"),
        (4, "folder.unknown_368"),
        (5, "node.id"),
        (6, "raw_58"),
        (7, "raw_58"),
        (8, "node.id"),
        (9, "entries[0].object.node.id"),
    ] {
        contract::check_word_edit_with_kernel(
            CASES[kind * 6 + 2].0,
            CASES[60 + kind].0,
            &format!("root.features[0].{path}"),
            Some(kernel()),
        );
    }
}
#[test]
fn smart_rejected_edits_preserve_the_original_document() {
    contract::check_rejected_edit(CASES[2].0, "root.features[0].model_feature.word_354");
    contract::check_rejected_edit_with_kernel(
        CASES[8].0,
        "root.features[0].raw_130",
        Some(kernel()),
    );
}
