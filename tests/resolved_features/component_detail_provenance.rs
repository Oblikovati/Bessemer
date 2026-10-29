use super::component_detail_provenance::require_internal_unit;
use super::{ConfigurationId, ModelField, ModelFieldValue, PartConfigurationArchive};
use crate::archive_codec::{ArchiveMapIndex, ObjectRecord};
use crate::header::ArchiveVersion;

/// Build the unchanged internal-part fixture; e.g. `source_archive()`.
pub(super) fn source_archive() -> PartConfigurationArchive {
    let document =
        crate::Document::from_bytes(include_bytes!("../fixtures/blank-part.SLDPRT")).unwrap();
    PartConfigurationArchive::from_document(
        &document,
        ConfigurationId::new(0),
        ArchiveVersion::new(19000),
        None,
        super::annotation_view_tests::LIMITS,
    )
    .unwrap()
}

fn current_unit(source: &PartConfigurationArchive) -> (ObjectRecord, String) {
    source
        .fields()
        .iter()
        .find_map(|field| {
            let ModelFieldValue::ObjectTag(tag) = &field.value else {
                return None;
            };
            let ObjectRecord::New { class, .. } = &tag.record else {
                return None;
            };
            (class.name == "moUnitComponent_c")
                .then(|| (tag.record.clone(), field.path.to_string()))
        })
        .unwrap()
}

fn changed(fields: &[ModelField], path: &str, value: ModelFieldValue) -> Vec<ModelField> {
    let mut result = fields.to_vec();
    result
        .iter_mut()
        .find(|f| f.path.as_ref() == path)
        .unwrap()
        .value = value;
    result
}

#[test]
fn completed_part_root_proof_matches_the_actual_parsed_new_root() {
    let source = source_archive();
    let root = source
        .fields()
        .iter()
        .find(|f| f.path.as_ref() == "root.root")
        .unwrap();
    let ModelFieldValue::ObjectTag(tag) = &root.value else {
        panic!("expected root tag")
    };
    let ObjectRecord::New { index, class } = &tag.record else {
        panic!("expected fresh root")
    };
    assert_eq!(class.name, "moPart_c");
    assert_eq!(source.parsed.verified_root().index(), *index);
}

#[test]
fn null_component_requires_no_current_part_proof() {
    assert!(require_internal_unit(&[], &ObjectRecord::Null, None).is_ok());
}

#[test]
fn complete_fresh_and_cached_unit_facts_require_the_same_current_part_proof() {
    let source = source_archive();
    let (unit, _) = current_unit(&source);
    let root = Some(source.parsed.verified_root());
    assert!(require_internal_unit(source.fields(), &unit, root).is_ok());
    let ObjectRecord::New { index, class } = unit else {
        panic!("expected new unit")
    };
    let cached = ObjectRecord::Reference { index, class };
    assert!(require_internal_unit(source.fields(), &cached, root).is_ok());
    assert!(require_internal_unit(source.fields(), &cached, None).is_err());
}

#[test]
fn typed_unit_name_does_not_make_a_different_mapped_owner_current() {
    let source = source_archive();
    let (unit, prefix) = current_unit(&source);
    let root = source.parsed.verified_root();
    let owner = super::object_values::stored_object_tag(source.fields(), &unit, "owner").unwrap();
    let ObjectRecord::Reference { class, .. } = owner else {
        panic!("expected owner reference")
    };
    let other = ObjectRecord::Reference {
        index: ArchiveMapIndex::new(root.index().value() + 1).unwrap(),
        class: class.clone(),
    };
    let fields = changed_owner(source.fields(), &format!("{prefix}.owner"), other);
    let error = require_internal_unit(&fields, &unit, Some(root)).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("expected identical mapped owner")
    );
}

#[test]
fn null_or_missing_owner_facts_never_select_the_internal_unit_branch() {
    let source = source_archive();
    let (unit, prefix) = current_unit(&source);
    let root = Some(source.parsed.verified_root());
    let null=source.fields().iter().find(|f|matches!(&f.value,ModelFieldValue::ObjectTag(tag) if matches!(tag.record,ObjectRecord::Null))).unwrap().value.clone();
    let fields = changed(source.fields(), &format!("{prefix}.owner"), null);
    assert!(require_internal_unit(&fields, &unit, root).is_err());
    assert!(require_internal_unit(&[], &unit, root).is_err());
}

#[test]
fn nonzero_or_wrongly_typed_external_presence_never_selects_internal_loading() {
    let source = source_archive();
    let (unit, prefix) = current_unit(&source);
    let root = Some(source.parsed.verified_root());
    for value in [
        ModelFieldValue::U8(1),
        ModelFieldValue::U8(255),
        ModelFieldValue::U16(0),
    ] {
        let fields = changed(
            source.fields(),
            &format!("{prefix}.has_external_model"),
            value,
        );
        assert!(require_internal_unit(&fields, &unit, root).is_err());
    }
    assert!(require_internal_unit(source.fields(), &unit, root).is_ok());
}

#[test]
fn unrelated_component_classes_never_gain_internal_loading_from_root_proof() {
    let source = source_archive();
    let root = source
        .fields()
        .iter()
        .find(|f| f.path.as_ref() == "root.root")
        .unwrap();
    let ModelFieldValue::ObjectTag(tag) = &root.value else {
        panic!("expected root tag")
    };
    assert!(
        require_internal_unit(
            source.fields(),
            &tag.record,
            Some(source.parsed.verified_root())
        )
        .is_err()
    );
}

fn changed_owner(fields: &[ModelField], path: &str, owner: ObjectRecord) -> Vec<ModelField> {
    let mut fields = fields.to_vec();
    let field = fields.iter_mut().find(|f| f.path.as_ref() == path).unwrap();
    let ModelFieldValue::ObjectTag(tag) = &mut field.value else {
        panic!("expected owner tag")
    };
    tag.record = owner;
    fields
}

#[test]
fn real_inherited_fields_and_class_context_do_not_grant_current_root_provenance() {
    use crate::archive_codec::{MaterialRead, ObjectReader};
    let source = source_archive();
    let bytes = include_bytes!("../fixtures/component-detail-references/current-2-0.bin");
    let mut reader = unproven_reader(&source, bytes);
    let mut map = ObjectReader::from_snapshot(&source.parsed.object_map);
    reader.i32("raw_prefix").unwrap();
    assert_eq!(reader.object_count("features.count").unwrap(), 1);
    let error = reader
        .object("features[0]", &mut map, super::ModelObjectRole::Any)
        .unwrap_err();
    assert!(error.to_string().contains("current-part proof None"));
    assert!(reader.finish().is_err());
}

fn unproven_reader<'a>(
    source: &'a PartConfigurationArchive,
    bytes: &'a [u8],
) -> super::ModelDecoder<'a> {
    super::ModelDecoder::new_in_context(
        bytes,
        ArchiveVersion::new(19000),
        super::annotation_view_tests::LIMITS,
        super::reader::ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap()
    .with_inherited_fields(source.fields())
}
