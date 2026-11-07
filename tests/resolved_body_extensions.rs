//! Complete inherited extension bodies, independent edits and isolated inputs (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::CompressedKernelContext;
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/body-extensions/owned-0-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-0-1.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-0-2.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-0-3.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-0-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-0-5.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-1-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-1-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-1-2.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-1-3.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-1-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-1-5.bin"),
        131,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-2-0.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-2-1.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-2-2.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-2-3.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-2-4.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-2-5.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-0-2-edited.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-1-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/body-extensions/owned-2-2-edited.bin"),
        107,
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
fn complete_extension_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[0].0, CASES[0].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn independent_extension_edits_reconstruct_exact_wire_bytes() {
    contract::check_word_edit_with_kernel(
        CASES[2].0,
        CASES[18].0,
        "root.features[0].items[0].payload.raw_810",
        Some(kernel()),
    );
    contract::check_word_edit_with_kernel(
        CASES[8].0,
        CASES[19].0,
        "root.features[0].items[0].extrusion_suffix.word_808",
        Some(kernel()),
    );
    contract::check_word_edit_with_kernel(
        CASES[14].0,
        CASES[20].0,
        "root.features[0].raw_96c",
        Some(kernel()),
    );
}
#[test]
fn rejected_extension_edits_preserve_the_original() {
    contract::check_rejected_edit(CASES[0].0, "root.features[0].payload.raw_810");
    contract::check_rejected_edit_with_kernel(
        CASES[2].0,
        "root.features[0].items[0].payload.raw_810",
        Some(kernel()),
    );
}
