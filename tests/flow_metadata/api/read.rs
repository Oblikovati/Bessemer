use super::*;

#[test]
fn every_frozen_row_map_and_assignment_matches_complete_typed_projection() {
    for case in fixture::state_cases() {
        let view = read_case(&case);
        assert_eq!(view.header_word(), case.version_word);
        assert_eq!(view.payload_bytes(), case.raw);
        assert_eq!(view.entries().len(), case.entries.len());
        for expected in &case.entries {
            check_row(view.entry(expected.ordinal as usize).unwrap(), expected);
        }
        assert!(view.entry(case.entries.len()).is_none());
        let existing = map(&case.existing);
        let projection = view.apply_to(&existing);
        check_projection(&projection, &case);
        check_coverage(&view, &case);
        assert_eq!(existing.entries().collect::<Vec<_>>(), case.existing);
        assert_eq!(projection.source().record(), view.record());
    }
}

fn check_row(actual: FlowIdStateEntry<'_>, expected: &StateEntry) {
    assert_eq!(actual.ordinal(), expected.ordinal as usize);
    assert_eq!(
        actual.payload_interval(),
        (expected.start as usize, expected.end as usize)
    );
    assert_eq!(actual.identifier(), expected.identifier);
    assert_eq!(actual.state_byte(), expected.state);
}

fn check_projection(actual: &FlowIdStateProjection, case: &StateCase) {
    assert_eq!(
        actual.existing().entries().collect::<Vec<_>>(),
        case.existing
    );
    assert_eq!(actual.result().entries().collect::<Vec<_>>(), case.expected);
    assert_eq!(actual.result().len(), case.expected.len());
    assert_eq!(actual.result().is_empty(), case.expected.is_empty());
    for (identifier, state) in &case.expected {
        assert_eq!(actual.result().get(*identifier), Some(*state));
    }
    assert_eq!(actual.assignments().len(), case.assignments.len());
    for (actual, expected) in actual.assignments().iter().zip(&case.assignments) {
        check_assignment(actual, expected);
    }
    check_projection_coverage(actual, case);
    assert_eq!(actual.source().payload_bytes(), case.raw);
    check_projection_diagnostics(actual);
}

fn check_assignment(actual: &FlowIdStateAssignment, expected: &StateAssignment) {
    assert_eq!(actual.entry_ordinal, expected.ordinal as usize);
    assert_eq!(actual.identifier, expected.identifier);
    assert_eq!(actual.previous, expected.previous);
    assert_eq!(actual.state, expected.state);
    assert_eq!(
        actual.relation,
        if expected.previous.is_some() {
            FlowIdStateAssignmentRelation::Derived
        } else {
            FlowIdStateAssignmentRelation::Created
        }
    );
}

fn check_projection_coverage(actual: &FlowIdStateProjection, case: &StateCase) {
    let coverage = actual.coverage();
    let created = case
        .assignments
        .iter()
        .filter(|assignment| assignment.previous.is_none())
        .count();
    assert_eq!(coverage.assignments, case.entries.len());
    assert_eq!(coverage.existing_keys, case.existing.len());
    assert_eq!(coverage.result_keys, case.expected.len());
    assert_eq!(coverage.created_keys, created);
    assert_eq!(coverage.derived_assignments, case.entries.len() - created);
    assert_eq!(
        coverage.unmentioned_existing_entries_preserved,
        case.preserved
    );
    assert_eq!(coverage.achieved_error, 0);
}

fn check_coverage(view: &FlowIdStates, case: &StateCase) {
    let coverage = view.coverage();
    assert_eq!(coverage.entries, case.entries.len());
    assert_eq!(coverage.payload_bytes, case.end as usize);
    assert_eq!(
        (coverage.framing_bytes, coverage.row_bytes),
        (8, case.entries.len() * 5)
    );
    assert_eq!(coverage.native_version_dispatch, case.native_dispatch);
    assert_eq!(coverage.achieved_error, 0);
    assert_eq!(view.record().ordinal(), 1);
    assert_eq!(view.source().records().len(), 3);
    assert_eq!(view.limits().maximum_entries, limits().maximum_entries);
    assert_eq!(view.diagnostics().len(), 3);
    assert!(
        view.diagnostics()
            .contains(&FlowIdStateDiagnostic::IdentifierRolesNotResolved)
    );
}

fn check_projection_diagnostics(actual: &FlowIdStateProjection) {
    assert_eq!(actual.diagnostics().len(), 3);
    assert!(
        actual
            .diagnostics()
            .contains(&FlowIdStateDiagnostic::NativeHashTableNotReproduced)
    );
}
