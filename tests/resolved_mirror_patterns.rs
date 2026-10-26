//! Complete public mirror read/edit/write operations against independent owned streams (#54).
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
const SHARED: &[u8] = include_bytes!("fixtures/mirror-patterns/owned-shared.bin");
const EDITED: &[u8] = include_bytes!("fixtures/mirror-patterns/owned-shared-edited.bin");

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
fn mirrors_reconstruct_complete_documents_with_actual_inherited_and_direct_member_slots() {
    for (features, slots) in [
        (
            include_bytes!("fixtures/mirror-patterns/owned-null.bin").as_slice(),
            101,
        ),
        (
            include_bytes!("fixtures/mirror-patterns/owned-direct.bin").as_slice(),
            105,
        ),
        (SHARED, 109),
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
fn direct_plane_edit_matches_independent_bytes_and_preserves_shared_component_identity() {
    let document = document_with(SHARED);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    let coordinate = fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].direct_plane_1138.root_point[0]")
        .unwrap();
    coordinate.value = ModelFieldValue::F64(StoredDouble::from_bits(3.5f64.to_bits()));
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 109));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), SHARED);
    assert_eq!(document.read_stream(STREAM).unwrap(), SHARED);
    assert_eq!(parsed(&document_with(EDITED)).to_bytes().unwrap(), EDITED);
}
