use super::*;

fn latest_material_body() -> SimulationShellProperty {
    contracts::owned_properties()
        .into_iter()
        .find(|body| {
            body.version().value() == 891106
                && body
                    .fields()
                    .iter()
                    .any(|field| field.path.ends_with(".material.property1.pair_count"))
        })
        .unwrap()
}

fn ordinal(body: &SimulationShellProperty, suffix: &str) -> usize {
    body.fields()
        .iter()
        .position(|field| field.path.ends_with(suffix))
        .unwrap()
}

fn with_limits(
    body: &SimulationShellProperty,
    limits: SimulationShellPropertyLimits,
) -> SimulationShellProperty {
    SimulationShellProperty::from_bytes(
        &body.to_bytes(),
        body.version(),
        body.context().clone(),
        limits,
    )
    .unwrap()
}

fn assert_unchanged(
    body: &SimulationShellProperty,
    before: &[u8],
    fields: &[SimulationShellPropertyField],
    parent: &SimulationShellPropertyFieldId,
) {
    assert_eq!(body.to_bytes(), before);
    assert_eq!(body.fields(), fields);
    assert_eq!(body.field(parent).unwrap(), &fields[parent.ordinal()]);
}

#[test]
fn oversized_inline_text_failure_retains_original_bytes_fields_context_and_identity() {
    let source = latest_material_body();
    let maximum = maximum_outer_text(&source);
    let body = text_limited_body(&source, maximum);
    let before = body.to_bytes();
    let fields = body.fields().to_vec();
    let parent = body.field_id(ordinal(&body, ".material.text20")).unwrap();
    let payload = vec![0_u8; maximum + 2];
    let replacement =
        SimulationPersistText::from_payload(Some(&payload), LIMITS.maximum_text_bytes).unwrap();
    let error = body
        .edit_field(&parent, SimulationShellPropertyValue::Text(replacement))
        .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(error.entity.contains(".material."));
    assert!(error.cause.contains("expected"));
    assert_unchanged(&body, &before, &fields, &parent);
    assert_eq!(body.context(), source.context());
}

fn maximum_outer_text(source: &SimulationShellProperty) -> usize {
    source
        .fields()
        .iter()
        .filter_map(|field| match &field.value {
            SimulationShellPropertyValue::Text(text) => Some(text.payload().len()),
            _ => None,
        })
        .max()
        .unwrap()
}

fn text_limited_body(source: &SimulationShellProperty, maximum: usize) -> SimulationShellProperty {
    with_limits(
        source,
        SimulationShellPropertyLimits {
            maximum_text_bytes: maximum,
            ..LIMITS
        },
    )
}

#[test]
fn absent_curve_count_budget_failure_after_partial_validation_preserves_both_material_bodies() {
    let source = latest_material_body();
    let body = entry_limited_body(&source);
    let before = body.to_bytes();
    let fields = body.fields().to_vec();
    let index = ordinal(&body, ".material.property1.pair_count");
    let parent = body.field_id(index).unwrap();
    let SimulationShellPropertyValue::Signed(count) = fields[index].value else {
        panic!("expected declared pair count")
    };
    let error = body
        .edit_field(&parent, SimulationShellPropertyValue::Signed(count + 1))
        .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(error.cause.contains("aggregate"));
    assert_unchanged(&body, &before, &fields, &parent);
    assert_eq!(body.context(), source.context());
}

fn entry_limited_body(source: &SimulationShellProperty) -> SimulationShellProperty {
    with_limits(
        source,
        SimulationShellPropertyLimits {
            maximum_entries: source.coverage().declared_entries,
            ..LIMITS
        },
    )
}

#[test]
fn uncoupled_blob_length_failure_does_not_mutate_reference_archives_or_outer_ids() {
    let mut checked = 0;
    for body in contracts::owned_properties()
        .into_iter()
        .filter(|body| body.version().value() == 840002)
    {
        let Some(index) = body
            .fields()
            .iter()
            .position(|field| matches!(field.value, SimulationShellPropertyValue::Bytes(_)))
        else {
            continue;
        };
        assert_blob_failure(&body, index);
        checked += 1;
    }
    assert_eq!(checked, 18);
}

fn assert_blob_failure(body: &SimulationShellProperty, index: usize) {
    let before = body.to_bytes();
    let fields = body.fields().to_vec();
    let parent = body.field_id(index).unwrap();
    let SimulationShellPropertyValue::Bytes(blob) = &fields[index].value else {
        panic!("expected opaque blob")
    };
    let mut replacement = blob.to_vec();
    replacement.push(0xff);
    assert!(
        body.edit_field(
            &parent,
            SimulationShellPropertyValue::Bytes(Arc::from(replacement))
        )
        .is_err()
    );
    assert_unchanged(body, &before, &fields, &parent);
}

#[test]
fn mutating_supplied_context_after_publication_keeps_original_runtime_profile_isolated() {
    let body = latest_material_body();
    let before = body.to_bytes();
    let original = body.context().clone();
    let mut supplied = original.clone();
    let SimulationShellLayerPresence::Known(values) = &mut supplied.layer_presence else {
        panic!("expected explicit populated context")
    };
    Arc::make_mut(values).fill(false);
    let _result = SimulationShellProperty::from_bytes(&before, body.version(), supplied, LIMITS);
    assert_eq!(body.context(), &original);
    assert_eq!(body.to_bytes(), before);
}
