//! Complete helices and wire-edge identifiers with shared references and branch-aware edits (#54).
use bessemer::Document;
use bessemer::header::{ArchiveVersion, StoredDouble};
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
const CASES: [&[u8]; 15] = [
    include_bytes!("fixtures/helices/owned-helix-0.bin"),
    include_bytes!("fixtures/helices/owned-helix-1.bin"),
    include_bytes!("fixtures/helices/owned-helix-2.bin"),
    include_bytes!("fixtures/helices/owned-helix-3.bin"),
    include_bytes!("fixtures/helices/owned-helix-4.bin"),
    include_bytes!("fixtures/helices/owned-helix-5.bin"),
    include_bytes!("fixtures/helices/owned-helix-6.bin"),
    include_bytes!("fixtures/helices/owned-helix-7.bin"),
    include_bytes!("fixtures/helices/owned-helix-8.bin"),
    include_bytes!("fixtures/helices/owned-helix-alias.bin"),
    include_bytes!("fixtures/helices/owned-wire-0.bin"),
    include_bytes!("fixtures/helices/owned-wire-1.bin"),
    include_bytes!("fixtures/helices/owned-wire-2.bin"),
    include_bytes!("fixtures/helices/owned-helix-alias-edited.bin"),
    include_bytes!("fixtures/helices/owned-wire-2-edited.bin"),
];
const SLOTS: [usize; 15] = [
    101, 101, 101, 101, 105, 105, 107, 103, 101, 111, 99, 105, 113, 111, 113,
];
const ORIGINALS: [&[u8]; 2] = [CASES[9], CASES[12]];
const EDITS: [&[u8]; 2] = [CASES[13], CASES[14]];
const WORD_PATHS: [&str; 2] = [
    "root.features[0].items[0].payload.raw_778",
    "root.features[0].items[1].source_entity.representation.unknown_38",
];

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

fn operation_words(archive: &PartResolvedFeaturesArchive, index: usize) -> Vec<ModelFieldValue> {
    let suffix = ["payload.raw_778", "source_entity.representation.unknown_38"][index];
    archive
        .fields()
        .iter()
        .filter(|field| field.path.ends_with(suffix))
        .map(|field| field.value.clone())
        .collect()
}

fn assert_operation_words(archive: &PartResolvedFeaturesArchive, first: i32, index: usize) {
    let expected = vec![ModelFieldValue::I32(first), ModelFieldValue::I32(i32::MIN)];
    assert_eq!(operation_words(archive, index), expected);
}

#[test]
fn complete_helix_and_wire_documents_reconstruct_all_inherited_and_shared_references() {
    for (features, slots) in CASES.into_iter().zip(SLOTS) {
        assert_document_reconstruction(features, slots);
    }
}

#[test]
fn editing_one_equal_helix_or_identifier_preserves_the_other_and_matches_independent_bytes() {
    for (index, (original, edited)) in ORIGINALS.into_iter().zip(EDITS).enumerate() {
        let document = document_with(original);
        let archive = parsed(&document);
        assert_operation_words(&archive, i32::MIN, index);
        let mut fields = archive.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == WORD_PATHS[index])
            .unwrap()
            .value = ModelFieldValue::I32(12345);
        let edit = archive.edit_fields(fields).unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), edited);
        assert_operation_words(edit.archive(), 12345, index);
        assert_eq!(edit.changes().len(), 1);
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}

#[test]
fn rejected_helix_or_identifier_word_type_edit_preserves_the_original_document_and_archive() {
    for (index, original) in ORIGINALS.into_iter().enumerate() {
        let document = document_with(original);
        let archive = parsed(&document);
        let mut fields = archive.fields().to_vec();
        fields
            .iter_mut()
            .find(|field| field.path.as_ref() == WORD_PATHS[index])
            .unwrap()
            .value = ModelFieldValue::U16(7);
        assert!(archive.edit_fields(fields).is_err());
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}

#[test]
fn serialized_helix_wire_requires_an_explicit_kernel_schema_without_modifying_input() {
    let original = CASES[4];
    let document = document_with(original);
    let error = PartResolvedFeaturesArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Missing);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}

fn rejected_branch_edit(path: &str, value: ModelFieldValue) {
    let original = CASES[9];
    let document = document_with(original);
    let archive = parsed(&document);
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == path)
        .unwrap()
        .value = value;
    assert!(archive.edit_fields(fields).is_err());
    assert_eq!(archive.to_bytes().unwrap(), original);
    assert_eq!(document.read_stream(STREAM).unwrap(), original);
}

#[test]
fn rejected_table_and_threshold_branch_edits_preserve_input_when_dependent_fields_remain() {
    rejected_branch_edit(
        "root.features[0].items[0].payload.dimension_table_presence",
        ModelFieldValue::U8(2),
    );
    rejected_branch_edit(
        "root.features[0].items[0].payload.data_788.raw_28",
        ModelFieldValue::F64(StoredDouble::from_bits(0)),
    );
}
