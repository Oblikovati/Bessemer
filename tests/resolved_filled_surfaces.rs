//! Complete filled surfaces, inline keepers and curve contacts with isolated edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod identifier_contract;
use identifier_contract::{check_document, check_rejected_edit, check_word_edit};

const CASES: [&[u8]; 27] = [
    include_bytes!("fixtures/filled-surfaces/owned-0-0.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-0-1.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-0-2.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-0-3.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-0-4.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-0-5.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-1-0.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-1-1.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-1-2.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-1-3.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-1-4.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-1-5.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-2-0.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-2-1.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-2-2.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-2-3.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-2-4.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-2-5.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-3-0.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-3-1.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-3-2.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-3-3.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-3-4.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-3-5.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-0-2-edited.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-2-2-edited.bin"),
    include_bytes!("fixtures/filled-surfaces/owned-3-2-edited.bin"),
];
const SLOTS: [usize; 27] = [
    101, 101, 121, 123, 127, 131, 99, 99, 107, 103, 107, 107, 99, 99, 111, 113, 117, 117, 99, 99,
    107, 101, 107, 107, 121, 111, 107,
];
const EDITS: [(usize, usize, &str); 3] = [
    (2, 24, "root.features[0].items[0].payload.raw_70"),
    (14, 25, "root.features[0].items[0].raw_98"),
    (20, 26, "root.features[0].items[0].wrapper_18.unknown_64"),
];

#[test]
fn all_filled_surface_and_contact_documents_reconstruct_without_changing_other_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        check_document(bytes, slots);
    }
}

#[test]
fn independent_filled_surface_and_contact_word_edits_preserve_distinct_second_targets() {
    for (original, edited, path) in EDITS {
        check_word_edit(CASES[original], CASES[edited], path);
    }
}

#[test]
fn rejected_filled_surface_and_contact_edits_preserve_originals() {
    for (original, _, path) in EDITS {
        check_rejected_edit(CASES[original], path);
    }
}
