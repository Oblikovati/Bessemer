//! Complete move/copy documents and independent edit goldens (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: &[(&[u8], usize)] = &[
    (
        include_bytes!("fixtures/move-copy-bodies/owned-0-0.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-0-1.bin"),
        117,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-0-2.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-0-3.bin"),
        117,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-0-4.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-0-5.bin"),
        117,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-1-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-1-1.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-1-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-1-3.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-1-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-1-5.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-2-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-2-1.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-2-2.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-2-3.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-2-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-2-5.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-0-2-edited.bin"),
        115,
    ),
    (
        include_bytes!("fixtures/move-copy-bodies/owned-2-2-edited.bin"),
        99,
    ),
];
#[test]
fn complete_move_copy_documents_reconstruct_reopen_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, *slots);
    }
}
#[test]
fn independent_move_copy_word_edits_reconstruct_exactly() {
    for (original, edited, path) in [
        (2, 18, "root.features[0].payload.raw_9f0"),
        (14, 19, "root.features[0].raw_08"),
    ] {
        contract::check_word_edit(CASES[original].0, CASES[edited].0, path);
    }
}
#[test]
fn rejected_move_copy_edits_preserve_the_original() {
    for (index, path) in [
        (2, "root.features[0].payload.raw_9f0"),
        (14, "root.features[0].raw_08"),
    ] {
        contract::check_rejected_edit(CASES[index].0, path);
    }
}
