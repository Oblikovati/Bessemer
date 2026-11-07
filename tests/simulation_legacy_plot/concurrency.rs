use super::*;
use std::sync::Arc;

fn worker_legacy_operations(
    plots: &[SimulationLegacyPlot],
    worker: usize,
) -> Vec<(usize, Vec<u8>)> {
    plots
        .iter()
        .enumerate()
        .skip(worker)
        .step_by(8)
        .map(|(index, plot)| (index, project_complete_operation(plot)))
        .collect()
}

#[test]
fn eight_workers_replay_every_owned_read_build_and_changed_field_against_the_serial_total_order() {
    let plots = Arc::new(owned_plots());
    let expected: Vec<_> = plots.iter().map(project_complete_operation).collect();
    assert_eq!(parallel_legacy_operations(&plots), expected);
    for plot in plots.iter() {
        check_read_coverage(plot);
    }
}

fn parallel_legacy_operations(plots: &Arc<Vec<SimulationLegacyPlot>>) -> Vec<Vec<u8>> {
    let workers: Vec<_> = (0..8)
        .map(|worker| {
            let plots = plots.clone();
            std::thread::spawn(move || worker_legacy_operations(&plots, worker))
        })
        .collect();
    let mut actual: Vec<_> = workers
        .into_iter()
        .flat_map(|worker| worker.join().unwrap())
        .collect();
    actual.sort_by_key(|(index, _)| *index);
    actual.into_iter().map(|(_, bytes)| bytes).collect()
}
