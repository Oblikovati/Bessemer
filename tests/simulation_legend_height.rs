//! C849 raw height multiplier and exact final product with explicit helper results (#166).
use bessemer::simulation::*;
#[path = "support/legend_height_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_height_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-height-operations.bin");
const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/legend-height-projection.bin");

#[test]
fn exact_signed32_products_overflow_and_missing_dependency_order_match_independent_results() {
    assert_eq!(
        case_support::replay_height_operations(OPERATIONS).len(),
        8864 * 4
    );
}

#[test]
fn all_public_types_are_send_sync_and_eight_workers_match_serial_products() {
    assert_height_thread_traits();
    let serial = case_support::replay_height_operations(OPERATIONS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_height_operations(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

fn assert_height_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendHeightInputs>();
    require::<SimulationLegendHeightPlan>();
    require::<SimulationLegendHeightHistory>();
    require::<SimulationLegendHeightCoverage>();
    require::<SimulationLegendHeightCompletion>();
    require::<SimulationLegendHeightDiagnostic>();
}

#[test]
fn completed_zero_and_nonzero_results_keep_original_parents_after_mutation() {
    for status in [1, 2] {
        let mut case = case_support::find_height_case(OPERATIONS, status);
        let plan = case
            .inputs
            .total_height_from_cell_height(case.cell)
            .unwrap();
        let before = plan;
        case.inputs.cell_count_raw = None;
        case.cell = None;
        case.words.fill(42);
        assert_eq!(plan, before);
        assert_ne!(plan.history().inputs, case.inputs);
        assert_ne!(Some(plan.history().cell_height_raw), case.cell);
    }
}

#[test]
fn all_owned_and_count_mutant_bodies_keep_raw_words_absence_and_bytes() {
    assert_eq!(
        projection_support::replay_height_projection(OWNED, 693).len(),
        693 * 5 * 4
    );
}

#[test]
fn owned_complete_profiles_and_arbitrary_count_words_match_on_eight_workers() {
    assert_eq!(
        projection_support::replay_height_parallel(OWNED, 693).len(),
        693 * 5 * 4
    );
}

#[test]
#[ignore = "Requires fresh pinned complete native height projections."]
fn native_count_views_and_hypothetical_products_preserve_complete_parents_and_parallel_results() {
    let packet = std::fs::read("target/simulation-legend-height-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_height_parallel(&packet, 276).len(),
        276 * 5 * 4
    );
}

#[test]
#[ignore = "Requires native projections; supports independent process replay."]
fn export_legend_height_for_fresh_process_replay() {
    let mut output = case_support::replay_height_operations(OPERATIONS);
    output.extend(projection_support::replay_height_projection(OWNED, 693));
    let packet = std::fs::read("target/simulation-legend-height-projection-native.bin").unwrap();
    output.extend(projection_support::replay_height_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_LEGEND_HEIGHT_OUTPUT").unwrap();
    let bytes: Vec<_> = output.into_iter().flat_map(u32::to_le_bytes).collect();
    std::fs::write(destination, bytes).unwrap();
}
