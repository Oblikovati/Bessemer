use super::*;

type TypedChoices = Vec<(usize, SimulationShellPropertyValue)>;

fn parallel_sources() -> Vec<SimulationShellProperty> {
    let mut selected = Vec::new();
    for body in contracts::owned_properties() {
        if body.version().value() != 891106
            || body.coverage().materials != 2
            || body.coverage().descriptors == 0
        {
            continue;
        }
        if selected
            .iter()
            .any(|previous: &SimulationShellProperty| previous.context() == body.context())
        {
            continue;
        }
        selected.push(body);
    }
    assert_eq!(selected.len(), 4);
    selected
}

fn assert_serial_equivalence(
    actual: &SimulationShellPropertyEdit,
    expected: &SimulationShellPropertyEdit,
) {
    assert_eq!(actual.archive().to_bytes(), expected.archive().to_bytes());
    assert_eq!(actual.archive().fields(), expected.archive().fields());
    assert_eq!(
        actual.archive().field_offsets(),
        expected.archive().field_offsets()
    );
    assert_eq!(actual.archive().context(), expected.archive().context());
    assert_eq!(actual.parent(), expected.parent());
    typed_load_history::assert_serial_outer(actual.history(), expected.history());
    typed_load_history::assert_serial_nested(actual.nested_history(), expected.nested_history());
}

fn run_worker(
    sources: &[SimulationShellProperty],
    options: &[TypedChoices],
    serial: &[Vec<SimulationShellPropertyEdit>],
    worker: usize,
) {
    for step in 0..32 {
        for body in 0..sources.len() {
            let selected = (worker * 32 + step) % options[body].len();
            let (index, value) = &options[body][selected];
            let actual = typed_edits::replace(&sources[body], *index, value.clone());
            assert_serial_equivalence(&actual, &serial[body][selected]);
            assert_eq!(actual.source().to_bytes(), sources[body].to_bytes());
        }
    }
}

fn run_workers(
    sources: &[SimulationShellProperty],
    options: &[TypedChoices],
    serial: &[Vec<SimulationShellPropertyEdit>],
) {
    std::thread::scope(|scope| {
        let workers: Vec<std::thread::ScopedJoinHandle<'_, ()>> = (0..8)
            .map(|worker| scope.spawn(move || run_worker(sources, options, serial, worker)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

#[test]
fn eight_workers_mix_all_typed_values_and_both_presence_contexts_against_full_serial_history() {
    let sources = parallel_sources();
    let original: Vec<Vec<u8>> = sources
        .iter()
        .map(SimulationShellProperty::to_bytes)
        .collect();
    let options: Vec<TypedChoices> = sources.iter().map(typed_values::choices).collect();
    assert_option_coverage(&options);
    let serial = serial_edits(&sources, &options);
    run_workers(&sources, &options, &serial);
    for (body, original) in sources.iter().zip(original) {
        assert_eq!(body.to_bytes(), original);
    }
}

fn serial_edits(
    sources: &[SimulationShellProperty],
    options: &[TypedChoices],
) -> Vec<Vec<SimulationShellPropertyEdit>> {
    sources
        .iter()
        .zip(options)
        .map(|(body, options)| {
            options
                .iter()
                .map(|(index, value)| typed_edits::replace(body, *index, value.clone()))
                .collect()
        })
        .collect()
}

fn assert_option_coverage(options: &[TypedChoices]) {
    for choices in options {
        assert!(choices.len() <= 8 * 32);
        let mut kinds = [false; 6];
        for (_, value) in choices {
            kinds[typed_edits::value_kind(value)] = true;
        }
        assert_eq!(kinds, [true; 6]);
    }
}
