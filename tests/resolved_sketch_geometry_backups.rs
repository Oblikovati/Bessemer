//! Complete geometry-backup and handle document reconstruction with independent edits (#54).
use bessemer::Document;
use bessemer::header::{ArchiveVersion, StoredDouble};
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelField, ModelFieldValue, PartResolvedFeaturesArchive,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const EMPTY: [&[u8]; 4] = [
    include_bytes!("fixtures/sketch-geometry-backups/owned-0-0.bin"),
    include_bytes!("fixtures/sketch-geometry-backups/owned-1-0.bin"),
    include_bytes!("fixtures/sketch-geometry-backups/owned-2-0.bin"),
    include_bytes!("fixtures/sketch-geometry-backups/owned-3-0.bin"),
];
const ORIGINALS: [&[u8]; 4] = [
    include_bytes!("fixtures/sketch-geometry-backups/owned-0-2.bin"),
    include_bytes!("fixtures/sketch-geometry-backups/owned-1-2.bin"),
    include_bytes!("fixtures/sketch-geometry-backups/owned-2-2.bin"),
    include_bytes!("fixtures/sketch-geometry-backups/owned-3-2.bin"),
];
const EDITS: [&[u8]; 4] = [
    include_bytes!("fixtures/sketch-geometry-backups/owned-0-2-edited.bin"),
    include_bytes!("fixtures/sketch-geometry-backups/owned-1-2-edited.bin"),
    include_bytes!("fixtures/sketch-geometry-backups/owned-2-2-edited.bin"),
    include_bytes!("fixtures/sketch-geometry-backups/owned-3-2-edited.bin"),
];
const OWNER_SLOTS: [usize; 4] = [105, 101, 101, 101];
const EDIT_PATHS: [&str; 4] = [
    "root.features[0].backup.raw_40",
    "root.features[0].backup.raw_38",
    "root.features[0].entity_handle.reference_id",
    "root.features[0].entity_handle.reference_id",
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

fn replacement_fields(kind: usize, archive: &PartResolvedFeaturesArchive) -> Vec<ModelField> {
    let mut fields = archive.fields().to_vec();
    let replacement = if kind < 2 {
        ModelFieldValue::F64(StoredDouble::from_bits(123.5f64.to_bits()))
    } else {
        ModelFieldValue::I32(12345)
    };
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == EDIT_PATHS[kind])
        .unwrap()
        .value = replacement;
    fields
}

#[test]
fn complete_parabola_cylinder_and_handle_documents_reconstruct_with_real_owners() {
    for features in EMPTY {
        assert_document_reconstruction(features, 99);
    }
    for (features, slots) in ORIGINALS
        .into_iter()
        .chain(EDITS)
        .zip(OWNER_SLOTS.into_iter().cycle())
    {
        assert_document_reconstruction(features, slots);
    }
}

#[test]
fn raw_geometry_and_handle_edits_match_independent_golden_bytes_and_preserve_sources() {
    for (kind, (original, edited)) in ORIGINALS.into_iter().zip(EDITS).enumerate() {
        let document = document_with(original);
        let archive = parsed(&document);
        let edit = archive
            .edit_fields(replacement_fields(kind, &archive))
            .unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), edited);
        assert_eq!(edit.archive().object_slots(), (97, OWNER_SLOTS[kind]));
        assert_eq!(edit.changes().len(), 1);
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}

#[test]
fn rejected_backup_owner_mode_edits_preserve_original_records_and_documents() {
    for original in &ORIGINALS[..2] {
        let document = document_with(original);
        let archive = parsed(&document);
        let mut fields = archive.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == "root.features[0].mode")
            .unwrap()
            .value = ModelFieldValue::I16(1);
        assert!(archive.edit_fields(fields).is_err());
        assert_eq!(archive.to_bytes().unwrap(), *original);
        assert_eq!(document.read_stream(STREAM).unwrap(), *original);
    }
}
