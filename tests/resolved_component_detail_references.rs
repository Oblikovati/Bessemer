//! Complete detail and guarded current-part component references (#54).
#[path = "common/resolved_identifier_contract.rs"]
mod contract;
const CASES: [(&[u8], usize); 28] = [
    (
        include_bytes!("fixtures/component-detail-references/owned-0-0.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-0-1.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-0-2.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-0-3.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-0-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-0-5.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-1-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-1-1.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-1-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-1-3.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-1-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-1-5.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-0-2-edited.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/component-detail-references/owned-1-2-edited.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-1-0.bin"),
        103,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-1-1.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-1-2.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-1-3.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-1-4.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-1-5.bin"),
        111,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-2-0.bin"),
        101,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-2-1.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-2-2.bin"),
        105,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-2-3.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-2-4.bin"),
        99,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-2-5.bin"),
        109,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-1-2-edited.bin"),
        107,
    ),
    (
        include_bytes!("fixtures/component-detail-references/current-2-2-edited.bin"),
        105,
    ),
];
#[test]
fn complete_null_fresh_and_cached_component_detail_documents_reconstruct_and_isolate_inputs() {
    for (bytes, slots) in CASES {
        contract::check_document(bytes, slots);
    }
}
#[test]
fn detail_and_component_signed_edits_match_independent_goldens() {
    contract::check_word_edit(CASES[2].0, CASES[12].0, "root.features[0].items[0].raw_78");
    contract::check_word_edit(CASES[8].0, CASES[13].0, "root.features[0].items[0].raw_100");
    contract::check_word_edit(
        CASES[16].0,
        CASES[26].0,
        "root.features[0].items[0].raw_100",
    );
    contract::check_word_edit(
        CASES[22].0,
        CASES[27].0,
        "root.features[0].items[0].raw_100",
    );
}
#[test]
fn rejected_component_suffix_types_preserve_original_inputs() {
    for index in [8, 16, 22] {
        contract::check_rejected_edit(CASES[index].0, "root.features[0].items[0].raw_100");
    }
}

use bessemer::header::ArchiveVersion;
use bessemer::model_archives::{
    ConfigurationId, ModelArchiveLimits, PartConfigurationArchive, PartResolvedFeaturesArchive,
};
const LIMITS: ModelArchiveLimits = ModelArchiveLimits {
    maximum_bytes: 30000,
    maximum_fields: 10000,
    maximum_entries: 10000,
    maximum_depth: 64,
    maximum_objects: 256,
};

fn parse_details(document: &bessemer::Document) -> bessemer::Result<PartResolvedFeaturesArchive> {
    PartResolvedFeaturesArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
}

fn source_with_null_unit_owner(document: &bessemer::Document) -> Vec<u8> {
    use bessemer::model_archives::ModelFieldValue;
    let source = parsed_part(document);
    let mut fields = source.fields().to_vec();
    let null=fields.iter().find(|f|matches!(&f.value,ModelFieldValue::ObjectTag(tag) if matches!(tag.record,bessemer::model_archives::ModelObjectRecord::Null))).unwrap().value.clone();
    fields
        .iter_mut()
        .find(|f| f.path.as_ref() == "root.unit_component.owner")
        .unwrap()
        .value = null;
    source
        .edit_fields(fields)
        .unwrap()
        .archive()
        .to_bytes()
        .unwrap()
}

#[test]
fn cached_unit_with_a_null_owner_is_rejected_without_mutating_the_input_document() {
    let mut document = contract::document_with(CASES[20].0);
    let configuration = source_with_null_unit_owner(&document);
    document
        .replace_stream("Contents/Config-0", configuration)
        .unwrap();
    let before = document.to_bytes().unwrap();
    let error = parse_details(&document).unwrap_err();
    assert!(error.to_string().contains("unit owner Null"));
    assert_eq!(document.to_bytes().unwrap(), before);
    document
        .replace_stream("Contents/Config-0-ResolvedFeatures", CASES[6].0.to_vec())
        .unwrap();
    assert!(parse_details(&document).is_ok());
}

fn parsed_part(document: &bessemer::Document) -> PartConfigurationArchive {
    PartConfigurationArchive::from_document(
        document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        LIMITS,
    )
    .unwrap()
}
