//! Complete alignment-feature documents through the public archive contract (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/alignment-groups/owned-0-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-0-1.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-0-2.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-0-3.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-0-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-0-5.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-1-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-1-1.bin"),
        139,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-1-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-1-3.bin"),
        139,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-1-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-1-5.bin"),
        139,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-2-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-2-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-2-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-2-3.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-2-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-2-5.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-3-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-3-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-3-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-3-3.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-3-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-3-5.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-4-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-4-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-4-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-4-3.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-4-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-4-5.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-5-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-5-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-5-2.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-5-3.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-5-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-5-5.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-0-2-edited.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-1-2-edited.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-2-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-3-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-4-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/alignment-groups/owned-5-2-edited.bin"),
        101,
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
fn alignment_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn independent_alignment_edits_match_owned_goldens() {
    for (index, edited, path) in [
        (2, 36, "payload.raw_3d8"),
        (8, 37, "suffix.raw_fe8"),
        (14, 38, "dcm_type"),
        (20, 39, "dcm_type"),
        (26, 40, "reference_count"),
        (32, 41, "texts[0].date_and_time"),
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
fn alignment_wrong_typed_edits_preserve_inputs() {
    contract::check_rejected_edit(CASES[26].0, "root.features[0].reference_count");
    contract::check_rejected_edit_with_kernel(
        CASES[2].0,
        "root.features[0].payload.raw_3d8",
        Some(kernel()),
    );
}

const ACTIVE_CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/alignment-groups/active-0.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/alignment-groups/active-1.bin"),
        105,
    ),
    (include_bytes!("fixtures/alignment-groups/active-2.bin"), 99),
    (include_bytes!("fixtures/alignment-groups/active-3.bin"), 99),
    (include_bytes!("fixtures/alignment-groups/active-4.bin"), 99),
    (
        include_bytes!("fixtures/alignment-groups/active-5.bin"),
        101,
    ),
];

#[test]
fn active_sheet_metal_state_preserves_ordinary_alignment_and_text_layouts() {
    for (bytes, slots) in ACTIVE_CASES {
        let mut document = contract::document_with(bytes);
        document
            .replace_stream(
                "Contents/CMgr",
                include_bytes!("fixtures/sheet-metal/manager-1.bin").to_vec(),
            )
            .unwrap();
        let before = document.to_bytes().unwrap();
        let archive = contract::parsed_with_kernel(&document, Some(kernel()));
        assert_eq!(archive.object_slots(), (97, *slots));
        assert_eq!(archive.to_bytes().unwrap(), *bytes);
        let reopened = bessemer::Document::from_bytes(&before).unwrap();
        assert_eq!(
            contract::parsed_with_kernel(&reopened, Some(kernel()))
                .to_bytes()
                .unwrap(),
            *bytes
        );
        assert_eq!(document.to_bytes().unwrap(), before);
    }
}
