use super::*;

pub(super) fn each_owned(
    mut check: impl FnMut(usize, SimulationPersistVersion, SimulationResultsProperty),
) {
    let mut cursor = EXPECTED;
    while !cursor.is_empty() {
        let (level, version, body) = read_body(&mut cursor);
        consume_expected_fields(&mut cursor);
        check(level, version, body);
    }
}

fn assert_noop(body: &SimulationResultsProperty, ordinal: usize) {
    let id = body.field_id(ordinal).unwrap();
    let field = body.field(&id).unwrap();
    let edit = body.edit_field(&id, field.value.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), body.to_bytes());
    assert_eq!(edit.result(), id);
    assert_eq!(edit.parent(), id);
    assert_eq!(edit.replacement(), &field.value);
    assert_eq!(
        edit.coverage().completion,
        SimulationResultsCompletion::Unchanged
    );
    assert!(
        edit.history()
            .iter()
            .all(|row| row.relation == SimulationResultsRelation::Copied)
    );
    assert!(edit.nested_history().is_empty());
}

#[test]
fn all_fields_preserve_noop_revisions_and_reject_foreign_or_invalid_requests() {
    each_owned(|_, _, body| {
        for ordinal in 0..body.fields().len() {
            assert_noop(&body, ordinal);
        }
        assert_invalid_requests(&body);
    });
}

fn assert_invalid_requests(body: &SimulationResultsProperty) {
    let id = body.field_id(0).unwrap();
    let copied =
        SimulationResultsProperty::from_fields(body.fields().to_vec(), body.context(), LIMITS)
            .unwrap();
    assert!(copied.field(&id).is_err());
    assert!(body.field_id(body.fields().len()).is_err());
    assert!(
        body.edit_field(&id, SimulationResultsValue::Unsigned(42))
            .is_err()
    );
    let edit = body
        .edit_field(&id, SimulationResultsValue::Signed(42))
        .unwrap();
    if body.fields()[0].value != SimulationResultsValue::Signed(42) {
        assert!(edit.archive().field(&id).is_err());
    }
}

#[test]
fn current_writers_are_explicit_and_only_match_compatible_reader_shapes() {
    each_owned(|level, version, body| {
        let writer = match (level, version.value()) {
            (0, 890605) => SimulationResultsContext::RootCurrentWriter,
            (1, 810010) => SimulationResultsContext::CategoryCurrentWriter,
            _ => return,
        };
        let rebuilt =
            SimulationResultsProperty::from_fields(body.fields().to_vec(), writer, LIMITS).unwrap();
        assert_eq!(rebuilt.to_bytes(), body.to_bytes());
        assert_eq!(rebuilt.context(), writer);
        assert_eq!(rebuilt.coverage().context, writer);
        assert_writer_edits(&rebuilt);
        let old = SimulationPersistVersion::new(0).unwrap();
        assert!(
            SimulationResultsProperty::from_bytes(&rebuilt.to_bytes(), context(level, old), LIMITS)
                .is_err()
        );
    });
}

fn assert_writer_edits(body: &SimulationResultsProperty) {
    for (index, replacement) in typed_edits::choices(body) {
        let edit = typed_edits::replace(body, index, replacement);
        assert_eq!(edit.archive().context(), body.context());
        assert_noop(body, index);
    }
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn every_public_results_contract_is_send_and_sync() {
    assert_send_sync::<SimulationResultsProperty>();
    assert_send_sync::<SimulationResultsContext>();
    assert_send_sync::<SimulationResultsEdit>();
    assert_send_sync::<SimulationResultsCoverage>();
    assert_send_sync::<SimulationResultsCompletion>();
    assert_send_sync::<SimulationResultsField>();
    assert_send_sync::<SimulationResultsValue>();
    assert_send_sync::<SimulationResultsFieldId>();
    assert_send_sync::<SimulationResultsHistory>();
    assert_send_sync::<SimulationResultsHistoryParent>();
    assert_send_sync::<SimulationResultsNestedHistory>();
    assert_send_sync::<SimulationResultsNestedParent>();
    assert_send_sync::<SimulationResultsRelation>();
    assert_send_sync::<SimulationResultsLimits>();
    assert_send_sync::<SimulationResultsCheckMode>();
    assert_send_sync::<SimulationResultsDiagnostic>();
}

fn assert_caller_isolation(body: &SimulationResultsProperty) {
    let original = body.to_bytes();
    let mut caller = original.clone();
    let read = SimulationResultsProperty::from_bytes(&caller, body.context(), LIMITS).unwrap();
    caller.fill(255);
    assert_eq!(read.to_bytes(), original);
    let mut fields = read.fields().to_vec();
    let built =
        SimulationResultsProperty::from_fields(fields.clone(), body.context(), LIMITS).unwrap();
    fields[0].path = "caller-mutated".into();
    fields[0].value = SimulationResultsValue::Unsigned(0);
    assert_eq!(built.fields(), body.fields());
    assert_eq!(built.to_bytes(), original);
    let id = read.field_id(0).unwrap();
    assert!(
        read.edit_field(&id, SimulationResultsValue::Unsigned(0))
            .is_err()
    );
    assert_eq!(read.field_id(0).unwrap(), id);
    assert_eq!(read.to_bytes(), original);
    assert_eq!(body.to_bytes(), original);
}

#[test]
fn mutable_caller_inputs_and_vectors_are_isolated_after_success_and_failure() {
    each_owned(|_, _, body| assert_caller_isolation(&body));
}
