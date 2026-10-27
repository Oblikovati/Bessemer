//! Complete reference-point operations with independent expected edits (#54).
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
const POPULATED: &[u8] = include_bytes!("fixtures/reference-points/owned-6-1-0.bin");
const EDITED: &[u8] = include_bytes!("fixtures/reference-points/owned-6-1-0-edited.bin");

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
fn reference_points_and_component_callers_reconstruct_whole_documents() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/reference-points/owned-0-0-0.bin").as_slice(),
            103,
        ),
        (
            include_bytes!("fixtures/reference-points/owned-4-1-0.bin").as_slice(),
            115,
        ),
        (
            include_bytes!("fixtures/reference-points/owned-5-1-0.bin").as_slice(),
            113,
        ),
        (POPULATED, 115),
        (
            include_bytes!("fixtures/reference-points/owned-6-1-1.bin").as_slice(),
            117,
        ),
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
fn along_curve_word_edit_preserves_curve_profile_surface_history_and_inputs() {
    let document = document_with(POPULATED);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let word = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].data_6f8.raw_40")
        .unwrap();
    assert_eq!(word.value, ModelFieldValue::I32(i32::MIN));
    word.value = ModelFieldValue::I32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 115));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), POPULATED);
    assert_eq!(document.read_stream(STREAM).unwrap(), POPULATED);
    assert_eq!(parsed(&document_with(EDITED)).to_bytes().unwrap(), EDITED);
}
