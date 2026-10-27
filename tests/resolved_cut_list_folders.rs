//! Complete cut-list folders and direct property edits with inherited identities (#54).
use bessemer::Document;
use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ArchiveString, ConfigurationId, ModelArchiveLimits, ModelFieldValue,
    PartResolvedFeaturesArchive, TextEncoding,
};

const STREAM: &str = "Contents/Config-0-ResolvedFeatures";
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};
const CASES: [&[u8]; 86] = [
    include_bytes!("fixtures/cut-list-folders/owned-0-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-0-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-0-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-0-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-0-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-0-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-1-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-1-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-1-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-1-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-1-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-1-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-2-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-2-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-2-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-2-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-2-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-2-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-3-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-3-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-3-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-3-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-3-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-3-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-4-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-4-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-4-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-4-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-4-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-4-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-5-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-5-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-5-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-5-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-5-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-5-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-6-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-6-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-6-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-6-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-6-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-6-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-7-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-7-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-7-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-7-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-7-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-7-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-8-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-8-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-8-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-8-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-8-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-8-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-9-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-9-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-9-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-9-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-9-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-9-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-10-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-10-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-10-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-10-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-10-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-10-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-11-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-11-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-11-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-11-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-11-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-11-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-12-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-12-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-12-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-12-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-12-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-12-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-13-0.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-13-1.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-13-2.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-13-3.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-13-4.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-13-5.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-4-2-edited.bin"),
    include_bytes!("fixtures/cut-list-folders/owned-5-2-edited.bin"),
];
const SLOTS: [usize; 86] = [
    101, 109, 115, 109, 109, 115, 101, 109, 115, 109, 109, 115, 101, 109, 115, 109, 109, 115, 101,
    111, 117, 111, 111, 117, 101, 111, 117, 111, 111, 117, 101, 113, 119, 113, 113, 119, 101, 103,
    109, 103, 103, 109, 101, 103, 109, 103, 103, 109, 101, 103, 109, 103, 103, 109, 101, 103, 109,
    103, 103, 109, 101, 103, 109, 103, 103, 109, 101, 103, 109, 103, 103, 109, 101, 103, 109, 103,
    103, 109, 101, 103, 109, 103, 103, 109, 117, 119,
];
const ORIGINALS: [&[u8]; 2] = [CASES[26], CASES[32]];
const EDITS: [&[u8]; 2] = [CASES[84], CASES[85]];
const PROPERTY_PATHS: [&str; 2] = [
    "root.features[0].items[0].direct_properties_448.properties[0].string_20",
    "root.features[0].items[0].sub_weld.direct_properties_448.properties[0].string_20",
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

fn property_values(archive: &PartResolvedFeaturesArchive) -> Vec<ModelFieldValue> {
    archive
        .fields()
        .iter()
        .filter(|field| {
            field.path.contains("direct_properties_448") && field.path.ends_with("string_20")
        })
        .map(|field| field.value.clone())
        .collect()
}

fn assert_property_values(archive: &PartResolvedFeaturesArchive, first: &str) {
    let mut values = vec!["Density@\n材料"; 6];
    values[0] = first;
    let expected: Vec<_> = values
        .into_iter()
        .map(|value| ModelFieldValue::Text(ArchiveString::new(value, TextEncoding::Utf16).unwrap()))
        .collect();
    assert_eq!(property_values(archive), expected);
}

fn changed_property(
    archive: &PartResolvedFeaturesArchive,
    index: usize,
    value: ModelFieldValue,
) -> Vec<bessemer::model_archives::ModelField> {
    let mut fields = archive.fields().to_vec();
    fields
        .iter_mut()
        .find(|field| field.path.as_ref() == PROPERTY_PATHS[index])
        .unwrap()
        .value = value;
    fields
}

#[test]
fn complete_cut_list_and_ancestor_documents_reconstruct_all_references_and_property_lists() {
    for (features, slots) in CASES.into_iter().zip(SLOTS) {
        assert_document_reconstruction(features, slots);
    }
}

#[test]
fn editing_one_equal_cut_list_property_preserves_the_other_and_matches_independent_bytes() {
    for (index, (original, edited)) in ORIGINALS.into_iter().zip(EDITS).enumerate() {
        let document = document_with(original);
        let archive = parsed(&document);
        assert_property_values(&archive, "Density@\n材料");
        let value =
            ModelFieldValue::Text(ArchiveString::new("Bessemer Ω", TextEncoding::Utf16).unwrap());
        let fields = changed_property(&archive, index, value);
        let edit = archive.edit_fields(fields).unwrap();
        assert_eq!(edit.archive().to_bytes().unwrap(), edited);
        assert_property_values(edit.archive(), "Bessemer Ω");
        assert_eq!(edit.changes().len(), 1);
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}

#[test]
fn rejected_cut_list_property_word_type_edit_preserves_the_original_document_and_archive() {
    for (index, original) in ORIGINALS.into_iter().enumerate() {
        let document = document_with(original);
        let archive = parsed(&document);
        let fields = changed_property(&archive, index, ModelFieldValue::I32(7));
        assert!(archive.edit_fields(fields).is_err());
        assert_eq!(archive.to_bytes().unwrap(), original);
        assert_eq!(document.read_stream(STREAM).unwrap(), original);
    }
}
