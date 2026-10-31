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
        let old = SimulationPersistVersion::new(0).unwrap();
        assert!(
            SimulationResultsProperty::from_bytes(&rebuilt.to_bytes(), context(level, old), LIMITS)
                .is_err()
        );
    });
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
