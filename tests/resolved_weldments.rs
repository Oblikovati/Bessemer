//! Complete weldment documents with independent direct-property edits (#54).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ArchiveString, ConfigurationId, ModelArchiveLimits, ModelFieldValue,
    PartResolvedFeaturesArchive, TextEncoding,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const RICH: &[u8] = include_bytes!("fixtures/weldments/owned-2.bin");
const EDITED: &[u8] = include_bytes!("fixtures/weldments/owned-2-edited.bin");

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
fn weldment_records_reconstruct_whole_documents_without_body_feature_fields_or_direct_map_slots() {
    for features in [
        include_bytes!("fixtures/weldments/owned-0.bin").as_slice(),
        include_bytes!("fixtures/weldments/owned-1.bin").as_slice(),
        RICH,
        EDITED,
    ] {
        let mut document = document_with(features);
        let original = document.to_bytes().unwrap();
        let archive = parsed(&document);
        assert_eq!(archive.object_slots(), (97, 101));
        assert_eq!(archive.to_bytes().unwrap(), features);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

#[test]
fn direct_property_edit_matches_independent_bytes_and_preserves_duplicate_neighbors_and_inputs() {
    let document = document_with(RICH);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| {
            field.path.as_ref() == "root.features[0].payload.properties_360.properties[0].string_20"
        })
        .unwrap()
        .value =
        ModelFieldValue::Text(ArchiveString::new("Bessemer Ω", TextEncoding::Utf16).unwrap());
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 101));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}

#[test]
fn rejected_property_count_edit_keeps_the_original_weldment_document_and_archive() {
    let document = document_with(RICH);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.ends_with("properties.count"))
        .unwrap()
        .value = ModelFieldValue::I32(-1);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}
