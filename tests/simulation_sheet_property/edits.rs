use super::*;
use bessemer::simulation::SimulationSheetValue as Value;
use bessemer::simulation::{
    SimulationSheetHistoryParent as Parent, SimulationSheetRelation as Relation,
};

pub(super) fn ordinal(property: &SimulationSheetProperty, path: &str) -> usize {
    property
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap()
}

pub(super) fn replace(
    property: &SimulationSheetProperty,
    path: &str,
    value: SimulationSheetValue,
) -> SimulationSheetEdit {
    property
        .edit_field(&property.field_id(ordinal(property, path)).unwrap(), value)
        .unwrap()
}

fn embedded_value(extended: bool) -> Value {
    Value::Embedded(Box::new(load_strings::framed_string_archive(
        extended,
        LIMITS.embedded,
    )))
}

pub(super) fn replacements() -> Vec<(&'static str, Value)> {
    let text = SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 32).unwrap();
    vec![
        ("words48_50[0]", Value::Signed(12345)),
        ("text38", Value::Text(text)),
        (
            "persistid.blob",
            Value::Bytes(std::sync::Arc::from([1_u8, 2, 3])),
        ),
        ("persistid.archive0", embedded_value(false)),
    ]
}

pub(super) fn assert_outer_history(edit: &SimulationSheetEdit, requested: usize) {
    assert_eq!(edit.history().len(), edit.source().fields().len());
    for (index, row) in edit.history().iter().enumerate() {
        let mut expected = vec![Parent::Source(edit.source().field_id(index).unwrap())];
        if index == requested {
            expected.push(Parent::Replacement);
        }
        assert_eq!(row.result, edit.archive().field_id(index).unwrap());
        assert_eq!(row.parents.as_ref(), expected);
        assert_eq!(
            row.relation,
            if index == requested {
                Relation::Derived
            } else {
                Relation::Copied
            }
        );
    }
}

pub(super) fn assert_preservation(edit: &SimulationSheetEdit, requested: usize) {
    let before = edit.source().to_bytes();
    let after = edit.archive().to_bytes();
    let (start, end) = edit.source().field_offsets()[requested];
    let (next_start, next_end) = edit.archive().field_offsets()[requested];
    assert_eq!(start, next_start);
    assert_eq!(before[..start], after[..next_start]);
    assert_eq!(before[end..], after[next_end..]);
    for index in 0..edit.source().fields().len() {
        if index != requested {
            assert_eq!(
                edit.source().fields()[index],
                edit.archive().fields()[index]
            );
        }
    }
}

