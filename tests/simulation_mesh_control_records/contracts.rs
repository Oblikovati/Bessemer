use super::*;

pub(super) fn owned_collections() -> Vec<SimulationMeshControlRecords> {
    let mut cursor = OWNED;
    let mut bodies = Vec::new();
    while !cursor.is_empty() {
        bodies.push(read_collection(&mut cursor));
        consume_expected_fields(&mut cursor);
    }
    assert_eq!(bodies.len(), 322);
    bodies
}

fn assert_noop(body: &SimulationMeshControlRecords, index: usize) {
    let id = body.field_id(index).unwrap();
    let edit = body
        .edit_field(&id, body.field(&id).unwrap().value.clone())
        .unwrap();
    assert_eq!(edit.parent(), id);
    assert_eq!(edit.result(), id);
    assert_eq!(edit.archive().to_bytes(), body.to_bytes());
    assert_eq!(edit.archive().record_spans(), body.record_spans());
    assert_eq!(
        edit.coverage().completion,
        SimulationMeshControlCompletion::Unchanged
    );
    assert_copied_outer(body, &edit);
    assert_copied_nested(&edit);
}

fn assert_copied_outer(body: &SimulationMeshControlRecords, edit: &SimulationMeshControlEdit) {
    for (index, row) in edit.history().iter().enumerate() {
        assert_eq!(row.result, body.field_id(index).unwrap());
        assert_eq!(row.relation, SimulationMeshControlRelation::Copied);
        assert_eq!(
            row.parents.as_ref(),
            &[SimulationMeshControlHistoryParent::Source(
                body.field_id(index).unwrap()
            )]
        );
    }
}

fn assert_copied_nested(edit: &SimulationMeshControlEdit) {
    for row in edit.nested_history() {
        assert_eq!(row.relation, SimulationMeshControlRelation::Copied);
        assert_eq!(row.outer_parent, row.outer_result);
        let [SimulationMeshControlNestedParent::Source(id)] = row.parents.as_ref() else {
            panic!("expected one original parent for each no-op embedded occurrence");
        };
        assert_eq!(row.result.as_ref(), Some(id));
    }
}

#[test]
fn every_owned_field_noop_keeps_complete_bytes_revision_spans_and_all_copied_parents() {
    for body in owned_collections() {
        for index in 0..body.fields().len() {
            assert_noop(&body, index);
        }
    }
}

fn assert_complete_input_isolation(original: &SimulationMeshControlRecords) {
    let mut raw = original.to_bytes();
    let body = SimulationMeshControlRecords::from_bytes(&raw, original.version(), LIMITS).unwrap();
    let mut fields = body.fields().to_vec();
    let constructed =
        SimulationMeshControlRecords::from_fields(fields.clone(), body.version(), LIMITS).unwrap();
    fields[0].path = "unassigned-count".into();
    fields[0].value = SimulationMeshControlValue::Unsigned(0);
    assert!(SimulationMeshControlRecords::from_fields(fields, body.version(), LIMITS).is_err());
    raw.fill(0xa5);
    assert_eq!(body.to_bytes(), original.to_bytes());
    assert_eq!(body.fields(), original.fields());
    assert_eq!(constructed.to_bytes(), original.to_bytes());
    assert_eq!(constructed.record_spans(), original.record_spans());
}

#[test]
fn every_owned_input_and_caller_field_vector_is_isolated_after_success_and_failure() {
    for body in owned_collections() {
        assert_complete_input_isolation(&body);
    }
}

fn invalid_constructions(
    body: &SimulationMeshControlRecords,
) -> [Vec<SimulationMeshControlField>; 4] {
    let mut path = body.fields().to_vec();
    path[0].path = "unassigned-count".into();
    let mut kind = body.fields().to_vec();
    kind[0].value = SimulationMeshControlValue::Unsigned(0);
    let mut missing = body.fields().to_vec();
    missing.pop();
    let mut extra = body.fields().to_vec();
    extra.push(body.fields()[0].clone());
    [path, kind, missing, extra]
}

#[test]
fn every_owned_complete_constructor_rejects_wrong_paths_kinds_and_missing_or_extra_fields() {
    for body in owned_collections() {
        for fields in invalid_constructions(&body) {
            assert!(
                SimulationMeshControlRecords::from_fields(fields, body.version(), LIMITS).is_err()
            );
        }
    }
}

fn scalar_target(body: &SimulationMeshControlRecords) -> usize {
    body.fields()
        .iter()
        .position(|field| field.path.as_ref() == "record0.base.integer")
        .unwrap()
}

#[test]
fn foreign_stale_and_out_of_bounds_ids_cannot_cross_immutable_record_revisions() {
    let bodies = owned_collections();
    let body = bodies
        .iter()
        .find(|body| !body.record_spans().is_empty())
        .unwrap();
    let independent =
        SimulationMeshControlRecords::from_bytes(&body.to_bytes(), body.version(), LIMITS).unwrap();
    let id = body.field_id(scalar_target(body)).unwrap();
    assert!(independent.field(&id).is_err());
    assert!(body.field_id(usize::MAX).is_err());
    let edit = body
        .edit_field(&id, SimulationMeshControlValue::Signed(91))
        .unwrap();
    assert!(edit.archive().field(&id).is_err());
    assert!(body.field(&edit.result()).is_err());
    assert!(body.field(&id).is_ok());
    assert!(edit.archive().field(&edit.result()).is_ok());
}

#[test]
fn failed_count_and_kind_edits_leave_every_original_record_and_identity_unchanged() {
    for body in owned_collections() {
        let raw = body.to_bytes();
        let count = body.field_id(0).unwrap();
        let replacement = body.record_spans().len() as i32 + 1;
        assert!(
            body.edit_field(&count, SimulationMeshControlValue::Signed(replacement))
                .is_err()
        );
        assert!(
            body.edit_field(&count, SimulationMeshControlValue::Unsigned(0))
                .is_err()
        );
        assert_eq!(body.to_bytes(), raw);
        assert_eq!(body.field_id(0).unwrap(), count);
    }
}

#[test]
fn every_owned_record_context_rejects_every_proper_prefix_and_trailing_byte_without_mutation() {
    let mut prefixes = 0;
    for body in owned_collections() {
        let raw = body.to_bytes();
        prefixes += assert_all_prefixes(&body, &raw);
        assert!(
            SimulationMeshControlRecords::from_bytes(
                &[raw.as_slice(), &[0]].concat(),
                body.version(),
                LIMITS
            )
            .is_err()
        );
        assert_eq!(body.to_bytes(), raw);
    }
    assert_eq!(prefixes, 390672);
}

fn assert_all_prefixes(body: &SimulationMeshControlRecords, raw: &[u8]) -> usize {
    let mut count = 0;
    for end in 0..raw.len() {
        assert!(
            SimulationMeshControlRecords::from_bytes(&raw[..end], body.version(), LIMITS).is_err()
        );
        count += 1;
    }
    count
}
