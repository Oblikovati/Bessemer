//! Complete shared sketch-handle document operations and retained escaped indices (#54).
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
const STANDALONE: [&[u8]; 2] = [
    include_bytes!("fixtures/shared-sketch-handles/owned-0-0.bin"),
    include_bytes!("fixtures/shared-sketch-handles/owned-1-0.bin"),
];
const ARRAYS: [&[u8]; 2] = [
    include_bytes!("fixtures/shared-sketch-handles/owned-0-1.bin"),
    include_bytes!("fixtures/shared-sketch-handles/owned-1-1.bin"),
];
const ORIGINALS: [&[u8]; 2] = [
    include_bytes!("fixtures/shared-sketch-handles/owned-0-2.bin"),
    include_bytes!("fixtures/shared-sketch-handles/owned-1-2.bin"),
];
const EDITS: [&[u8]; 2] = [
    include_bytes!("fixtures/shared-sketch-handles/owned-0-2-edited.bin"),
    include_bytes!("fixtures/shared-sketch-handles/owned-1-2-edited.bin"),
];

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
fn complete_shared_handle_documents_reconstruct_standalone_aliased_and_component_records() {
    for (features, slots) in STANDALONE.into_iter().map(|bytes| (bytes, 99)).chain(
        ARRAYS
            .into_iter()
            .chain(ORIGINALS)
            .chain(EDITS)
            .map(|bytes| (bytes, 101)),
    ) {
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
}

#[test]
fn reference_id_edits_match_independent_bytes_without_canonicalizing_escaped_small_indices() {
    for (original, edited) in ORIGINALS.into_iter().zip(EDITS) {
        let document = document_with(original);
        let archive = parsed(&document);
        let mut fields = archive.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == "root.features[0].entity_handle.reference_id")
            .unwrap()
            .value = ModelFieldValue::I32(12345);
        let edit = archive.edit_fields(fields).unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), edited);
        assert_eq!(edit.changes().len(), 1);
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}

#[test]
fn rejected_escape_marker_edit_preserves_source_handle_and_document() {
    for original in ORIGINALS {
        let document = document_with(original);
        let archive = parsed(&document);
        let mut fields = archive.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == "root.features[0].entity_handle.entity_index")
            .unwrap()
            .value = ModelFieldValue::U16(7);
        assert!(archive.edit_fields(fields).is_err());
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}
