use super::*;

type PropertyChoices = Vec<(usize, SimulationMeshPropertyValue)>;
fn concurrent_choices(body: &SimulationMeshProperty) -> PropertyChoices {
    let mut covered = std::collections::BTreeSet::new();
    typed_values::choices(body)
        .into_iter()
        .filter(|(index, value)| covered.insert(choice_key(&body.fields()[*index], value)))
        .collect()
}

fn choice_key(
    field: &SimulationMeshPropertyField,
    value: &SimulationMeshPropertyValue,
) -> (usize, usize, usize) {
    let kind = typed_load_values::value_kind(value);
    match value {
        SimulationMeshPropertyValue::Bytes(raw) => (kind, raw.len(), 0),
        SimulationMeshPropertyValue::Embedded(archive) => {
            let raw = archive.to_bytes().unwrap();
            (
                kind,
                u32::from_le_bytes(raw[..4].try_into().unwrap()) as usize,
                raw.len(),
            )
        }
        SimulationMeshPropertyValue::Text(_) => (
            kind,
            usize::from(field.path.contains("text_list")),
            usize::from(field.path.starts_with("mesh.text6")),
        ),
        _ => (kind, 0, 0),
    }
}

fn serial_choices(
    body: &SimulationMeshProperty,
    choices: &PropertyChoices,
) -> Vec<SimulationMeshPropertyEdit> {
    choices
        .iter()
        .map(|(index, value)| typed_edits::replace(body, *index, value.clone()))
        .collect()
}

fn assert_equivalent(actual: &SimulationMeshPropertyEdit, serial: &SimulationMeshPropertyEdit) {
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

fn assert_archive_equivalent(actual: &SimulationMeshProperty, serial: &SimulationMeshProperty) {
    assert_eq!(actual.to_bytes(), serial.to_bytes());
    assert_eq!(actual.fields(), serial.fields());
    assert_eq!(actual.field_offsets(), serial.field_offsets());
    assert_eq!(actual.record_spans(), serial.record_spans());
}

fn assert_coverage_equivalent(
    actual: SimulationMeshPropertyCoverage,
    serial: SimulationMeshPropertyCoverage,
) {
    assert_eq!(actual.version, serial.version);
    assert_eq!(actual.completion, serial.completion);
    assert_eq!(actual.check_mode, serial.check_mode);
    assert_eq!(actual.checked_bytes, serial.checked_bytes);
    assert_eq!(actual.checked_fields, serial.checked_fields);
    assert_eq!(actual.record_count, serial.record_count);
    assert_eq!(actual.nested_record_count, serial.nested_record_count);
    assert_eq!(actual.body_count, serial.body_count);
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
    body: &SimulationMeshProperty,
    choices: &PropertyChoices,
    serial: &[SimulationMeshPropertyEdit],
    worker: usize,
    visits: &mut [usize; 6],
) {
    for step in 0..choices.len() {
        let selected = (worker * 17 + step) % choices.len();
        let (index, value) = &choices[selected];
        let actual = body
            .edit_field(&body.field_id(*index).unwrap(), value.clone())
            .unwrap();
        assert_equivalent(&actual, &serial[selected]);
        let kind = typed_load_values::value_kind(value);
        visits[kind] += 1;
    }
}

fn run_worker(
    sources: &[SimulationMeshProperty],
    options: &[PropertyChoices],
    serial: &[Vec<SimulationMeshPropertyEdit>],
    worker: usize,
) -> [usize; 6] {
    let mut visits = [0_usize; 6];
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
    visits
}

fn run_workers(
    sources: &[SimulationMeshProperty],
    options: &[PropertyChoices],
    serial: &[Vec<SimulationMeshPropertyEdit>],
) {
    std::thread::scope(|scope| {
        let workers: Vec<std::thread::ScopedJoinHandle<'_, [usize; 6]>> = (0..8)
            .map(|worker| scope.spawn(move || run_worker(sources, options, serial, worker)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected_visits(options));
        }
    });
}

fn expected_visits(options: &[PropertyChoices]) -> [usize; 6] {
    let mut visits = [0_usize; 6];
    for choices in options {
        for (_, value) in choices {
            visits[typed_load_values::value_kind(value)] += 1;
        }
    }
    assert_eq!(visits, [588, 0, 588, 1326, 557, 580]);
    visits
}

#[test]
fn eight_workers_mix_every_owned_context_and_stored_value_family_against_complete_serial_results() {
    let sources = contracts::owned_properties();
    let original: Vec<Vec<u8>> = sources
        .iter()
        .map(SimulationMeshProperty::to_bytes)
        .collect();
    let options: Vec<PropertyChoices> = sources.iter().map(concurrent_choices).collect();
    let serial: Vec<Vec<SimulationMeshPropertyEdit>> = sources
        .iter()
        .zip(&options)
        .map(|(body, choices)| serial_choices(body, choices))
        .collect();
    run_workers(&sources, &options, &serial);
    for (body, original) in sources.iter().zip(original) {
        assert_eq!(body.to_bytes(), original);
    }
}
