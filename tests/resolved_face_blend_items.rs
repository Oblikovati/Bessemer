//! Complete face-blend pairs and optional-point items with shared references (#54).
use bessemer::Document;
use bessemer::header::{ArchiveVersion, StoredDouble};
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, ModelFieldValue, PartResolvedFeaturesArchive,
};

const EDIT_POINT: ModelFieldValue =
    ModelFieldValue::F64(StoredDouble::from_bits(123.5_f64.to_bits()));
const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const EMPTY: [&[u8]; 2] = [
    include_bytes!("fixtures/face-blend-items/owned-0-0.bin"),
    include_bytes!("fixtures/face-blend-items/owned-1-0.bin"),
];
const FRESH: [&[u8]; 2] = [
    include_bytes!("fixtures/face-blend-items/owned-0-1.bin"),
    include_bytes!("fixtures/face-blend-items/owned-1-1.bin"),
];
const ORIGINALS: [&[u8]; 2] = [
    include_bytes!("fixtures/face-blend-items/owned-0-2.bin"),
    include_bytes!("fixtures/face-blend-items/owned-1-2.bin"),
];
const EDITS: [&[u8]; 2] = [
    include_bytes!("fixtures/face-blend-items/owned-0-2-edited.bin"),
    include_bytes!("fixtures/face-blend-items/owned-1-2-edited.bin"),
];
const ITEM_PATH: &str = "root.features[0].items[0].point_18[0]";
const PAIR_PATH: &str = "root.features[0].items[0].first_08.point_18[0]";

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

fn point_words(archive: &PartResolvedFeaturesArchive) -> Vec<ModelFieldValue> {
    archive
        .fields()
        .iter()
        .filter(|field| field.path.ends_with("point_18[0]"))
        .map(|field| field.value.clone())
        .collect()
}

fn assert_point_words(archive: &PartResolvedFeaturesArchive, first: f64) {
    let expected = vec![
        ModelFieldValue::F64(StoredDouble::from_bits(first.to_bits())),
        ModelFieldValue::F64(StoredDouble::from_bits((-0.0_f64).to_bits())),
    ];
    assert_eq!(point_words(archive), expected);
}

fn changed_point(
    archive: &PartResolvedFeaturesArchive,
    index: usize,
    value: ModelFieldValue,
) -> Vec<bessemer::model_archives::ModelField> {
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == [ITEM_PATH, PAIR_PATH][index])
        .unwrap()
        .value = value;
    fields
}

#[test]
fn complete_face_blend_item_documents_reconstruct_null_fresh_and_shared_feature_references() {
    for (index, features) in EMPTY.into_iter().enumerate() {
        assert_document_reconstruction(features, [99, 99][index]);
    }
    for (index, features) in FRESH.into_iter().enumerate() {
        assert_document_reconstruction(features, [103, 105][index]);
    }
    for (index, features) in ORIGINALS.into_iter().chain(EDITS).enumerate() {
        assert_document_reconstruction(features, [107, 111, 107, 111][index]);
    }
}

#[test]
fn editing_one_equal_face_blend_item_preserves_the_other_and_matches_independent_bytes() {
    for (index, (original, edited)) in ORIGINALS.into_iter().zip(EDITS).enumerate() {
        let document = document_with(original);
        let archive = parsed(&document);
        assert_point_words(&archive, -0.0);
        let fields = changed_point(&archive, index, EDIT_POINT);
        let edit = archive.edit_fields(fields).unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), edited);
        assert_point_words(edit.archive(), 123.5);
        assert_eq!(edit.changes().len(), 1);
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}

#[test]
fn rejected_face_blend_item_word_type_edit_preserves_the_original_document_and_archive() {
    for (index, original) in ORIGINALS.into_iter().enumerate() {
        let document = document_with(original);
        let archive = parsed(&document);
        let fields = changed_point(&archive, index, ModelFieldValue::U16(7));
        assert!(archive.edit_fields(fields).is_err());
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}