#[test]
fn all_four_value_kinds_retain_complete_inputs_outputs_histories_and_exact_preservation() {
    let source = owned_property(840003, 1);
    let original = source.to_bytes();
    for (path, value) in replacements() {
        assert_edit_contract(&source, path, value);
    }
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn every_byte_identical_noop_preserves_all_revision_bound_outer_identities() {
    let source = owned_property(840003, 1);
    for (index, field) in source.fields().iter().enumerate() {
        let id = source.field_id(index).unwrap();
        let edit = source.edit_field(&id, field.value.clone()).unwrap();
        assert_eq!(edit.parent(), id);
        assert_eq!(edit.result(), id);
        assert_eq!(edit.archive().to_bytes(), source.to_bytes());
        assert_eq!(
            edit.coverage().completion,
            SimulationSheetCompletion::Unchanged
        );
        for (index, row) in edit.history().iter().enumerate() {
            assert_eq!(row.result, source.field_id(index).unwrap());
            assert_eq!(row.relation, Relation::Copied);
            assert_eq!(row.parents.as_ref(), [Parent::Source(row.result.clone())]);
        }
    }
}

#[test]
fn wire_distinct_embedded_cstring_prefix_widths_produce_real_isolated_edits() {
    let source = owned_property(840003, 1);
    let long = load_strings::framed_string_archive(true, LIMITS.embedded);
    let first = replace_embedded(&source, false);
    let second = replace_embedded(first.archive(), true);
    let index = ordinal(&source, "persistid.archive0");
    assert_eq!(
        second.archive().to_bytes().len(),
        first.archive().to_bytes().len() + 2
    );
    assert_eq!(
        second.coverage().completion,
        SimulationSheetCompletion::Changed
    );
    load_strings::assert_wire_value(&second.archive().fields()[index].value, &long);
    assert_outer_history(&second, index);
    assert_preservation(&second, index);
}

#[test]
fn foreign_stale_out_of_range_and_structural_requests_fail_without_mutation() {
    let source = owned_property(840003, 1);
    let foreign = owned_property(840003, 1);
    let original = source.to_bytes();
    assert!(source.field(&foreign.field_id(0).unwrap()).is_err());
    assert!(source.field_id(source.fields().len()).is_err());
    assert_invalid_requests(&source);
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn constructors_reject_fabricated_paths_and_value_kinds_without_mutation() {
    let source = owned_property(840003, 1);
    let original = source.to_bytes();
    let mut fields = source.fields().to_vec();
    fields[0].path = "fabricated.count".into();
    let error = SimulationSheetProperty::from_fields(fields, source.version(), LIMITS).unwrap_err();
    assert!(error.cause.contains("fabricated.count") && error.cause.contains("persistid.words[0]"));
    let mut fields = source.fields().to_vec();
    fields[0].value = SimulationSheetValue::Unsigned(1);
    let error = SimulationSheetProperty::from_fields(fields, source.version(), LIMITS).unwrap_err();
    assert!(error.cause.contains("Unsigned(1)") && error.cause.contains("Signed(1)"));
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn properties_own_source_inputs_mutable_copies_and_returned_buffers() {
    let expected = owned_property(840003, 1);
    let original = expected.to_bytes();
    let mut input = original.clone();
    let property = SimulationSheetProperty::from_bytes(&input, expected.version(), LIMITS).unwrap();
    input.fill(0);
    let mut returned = property.to_bytes();
    returned.fill(0xff);
    let mut fields = property.fields().to_vec();
    fields[0].path = "changed".into();
    fields.clear();
    assert_eq!(property.to_bytes(), original);
    assert_eq!(property.fields(), expected.fields());
}

#[test]
fn all_published_sheet_property_types_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<SimulationSheetProperty>();
    shared::<SimulationSheetEdit>();
    shared::<SimulationSheetField>();
    shared::<SimulationSheetValue>();
    shared::<SimulationSheetFieldId>();
    shared::<SimulationSheetHistory>();
    shared::<SimulationSheetHistoryParent>();
    shared::<SimulationSheetNestedHistory>();
    shared::<SimulationSheetNestedParent>();
    shared::<SimulationSheetRelation>();
    shared::<SimulationSheetLimits>();
    shared::<SimulationSheetCoverage>();
    shared::<SimulationSheetCompletion>();
    shared::<SimulationSheetCheckMode>();
    shared::<SimulationSheetDiagnostic>();
}

fn assert_edit_contract(source: &SimulationSheetProperty, path: &str, value: Value) {
    let edit = replace(source, path, value.clone());
    let requested = ordinal(source, path);
    assert_eq!(edit.source().to_bytes(), source.to_bytes());
    assert_eq!(edit.parent(), source.field_id(requested).unwrap());
    assert_eq!(edit.result(), edit.archive().field_id(requested).unwrap());
    assert_eq!(*edit.replacement(), value);
    assert_eq!(edit.archive().field(&edit.result()).unwrap().value, value);
    assert_edit_coverage(&edit);
    assert_outer_history(&edit, requested);
    assert_preservation(&edit, requested);
}

fn assert_edit_coverage(edit: &SimulationSheetEdit) {
    let coverage = edit.coverage();
    assert_eq!(coverage.completion, SimulationSheetCompletion::Changed);
    assert_eq!(
        coverage.check_mode,
        SimulationSheetCheckMode::ExactSelectedProfilesAndPreservation
    );
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.checked_bytes, edit.archive().to_bytes().len());
    assert_eq!(coverage.checked_fields, edit.archive().fields().len());
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.diagnostics.len(), 4);
}

fn assert_invalid_requests(source: &SimulationSheetProperty) {
    let id = source.field_id(ordinal(source, "words48_50[0]")).unwrap();
    assert!(source.edit_field(&id, Value::Unsigned(1)).is_err());
    let changed = source.edit_field(&id, Value::Signed(42)).unwrap();
    assert!(changed.archive().field(&id).is_err());
    let count = source
        .field_id(ordinal(source, "persistid.blob_count"))
        .unwrap();
    for value in [-1, 2] {
        assert!(source.edit_field(&count, Value::Signed(value)).is_err());
    }
}

fn replace_embedded(property: &SimulationSheetProperty, extended: bool) -> SimulationSheetEdit {
    replace(property, "persistid.archive0", embedded_value(extended))
}

#[test]
fn every_owned_raw_frame_and_field_noop_keeps_its_wire_identity() {
    let mut cursor = EXPECTED;
    while !cursor.is_empty() {
        let body = read_original(&mut cursor);
        consume_expected_fields(&mut cursor);
        assert_owned_noops(&body);
    }
}

fn assert_owned_noops(body: &SimulationSheetProperty) {
    for (ordinal, field) in body.fields().iter().enumerate() {
        let id = body.field_id(ordinal).unwrap();
        let edit = body.edit_field(&id, field.value.clone()).unwrap();
        assert_eq!(edit.parent(), id);
        assert_eq!(edit.result(), id);
        assert_eq!(edit.archive().to_bytes(), body.to_bytes());
        assert_eq!(
            edit.coverage().completion,
            SimulationSheetCompletion::Unchanged
        );
    }
}

#[test]
fn every_raw_text_frame_belongs_to_the_complete_published_result_parent() {
    let source = owned_property(840003, 3);
    let edit = replace(
        &source,
        "text38",
        Value::Text(SimulationPersistText::from_units(Some(&[65]), 8).unwrap()),
    );
    for body in [&source, edit.source(), edit.archive()] {
        for field in body.fields() {
            if let Value::Text(text) = &field.value {
                assert_eq!(text.source_parent(), body.to_bytes());
            }
        }
    }
    let Value::Text(odd) = &edit.archive().fields()[ordinal(edit.archive(), "text40")].value else {
        panic!("text40; expected raw text");
    };
    assert!(!odd.is_null());
    assert_eq!(odd.utf16_units(), None);
    assert_eq!(odd.payload(), &[0x33, 0, 0]);
}
