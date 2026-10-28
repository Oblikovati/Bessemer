//! Complete note/text documents and isolated edits (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: &[(&[u8], usize)] = &[
    (include_bytes!("fixtures/display-notes/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/display-notes/owned-0-1.bin"), 129),
    (include_bytes!("fixtures/display-notes/owned-0-2.bin"), 103),
    (include_bytes!("fixtures/display-notes/owned-0-3.bin"), 129),
    (include_bytes!("fixtures/display-notes/owned-0-4.bin"), 103),
    (include_bytes!("fixtures/display-notes/owned-0-5.bin"), 129),
    (include_bytes!("fixtures/display-notes/owned-1-0.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-1-1.bin"), 115),
    (include_bytes!("fixtures/display-notes/owned-1-2.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-1-3.bin"), 115),
    (include_bytes!("fixtures/display-notes/owned-1-4.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-1-5.bin"), 115),
    (include_bytes!("fixtures/display-notes/owned-2-0.bin"), 103),
    (include_bytes!("fixtures/display-notes/owned-2-1.bin"), 113),
    (include_bytes!("fixtures/display-notes/owned-2-2.bin"), 105),
    (include_bytes!("fixtures/display-notes/owned-2-3.bin"), 113),
    (include_bytes!("fixtures/display-notes/owned-2-4.bin"), 105),
    (include_bytes!("fixtures/display-notes/owned-2-5.bin"), 113),
    (include_bytes!("fixtures/display-notes/owned-3-0.bin"), 101),
    (include_bytes!("fixtures/display-notes/owned-3-1.bin"), 107),
    (include_bytes!("fixtures/display-notes/owned-3-2.bin"), 103),
    (include_bytes!("fixtures/display-notes/owned-3-3.bin"), 107),
    (include_bytes!("fixtures/display-notes/owned-3-4.bin"), 103),
    (include_bytes!("fixtures/display-notes/owned-3-5.bin"), 107),
    (include_bytes!("fixtures/display-notes/owned-4-0.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-4-1.bin"), 105),
    (include_bytes!("fixtures/display-notes/owned-4-2.bin"), 101),
    (include_bytes!("fixtures/display-notes/owned-4-3.bin"), 105),
    (include_bytes!("fixtures/display-notes/owned-4-4.bin"), 101),
    (include_bytes!("fixtures/display-notes/owned-4-5.bin"), 105),
    (include_bytes!("fixtures/display-notes/owned-5-0.bin"), 101),
    (include_bytes!("fixtures/display-notes/owned-5-1.bin"), 105),
    (include_bytes!("fixtures/display-notes/owned-5-2.bin"), 103),
    (include_bytes!("fixtures/display-notes/owned-5-3.bin"), 105),
    (include_bytes!("fixtures/display-notes/owned-5-4.bin"), 103),
    (include_bytes!("fixtures/display-notes/owned-5-5.bin"), 105),
    (include_bytes!("fixtures/display-notes/owned-6-0.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-6-1.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-6-2.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-6-3.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-6-4.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-6-5.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-7-0.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-7-1.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-7-2.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-7-3.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-7-4.bin"), 99),
    (include_bytes!("fixtures/display-notes/owned-7-5.bin"), 99),
    (
        include_bytes!("fixtures/display-notes/owned-0-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/display-notes/owned-1-2-edited.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/display-notes/owned-2-2-edited.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/display-notes/owned-3-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/display-notes/owned-4-2-edited.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/display-notes/owned-5-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/display-notes/owned-6-2-edited.bin"),
        99,
    ),
];
const PATHS: [&str; 7] = [
    "root.features[0].note.raw_784",
    "root.features[0].raw_100",
    "root.features[0].detail.raw_78",
    "root.features[0].raw_160",
    "root.features[0].raw_128",
    "root.features[0].raw_08",
    "root.features[0].raw_08",
];
#[test]
fn complete_note_documents_reconstruct_and_preserve_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, *slots);
    }
}
#[test]
fn note_and_text_word_edits_match_independent_goldens() {
    for (kind, path) in PATHS.iter().enumerate() {
        contract::check_word_edit(CASES[kind * 6 + 2].0, CASES[48 + kind].0, path);
    }
}
#[test]
fn rejected_note_edits_preserve_archives_and_documents() {
    for (kind, path) in PATHS.iter().enumerate() {
        contract::check_rejected_edit(CASES[kind * 6 + 2].0, path);
    }
}
