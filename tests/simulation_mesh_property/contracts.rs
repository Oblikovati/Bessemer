use super::*;

pub(super) fn owned_properties() -> Vec<SimulationMeshProperty> {
    let mut cursor = OWNED;
    let mut properties = Vec::new();
    while !cursor.is_empty() {
        properties.push(read_property(&mut cursor));
        consume_expected_fields(&mut cursor);
    }
    assert_eq!(properties.len(), 588);
    properties
}

fn assert_noop(property: &SimulationMeshProperty, index: usize) {
    let id = property.field_id(index).unwrap();
    let edit = property
        .edit_field(&id, property.field(&id).unwrap().value.clone())
        .unwrap();
    assert_eq!(edit.parent(), id);
    assert_eq!(edit.result(), id);
    assert_eq!(edit.archive().to_bytes(), property.to_bytes());
    assert_eq!(edit.archive().record_spans(), property.record_spans());
    assert_eq!(
        edit.coverage().completion,
        SimulationMeshPropertyCompletion::Unchanged
    );
    assert_copied_history(property, &edit);
}

fn assert_copied_history(property: &SimulationMeshProperty, edit: &SimulationMeshPropertyEdit) {
    assert_eq!(edit.history().len(), property.fields().len());
    for (index, row) in edit.history().iter().enumerate() {
        assert_eq!(row.result, property.field_id(index).unwrap());
        assert_eq!(row.relation, SimulationMeshPropertyRelation::Copied);
        assert_eq!(
            row.parents.as_ref(),
            &[SimulationMeshPropertyHistoryParent::Source(
                property.field_id(index).unwrap()
            )]
        );
    }
    for row in edit.nested_history() {
        assert_eq!(row.relation, SimulationMeshPropertyRelation::Copied);
        assert_eq!(row.outer_parent, row.outer_result);
        let [SimulationMeshPropertyNestedParent::Source(id)] = row.parents.as_ref() else {
            panic!("expected one original no-op nested parent");
        };
        assert_eq!(row.result.as_ref(), Some(id));
    }
}

#[test]
fn every_owned_field_noop_keeps_wire_identity_extents_and_all_original_parents() {
    for property in owned_properties() {
        for index in 0..property.fields().len() {
            assert_noop(&property, index);
        }
    }
}

#[test]
fn foreign_stale_and_bounds_ids_cannot_cross_immutable_property_revisions() {
    let property = owned_properties().remove(0);
    let independent =
        SimulationMeshProperty::from_bytes(&property.to_bytes(), property.version(), LIMITS)
            .unwrap();
    let parent = property.field_id(0).unwrap();
    assert!(independent.field(&parent).is_err());
    assert!(property.field_id(usize::MAX).is_err());
    let edit = property
        .edit_field(&parent, SimulationMeshPropertyValue::Signed(91))
        .unwrap();
    assert!(edit.archive().field(&parent).is_err());
    assert!(property.field(&edit.result()).is_err());
    assert_eq!(property.field(&parent).unwrap(), &property.fields()[0]);
    assert_eq!(
        edit.archive().field(&edit.result()).unwrap(),
        &edit.archive().fields()[0]
    );
}

fn assert_caller_isolation(original: &SimulationMeshProperty) {
    let mut raw = original.to_bytes();
    let property = SimulationMeshProperty::from_bytes(&raw, original.version(), LIMITS).unwrap();
    let mut fields = property.fields().to_vec();
    let constructed =
        SimulationMeshProperty::from_fields(fields.clone(), property.version(), LIMITS).unwrap();
    fields[0].path = "unassigned-field".into();
    fields[0].value = SimulationMeshPropertyValue::Unsigned(0);
    assert!(SimulationMeshProperty::from_fields(fields, property.version(), LIMITS).is_err());
    raw.fill(0xa5);
    assert_eq!(property.to_bytes(), original.to_bytes());
    assert_eq!(property.fields(), original.fields());
    assert_eq!(constructed.to_bytes(), original.to_bytes());
    assert_eq!(constructed.record_spans(), original.record_spans());
}

#[test]
fn all_owned_inputs_and_caller_field_vectors_remain_isolated_after_success_and_failure() {
    for property in owned_properties() {
        assert_caller_isolation(&property);
    }
}

#[test]
fn failed_count_kind_and_missing_or_extra_field_requests_preserve_original_identity() {
    for property in owned_properties() {
        let before = property.to_bytes();
        let parent = property.field_id(0).unwrap();
        assert_count_edit_rejected(&property);
        assert!(
            property
                .edit_field(&parent, SimulationMeshPropertyValue::Unsigned(0))
                .is_err()
        );
        let mut missing = property.fields().to_vec();
        missing.pop();
        assert!(SimulationMeshProperty::from_fields(missing, property.version(), LIMITS).is_err());
        let mut extra = property.fields().to_vec();
        extra.push(property.fields()[0].clone());
        assert!(SimulationMeshProperty::from_fields(extra, property.version(), LIMITS).is_err());
        assert_eq!(property.to_bytes(), before);
        assert_eq!(property.field_id(0).unwrap(), parent);
    }
}

fn assert_count_edit_rejected(property: &SimulationMeshProperty) {
    let Some(index) = property
        .fields()
        .iter()
        .position(|field| field.path.as_ref() == "mesh.text_list.count")
    else {
        return;
    };
    let SimulationMeshPropertyValue::Signed(count) = property.fields()[index].value else {
        panic!("expected a signed declared text count");
    };
    let id = property.field_id(index).unwrap();
    assert!(
        property
            .edit_field(&id, SimulationMeshPropertyValue::Signed(count + 1))
            .is_err()
    );
}
