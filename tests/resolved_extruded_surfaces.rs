//! Complete extruded-surface operations against independent helper/keeper graphs (#54).
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
const POPULATED: &[u8] = include_bytes!("fixtures/extruded-surfaces/owned-populated-header.bin");
const EDITED: &[u8] =
    include_bytes!("fixtures/extruded-surfaces/owned-populated-header-edited.bin");

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
fn surfaces_reconstruct_whole_documents_with_null_empty_and_populated_helpers() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/extruded-surfaces/owned-null.bin").as_slice(),
            103,
        ),
        (
            include_bytes!("fixtures/extruded-surfaces/owned-empty.bin").as_slice(),
            117,
        ),
        (
            include_bytes!("fixtures/extruded-surfaces/owned-populated.bin").as_slice(),
            119,
        ),
        (POPULATED, 121),
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
fn surface_word_edit_preserves_ordered_groups_keeper_identities_and_source() {
    let document = document_with(POPULATED);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let word = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].payload.raw_810")
        .unwrap();
    assert!(matches!(word.value, ModelFieldValue::I32(-1)));
    word.value = ModelFieldValue::I32(7);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 121));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), POPULATED);
    assert_eq!(document.read_stream(STREAM).unwrap(), POPULATED);
    assert_eq!(parsed(&document_with(EDITED)).to_bytes().unwrap(), EDITED);
}
