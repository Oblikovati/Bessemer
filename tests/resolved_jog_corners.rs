//! Complete jog and break-corner documents with independent edit goldens (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
use bessemer::model_archives::{ModelFieldValue, PartResolvedFeaturesArchive};
const CASES: [(&[u8], usize); 14] = [
    (include_bytes!("fixtures/jog-corners/owned-0-0.bin"), 101),
    (include_bytes!("fixtures/jog-corners/owned-0-1.bin"), 113),
    (include_bytes!("fixtures/jog-corners/owned-0-2.bin"), 121),
    (include_bytes!("fixtures/jog-corners/owned-0-3.bin"), 117),
    (include_bytes!("fixtures/jog-corners/owned-0-4.bin"), 101),
    (include_bytes!("fixtures/jog-corners/owned-0-5.bin"), 123),
    (include_bytes!("fixtures/jog-corners/owned-1-0.bin"), 101),
    (include_bytes!("fixtures/jog-corners/owned-1-1.bin"), 113),
    (include_bytes!("fixtures/jog-corners/owned-1-2.bin"), 121),
    (include_bytes!("fixtures/jog-corners/owned-1-3.bin"), 115),
    (include_bytes!("fixtures/jog-corners/owned-1-4.bin"), 101),
    (include_bytes!("fixtures/jog-corners/owned-1-5.bin"), 125),
    (
        include_bytes!("fixtures/jog-corners/owned-0-2-edited.bin"),
        121,
    ),
    (
        include_bytes!("fixtures/jog-corners/owned-1-2-edited.bin"),
        121,
    ),
];
const PATHS: [&str; 2] = [
    "root.features[0].items[0].payload.raw_8f4",
    "root.features[0].items[0].payload.raw_8a0",
];
const ACTIVE: [(&[u8], &[u8], usize, &str); 2] = [
    (
        include_bytes!("fixtures/jog-corners/active-0.bin"),
        include_bytes!("fixtures/jog-corners/active-0-edited.bin"),
        117,
        "root.features[0].payload.raw_8f4",
    ),
    (
        include_bytes!("fixtures/jog-corners/active-1.bin"),
        include_bytes!("fixtures/jog-corners/active-1-edited.bin"),
        115,
        "root.features[0].payload.raw_8a0",
    ),
];
#[test]
fn complete_jog_corner_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn independent_jog_corner_edits_match_frozen_goldens_and_preserve_inputs() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_word_edit(CASES[kind * 6 + 2].0, CASES[12 + kind].0, path);
    }
}
#[test]
fn rejected_jog_corner_scalar_types_preserve_original_documents() {
    for (kind, path) in PATHS.into_iter().enumerate() {
        contract::check_rejected_edit(CASES[kind * 6 + 2].0, path);
    }
}
fn active_document(original: &[u8]) -> bessemer::Document {
    let mut document = contract::document_with(original);
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("fixtures/jog-corners/manager-1.bin").to_vec(),
        )
        .unwrap();
    document
}
fn check_active_edit(archive: &PartResolvedFeaturesArchive, edited: &[u8], path: &str) {
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let changed = archive.edit_fields(fields).unwrap();
    assert_eq!(changed.archive().to_bytes().unwrap(), edited);
}
#[test]
fn actual_saved_manager_preserves_inherited_jog_and_break_corner_layouts() {
    for (original, edited, slots, path) in ACTIVE {
        let document = active_document(original);
        let archive = contract::parsed_with_kernel(&document, None);
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), original);
        check_active_edit(&archive, edited, path);
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(
            document
                .read_stream("Contents/Config-0-ResolvedFeatures")
                .unwrap(),
            original
        );
    }
}
