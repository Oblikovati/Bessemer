//! Complete solid references, selected kernel bodies and isolated unsigned-word edits (#54).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    CompressedKernelContext, ConfigurationId, ModelArchiveLimits, ModelFieldValue,
    PartResolvedFeaturesArchive,
};
use bessemer::parasolid::{BaseSchema, TransmitLimits};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const CASES: [&[u8]; 10] = [
    include_bytes!("fixtures/solid-references/owned-0-0.bin"),
    include_bytes!("fixtures/solid-references/owned-0-1.bin"),
    include_bytes!("fixtures/solid-references/owned-0-2.bin"),
    include_bytes!("fixtures/solid-references/owned-1-0.bin"),
    include_bytes!("fixtures/solid-references/owned-1-1.bin"),
    include_bytes!("fixtures/solid-references/owned-1-2.bin"),
    include_bytes!("fixtures/solid-references/owned-2-0.bin"),
    include_bytes!("fixtures/solid-references/owned-2-1.bin"),
    include_bytes!("fixtures/solid-references/owned-2-2.bin"),
    include_bytes!("fixtures/solid-references/owned-2-2-edited.bin"),
];
const SLOTS: [usize; 10] = [99, 101, 101, 101, 103, 103, 105, 107, 107, 107];
const ORIGINAL: &[u8] = CASES[8];
const EDITED: &[u8] = CASES[9];
const WORD_PATH: &str = "root.features[0].items[0].raw_a8";

fn kernel() -> CompressedKernelContext {
    CompressedKernelContext {
        base: BaseSchema::from_bytes(include_bytes!("fixtures/parasolid/base-schema.s_t")).unwrap(),
        maximum_expanded: 1024 * 1024,
        transmit_limits: TransmitLimits {
            maximum_records: 1000,
            maximum_field_elements: 10000,
        },
    }
}

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
        Some(kernel()),
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

fn solid_reference_words(archive: &PartResolvedFeaturesArchive) -> Vec<ModelFieldValue> {
    archive
        .fields()
        .iter()
        .filter(|field| field.path.ends_with("raw_a8"))
        .map(|field| field.value.clone())
        .collect()
}

fn assert_solid_reference_words(archive: &PartResolvedFeaturesArchive, first: u32) {
    let expected = vec![
        ModelFieldValue::U32(first),
        ModelFieldValue::U32(0x80000000),
    ];
    assert_eq!(solid_reference_words(archive), expected);
}

#[test]
fn complete_solid_reference_and_data_documents_reconstruct_all_tagged_and_direct_records() {
    for (features, slots) in CASES.into_iter().zip(SLOTS) {
        assert_document_reconstruction(features, slots);
    }
}

#[test]
fn editing_one_equal_solid_reference_preserves_the_other_and_matches_independent_bytes() {
    let (original, edited) = (ORIGINAL, EDITED);
    let document = document_with(original);
    let archive = parsed(&document);
    assert_solid_reference_words(&archive, 0x80000000);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == WORD_PATH)
        .unwrap()
        .value = ModelFieldValue::U32(0x12345678);
    let edit = archive.edit_fields(fields).unwrap();
    assert_eq!(edit.archive().to_bytes().unwrap(), edited);
    assert_solid_reference_words(edit.archive(), 0x12345678);
    assert_eq!(edit.changes().len(), 1);
    assert_eq!(archive.to_bytes().unwrap(), original);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}

#[test]
fn rejected_solid_reference_word_type_edit_preserves_the_original_document_and_archive() {
    let original = ORIGINAL;
    let document = document_with(original);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == WORD_PATH)
        .unwrap()
        .value = ModelFieldValue::U16(7);
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), original);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}

#[test]
fn serialized_solid_reference_requires_explicit_kernel_context_without_input_mutation() {
    let document = document_with(ORIGINAL);
    let error = PartResolvedFeaturesArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Missing);
    assert_eq!(document.read_stream(STREAM).unwrap(), ORIGINAL);
}
