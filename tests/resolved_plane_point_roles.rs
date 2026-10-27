//! Public plane-point subclass regressions with complete independent documents (#46).
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
const ENDPOINT: &[u8] = include_bytes!("fixtures/plane-point-roles/owned-0-1.bin");
const EDITED: &[u8] = include_bytes!("fixtures/plane-point-roles/owned-0-1-edited.bin");

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
fn all_three_plane_kinds_reconstruct_documents_with_each_complete_point_subtype() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/plane-point-roles/owned-0-0.bin").as_slice(),
            105,
        ),
        (ENDPOINT, 105),
        (
            include_bytes!("fixtures/plane-point-roles/owned-0-2.bin").as_slice(),
            105,
        ),
        (
            include_bytes!("fixtures/plane-point-roles/owned-1-0.bin").as_slice(),
            107,
        ),
        (
            include_bytes!("fixtures/plane-point-roles/owned-1-1.bin").as_slice(),
            107,
        ),
        (
            include_bytes!("fixtures/plane-point-roles/owned-1-2.bin").as_slice(),
            107,
        ),
        (
            include_bytes!("fixtures/plane-point-roles/owned-2-0.bin").as_slice(),
            107,
        ),
        (
            include_bytes!("fixtures/plane-point-roles/owned-2-1.bin").as_slice(),
            107,
        ),
        (
            include_bytes!("fixtures/plane-point-roles/owned-2-2.bin").as_slice(),
            107,
        ),
        (EDITED, 105),
    ] {
        let mut document = document_with(features);
        let original = document.to_bytes().unwrap();
        let archive = parsed(&document).unwrap();
        assert_eq!(archive.object_slots(), (97, slots));
        assert_eq!(archive.to_bytes().unwrap(), features);
        document
            .replace_stream(STREAM, archive.to_bytes().unwrap())
            .unwrap();
        assert_eq!(document.to_bytes().unwrap(), original);
    }
}

#[test]
fn endpoint_coordinate_edit_matches_independent_bytes_and_preserves_source_inputs() {
    let document = document_with(ENDPOINT);
    let archive = parsed(&document).unwrap();
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| {
            field.path.as_ref() == "root.features[0].plane_data.point_reference_88.point_a8[0]"
        })
        .unwrap()
        .value = ModelFieldValue::F64(StoredDouble::from_bits(123.5f64.to_bits()));
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), ENDPOINT);
    assert_eq!(document.read_stream(STREAM).unwrap(), ENDPOINT);
}

#[test]
fn unrelated_wrapper_document_still_rejects_at_the_point_role_and_preserves_inputs() {
    let bytes = include_bytes!("fixtures/plane-point-roles/owned-0-3.bin");
    let document = document_with(bytes);
    let error = parsed(&document).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(
        error.cause.contains("point_reference_88"),
        "{}",
        error.cause
    );
    assert!(error.cause.contains("moRefWrapper_c"), "{}", error.cause);
    assert_eq!(document.read_stream(STREAM).unwrap(), bytes);
}
