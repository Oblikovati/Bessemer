//! Complete sweep operations against independently decoded option and reference graphs (#54).
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
const POPULATED: &[u8] = include_bytes!("fixtures/sweeps/owned-populated-enabled.bin");
const EDITED: &[u8] = include_bytes!("fixtures/sweeps/owned-populated-enabled-edited.bin");

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
fn sweeps_reconstruct_whole_documents_with_enabled_and_disabled_options() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/sweeps/owned-minimal-disabled.bin").as_slice(),
            101,
        ),
        (
            include_bytes!("fixtures/sweeps/owned-minimal-enabled.bin").as_slice(),
            109,
        ),
        (
            include_bytes!("fixtures/sweeps/owned-populated-disabled.bin").as_slice(),
            107,
        ),
        (POPULATED, 115),
        (
            include_bytes!("fixtures/sweeps/owned-minimal-disabled-cut.bin").as_slice(),
            101,
        ),
        (
            include_bytes!("fixtures/sweeps/owned-populated-enabled-cut.bin").as_slice(),
            115,
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
fn sweep_word_edit_preserves_options_references_and_source() {
    let document = document_with(POPULATED);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let word = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].payload.raw_c04")
        .unwrap();
    assert!(matches!(word.value, ModelFieldValue::I32(-5)));
    word.value = ModelFieldValue::I32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 115));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), POPULATED);
    assert_eq!(document.read_stream(STREAM).unwrap(), POPULATED);
    assert_eq!(parsed(&document_with(EDITED)).to_bytes().unwrap(), EDITED);
}
