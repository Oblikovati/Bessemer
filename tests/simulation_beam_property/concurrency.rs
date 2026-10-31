use super::edits::{assert_isolated, assert_relations, modern_property, replace, replacements};
use super::*;

fn assert_nested(actual: &SimulationBeamEdit, serial: &SimulationBeamEdit) {
    assert_eq!(actual.nested_history().len(), serial.nested_history().len());
    for (row, expected) in actual.nested_history().iter().zip(serial.nested_history()) {
        assert_eq!(row.path, expected.path);
        assert_eq!(row.relation, expected.relation);
        assert_eq!(row.outer_parent, expected.outer_parent);
        assert_eq!(row.outer_result.ordinal(), expected.outer_result.ordinal());
        assert_eq!(row.parents, expected.parents);
        assert_eq!(
            row.result.as_ref().map(|id| id.index()),
            expected.result.as_ref().map(|id| id.index())
        );
        if let Some(id) = &row.result {
            let SimulationBeamValue::Shared(SimulationThermalValue::Embedded(archive)) =
                &actual.archive().fields()[row.outer_result.ordinal()].value
            else {
                panic!("expected embedded result");
            };
            assert_eq!(*id, archive.field_handle(id.index()).unwrap());
        }
    }
}

fn assert_serial(actual: &SimulationBeamEdit, serial: &SimulationBeamEdit) {
    assert_eq!(actual.archive().to_bytes(), serial.archive().to_bytes());
    assert_eq!(actual.archive().fields(), serial.archive().fields());
    assert_eq!(actual.parent(), serial.parent());
    assert_eq!(
        actual.coverage().checked_bytes,
        serial.coverage().checked_bytes
    );
    assert_eq!(
        actual.coverage().checked_fields,
        serial.coverage().checked_fields
    );
    assert_eq!(actual.coverage().completion, serial.coverage().completion);
    assert_eq!(actual.source().to_bytes(), serial.source().to_bytes());
    assert_nested(actual, serial);
    assert_relations(actual, actual.parent().ordinal());
    assert_isolated(actual, actual.parent().ordinal());
}

fn run_worker(
    source: &SimulationBeamProperty,
    options: &[(&str, SimulationBeamValue)],
    serial: &[SimulationBeamEdit],
    worker: usize,
) {
    for step in 0..32 {
        let selected = (worker + step) % options.len();
        let (path, value) = &options[selected];
        let edit = replace(source, path, value.clone());
        assert_serial(&edit, &serial[selected]);
    }
}

fn run_workers(
    source: &SimulationBeamProperty,
    options: &[(&str, SimulationBeamValue)],
    serial: &[SimulationBeamEdit],
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
fn mixed_concurrent_edits_match_serial_bytes_fields_coverage_and_all_parent_relations() {
    let source = modern_property();
    let original = source.to_bytes();
    let options = replacements();
    let serial: Vec<SimulationBeamEdit> = options
        .iter()
        .map(|(path, value)| replace(&source, path, value.clone()))
        .collect();
    run_workers(&source, &options, &serial);
    assert_eq!(source.to_bytes(), original);
}
