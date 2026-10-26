use super::annotation_view_tests::{LIMITS, decode};
use super::object_values::stored_object_integer;
use super::reader::ModelArchiveContext;
use super::{ModelDecoder, ModelField, ModelFieldValue};
use crate::FailureClass;
use crate::archive_codec::{ArchiveMapIndex, ObjectRecord};
use crate::header::ArchiveVersion;

fn dimension_fields() -> Vec<ModelField> {
    let source = include_bytes!("../fixtures/model-archives/sketch-text-original.bin");
    decode(19000, source).unwrap()
}

fn original_record(fields: &[ModelField]) -> ObjectRecord {
    let ModelFieldValue::ObjectTag(tag) = &fields[0].value else {
        panic!("expected object tag");
    };
    tag.record.clone()
}

fn alias(record: ObjectRecord, replacement_index: Option<u32>) -> ObjectRecord {
    let ObjectRecord::New { index, class } = record else {
        panic!("expected new object");
    };
    ObjectRecord::Reference {
        index: replacement_index.map_or(index, |value| ArchiveMapIndex::nonzero(value).unwrap()),
        class,
    }
}

fn curve_flag(fields: &[ModelField], record: &ObjectRecord) -> i32 {
    stored_object_integer(fields, record, "has_curve_data").unwrap()
}

#[test]
fn stored_integer_lookup_resolves_new_objects_and_aliases_by_typed_identity() {
    let fields = dimension_fields();
    let record = original_record(&fields);
    assert_eq!(curve_flag(&fields, &record), 1);
    let reference = alias(record, None);
    assert_eq!(curve_flag(&fields, &reference), 1);
    let mut changed = fields.clone();
    let field = changed
        .iter_mut()
        .find(|field| field.path.as_ref() == "root.object.has_curve_data")
        .unwrap();
    field.value = ModelFieldValue::I32(255);
    assert_eq!(curve_flag(&changed, &reference), 255);
    assert_eq!(curve_flag(&fields, &reference), 1);
}

#[test]
fn stored_integer_lookup_never_defaults_missing_or_wrong_typed_body_facts() {
    let fields = dimension_fields();
    let record = original_record(&fields);
    let missing = alias(record.clone(), Some(12345));
    let error = stored_object_integer(&fields, &missing, "has_curve_data").unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("12345"));
    for label in ["missing_fact", "text"] {
        assert_eq!(
            stored_object_integer(&fields, &record, label)
                .unwrap_err()
                .class,
            FailureClass::Missing
        );
    }
    let error = stored_object_integer(&fields, &ObjectRecord::Null, "has_curve_data").unwrap_err();
    assert_eq!(error.class, FailureClass::Invalid);
    assert!(error.cause.contains("Null"));
}

#[test]
fn missing_referenced_body_facts_poison_the_operation_before_publication() {
    let mut reader = ModelDecoder::new_in_context(
        &[],
        ArchiveVersion::new(19000),
        LIMITS,
        ModelArchiveContext::OrdinaryPartExternalXml,
    )
    .unwrap();
    let record = alias(original_record(&dimension_fields()), Some(12345));
    assert_eq!(
        reader
            .object_integer(&record, "has_curve_data")
            .unwrap_err()
            .class,
        FailureClass::Missing
    );
    assert_eq!(reader.finish().unwrap_err().class, FailureClass::Missing);
}

fn list_fields(extended: bool) -> Vec<ModelField> {
    let name = b"sgSplineList_c";
    let mut bytes = [
        65535u16.to_le_bytes(),
        7u16.to_le_bytes(),
        (name.len() as u16).to_le_bytes(),
    ]
    .concat();
    bytes.extend_from_slice(name);
    bytes.extend_from_slice(if extended {
        &[255, 255, 2, 0, 0, 0]
    } else {
        &[2, 0]
    });
    bytes.extend_from_slice(&[0, 0, 2, 0]);
    decode(19000, &bytes).unwrap()
}

#[test]
fn stored_counts_resolve_compact_extended_and_alias_list_facts() {
    use super::object_values::stored_object_count;
    for extended in [false, true] {
        let fields = list_fields(extended);
        let record = original_record(&fields);
        assert_eq!(
            stored_object_count(&fields, &record, "items.count").unwrap(),
            2
        );
        let reference = alias(record, None);
        assert_eq!(
            stored_object_count(&fields, &reference, "items.count").unwrap(),
            2
        );
    }
}

#[test]
fn stored_counts_reject_missing_extended_payloads_and_wrong_short_types() {
    use super::object_values::stored_object_count;
    let mut fields = list_fields(false);
    let record = original_record(&fields);
    let index = fields
        .iter()
        .position(|field| field.path.as_ref() == "root.object.items.count.count_word")
        .unwrap();
    for value in [ModelFieldValue::U16(u16::MAX), ModelFieldValue::I32(2)] {
        fields[index].value = value;
        let error = stored_object_count(&fields, &record, "items.count").unwrap_err();
        assert_eq!(error.class, FailureClass::Missing);
    }
}

#[test]
fn stored_counts_reject_wrong_extended_field_types() {
    use super::object_values::stored_object_count;
    let mut fields = list_fields(true);
    let record = original_record(&fields);
    fields
        .iter_mut()
        .find(|field| field.path.ends_with("count_dword"))
        .unwrap()
        .value = ModelFieldValue::I32(2);
    assert_eq!(
        stored_object_count(&fields, &record, "items.count")
            .unwrap_err()
            .class,
        FailureClass::Missing
    );
}
