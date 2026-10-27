//! Complete combine-body documents, direct data arrays and isolated signed-word edits (#54).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartResolvedFeaturesArchive,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const CASES: [&[u8]; 19] = [
    include_bytes!("fixtures/combine-bodies/owned-0-0.bin"),
    include_bytes!("fixtures/combine-bodies/owned-0-1.bin"),
    include_bytes!("fixtures/combine-bodies/owned-0-2.bin"),
    include_bytes!("fixtures/combine-bodies/owned-0-3.bin"),
    include_bytes!("fixtures/combine-bodies/owned-0-4.bin"),
    include_bytes!("fixtures/combine-bodies/owned-0-5.bin"),
    include_bytes!("fixtures/combine-bodies/owned-1-0.bin"),
    include_bytes!("fixtures/combine-bodies/owned-1-1.bin"),
    include_bytes!("fixtures/combine-bodies/owned-1-2.bin"),
    include_bytes!("fixtures/combine-bodies/owned-1-3.bin"),
    include_bytes!("fixtures/combine-bodies/owned-1-4.bin"),
    include_bytes!("fixtures/combine-bodies/owned-1-5.bin"),
    include_bytes!("fixtures/combine-bodies/owned-2-0.bin"),
    include_bytes!("fixtures/combine-bodies/owned-2-1.bin"),
    include_bytes!("fixtures/combine-bodies/owned-2-2.bin"),
    include_bytes!("fixtures/combine-bodies/owned-2-3.bin"),
    include_bytes!("fixtures/combine-bodies/owned-2-4.bin"),
    include_bytes!("fixtures/combine-bodies/owned-2-5.bin"),
    include_bytes!("fixtures/combine-bodies/owned-0-2-edited.bin"),
];
const SLOTS: [usize; 19] = [
    101, 105, 111, 107, 109, 117, 101, 105, 111, 107, 109, 117, 99, 101, 105, 101, 103, 107, 111,
];
const ORIGINAL: &[u8] = CASES[2];
const EDITED: &[u8] = CASES[18];
const WORD_PATH: &str = "root.features[0].items[0].raw_828";

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
        LIMITS,
    )
    .unwrap()
}

fn assert_document_reconstruction(features: &[u8], slots: usize) {
    let mut document = document_with(features);
    let before = document.to_bytes().unwrap();
    let archive = parsed(&document);
    assert_eq!(archive.object_slots(), (97, slots));
    let reconstructed = archive.to_bytes().unwrap();
    assert_eq!(reconstructed, features);
    document.replace_stream(STREAM, reconstructed).unwrap();
    assert_eq!(document.to_bytes().unwrap(), before);
    document.replace_stream(STREAM, vec![0xa5]).unwrap();
    assert_eq!(archive.to_bytes().unwrap(), features);
}

fn combine_body_words(archive: &PartResolvedFeaturesArchive) -> Vec<ModelFieldValue> {
    archive
        .fields()
        .iter()
        .filter(|field| field.path.ends_with("raw_828"))
        .map(|field| field.value.clone())
        .collect()
}

fn assert_combine_body_words(archive: &PartResolvedFeaturesArchive, first: i32) {
    let expected = vec![ModelFieldValue::I32(first), ModelFieldValue::I32(i32::MIN)];
    assert_eq!(combine_body_words(archive), expected);
}

#[test]
fn complete_combine_body_and_data_documents_reconstruct_all_tagged_and_direct_records() {
    for (features, slots) in CASES.into_iter().zip(SLOTS) {
        assert_document_reconstruction(features, slots);
    }
}

#[test]
fn editing_one_equal_combine_body_preserves_the_other_and_matches_independent_bytes() {
    let (original, edited) = (ORIGINAL, EDITED);
    let document = document_with(original);
    let archive = parsed(&document);
    assert_combine_body_words(&archive, i32::MIN);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == WORD_PATH)
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), edited);
    assert_combine_body_words(edit.archive(), 12345);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), original);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}

#[test]
fn rejected_combine_body_word_type_edit_preserves_the_original_document_and_archive() {
    let original = ORIGINAL;
    let document = document_with(original);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == WORD_PATH)
        .unwrap()
        .value = ModelFieldValue::U16(7);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), original);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}
