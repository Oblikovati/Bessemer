use super::*;

const ALIAS_GRAPHS: [&[u8]; 5] = [
    include_bytes!("../fixtures/spline-equation-state/alias-graphs/graph-0.bin"),
    include_bytes!("../fixtures/spline-equation-state/alias-graphs/graph-1.bin"),
    include_bytes!("../fixtures/spline-equation-state/alias-graphs/graph-2.bin"),
    include_bytes!("../fixtures/spline-equation-state/alias-graphs/graph-3.bin"),
    include_bytes!("../fixtures/spline-equation-state/alias-graphs/graph-4.bin"),
];

#[test]
fn aliases_and_null_roots_do_not_duplicate_physical_equation_states() {
    let source = alias_equation_source();
    let states = source.spline_equation_states().unwrap();
    assert_eq!(source.root_count(), 5);
    assert_eq!(source.object_slots(), (97, 101));
    assert_eq!(states.len(), 4);
    let selected: std::collections::BTreeSet<usize> = states
        .iter()
        .map(|state| state.source_fields()[0].index())
        .collect();
    assert_eq!(selected.len(), 4);
    for (state, expected) in states.iter().zip([4, 0x34, 5, 0x80000004]) {
        assert_alias_projection(&source, state, expected);
    }
}

fn alias_equation_source() -> PartResolvedFeaturesArchive {
    read_owned_resolved_document(&equation_fixture_document(ALIAS_GRAPHS[0]), EQUATION_LIMITS)
}

#[test]
fn eight_workers_preserve_shared_alias_graphs_and_match_independent_goldens() {
    let source = alias_equation_source();
    let states = source.spline_equation_states().unwrap();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let source = &source;
            let states = &states;
            scope.spawn(move || replay_alias_worker(source, states));
        }
    });
    assert_eq!(source.to_bytes().unwrap(), ALIAS_GRAPHS[0]);
}

fn replay_alias_worker(
    source: &PartResolvedFeaturesArchive,
    states: &[ModelSplineEquationStoredState],
) {
    for _ in 0..4 {
        for (ordinal, state) in states.iter().enumerate() {
            let edit = ModelSplineEquationStoredEdit::LockStart(!state.lock_start());
            let result = source.edit_spline_equation(state, edit).unwrap();
            assert_eq!(
                result.archive().to_bytes().unwrap(),
                ALIAS_GRAPHS[ordinal + 1]
            );
            assert_alias_edit_contract(source, &result, state);
        }
    }
}

#[test]
fn each_physical_spline_edit_preserves_aliases_and_matches_independent_full_graph() {
    let document = equation_fixture_document(ALIAS_GRAPHS[0]);
    let source = read_owned_resolved_document(&document, EQUATION_LIMITS);
    let states = source.spline_equation_states().unwrap();
    for (ordinal, state) in states.iter().enumerate() {
        let edit = ModelSplineEquationStoredEdit::LockStart(!state.lock_start());
        let result = source.edit_spline_equation(state, edit).unwrap();
        assert_eq!(
            result.archive().to_bytes().unwrap(),
            ALIAS_GRAPHS[ordinal + 1]
        );
        assert_alias_edit_contract(&source, &result, state);
        assert_reopened_equation_document(&document, &source, &result);
        assert_alias_noop_and_stale_isolation(&source, &result, &states);
    }
}

fn assert_alias_edit_contract(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    selected: &ModelSplineEquationStoredState,
) {
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(result.changes().len(), 1);
    assert_eq!(
        result.changes()[0].parents(),
        &selected.source_fields()[..1]
    );
    assert_eq!(result.unchanged_fields(), source.fields().len() - 1);
    assert_eq!(result.archive().spline_equation_states().unwrap().len(), 4);
    for (index, (before, after)) in source
        .fields()
        .iter()
        .zip(result.archive().fields())
        .enumerate()
    {
        assert_eq!(
            before == after,
            index != selected.source_fields()[0].index()
        );
    }
}

fn assert_alias_noop_and_stale_isolation(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    original_states: &[ModelSplineEquationStoredState],
) {
    for state in original_states {
        assert!(
            result
                .archive()
                .edit_spline_equation(state, ModelSplineEquationStoredEdit::SavedOption(123))
                .is_err()
        );
        assert_alias_noop(source, state);
    }
    assert_eq!(source.to_bytes().unwrap(), ALIAS_GRAPHS[0]);
}

fn assert_alias_projection(
    source: &PartResolvedFeaturesArchive,
    state: &ModelSplineEquationStoredState,
    expected: u32,
) {
    assert_eq!(state.flags_word(), expected);
    assert_eq!(state.source().to_bytes().unwrap(), ALIAS_GRAPHS[0]);
    assert_eq!(
        source
            .spline_equation_state(&state.source_fields()[0])
            .unwrap()
            .source_fields(),
        state.source_fields()
    );
}

fn assert_alias_noop(source: &PartResolvedFeaturesArchive, state: &ModelSplineEquationStoredState) {
    let noop = source
        .edit_spline_equation(
            state,
            ModelSplineEquationStoredEdit::SavedOption(state.saved_option()),
        )
        .unwrap();
    assert_eq!(noop.completion(), ModelEditCompletion::Unchanged);
    assert_eq!(noop.archive().to_bytes().unwrap(), ALIAS_GRAPHS[0]);
    assert!(
        noop.archive()
            .resolve_field(&state.source_fields()[0])
            .is_ok()
    );
}
