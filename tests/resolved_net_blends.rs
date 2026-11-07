//! Complete net-blend/cut features, component dependencies and isolated signed-word edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod identifier_contract;
use identifier_contract::{check_document, check_rejected_edit, check_word_edit};

const CASES: [&[u8]; 34] = [
    include_bytes!("fixtures/net-blends/owned-0-0.bin"),
    include_bytes!("fixtures/net-blends/owned-0-1.bin"),
    include_bytes!("fixtures/net-blends/owned-0-2.bin"),
    include_bytes!("fixtures/net-blends/owned-0-3.bin"),
    include_bytes!("fixtures/net-blends/owned-0-4.bin"),
    include_bytes!("fixtures/net-blends/owned-0-5.bin"),
    include_bytes!("fixtures/net-blends/owned-1-0.bin"),
    include_bytes!("fixtures/net-blends/owned-1-1.bin"),
    include_bytes!("fixtures/net-blends/owned-1-2.bin"),
    include_bytes!("fixtures/net-blends/owned-1-3.bin"),
    include_bytes!("fixtures/net-blends/owned-1-4.bin"),
    include_bytes!("fixtures/net-blends/owned-1-5.bin"),
    include_bytes!("fixtures/net-blends/owned-2-0.bin"),
    include_bytes!("fixtures/net-blends/owned-2-1.bin"),
    include_bytes!("fixtures/net-blends/owned-2-2.bin"),
    include_bytes!("fixtures/net-blends/owned-2-3.bin"),
    include_bytes!("fixtures/net-blends/owned-2-4.bin"),
    include_bytes!("fixtures/net-blends/owned-2-5.bin"),
    include_bytes!("fixtures/net-blends/owned-3-0.bin"),
    include_bytes!("fixtures/net-blends/owned-3-1.bin"),
    include_bytes!("fixtures/net-blends/owned-3-2.bin"),
    include_bytes!("fixtures/net-blends/owned-3-3.bin"),
    include_bytes!("fixtures/net-blends/owned-3-4.bin"),
    include_bytes!("fixtures/net-blends/owned-3-5.bin"),
    include_bytes!("fixtures/net-blends/owned-4-0.bin"),
    include_bytes!("fixtures/net-blends/owned-4-1.bin"),
    include_bytes!("fixtures/net-blends/owned-4-2.bin"),
    include_bytes!("fixtures/net-blends/owned-4-3.bin"),
    include_bytes!("fixtures/net-blends/owned-4-4.bin"),
    include_bytes!("fixtures/net-blends/owned-4-5.bin"),
    include_bytes!("fixtures/net-blends/owned-0-2-edited.bin"),
    include_bytes!("fixtures/net-blends/owned-1-2-edited.bin"),
    include_bytes!("fixtures/net-blends/owned-3-2-edited.bin"),
    include_bytes!("fixtures/net-blends/owned-4-2-edited.bin"),
];
const SLOTS: [usize; 34] = [
    101, 101, 119, 115, 119, 123, 101, 101, 119, 115, 119, 123, 99, 109, 113, 109, 113, 113, 99,
    109, 113, 109, 117, 113, 99, 109, 113, 109, 117, 113, 119, 119, 113, 113,
];
const EDITS: [(usize, usize, &str); 4] = [
    (2, 30, "root.features[0].items[0].data.raw_8d0"),
    (8, 31, "root.features[0].items[0].data.raw_8d0"),
    (20, 32, "root.features[0].items[0].raw_e8"),
    (26, 33, "root.features[0].items[0].raw_impl_30"),
];

#[test]
fn all_net_blend_and_component_documents_reconstruct_without_changing_other_storage() {
    for (bytes, slots) in CASES.into_iter().zip(SLOTS) {
        check_document(bytes, slots);
    }
}

#[test]
fn independent_net_blend_cut_group_and_multi_edge_edits_match_golden_streams() {
    for (original, edited, path) in EDITS {
        check_word_edit(CASES[original], CASES[edited], path);
    }
}

#[test]
fn rejected_net_blend_and_component_word_edits_preserve_archives_and_documents() {
    for (original, _, path) in EDITS {
        check_rejected_edit(CASES[original], path);
    }
}
