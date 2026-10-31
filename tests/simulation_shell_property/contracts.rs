use super::*;

pub(super) fn owned_properties() -> Vec<SimulationShellProperty> {
    let mut cursor = EXPECTED;
    let mut properties = Vec::new();
    while !cursor.is_empty() {
        properties.push(read_original(&mut cursor));
        consume_expected_fields(&mut cursor);
    }
    assert_eq!(properties.len(), 2284);
    properties
}

fn assert_noop(body: &SimulationShellProperty, index: usize) {
    let parent = body.field_id(index).unwrap();
    let edit = body
        .edit_field(&parent, body.fields()[index].value.clone())
        .unwrap();
    assert_eq!(edit.result(), parent);
    assert_eq!(edit.archive().to_bytes(), body.to_bytes());
    assert_eq!(edit.archive().context(), body.context());
    assert_eq!(
        edit.coverage().completion,
        SimulationShellPropertyCompletion::Unchanged
    );
    assert_copied_history(body, &edit);
}

fn assert_copied_history(body: &SimulationShellProperty, edit: &SimulationShellPropertyEdit) {
    for (ordinal, row) in edit.history().iter().enumerate() {
        assert_eq!(row.relation, SimulationShellPropertyRelation::Copied);
        assert_eq!(row.result, body.field_id(ordinal).unwrap());
        assert_eq!(
            row.parents.as_ref(),
            &[SimulationShellPropertyHistoryParent::Source(
                body.field_id(ordinal).unwrap()
            )]
        );
    }
}

#[test]
fn every_owned_field_noop_preserves_complete_wire_context_identity_and_all_copied_parents() {
    for body in owned_properties() {
        for index in 0..body.fields().len() {
            assert_noop(&body, index);
        }
    }
}

fn base_scalar(body: &SimulationShellProperty) -> usize {
    body.fields()
        .iter()
        .position(|field| field.path.as_ref() == "property.base.integer")
        .unwrap()
}

fn assert_revision_ownership(body: &SimulationShellProperty) {
    let raw = body.to_bytes();
    let independent =
        SimulationShellProperty::from_bytes(&raw, body.version(), body.context().clone(), LIMITS)
            .unwrap();
    let parent = body.field_id(base_scalar(body)).unwrap();
    assert!(independent.field(&parent).is_err());
    assert!(body.field_id(usize::MAX).is_err());
    let SimulationShellPropertyValue::Signed(word) = body.fields()[base_scalar(body)].value else {
        panic!("expected base scalar")
    };
    let edit = body
        .edit_field(&parent, SimulationShellPropertyValue::Signed(word ^ 32))
        .unwrap();
    assert!(edit.archive().field(&parent).is_err());
    assert!(body.field(&edit.result()).is_err());
    assert!(body.field(&parent).is_ok());
    assert!(edit.archive().field(&edit.result()).is_ok());
    assert_eq!(body.to_bytes(), raw);
}

#[test]
fn all_contexts_reject_foreign_stale_and_out_of_bounds_revision_handles() {
    for body in owned_properties() {
        assert_revision_ownership(&body);
    }
}

fn invalid_field_sets(body: &SimulationShellProperty) -> [Vec<SimulationShellPropertyField>; 4] {
    let mut path = body.fields().to_vec();
    path[0].path = "unbound-shell-property-member".into();
    let mut kind = body.fields().to_vec();
    kind[0].value = SimulationShellPropertyValue::DoubleBits(0);
    let mut missing = body.fields().to_vec();
    missing.pop();
    let mut extra = body.fields().to_vec();
    extra.push(body.fields()[0].clone());
    [path, kind, missing, extra]
}

#[test]
fn all_complete_constructors_reject_wrong_paths_kinds_and_missing_or_extra_fields() {
    for body in owned_properties() {
        for fields in invalid_field_sets(&body) {
            assert!(
                SimulationShellProperty::from_fields(
                    fields,
                    body.version(),
                    body.context().clone(),
                    LIMITS
                )
                .is_err()
            );
        }
    }
}

