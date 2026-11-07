//! Complete component coordinate systems with independent fixed-word edits (#54).
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
const POPULATED: &[u8] = include_bytes!("fixtures/component-coordinate-systems/owned-1-1.bin");
const EDITED: &[u8] = include_bytes!("fixtures/component-coordinate-systems/owned-1-1-edited.bin");

fn document_with(features: &[u8]) -> Document {
    let mut document = Document::from_bytes(include_bytes!("fixtures/blank-part.SLDPRT")).unwrap();
    document.replace_stream(STREAM, features.to_vec()).unwrap();
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
fn component_coordinate_null_fresh_cached_and_wrapped_frames_reconstruct_complete_documents() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/component-coordinate-systems/owned-0-0.bin").as_slice(),
            99,
        ),
        (
            include_bytes!("fixtures/component-coordinate-systems/owned-1-0.bin").as_slice(),
            99,
        ),
        (
            include_bytes!("fixtures/component-coordinate-systems/owned-1-2.bin").as_slice(),
            115,
        ),
        (
            include_bytes!("fixtures/component-coordinate-systems/owned-1-3.bin").as_slice(),
            115,
        ),
        (POPULATED, 113),
    ] {
        let mut document = document_with(features);
        let original = document.to_bytes().unwrap();
        let archive = parsed(&document);
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), features);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

#[test]
fn fixed_word_edit_preserves_geometry_aliases_other_words_and_inputs() {
    let document = document_with(POPULATED);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let label = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].transform_100[0]")
        .unwrap();
    label.value = ModelFieldValue::U32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 113));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), POPULATED);
    assert_eq!(document.read_stream(STREAM).unwrap(), POPULATED);
    assert_eq!(parsed(&document_with(EDITED)).to_bytes().unwrap(), EDITED);
}

#[test]
fn unsupported_fixed_kind_edit_rejects_without_mutating_archive_or_document() {
    let document = document_with(POPULATED);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].transform_100[28]")
        .unwrap()
        .value = ModelFieldValue::U32(0x123400f0);
    let error = archive.edit_fields(fields).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Unsupported);
    assert!(error.cause.contains("0x123400f0"), "{}", error.cause);
    assert_eq!(archive.to_bytes().unwrap(), POPULATED);
    assert_eq!(document.read_stream(STREAM).unwrap(), POPULATED);
}
