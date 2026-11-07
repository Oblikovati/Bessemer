//! Complete PLine subtypes, required owners and base-surface records (#54, #46).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::{ModelFieldValue, StoredDouble};
const CASES: [&[u8]; 35] = [
    include_bytes!("fixtures/projected-line-subtypes/owned-0-0.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-0-1.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-0-2.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-0-3.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-0-4.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-0-5.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-1-0.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-1-1.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-1-2.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-1-3.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-1-4.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-1-5.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-2-0.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-2-1.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-2-2.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-2-3.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-2-4.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-2-5.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-3-0.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-3-1.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-3-2.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-3-3.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-3-4.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-3-5.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-4-0.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-4-1.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-4-2.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-4-3.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-4-4.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-4-5.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-0-2-edited.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-1-2-edited.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-2-2-edited.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-3-2-edited.bin"),
    include_bytes!("fixtures/projected-line-subtypes/owned-4-2-edited.bin"),
];
const SLOTS: [usize; 35] = [
    103, 103, 117, 111, 109, 121, 103, 105, 117, 113, 109, 123, 105, 105, 113, 109, 109, 113, 105,
    107, 113, 111, 109, 115, 99, 99, 103, 99, 105, 103, 117, 117, 113, 113, 103,
];
const PATHS: [&str; 5] = [
    "root.features[0].items[0].data_808.raw_18",
    "root.features[0].items[0].data_808.raw_38",
    "root.features[0].items[0].raw_18",
    "root.features[0].items[0].raw_38",
    "root.features[0].items[0].unknown_64",
];

#[test]
fn all_owned_projected_line_and_surface_documents_reconstruct_unchanged_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        contract::check_document(bytes, slots);
    }
}

#[test]
fn independent_subtype_and_surface_edits_preserve_shared_targets_other_roots_and_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        if kind == 0 || kind == 2 {
            contract::check_value_edit(
                CASES[kind * 6 + 2],
                CASES[30 + kind],
                path,
                ModelFieldValue::F64(StoredDouble::from_bits(0x4045000000000000)),
                None,
            );
        } else {
            contract::check_word_edit(CASES[kind * 6 + 2], CASES[30 + kind], PATHS[kind]);
        }
    }
}

#[test]
fn rejected_subtype_and_surface_scalar_types_preserve_complete_archives_and_documents() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(CASES[kind * 6 + 2], path);
    }
}
