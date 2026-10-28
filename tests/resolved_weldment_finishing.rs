//! Complete gusset, weld-bead and side-data documents with isolated edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [(&[u8], usize); 21] = [
    (
        include_bytes!("fixtures/weldment-finishing/owned-0-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-0-1.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-0-2.bin"),
        117,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-0-3.bin"),
        125,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-0-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-0-5.bin"),
        141,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-1-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-1-1.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-1-2.bin"),
        121,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-1-3.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-1-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-1-5.bin"),
        125,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-2-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-2-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-2-2.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-2-3.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-2-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-2-5.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-0-2-edited.bin"),
        117,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-1-2-edited.bin"),
        121,
    ),
    (
        include_bytes!("fixtures/weldment-finishing/owned-2-2-edited.bin"),
        107,
    ),
];
const EDIT_PATHS: [&str; 3] = ["payload.raw_868", "payload.raw_948", "raw_50"];

#[test]
fn complete_finishing_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn independent_finishing_edits_preserve_other_roots_and_shared_children() {
    for (kind, suffix) in EDIT_PATHS.into_iter().enumerate() {
        contract::check_word_edit(
            CASES[kind * 6 + 2].0,
            CASES[18 + kind].0,
            &format!("root.features[0].items[0].{suffix}"),
        );
    }
}

#[test]
fn rejected_finishing_scalar_types_preserve_original_documents() {
    for (kind, suffix) in EDIT_PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(
            CASES[kind * 6 + 2].0,
            &format!("root.features[0].items[0].{suffix}"),
        );
    }
}
