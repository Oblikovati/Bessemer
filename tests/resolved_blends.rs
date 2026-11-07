//! Complete blend/cut document operations and independent numeric-column edits (#54).
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
const RICH: &[u8] = include_bytes!("fixtures/blends/owned-blend.bin");
const EDITED: &[u8] = include_bytes!("fixtures/blends/owned-blend-edited.bin");

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
fn blend_and_cut_records_reconstruct_complete_documents_with_imported_shared_targets() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/blends/owned-empty.bin").as_slice(),
            101,
        ),
        (
            include_bytes!("fixtures/blends/owned-cut.bin").as_slice(),
            109,
        ),
        (RICH, 109),
        (EDITED, 109),
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
fn signed_column_edit_matches_independent_bytes_and_preserves_other_columns_and_inputs() {
    let document = document_with(RICH);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].payload.data.raw_words_2e8[1]")
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 109));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}

#[test]
fn rejected_negative_declared_count_edit_keeps_the_document_and_archive_unchanged() {
    let document = document_with(RICH);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.ends_with(".declared_count_418"))
        .unwrap()
        .value = ModelFieldValue::I32(-1);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), RICH);
    assert_eq!(document.read_stream(STREAM).unwrap(), RICH);
}
