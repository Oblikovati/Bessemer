//! Full boss/revolution operations exercising complete derived point-reference roles (#46).
use bessemer::Document;
use bessemer::header::{ArchiveVersion, StoredDouble};
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
const ORIGINALS: [&[u8]; 2] = [
    include_bytes!("fixtures/end-spec-point-roles/owned-0-1.bin"),
    include_bytes!("fixtures/end-spec-point-roles/owned-1-1.bin"),
];
const EDITS: [&[u8]; 2] = [
    include_bytes!("fixtures/end-spec-point-roles/owned-0-1-edited.bin"),
    include_bytes!("fixtures/end-spec-point-roles/owned-1-1-edited.bin"),
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

fn parsed(document: &Document) -> bessemer::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
}

#[test]
fn complete_boss_and_revolution_documents_reconstruct_base_endpoint_and_loft_points() {
    for features in [
        include_bytes!("fixtures/end-spec-point-roles/owned-0-0.bin").as_slice(),
        include_bytes!("fixtures/end-spec-point-roles/owned-0-2.bin").as_slice(),
        include_bytes!("fixtures/end-spec-point-roles/owned-1-0.bin").as_slice(),
        include_bytes!("fixtures/end-spec-point-roles/owned-1-2.bin").as_slice(),
        ORIGINALS[0],
        ORIGINALS[1],
        EDITS[0],
        EDITS[1],
    ] {
        let mut document = document_with(features);
        let before = document.to_bytes().unwrap();
        let archive = parsed(&document).unwrap();
        assert_eq!(archive.object_slots(), (97, 107));
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
fn shared_endpoint_coordinate_edit_matches_independent_bytes_for_both_owner_classes() {
    for (original, edited) in ORIGINALS.into_iter().zip(EDITS) {
        let document = document_with(original);
        let archive = parsed(&document).unwrap();
        let mut fields = archive.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == "root.features[0].items[0].point_a8[0]")
            .unwrap()
            .value = ModelFieldValue::F64(StoredDouble::from_bits(123.5f64.to_bits()));
        let edit = archive.edit_fields(fields).unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), edited);
        assert_eq!(edit.archive().object_slots(), (97, 107));
        assert_eq!(edit.changes().len(), 1);
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}

#[test]
fn unrelated_point_wrappers_still_reject_without_altering_complete_documents() {
    for features in [
        include_bytes!("fixtures/end-spec-point-roles/owned-0-3.bin").as_slice(),
        include_bytes!("fixtures/end-spec-point-roles/owned-1-3.bin").as_slice(),
    ] {
        let document = document_with(features);
        let before = document.to_bytes().unwrap();
        let error = parsed(&document).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Invalid);
        assert!(error.cause.contains("moRefWrapper_c"));
        assert_eq!(document.to_bytes().unwrap(), before);
    }
}

#[test]
fn rejected_coordinate_field_edit_leaves_source_archive_and_document_unchanged() {
    for original in ORIGINALS {
        let document = document_with(original);
        let archive = parsed(&document).unwrap();
        let mut fields = archive.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == "root.features[0].items[0].point_a8[0]")
            .unwrap()
            .value = ModelFieldValue::I32(1);
        assert!(archive.edit_fields(fields).is_err());
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}
