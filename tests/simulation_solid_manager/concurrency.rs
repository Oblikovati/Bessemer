use super::edits::{assert_history, assert_preserved, replace, replacements};
use super::*;

fn options() -> Vec<(&'static str, SimulationSolidManagerValue)> {
    let mut values = replacements();
    let odd = SimulationPersistText::from_payload(Some(&[0x44, 0, 0]), 32).unwrap();
    values.push(("liste8.0", SimulationSolidManagerValue::Text(odd)));
    values
}

fn assert_serial(actual: &SimulationSolidManagerEdit, expected: &SimulationSolidManagerEdit) {
    assert_eq!(actual.archive().to_bytes(), expected.archive().to_bytes());
    assert_eq!(actual.archive().fields(), expected.archive().fields());
    assert_eq!(actual.parent(), expected.parent());
    assert_eq!(actual.source().to_bytes(), expected.source().to_bytes());
    assert_eq!(
        actual.result(),
        actual
            .archive()
            .field_id(actual.result().ordinal())
            .unwrap()
    );
    assert_coverage(actual.coverage(), expected.coverage());
    assert_history(actual);
    assert_preserved(actual);
}

fn assert_coverage(
    actual: SimulationSolidManagerCoverage,
    expected: SimulationSolidManagerCoverage,
) {
    assert_eq!(actual.version, expected.version);
    assert_eq!(actual.limits, expected.limits);
    assert_eq!(actual.completion, expected.completion);
    assert_eq!(actual.check_mode, expected.check_mode);
    assert_eq!(actual.checked_bytes, expected.checked_bytes);
    assert_eq!(actual.checked_fields, expected.checked_fields);
    assert_eq!(actual.list_entries, expected.list_entries);
    assert_eq!(actual.diagnostics, expected.diagnostics);
    assert_eq!(
        actual.requested_pattern_error_bits,
        expected.requested_pattern_error_bits
    );
}

fn run_worker(
    source: &SimulationSolidManagerProperty,
    options: &[(&str, SimulationSolidManagerValue)],
    serial: &[SimulationSolidManagerEdit],
    worker: usize,
) {
    for step in 0..32 {
        let selected = (worker + step) % options.len();
        let (path, value) = &options[selected];
        assert_serial(&replace(source, path, value.clone()), &serial[selected]);
    }
}

fn run_workers(
    source: &SimulationSolidManagerProperty,
    options: &[(&str, SimulationSolidManagerValue)],
    serial: &[SimulationSolidManagerEdit],
) {
    std::thread::scope(|scope| {
        let workers: Vec<std::thread::ScopedJoinHandle<'_, ()>> = (0..8)
            .map(|worker| scope.spawn(move || run_worker(source, options, serial, worker)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

#[test]
fn mixed_signed_utf16_and_odd_frame_edits_match_serial_bytes_fields_coverage_and_all_parents() {
    let source = owned_header(870008, 5);
    let original = source.to_bytes();
    let options = options();
    let serial: Vec<SimulationSolidManagerEdit> = options
        .iter()
        .map(|(path, value)| replace(&source, path, value.clone()))
        .collect();
    run_workers(&source, &options, &serial);
    assert_eq!(source.to_bytes(), original);
}
