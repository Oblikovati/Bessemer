//! Complete document operations for radiated-edge identifiers (#54).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartResolvedFeaturesArchive,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const ORIGINAL: &[u8] = include_bytes!("fixtures/radiated-edge-identifiers/owned-alias.bin");
const EDITED: &[u8] = include_bytes!("fixtures/radiated-edge-identifiers/owned-alias-edited.bin");
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
fn complete_documents_reconstruct_fresh_and_cached_representation_roles() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/radiated-edge-identifiers/owned-empty.bin").as_slice(),
            99,
        ),
        (
            include_bytes!("fixtures/radiated-edge-identifiers/owned-owner.bin").as_slice(),
            103,
        ),
        (ORIGINAL, 107),
        (EDITED, 107),
    ] {
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
}

#[test]
fn signed_word_edit_matches_independent_bytes_and_preserves_all_aliases() {
    let document = document_with(ORIGINAL);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| {
            field
                .path
                .ends_with("source_entity.representation.unknown_38")
        })
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(edit.archive().object_slots(), (97, 107));
    assert_eq!(archive.to_bytes().unwrap(), ORIGINAL);
    assert_eq!(document.read_stream(STREAM).unwrap(), ORIGINAL);
}

#[test]
fn rejected_reference_edit_keeps_source_archive_and_document_unchanged() {
    let document = document_with(ORIGINAL);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let field = fields
        .iter_mut()
        .find(|field| field.path.ends_with("source_entity.representation"))
        .unwrap();
    field.value = ModelFieldValue::U16(2);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), ORIGINAL);
    assert_eq!(document.read_stream(STREAM).unwrap(), ORIGINAL);
}
