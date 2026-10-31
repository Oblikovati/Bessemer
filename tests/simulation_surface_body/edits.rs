use super::*;
use bessemer::simulation::SimulationSurfaceBodyValue as Value;
use bessemer::simulation::{
    SimulationSurfaceBodyHistoryParent as Parent, SimulationSurfaceBodyRelation as Relation,
};

pub(super) fn ordinal(property: &SimulationSurfaceBody, path: &str) -> usize {
    property
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == path)
        .unwrap()
}

pub(super) fn replace(
    property: &SimulationSurfaceBody,
    path: &str,
    value: SimulationSurfaceBodyValue,
) -> SimulationSurfaceBodyEdit {
    property
        .edit_field(&property.field_id(ordinal(property, path)).unwrap(), value)
        .unwrap()
}

pub(super) fn replacements() -> Vec<(&'static str, Value)> {
    let text = SimulationPersistText::from_units(Some(&[0xd800, 0, 65]), 32).unwrap();
    vec![
        ("word70", Value::Signed(12345)),
        ("text68", Value::Text(text)),
        ("doublesa8_c8[0]", Value::DoubleBits(0x7ff800000000abcd)),
    ]
}

pub(super) fn assert_outer_history(edit: &SimulationSurfaceBodyEdit, requested: usize) {
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

pub(super) fn assert_preservation(edit: &SimulationSurfaceBodyEdit, requested: usize) {
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
fn all_three_value_kinds_retain_complete_inputs_outputs_histories_and_exact_preservation() {
    let source = owned_property(870022, 1);
    let original = source.to_bytes();
    for (path, value) in replacements() {
        assert_edit_contract(&source, path, value);
    }
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn every_byte_identical_noop_preserves_all_revision_bound_outer_identities() {
    let source = owned_property(870022, 1);
    for (index, field) in source.fields().iter().enumerate() {
        let id = source.field_id(index).unwrap();
        let edit = source.edit_field(&id, field.value.clone()).unwrap();
        assert_eq!(edit.parent(), id);
        assert_eq!(edit.result(), id);
        assert_eq!(edit.archive().to_bytes(), source.to_bytes());
        assert_eq!(
            edit.coverage().completion,
            SimulationSurfaceBodyCompletion::Unchanged
        );
        for (index, row) in edit.history().iter().enumerate() {
            assert_eq!(row.result, source.field_id(index).unwrap());
            assert_eq!(row.relation, Relation::Copied);
            assert_eq!(row.parents.as_ref(), [Parent::Source(row.result.clone())]);
        }
    }
}

#[test]
fn foreign_stale_out_of_range_and_structural_requests_fail_without_mutation() {
    let source = owned_property(870022, 1);
    let foreign = owned_property(870022, 1);
    let original = source.to_bytes();
    assert!(source.field(&foreign.field_id(0).unwrap()).is_err());
    assert!(source.field_id(source.fields().len()).is_err());
    assert_invalid_requests(&source);
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn constructors_reject_fabricated_paths_and_value_kinds_without_mutation() {
    let source = owned_property(870022, 1);
    let original = source.to_bytes();
    let mut fields = source.fields().to_vec();
    fields[0].path = "fabricated.count".into();
    let error = SimulationSurfaceBody::from_fields(fields, source.version(), LIMITS).unwrap_err();
    assert!(error.cause.contains("fabricated.count") && error.cause.contains("word70"));
    let mut fields = source.fields().to_vec();
    fields[0].value = SimulationSurfaceBodyValue::Unsigned(1);
    let error = SimulationSurfaceBody::from_fields(fields, source.version(), LIMITS).unwrap_err();
    assert!(error.cause.contains("Unsigned(1)") && error.cause.contains("Signed(1)"));
    assert_eq!(source.to_bytes(), original);
}

#[test]
fn properties_own_source_inputs_mutable_copies_and_returned_buffers() {
    let expected = owned_property(870022, 1);
    let original = expected.to_bytes();
    let mut input = original.clone();
    let property = SimulationSurfaceBody::from_bytes(&input, expected.version(), LIMITS).unwrap();
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
fn all_published_surface_body_types_are_send_and_sync() {
    fn shared<T: Send + Sync>() {}
    shared::<SimulationSurfaceBody>();
    shared::<SimulationSurfaceBodyEdit>();
    shared::<SimulationSurfaceBodyField>();
    shared::<SimulationSurfaceBodyValue>();
    shared::<SimulationSurfaceBodyFieldId>();
    shared::<SimulationSurfaceBodyHistory>();
    shared::<SimulationSurfaceBodyHistoryParent>();
    shared::<SimulationSurfaceBodyRelation>();
    shared::<SimulationSurfaceBodyLimits>();
    shared::<SimulationSurfaceBodyCoverage>();
    shared::<SimulationSurfaceBodyCompletion>();
    shared::<SimulationSurfaceBodyCheckMode>();
    shared::<SimulationSurfaceBodyDiagnostic>();
}

fn assert_edit_contract(source: &SimulationSurfaceBody, path: &str, value: Value) {
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

fn assert_edit_coverage(edit: &SimulationSurfaceBodyEdit) {
    let coverage = edit.coverage();
    assert_eq!(
        coverage.completion,
        SimulationSurfaceBodyCompletion::Changed
    );
    assert_eq!(
        coverage.check_mode,
        SimulationSurfaceBodyCheckMode::ExactBodyAndPreservation
    );
    assert_eq!(coverage.requested_pattern_error_bits, 0);
    assert_eq!(coverage.checked_bytes, edit.archive().to_bytes().len());
    assert_eq!(coverage.checked_fields, edit.archive().fields().len());
    assert_eq!(coverage.body_count, 1);
    assert_eq!(coverage.diagnostics.len(), 4);
}

fn assert_invalid_requests(source: &SimulationSurfaceBody) {
    let id = source.field_id(ordinal(source, "word70")).unwrap();
    assert!(source.edit_field(&id, Value::Unsigned(1)).is_err());
    let changed = source.edit_field(&id, Value::Signed(42)).unwrap();
    assert!(changed.archive().field(&id).is_err());
    let count = source.field_id(ordinal(source, "list108.count")).unwrap();
    for value in [-1, 2] {
        assert!(source.edit_field(&count, Value::Signed(value)).is_err());
    }
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

fn assert_owned_noops(body: &SimulationSurfaceBody) {
    for (ordinal, field) in body.fields().iter().enumerate() {
        let id = body.field_id(ordinal).unwrap();
        let edit = body.edit_field(&id, field.value.clone()).unwrap();
        assert_eq!(edit.parent(), id);
        assert_eq!(edit.result(), id);
        assert_eq!(edit.archive().to_bytes(), body.to_bytes());
        assert_eq!(
            edit.coverage().completion,
            SimulationSurfaceBodyCompletion::Unchanged
        );
    }
}

#[test]
fn every_raw_text_frame_belongs_to_the_complete_published_result_parent() {
    let source = owned_property(870022, 3);
    let edit = replace(
        &source,
        "text68",
        Value::Text(SimulationPersistText::from_units(Some(&[65]), 8).unwrap()),
    );
    for body in [&source, edit.source(), edit.archive()] {
        for field in body.fields() {
            if let Value::Text(text) = &field.value {
                assert_eq!(text.source_parent(), body.to_bytes());
            }
        }
    }
    let Value::Text(odd) = &edit.archive().fields()[ordinal(edit.archive(), "list108.0")].value
    else {
        panic!("list108.0; expected raw text");
    };
    assert!(!odd.is_null());
    assert_eq!(odd.utf16_units(), None);
    assert_eq!(odd.payload(), &[0x33, 0, 0]);
}
