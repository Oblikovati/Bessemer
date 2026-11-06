use super::*;
#[test]
fn every_frozen_selection_outcome_matches_typed_priority_latest_lookup_and_parse_behavior() {
    for row in cases().1 {
        check_case(&row);
    }
}
pub(super) fn check_case(row: &SelectionCase) -> Vec<u8> {
    let source = carrier(&row.raw);
    check_outcome(&source, &row.outcome, &row.name);
    assert_eq!(source.as_bytes(), row.raw);
    source.as_bytes().to_vec()
}
fn check_outcome(source: &FlowCarrier, outcome: &SelectionOutcome, name: &str) {
    let result = source.owner_state(options());
    match outcome {
        SelectionOutcome::Empty => assert!(result.unwrap().is_none()),
        SelectionOutcome::Rejected => {
            assert!(result.is_err(), "{}", name);
        }
        SelectionOutcome::Selected {
            identifier,
            ordinal,
            updates,
        } => {
            verify_selected(
                &result.unwrap().unwrap(),
                source,
                *identifier,
                *ordinal,
                updates,
            );
        }
    }
}

fn verify_selected(
    view: &FlowOwnerState,
    source: &FlowCarrier,
    identifier: u32,
    ordinal: u32,
    updates: &Updates,
) {
    assert_eq!(view.record().ordinal(), ordinal as usize);
    assert_eq!(projection(view), *updates);
    assert_eq!(
        source.records().nth(ordinal as usize).unwrap().identifier,
        identifier
    );
    check_selected_edit(view);
}

fn check_selected_edit(view: &FlowOwnerState) {
    let edit = view
        .replace_word(0, !view.words().raw_word(0).unwrap())
        .unwrap();
    assert_eq!(edit.result().record().ordinal(), view.record().ordinal());
    let before: Vec<_> = edit.carrier_edit().source().records().collect();
    let after: Vec<_> = edit.carrier_edit().result().records().collect();
    assert_eq!(before.len(), after.len());
    assert_eq!(before.len(), edit.carrier_edit().history().len());
    for ((before, after), history) in before.iter().zip(&after).zip(edit.carrier_edit().history()) {
        assert_eq!((&history.source, &history.result), (&before.id, &after.id));
        let relation = if before.payload == after.payload {
            FlowPayloadRelation::Copied
        } else {
            FlowPayloadRelation::Derived
        };
        assert_eq!(history.relation, relation);
        if before.id.ordinal() != view.record().ordinal() {
            assert_eq!(before.payload, after.payload);
        }
    }
}
#[test]
fn absent_selection_with_zero_budget_is_completed_empty_but_disabled_legacy_is_present() {
    let empty = carrier(&8u16.to_le_bytes());
    let zero = FlowOwnerStateOptions {
        maximum_payload_bytes: 0,
    };
    assert!(empty.owner_state(zero).unwrap().is_none());
    let row = cases()
        .0
        .into_iter()
        .find(|row| row.identifier == 122 && !row.gate)
        .unwrap();
    let source = state_carrier::carrier(122, &row.raw);
    let selected = source.owner_state(options()).unwrap().unwrap();
    assert_eq!(selected.record().ordinal(), 1);
    assert!(selected.updates().is_none());
    let too_small = FlowOwnerStateOptions {
        maximum_payload_bytes: 199,
    };
    assert!(source.owner_state(too_small).is_err());
}
