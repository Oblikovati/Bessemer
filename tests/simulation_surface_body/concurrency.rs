use super::edits::{assert_outer_history, assert_preservation, ordinal, replace, replacements};
use super::*;

fn assert_worker(
    source: &SimulationSurfaceBody,
    options: &[(&str, SimulationSurfaceBodyValue)],
    serial: &[SimulationSurfaceBodyEdit],
    worker: usize,
) {
    for step in 0..32 {
        let index = (worker + step) % options.len();
        let (path, replacement) = &options[index];
        let edit = replace(source, path, replacement.clone());
        assert_eq!(
            edit.archive().to_bytes(),
            serial[index].archive().to_bytes()
        );
        assert_eq!(edit.archive().fields(), serial[index].archive().fields());
        assert_eq!(edit.parent(), serial[index].parent());
        assert_eq!(edit.source().to_bytes(), source.to_bytes());
        assert_outer_history(&edit, ordinal(source, path));
        assert_preservation(&edit, ordinal(source, path));
    }
}

#[test]
fn concurrent_surface_edits_match_every_serial_field_and_ordered_parent() {
    let source = owned_property(870022, 1);
    let before = source.to_bytes();
    let options = replacements();
    let serial: Vec<SimulationSurfaceBodyEdit> = options
        .iter()
        .map(|(path, value)| replace(&source, path, value.clone()))
        .collect();
    std::thread::scope(|scope| {
        for worker in 0..8 {
            let source = &source;
            let options = &options;
            let serial = &serial;
            scope.spawn(move || assert_worker(source, options, serial, worker));
        }
    });
    assert_eq!(source.to_bytes(), before);
}
