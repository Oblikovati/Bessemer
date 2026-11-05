use super::*;

#[test]
fn eight_workers_match_serial_edits_and_preserve_shared_sources() {
    let sources = [
        equation_source(0, 0),
        equation_source(0, 4),
        equation_source(1, 2),
    ];
    let serial: Vec<Vec<u8>> = sources.iter().map(equation_replay).collect();
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let sources = &sources;
            let serial = &serial;
            scope.spawn(move || replay_shared_equations(sources, serial));
        }
    });
    assert_shared_sources_unchanged(&sources);
}

fn assert_shared_sources_unchanged(sources: &[PartResolvedFeaturesArchive]) {
    for (index, source) in sources.iter().enumerate() {
        assert_eq!(
            source.to_bytes().unwrap(),
            OWNED_EQUATIONS[usize::from(index == 2)][[0, 4, 2][index]]
        );
    }
}

fn equation_source(kind: usize, style: usize) -> PartResolvedFeaturesArchive {
    read_owned_resolved_document(
        &equation_fixture_document(OWNED_EQUATIONS[kind][style]),
        EQUATION_LIMITS,
    )
}

fn equation_replay(source: &PartResolvedFeaturesArchive) -> Vec<u8> {
    let state = source.spline_equation_states().unwrap().remove(0);
    let result = source
        .edit_spline_equation(&state, ModelSplineEquationStoredEdit::SavedOption(12345))
        .unwrap();
    assert_eq!(result.completion(), ModelEditCompletion::Changed);
    assert_eq!(result.fidelity(), ModelEditFidelity::ExactStoredFields);
    assert_eq!(result.diagnostics(), source.diagnostics());
    assert_eq!(result.changes().len(), 1);
    assert_eq!(result.changes()[0].parents(), &state.source_fields()[8..9]);
    result.archive().to_bytes().unwrap()
}

fn replay_shared_equations(sources: &[PartResolvedFeaturesArchive], serial: &[Vec<u8>]) {
    for _ in 0..16 {
        for (source, expected) in sources.iter().zip(serial) {
            assert_eq!(equation_replay(source), *expected);
        }
    }
}
