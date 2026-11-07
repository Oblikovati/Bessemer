//! Complete frontier-edge identifiers, direct entity-ID members and isolated word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod identifier_contract;
use identifier_contract::{check_document, check_rejected_edit, check_word_edit};

const CASES: [&[u8]; 14] = [
    include_bytes!("fixtures/frontier-edge-identifiers/owned-0-0.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-0-1.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-0-2.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-0-3.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-0-4.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-0-5.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-1-0.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-1-1.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-1-2.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-1-3.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-1-4.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-1-5.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-0-2-edited.bin"),
    include_bytes!("fixtures/frontier-edge-identifiers/owned-1-2-edited.bin"),
];
const SLOTS: [usize; 14] = [
    99, 101, 109, 119, 107, 127, 99, 101, 111, 123, 109, 131, 109, 111,
];
const LOCAL_PATH: &str = "root.features[0].items[0].unknown_38";
const SOURCE_PATH: &str = "root.features[0].items[0].unknown_48";

#[test]
fn all_frontier_edge_identifier_documents_rebuild_without_changing_untouched_storage() {
    for (features, slots) in CASES.into_iter().zip(SLOTS) {
        check_document(features, slots);
    }
}

#[test]
fn independent_trim_wire_and_pattern_body_word_edits_preserve_the_other_equal_feature() {
    check_word_edit(CASES[2], CASES[12], LOCAL_PATH);
    check_word_edit(CASES[8], CASES[13], SOURCE_PATH);
}

#[test]
fn rejected_trim_wire_and_pattern_body_word_types_preserve_input_archives_and_documents() {
    check_rejected_edit(CASES[2], LOCAL_PATH);
    check_rejected_edit(CASES[8], SOURCE_PATH);
}
