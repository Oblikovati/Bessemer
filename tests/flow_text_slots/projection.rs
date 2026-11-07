use super::*;
pub(super) fn previous(row: &TextSlotCase) -> FlowTextSlotState {
    let values = std::array::from_fn(|index| units(&row.slots[index].previous));
    let count = row.slots.iter().map(|slot| slot.previous.len() / 2).sum();
    FlowTextSlotState::from_units(values, count).unwrap()
}
pub(super) fn check_projection(view: &FlowTextSlots, row: &TextSlotCase) -> FlowTextSlotProjection {
    let previous = previous(row);
    let count = row.slots.iter().map(|slot| slot.applied.len() / 2).sum();
    let projection = view.project(&previous, count).unwrap();
    assert_eq!(projection.source().decoded_bytes(), row.raw);
    assert_eq!(projection.previous(), &previous);
    for (index, slot) in row.slots.iter().enumerate() {
        check_value(&projection, index, slot);
    }
    check_projection_coverage(&projection, view, count);
    check_state_bounds(&previous);
    if count > 0 {
        assert!(view.project(&previous, count - 1).is_err());
    }
    assert_eq!(previous, projection.previous().clone());
    projection
}
fn check_projection_coverage(
    projection: &FlowTextSlotProjection,
    view: &FlowTextSlots,
    count: usize,
) {
    let proof = projection.coverage();
    assert_eq!(proof.source, view.coverage());
    assert_eq!(proof.previous_units, projection.previous().total_units());
    assert_eq!(proof.result_units, count);
    assert_eq!(proof.maximum_result_units, count);
    assert_eq!(proof.retained_slots, view.coverage().empty_prefixes);
    assert_eq!(proof.replaced_slots + proof.retained_slots, 50);
    assert_eq!(projection.result().total_units(), count);
    assert_eq!(projection.diagnostics(), view.diagnostics());
}

fn check_value(projection: &FlowTextSlotProjection, index: usize, slot: &TextSlot) {
    assert_eq!(
        projection.previous().units(index).unwrap(),
        units(&slot.previous)
    );
    assert_eq!(
        projection.result().units(index).unwrap(),
        units(&slot.applied)
    );
    let relation = if slot.prefix.is_empty() {
        FlowTextSlotRelation::RetainedPrevious
    } else {
        FlowTextSlotRelation::AppliedNativePrefix
    };
    assert_eq!(projection.history()[index], relation);
}
fn check_state_bounds(state: &FlowTextSlotState) {
    assert!(state.units(50).is_err() && state.units(usize::MAX).is_err());
}
#[test]
fn zero_result_budget_preserves_fifty_complete_empty_values() {
    let limits = FlowTextSlotOptions {
        maximum_payload_bytes: 204,
        maximum_units: 0,
        ..options()
    };
    let view = open(&[0; 204], limits).unwrap();
    let previous = FlowTextSlotState::from_units(std::array::from_fn(|_| Vec::new()), 0).unwrap();
    let projection = view.project(&previous, 0).unwrap();
    assert_eq!(projection.result(), &previous);
    assert_eq!(projection.coverage().result_units, 0);
    assert_eq!(projection.coverage().retained_slots, 50);
    assert!(
        projection
            .history()
            .iter()
            .all(|relation| *relation == FlowTextSlotRelation::RetainedPrevious)
    );
}
#[test]
fn all_owned_existing_and_applied_states_match_exact_source_projections_and_history() {
    for row in cases() {
        let view = check_case(&row);
        check_projection(&view, &row);
    }
}
#[test]
fn old_state_exact_bounds_reject_one_below_and_zero_bounds_admit_fifty_empty_values() {
    for row in cases() {
        let count: usize = row.slots.iter().map(|slot| slot.previous.len() / 2).sum();
        let values = std::array::from_fn(|index| units(&row.slots[index].previous));
        assert!(FlowTextSlotState::from_units(values, count).is_ok());
        if count == 0 {
            continue;
        }
        let values = std::array::from_fn(|index| units(&row.slots[index].previous));
        assert!(FlowTextSlotState::from_units(values, count - 1).is_err());
    }
    let empty = FlowTextSlotState::from_units(std::array::from_fn(|_| Vec::new()), 0).unwrap();
    assert_eq!(empty.total_units(), 0);
    assert!(empty.units(49).unwrap().is_empty());
}
#[test]
fn equal_nonempty_prefix_replacement_still_has_native_source_provenance() {
    let raw = first_slot_payload(0, &[65, 0]);
    let view = open(&raw, options()).unwrap();
    let values = std::array::from_fn(|index| if index == 0 { vec![65] } else { Vec::new() });
    let previous = FlowTextSlotState::from_units(values, 1).unwrap();
    let projected = view.project(&previous, 1).unwrap();
    assert_eq!(projected.result(), &previous);
    assert_eq!(
        projected.history()[0],
        FlowTextSlotRelation::AppliedNativePrefix
    );
    assert_eq!(
        projected.history()[1],
        FlowTextSlotRelation::RetainedPrevious
    );
}
