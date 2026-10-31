use super::*;

pub(super) fn each_owned(mut check: impl FnMut(SimulationPostGraphParameters)) {
    let mut cursor = EXPECTED;
    while !cursor.is_empty() {
        let body = read_body(&mut cursor);
        consume_expected_fields(&mut cursor);
        check(body);
    }
}

fn assert_noop(body: &SimulationPostGraphParameters, ordinal: usize) {
    let id = body.field_id(ordinal).unwrap();
    let value = body.field(&id).unwrap().value.clone();
    let edit = body.edit_field(&id, value.clone()).unwrap();
    assert_eq!(edit.archive().to_bytes(), body.to_bytes());
    assert_eq!(edit.parent(), id);
    assert_eq!(edit.result(), id);
    assert_eq!(edit.replacement(), &value);
    assert_eq!(
        edit.coverage().completion,
        SimulationPostGraphCompletion::Unchanged
    );
    assert!(
        edit.history()
            .iter()
            .all(|row| row.relation == SimulationPostGraphRelation::Copied)
    );
    assert!(edit.nested_history().is_empty());
}

fn assert_revision_failures(body: &SimulationPostGraphParameters) {
    let id = body.field_id(0).unwrap();
    let rebuilt =
        SimulationPostGraphParameters::from_fields(body.fields().to_vec(), body.version(), LIMITS)
            .unwrap();
    assert!(rebuilt.field(&id).is_err());
    assert!(body.field_id(body.fields().len()).is_err());
    assert!(
        body.edit_field(&id, SimulationPostGraphValue::Raw64([0; 8]))
            .is_err()
    );
    let changed = body
        .edit_field(&id, SimulationPostGraphValue::Signed(42))
        .unwrap();
    assert!(changed.archive().field(&id).is_err());
    assert_eq!(changed.source().field_id(0).unwrap(), id);
}

#[test]
fn every_owned_field_preserves_noop_identity_and_rejects_foreign_stale_or_wrong_kind_requests() {
    each_owned(|body| {
        for index in 0..body.fields().len() {
            assert_noop(&body, index);
        }
        assert_revision_failures(&body);
    });
}

#[test]
fn atomic_character_text_preserves_code_units_and_enforces_complete_bounded_payloads() {
    let text = SimulationPostGraphText::from_payload(&[0, 0xd8, 0, 0], 4).unwrap();
    assert_eq!(text.character_count(), 2);
    assert_eq!(text.payload(), &[0, 0xd8, 0, 0]);
    assert_eq!(text.to_bytes(), [2, 0, 0, 0, 0, 0xd8, 0, 0]);
    assert!(SimulationPostGraphText::from_payload(&[1], 2048).is_err());
    assert!(SimulationPostGraphText::from_payload(&[0, 0], 1).is_err());
    assert!(SimulationPostGraphText::from_payload(&vec![0; 2050], 4000).is_err());
    let maximum = SimulationPostGraphText::from_payload(&vec![0; 2048], 2048).unwrap();
    assert_eq!(maximum.character_count(), 1024);
    assert!(
        SimulationPostGraphText::from_payload(&[], 0)
            .unwrap()
            .payload()
            .is_empty()
    );
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn every_public_graph_contract_is_send_and_sync() {
    assert_send_sync::<SimulationPostGraphParameters>();
    assert_send_sync::<SimulationPostGraphEdit>();
    assert_send_sync::<SimulationPostGraphField>();
    assert_send_sync::<SimulationPostGraphValue>();
    assert_send_sync::<SimulationPostGraphText>();
    assert_send_sync::<SimulationPostGraphFieldId>();
    assert_send_sync::<SimulationPostGraphHistory>();
    assert_send_sync::<SimulationPostGraphHistoryParent>();
    assert_send_sync::<SimulationPostGraphNestedHistory>();
    assert_send_sync::<SimulationPostGraphNestedParent>();
    assert_send_sync::<SimulationPostGraphRelation>();
    assert_send_sync::<SimulationPostGraphCoverage>();
    assert_send_sync::<SimulationPostGraphCompletion>();
    assert_send_sync::<SimulationPostGraphLimits>();
    assert_send_sync::<SimulationPostGraphCheckMode>();
    assert_send_sync::<SimulationPostGraphDiagnostic>();
}

fn assert_caller_isolation(body: &SimulationPostGraphParameters) {
    let original = body.to_bytes();
    let mut caller = original.clone();
    let read = SimulationPostGraphParameters::from_bytes(&caller, body.version(), LIMITS).unwrap();
    caller.fill(255);
    assert_eq!(read.to_bytes(), original);
    let mut fields = read.fields().to_vec();
    let built =
        SimulationPostGraphParameters::from_fields(fields.clone(), body.version(), LIMITS).unwrap();
    fields[0].path = "caller-mutated".into();
    fields[0].value = SimulationPostGraphValue::Raw64([255; 8]);
    assert_eq!(built.fields(), body.fields());
    assert_eq!(built.to_bytes(), original);
    let id = read.field_id(0).unwrap();
    assert!(
        read.edit_field(&id, SimulationPostGraphValue::Raw64([0; 8]))
            .is_err()
    );
    assert_eq!(read.field_id(0).unwrap(), id);
    assert_eq!(read.to_bytes(), original);
    assert_eq!(body.to_bytes(), original);
}

#[test]
fn mutable_caller_buffers_and_field_vectors_remain_isolated_after_success_and_failure() {
    each_owned(|body| assert_caller_isolation(&body));
    let mut code_units = vec![0, 0xd8, 0, 0];
    let text = SimulationPostGraphText::from_payload(&code_units, 2048).unwrap();
    code_units.fill(255);
    assert_eq!(text.payload(), &[0, 0xd8, 0, 0]);
}

fn assert_raw_text_kind_boundary(body: &SimulationPostGraphParameters) {
    let index = body
        .fields()
        .iter()
        .position(|field| matches!(field.value, SimulationPostGraphValue::Text(_)))
        .unwrap();
    let id = body.field_id(index).unwrap();
    let raw = SimulationPostGraphValue::Raw64([2, 0, 0, 0, 0, 0xd8, 0, 0]);
    let error = body.edit_field(&id, raw.clone()).unwrap_err();
    assert!(error.to_string().contains("Raw64") && error.to_string().contains("Text"));
    let mut fields = body.fields().to_vec();
    fields[index].value = raw;
    assert!(SimulationPostGraphParameters::from_fields(fields, body.version(), LIMITS).is_err());
}

#[test]
fn raw64_and_valid_character_text_frames_remain_distinct_typed_boundaries() {
    each_owned(|body| assert_raw_text_kind_boundary(&body));
}

#[test]
fn graph_reader_rejects_versions_below_its_explicit_minimum() {
    for value in [0, 800017] {
        let version = SimulationPersistVersion::new(value).unwrap();
        assert!(SimulationPostGraphParameters::from_bytes(&[0; 168], version, LIMITS).is_err());
    }
}
