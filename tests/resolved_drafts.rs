//! Complete draft document reconstruction, raw-mode preservation and independent edits (#54).
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
const ORIGINAL: &[u8] = include_bytes!("fixtures/drafts/owned-1.bin");
const EDITED: &[u8] = include_bytes!("fixtures/drafts/owned-1-edited.bin");
const VARIANTS: [&[u8]; 6] = [
    include_bytes!("fixtures/drafts/owned-0.bin"),
    ORIGINAL,
    include_bytes!("fixtures/drafts/owned-2.bin"),
    include_bytes!("fixtures/drafts/owned-3.bin"),
    include_bytes!("fixtures/drafts/owned-4.bin"),
    include_bytes!("fixtures/drafts/owned-5.bin"),
];
const KEEPERS: [&[u8]; 2] = [
    include_bytes!("fixtures/drafts/owned-keeper-1.bin"),
    include_bytes!("fixtures/drafts/owned-keeper-2.bin"),
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

fn assert_raw_word(archive: &PartResolvedFeaturesArchive, label: &str, expected: i32) {
    let path = format!("root.features[0].payload.{label}");
    let field = archive
        .fields()
        .iter()
        .find(|field| field.path.as_ref() == path)
        .unwrap();
    assert_eq!(field.value, ModelFieldValue::I32(expected));
}

#[test]
fn all_complete_draft_variants_and_both_tagged_keepers_reconstruct_documents() {
    for (features, slots) in VARIANTS.into_iter().zip([101, 115, 115, 115, 117, 115]) {
        assert_document_reconstruction(features, slots);
    }
    assert_document_reconstruction(EDITED, 115);
    for features in KEEPERS {
        assert_document_reconstruction(features, 103);
    }
}

#[test]
fn full_width_draft_word_edit_matches_independent_bytes_without_changing_neighbors() {
    let document = document_with(ORIGINAL);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.features[0].payload.raw_830")
        .unwrap()
        .value = ModelFieldValue::I32(12345);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), EDITED);
    assert_eq!(edit.archive().object_slots(), (97, 115));
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), ORIGINAL);
    assert_eq!(document.read_stream(STREAM).unwrap(), ORIGINAL);
}

#[test]
fn invalid_group_count_and_stride_edits_leave_the_source_draft_and_document_unchanged() {
    let document = document_with(ORIGINAL);
    let archive = parsed(&document);
    for (label, value) in [
        ("face_groups_890.count", ModelFieldValue::I32(-1)),
        ("dynamic_doubles_878.element_size", ModelFieldValue::U32(6)),
    ] {
        let mut fields = archive.fields().to_vec();
        let path = format!("root.features[0].payload.{label}");
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == path)
            .unwrap()
            .value = value;
        assert!(archive.edit_fields(fields).is_err());
        assert_eq!(archive.to_bytes().unwrap(), ORIGINAL);
        assert_eq!(document.read_stream(STREAM).unwrap(), ORIGINAL);
    }
}

#[test]
fn independent_signed_presence_words_and_zero_before_native_option_conversion_are_retained() {
    let words = [
        (0, 0, 0),
        (-1, -1, -1),
        (i32::MIN, 1, i32::MIN),
        (0, i32::MIN, 0),
        (1, 0, 1),
        (i32::MAX, 2, i32::MAX),
    ];
    for (features, (array, dynamic, groups)) in VARIANTS.into_iter().zip(words) {
        let archive = parsed(&document_with(features));
        assert_raw_word(&archive, "array_818.presence", array);
        assert_raw_word(&archive, "array_820.presence", array);
        assert_raw_word(&archive, "raw_870", dynamic);
        assert_raw_word(&archive, "face_groups_presence", groups);
        assert_raw_word(&archive, "raw_844", 0);
    }
}
