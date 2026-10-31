use super::*;

pub(super) fn owned_composites() -> Vec<SimulationShellComposite> {
    let mut cursor = EXPECTED;
    let mut bodies = Vec::new();
    while !cursor.is_empty() {
        bodies.push(read_original(&mut cursor));
        consume_expected_fields(&mut cursor);
    }
    assert_eq!(bodies.len(), 270);
    bodies
}

fn assert_noop(body: &SimulationShellComposite, index: usize) {
    let parent = body.field_id(index).unwrap();
    let edit = body
        .edit_field(&parent, body.fields()[index].value.clone())
        .unwrap();
    assert_eq!(edit.result(), parent);
    assert_eq!(edit.archive().to_bytes(), body.to_bytes());
    assert_eq!(
        edit.coverage().completion,
        SimulationShellCompositeCompletion::Unchanged
    );
    for (ordinal, row) in edit.history().iter().enumerate() {
        assert_eq!(row.relation, SimulationShellCompositeRelation::Copied);
        assert_eq!(row.result, body.field_id(ordinal).unwrap());
        assert_eq!(
            row.parents.as_ref(),
            &[SimulationShellCompositeHistoryParent::Source(
                body.field_id(ordinal).unwrap()
            )]
        );
    }
}

#[test]
fn every_field_noop_preserves_wire_identity_and_all_copied_parents() {
    for body in owned_composites() {
        for index in 0..body.fields().len() {
            assert_noop(&body, index);
        }
    }
}

fn assert_revision_ownership(body: &SimulationShellComposite) {
    let raw = body.to_bytes();
    let independent =
        SimulationShellComposite::from_bytes(&raw, body.version(), body.profile(), LIMITS).unwrap();
    let parent = body.field_id(0).unwrap();
    assert!(independent.field(&parent).is_err());
    assert!(body.field_id(usize::MAX).is_err());
    if matches!(
        body.profile(),
        SimulationShellCompositeProfile::FlipTurnReader
            | SimulationShellCompositeProfile::FlipTurnWriter
    ) {
        return;
    }
    assert_changed_revision(body, parent);
}

fn assert_changed_revision(
    body: &SimulationShellComposite,
    parent: SimulationShellCompositeFieldId,
) {
    let SimulationShellCompositeValue::Signed(word) = body.fields()[0].value else {
        panic!("expected signed member37c")
    };
    let edit = body
        .edit_field(&parent, SimulationShellCompositeValue::Signed(word ^ 32))
        .unwrap();
    assert!(edit.archive().field(&parent).is_err());
    assert!(body.field(&edit.result()).is_err());
    assert!(body.field(&parent).is_ok());
    assert!(edit.archive().field(&edit.result()).is_ok());
}

#[test]
fn revision_ownership_rejects_foreign_stale_and_out_of_bounds_handles() {
    for body in owned_composites() {
        assert_revision_ownership(&body);
    }
}

fn invalid_field_sets(body: &SimulationShellComposite) -> [Vec<SimulationShellCompositeField>; 4] {
    let mut path = body.fields().to_vec();
    path[0].path = "unbound-composite-member".into();
    let mut kind = body.fields().to_vec();
    kind[0].value = SimulationShellCompositeValue::DoubleBits(0);
    let mut missing = body.fields().to_vec();
    missing.pop();
    let mut extra = body.fields().to_vec();
    extra.push(body.fields()[0].clone());
    [path, kind, missing, extra]
}

#[test]
fn construction_rejects_wrong_paths_kinds_and_missing_or_extra_fields() {
    for body in owned_composites() {
        for fields in invalid_field_sets(&body) {
            assert!(
                SimulationShellComposite::from_fields(
                    fields,
                    body.version(),
                    body.profile(),
                    LIMITS
                )
                .is_err()
            );
        }
    }
}

fn assert_input_isolation(body: &SimulationShellComposite) {
    let original = body.to_bytes();
    let mut bytes = original.clone();
    let parsed =
        SimulationShellComposite::from_bytes(&bytes, body.version(), body.profile(), LIMITS)
            .unwrap();
    bytes.fill(255);
    assert_eq!(parsed.to_bytes(), original);
    assert_caller_field_isolation(body);
}

fn assert_caller_field_isolation(body: &SimulationShellComposite) {
    let original = body.to_bytes();
    let mut fields = body.fields().to_vec();
    let constructed = SimulationShellComposite::from_fields(
        fields.clone(),
        body.version(),
        body.profile(),
        LIMITS,
    )
    .unwrap();
    fields[0].path = "invalid-mutated-caller-field".into();
    assert!(
        SimulationShellComposite::from_fields(fields, body.version(), body.profile(), LIMITS)
            .is_err()
    );
    assert_eq!(constructed.to_bytes(), original);
    assert_eq!(constructed.fields(), body.fields());
}

#[test]
fn mutable_inputs_and_caller_fields_are_isolated_after_success_and_failure() {
    for body in owned_composites() {
        assert_input_isolation(&body);
    }
}

#[test]
fn every_value_kind_rejects_incompatible_replacement_without_mutation() {
    for body in owned_composites() {
        let original = body.to_bytes();
        for index in 0..body.fields().len() {
            let replacement = match body.fields()[index].value {
                SimulationShellCompositeValue::Signed(_) => {
                    SimulationShellCompositeValue::DoubleBits(0)
                }
                _ => SimulationShellCompositeValue::Signed(0),
            };
            let error = body
                .edit_field(&body.field_id(index).unwrap(), replacement)
                .unwrap_err();
            assert_eq!(error.class, bessemer::FailureClass::Invalid);
            assert_eq!(body.to_bytes(), original);
        }
    }
}
