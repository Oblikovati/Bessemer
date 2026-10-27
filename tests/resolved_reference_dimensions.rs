//! Complete document reconstruction and independently assembled reference-dimension edits (#54).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartResolvedFeaturesArchive,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const ORIGINAL: &[u8] = include_bytes!("fixtures/reference-dimensions/owned-two.bin");
const EDITED: &[u8] = include_bytes!("fixtures/reference-dimensions/owned-two-edited.bin");
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

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

#[test]
fn complete_reference_dimension_documents_reconstruct_and_own_their_fields() {
    for features in [
        include_bytes!("fixtures/reference-dimensions/owned-base.bin").as_slice(),
        include_bytes!("fixtures/reference-dimensions/owned-one.bin").as_slice(),
        include_bytes!("fixtures/reference-dimensions/owned-handle.bin").as_slice(),
        ORIGINAL,
        EDITED,
    ] {
        let mut document = document_with(features);
        let before = document.to_bytes().unwrap();
        let archive = parsed(&document);
        assert_eq!(archive.object_slots().0, 97);
        assert_eq!(archive.to_bytes().unwrap(), features);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), before);
        document.replace_stream(STREAM, vec![0xa5]).unwrap();
        assert_eq!(archive.to_bytes().unwrap(), features);
    }
}

#[test]
fn two_entity_raw_word_edit_matches_independent_golden_without_normalization() {
    let document = document_with(ORIGINAL);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].raw_118")
        .unwrap()
        .value = ModelFieldValue::U16(65535);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), ORIGINAL);
    assert_eq!(document.read_stream(STREAM).unwrap(), ORIGINAL);
}

#[test]
fn rejected_optional_vector_edit_preserves_dimension_and_source_document() {
    let document = document_with(ORIGINAL);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].has_vector_88")
        .unwrap()
        .value = ModelFieldValue::U8(0);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), ORIGINAL);
    assert_eq!(document.read_stream(STREAM).unwrap(), ORIGINAL);
}
