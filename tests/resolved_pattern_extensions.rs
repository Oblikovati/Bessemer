//! Complete pattern-extension documents through the public archive contract (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::{CompressedKernelContext, ModelFieldValue};
use bessemer::parasolid::{BaseSchema, TransmitLimits};
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/pattern-extensions/owned-0-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-0-1.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-0-2.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-0-3.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-0-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-0-5.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-1-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-1-1.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-1-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-1-3.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-1-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-1-5.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-2-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-2-1.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-2-2.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-2-3.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-2-4.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-2-5.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-3-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-3-1.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-3-2.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-3-3.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-3-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-3-5.bin"),
        119,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-4-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-4-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-4-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-4-3.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-4-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-4-5.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-5-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-5-1.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-5-2.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-5-3.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-5-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-5-5.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-0-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-1-2-edited.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-2-2-edited.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-3-2-edited.bin"),
        113,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-4-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/pattern-extensions/owned-5-2-edited.bin"),
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
fn pattern_extension_documents_reconstruct_reopen_and_isolate_inputs() {
    contract::check_document(CASES[24].0, CASES[24].1);
    for (bytes, slots) in CASES {
        contract::check_document_with_kernel(bytes, *slots, Some(kernel()));
    }
}
#[test]
fn independent_pattern_extension_edits_match_owned_goldens() {
    for (kind, path) in [
        (0, "pattern.tail.word_1120"),
        (1, "pattern.tail.word_1120"),
        (2, "payload.raw_1650"),
        (3, "payload.raw_800"),
        (4, "unknown_48"),
    ] {
        contract::check_word_edit_with_kernel(
            CASES[kind * 6 + 2].0,
            CASES[36 + kind].0,
            &format!("root.features[0].{path}"),
            Some(kernel()),
        );
    }
    contract::check_value_edit(
        CASES[32].0,
        CASES[41].0,
        "root.features[0].raw_10",
        ModelFieldValue::U8(123),
        Some(kernel()),
    );
}
#[test]
fn pattern_extension_wrong_typed_edits_preserve_inputs() {
    contract::check_rejected_edit(CASES[26].0, "root.features[0].unknown_48");
    contract::check_rejected_edit_with_kernel(
        CASES[2].0,
        "root.features[0].pattern.tail.word_1120",
        Some(kernel()),
    );
}
