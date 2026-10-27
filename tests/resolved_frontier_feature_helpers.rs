//! Native-bound lock/environment features, pattern items and edge/radius helpers (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod identifier_contract;
use identifier_contract::{check_document, check_rejected_edit, check_word_edit};

const CASES: [&[u8]; 34] = [
    include_bytes!("fixtures/frontier-feature-helpers/owned-0-0.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-0-1.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-0-2.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-0-3.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-0-4.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-0-5.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-1-0.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-1-1.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-1-2.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-1-3.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-1-4.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-1-5.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-2-0.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-2-1.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-2-2.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-2-3.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-2-4.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-2-5.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-3-0.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-3-1.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-3-2.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-3-3.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-3-4.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-3-5.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-4-0.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-4-1.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-4-2.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-4-3.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-4-4.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-4-5.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-0-2-edited.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-1-2-edited.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-2-2-edited.bin"),
    include_bytes!("fixtures/frontier-feature-helpers/owned-3-2-edited.bin"),
];
const SLOTS: [usize; 34] = [
    101, 101, 107, 101, 107, 107, 101, 101, 107, 101, 107, 107, 99, 99, 103, 99, 103, 103, 99, 107,
    111, 109, 113, 111, 99, 109, 111, 109, 109, 109, 107, 107, 103, 111,
];
const EDITS: [(usize, usize, &str); 4] = [
    (2, 30, "root.features[0].items[0].unknown_290"),
    (8, 31, "root.features[0].items[0].word_354"),
    (14, 32, "root.features[0].items[0].raw_20"),
    (20, 33, "root.features[0].items[1].edge_radius.word_20"),
];

#[test]
fn all_frontier_helper_documents_rebuild_without_changing_other_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        check_document(bytes, slots);
    }
}

#[test]
fn independent_frontier_feature_and_helper_word_edits_match_golden_streams() {
    for (original, edited, path) in EDITS {
        check_word_edit(CASES[original], CASES[edited], path);
    }
}

#[test]
fn rejected_frontier_helper_edits_leave_original_archives_and_documents_intact() {
    for (original, _, path) in EDITS {
        check_rejected_edit(CASES[original], path);
    }
}