fn assert_input_isolation(body: &SimulationShellProperty) {
    let original = body.to_bytes();
    let mut bytes = original.clone();
    let parsed =
        SimulationShellProperty::from_bytes(&bytes, body.version(), body.context().clone(), LIMITS)
            .unwrap();
    bytes.fill(255);
    assert_eq!(parsed.to_bytes(), original);
    assert_field_vector_isolation(body);
}

fn assert_field_vector_isolation(body: &SimulationShellProperty) {
    let original = body.to_bytes();
    let mut fields = body.fields().to_vec();
    let constructed = construct_fields(body, fields.clone()).unwrap();
    fields[0].path = "invalid-mutated-caller-field".into();
    assert!(construct_fields(body, fields).is_err());
    assert_eq!(constructed.to_bytes(), original);
    assert_eq!(constructed.fields(), body.fields());
}

fn construct_fields(
    body: &SimulationShellProperty,
    fields: Vec<SimulationShellPropertyField>,
) -> bessemer::Result<SimulationShellProperty> {
    SimulationShellProperty::from_fields(fields, body.version(), body.context().clone(), LIMITS)
}

#[test]
fn mutable_inputs_and_caller_vectors_are_isolated_after_success_and_failure() {
    for body in owned_properties() {
        assert_input_isolation(&body);
    }
}

fn assert_required_context(body: &SimulationShellProperty) -> usize {
    let SimulationShellLayerPresence::Known(presence) = &body.context().layer_presence else {
        return 0;
    };
    if presence.is_empty() {
        return 0;
    }
    for supplied in [
        SimulationShellLayerPresence::Unknown,
        SimulationShellLayerPresence::Known(Arc::from([])),
        SimulationShellLayerPresence::Known(Arc::from([true; 3])),
    ] {
        assert_context_error(body, supplied);
    }
    1
}

fn assert_context_error(body: &SimulationShellProperty, supplied: SimulationShellLayerPresence) {
    let expected = if supplied == SimulationShellLayerPresence::Unknown {
        bessemer::FailureClass::Unsupported
    } else {
        bessemer::FailureClass::Invalid
    };
    let context = shell_context(body.context().kind, supplied);
    let error = SimulationShellProperty::from_bytes(
        &body.to_bytes(),
        body.version(),
        context.clone(),
        LIMITS,
    )
    .unwrap_err();
    assert_eq!(error.class, expected);
    assert_eq!(error.operation, "decode shell Property");
    assert_eq!(error.entity, "layer_presence");
    assert!(error.cause.contains("expected"));
    assert_invalid_context_construction(body, context);
}

fn assert_invalid_context_construction(
    body: &SimulationShellProperty,
    context: SimulationShellPropertyContext,
) {
    assert!(
        SimulationShellProperty::from_fields(
            body.fields().to_vec(),
            body.version(),
            context,
            LIMITS
        )
        .is_err()
    );
}

#[test]
fn missing_required_runtime_context_is_incomplete_and_wrong_lengths_are_invalid() {
    assert_eq!(
        owned_properties()
            .iter()
            .map(assert_required_context)
            .sum::<usize>(),
        36
    );
}

fn assert_shared<T: Send + Sync>() {}

#[test]
fn every_public_property_type_is_send_and_sync() {
    assert_shared::<SimulationShellProperty>();
    assert_shared::<SimulationShellPropertyEdit>();
    assert_shared::<SimulationShellPropertyKind>();
    assert_shared::<SimulationShellLayerPresence>();
    assert_shared::<SimulationShellPropertyContext>();
    assert_shared::<SimulationShellPropertyField>();
    assert_shared::<SimulationShellPropertyValue>();
    assert_shared::<SimulationShellPropertyFieldId>();
    assert_shared::<SimulationShellPropertyHistory>();
    assert_shared::<SimulationShellPropertyHistoryParent>();
    assert_shared::<SimulationShellPropertyNestedHistory>();
    assert_shared::<SimulationShellPropertyNestedParent>();
    assert_shared::<SimulationShellPropertyRelation>();
    assert_shared::<SimulationShellPropertyLimits>();
    assert_shared::<SimulationShellPropertyCoverage>();
    assert_shared::<SimulationShellPropertyCompletion>();
    assert_shared::<SimulationShellPropertyCheckMode>();
    assert_shared::<SimulationShellPropertyDiagnostic>();
}
