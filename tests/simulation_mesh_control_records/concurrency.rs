use super::*;

type RecordChoices = Vec<(usize, SimulationMeshControlValue)>;
const VISITS: [usize; 7] = [446, 446, 446, 6983, 248, 256, 62];

fn concurrent_choices(body: &SimulationMeshControlRecords) -> RecordChoices {
    if body.record_spans().is_empty() {
        return vec![(0, SimulationMeshControlValue::Signed(0))];
    }
    typed_values::choices(body)
}

fn serial_choices(
    body: &SimulationMeshControlRecords,
    choices: &RecordChoices,
) -> Vec<SimulationMeshControlEdit> {
    choices
        .iter()
        .map(|(index, value)| {
            if body.record_spans().is_empty() {
                return body
                    .edit_field(&body.field_id(*index).unwrap(), value.clone())
                    .unwrap();
            }
            typed_edits::replace(body, *index, value.clone())
        })
        .collect()
}

fn assert_equivalent(actual: &SimulationMeshControlEdit, serial: &SimulationMeshControlEdit) {
    assert_eq!(actual.source().to_bytes(), serial.source().to_bytes());
    assert_archive_equivalent(actual.archive(), serial.archive());
    assert_eq!(actual.parent(), serial.parent());
    assert_eq!(actual.result().ordinal(), serial.result().ordinal());
    assert_eq!(
        actual.result() == actual.parent(),
        serial.result() == serial.parent()
    );
    assert_eq!(actual.replacement(), serial.replacement());
    typed_load_history::assert_serial_outer(actual.history(), serial.history());
    typed_load_history::assert_serial_nested(actual.nested_history(), serial.nested_history());
    assert_coverage_equivalent(actual.coverage(), serial.coverage());
}

fn assert_archive_equivalent(
    actual: &SimulationMeshControlRecords,
    serial: &SimulationMeshControlRecords,
) {
    assert_eq!(actual.to_bytes(), serial.to_bytes());
    assert_eq!(actual.fields(), serial.fields());
    assert_eq!(actual.field_offsets(), serial.field_offsets());
    assert_eq!(actual.record_spans(), serial.record_spans());
}

fn assert_coverage_equivalent(
    actual: SimulationMeshControlCoverage,
    serial: SimulationMeshControlCoverage,
) {
    assert_eq!(actual.version, serial.version);
    assert_eq!(actual.completion, serial.completion);
    assert_eq!(actual.check_mode, serial.check_mode);
    assert_eq!(actual.checked_bytes, serial.checked_bytes);
    assert_eq!(actual.checked_fields, serial.checked_fields);
    assert_eq!(actual.record_count, serial.record_count);
    assert_eq!(actual.declared_entries, serial.declared_entries);
    assert_eq!(actual.embedded_archives, serial.embedded_archives);
    assert_eq!(actual.embedded_fields, serial.embedded_fields);
    assert_eq!(actual.diagnostics, serial.diagnostics);
    assert_eq!(
        actual.requested_pattern_error_bits,
        serial.requested_pattern_error_bits
    );
    assert_eq!(
        budgets::limit_signature(actual.limits),
        budgets::limit_signature(serial.limits)
    );
}

fn run_context(
    body: &SimulationMeshControlRecords,
    choices: &RecordChoices,
    serial: &[SimulationMeshControlEdit],
    worker: usize,
    visits: &mut [usize; 7],
) {
    for step in 0..choices.len() {
        let selected = (worker * 17 + step) % choices.len();
        let (index, value) = &choices[selected];
        let actual = body
            .edit_field(&body.field_id(*index).unwrap(), value.clone())
            .unwrap();
        assert_equivalent(&actual, &serial[selected]);
        let kind = if body.record_spans().is_empty() {
            6
        } else {
            typed_load_values::value_kind(value)
        };
        visits[kind] += 1;
    }
}

fn run_worker(
    sources: &[SimulationMeshControlRecords],
    options: &[RecordChoices],
    serial: &[Vec<SimulationMeshControlEdit>],
    worker: usize,
) {
    let mut visits = [0_usize; 7];
    for step in 0..sources.len() {
        let context = (worker * 37 + step) % sources.len();
        run_context(
            &sources[context],
            &options[context],
            &serial[context],
            worker,
            &mut visits,
        );
    }
    assert_eq!(visits, VISITS);
}

fn run_workers(
    sources: &[SimulationMeshControlRecords],
    options: &[RecordChoices],
    serial: &[Vec<SimulationMeshControlEdit>],
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
fn eight_workers_mix_every_owned_context_and_all_six_changed_kinds_against_complete_serial_results()
{
    let sources = contracts::owned_collections();
    let original: Vec<Vec<u8>> = sources
        .iter()
        .map(SimulationMeshControlRecords::to_bytes)
        .collect();
    let options: Vec<RecordChoices> = sources.iter().map(concurrent_choices).collect();
    let serial: Vec<Vec<SimulationMeshControlEdit>> = sources
        .iter()
        .zip(&options)
        .map(|(body, choices)| serial_choices(body, choices))
        .collect();
    run_workers(&sources, &options, &serial);
    for (body, original) in sources.iter().zip(original) {
        assert_eq!(body.to_bytes(), original);
    }
}
