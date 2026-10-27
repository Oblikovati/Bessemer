//! Complete trim-edge identifiers, direct entity-ID members and isolated word edits (#54).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartResolvedFeaturesArchive,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const CASES: [&[u8]; 14] = [
    include_bytes!("fixtures/trim-edge-identifiers/owned-0-0.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-0-1.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-0-2.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-0-3.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-0-4.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-0-5.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-1-0.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-1-1.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-1-2.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-1-3.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-1-4.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-1-5.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-0-2-edited.bin"),
    include_bytes!("fixtures/trim-edge-identifiers/owned-1-2-edited.bin"),
];
const SLOTS: [usize; 14] = [
    99, 101, 111, 123, 109, 131, 99, 101, 111, 123, 109, 131, 111, 111,
];
const WORD_PATH: &str = "root.features[0].items[0].unknown_48";

fn document_with(features: &[u8]) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
    document
        .replace_stream(
            "Contents/CMgr",
            include_bytes!("fixtures/sheet-metal/manager-0.bin").to_vec(),
        )
        .unwrap();
    document
}

fn parsed(document: &Document) -> PartResolvedFeaturesArchive {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        ModelArchiveLimits {
            maximum_bytes: 30000,
            maximum_fields: 10000,
            maximum_entries: 10000,
            maximum_depth: 64,
            maximum_objects: 256,
        },
    )
    .unwrap()
}

fn check_document(features: &[u8], slots: usize) {
    let mut document = document_with(features);
    let before = document.to_bytes().unwrap();
    let archive = parsed(&document);
    assert_eq!(archive.object_slots(), (97, slots));
    assert_eq!(archive.to_bytes().unwrap(), features);
    document
        .replace_stream(STREAM, archive.to_bytes().unwrap())
        .unwrap();
    assert_eq!(document.to_bytes().unwrap(), before);
    document.replace_stream(STREAM, vec![0xa5]).unwrap();
    assert_eq!(archive.to_bytes().unwrap(), features);
}

#[test]
fn all_trim_edge_identifier_documents_rebuild_without_changing_untouched_storage() {
    for (features, slots) in CASES.into_iter().zip(SLOTS) {
        check_document(features, slots);
    }
}

fn check_word_edit(original: &[u8], edited: &[u8], path: &str) {
    let document = document_with(original);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), edited);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), original);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}

#[test]
fn independent_left_and_right_trim_word_edits_preserve_the_other_equal_feature() {
    check_word_edit(CASES[2], CASES[12], WORD_PATH);
    check_word_edit(CASES[8], CASES[13], WORD_PATH);
}

fn check_rejected_edit(original: &[u8], path: &str) {
    let document = document_with(original);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = ModelFieldValue::U16(7);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), original);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}

#[test]
fn rejected_left_and_right_trim_word_types_preserve_input_archives_and_documents() {
    check_rejected_edit(CASES[2], WORD_PATH);
    check_rejected_edit(CASES[8], WORD_PATH);
}
