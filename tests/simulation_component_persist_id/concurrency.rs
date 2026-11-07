use super::edits::{assert_outer_history, assert_preservation, ordinal, replace, replacements};
use super::*;

fn assert_nested_origins(
    actual: &SimulationComponentPersistIdEdit,
    serial: &SimulationComponentPersistIdEdit,
) {
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
            let SimulationComponentPersistIdValue::Embedded(archive) =
                &actual.archive().fields()[row.outer_result.ordinal()].value
            else {
                panic!("{:?}; expected embedded result", row.path);
            };
            assert_eq!(*id, archive.field_handle(id.index()).unwrap());
        }
    }
}

fn assert_worker(
    source: &SimulationComponentPersistId,
    options: &[(&str, SimulationComponentPersistIdValue)],
    serial: &[SimulationComponentPersistIdEdit],
    worker: usize,
) {
    for step in 0..32 {
        let selected = (worker + step) % options.len();
        let (path, value) = &options[selected];
        let edit = replace(source, path, value.clone());
        assert_eq!(
            edit.archive().to_bytes(),
            serial[selected].archive().to_bytes()
        );
        assert_eq!(edit.parent(), serial[selected].parent());
        assert_eq!(edit.source().to_bytes(), source.to_bytes());
        assert_outer_history(&edit, ordinal(source, path));
        assert_preservation(&edit, ordinal(source, path));
        assert_nested_origins(&edit, &serial[selected]);
    }
}

fn run_workers(
    source: &SimulationComponentPersistId,
    options: &[(&str, SimulationComponentPersistIdValue)],
    serial: &[SimulationComponentPersistIdEdit],
) {
    std::thread::scope(|scope| {
        let workers: Vec<std::thread::ScopedJoinHandle<'_, ()>> = (0..8)
            .map(|worker| scope.spawn(move || assert_worker(source, options, serial, worker)))
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    });
}

#[test]
fn mixed_parallel_edits_match_serial_bytes_and_all_parent_relations() {
    let source = owned_property(840003, 1);
    let original = source.to_bytes();
    let options = replacements();
    let serial: Vec<SimulationComponentPersistIdEdit> = options
        .iter()
        .map(|(path, value)| replace(&source, path, value.clone()))
        .collect();
    run_workers(&source, &options, &serial);
    assert_eq!(source.to_bytes(), original);
}
