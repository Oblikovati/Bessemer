//! C848 raw controls and exact bounded dispatch, independent of destination algorithms (#166).
use bessemer::simulation::*;
#[path = "support/legend_palette_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_palette_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-palette-operations.bin");
const OWNED: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-palette-projection.bin");

#[test]
fn every_raw_diagnostic_guard_and_missing_or_null_context_matches_independent_results() {
    assert_eq!(
        case_support::replay_palette_operations(OPERATIONS).len(),
        74592 * 11
    );
}

#[test]
fn eight_workers_match_serial_operations_and_all_public_types_are_send_sync() {
    assert_palette_thread_traits();
    let serial = case_support::replay_palette_operations(OPERATIONS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_palette_operations(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

fn assert_palette_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendPaletteInputs>();
    require::<SimulationLegendPaletteRuntime>();
    require::<SimulationLegendPaletteDispatch>();
    require::<SimulationLegendPaletteCompletion>();
    require::<SimulationLegendPaletteCoverage>();
    require::<SimulationLegendPaletteHistory>();
    require::<SimulationLegendPaletteDiagnostic>();
    require::<SimulationLegendPalettePlan>();
}

#[test]
fn all_four_completed_routes_keep_supplied_parents_after_source_mutation() {
    for status in 1..=4 {
        let mut case = case_support::find_palette_case(OPERATIONS, status);
        let plan = case.inputs.dispatch_palette(case.runtime).unwrap();
        let before = plan;
        case.inputs.gray_above_yield_raw = None;
        case.inputs.mapping_mode_raw = Some(42);
        case.runtime.gray_helper_return_raw = Some(42);
        case.runtime.plot_pointer_present = None;
        case.runtime.plot_word18_raw = Some(42);
        case.words.fill(42);
        assert_eq!(plan, before);
        assert_ne!(plan.history().inputs, case.inputs);
        assert_ne!(plan.history().runtime, case.runtime);
    }
}

#[test]
fn all_owned_and_mutated_controls_preserve_complete_bodies_and_fixed_wire_slots() {
    assert_eq!(
        projection_support::replay_palette_projection(OWNED, 2630).len(),
        2630 * 7 * 11
    );
}

#[test]
fn owned_profiles_and_out_of_slot_counts_match_serial_on_eight_workers() {
    assert_eq!(
        projection_support::replay_palette_parallel(OWNED, 2630).len(),
        2630 * 7 * 11
    );
}

#[test]
#[ignore = "Requires fresh pinned complete native palette projections."]
fn all_native_inputs_preserve_words_missing_context_and_parallel_results() {
    let packet = std::fs::read("target/simulation-legend-palette-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_palette_parallel(&packet, 276).len(),
        276 * 7 * 11
    );
}

#[test]
#[ignore = "Requires native projections; supports independent fresh-process replay."]
fn export_legend_palette_for_fresh_process_replay() {
    let mut output = case_support::replay_palette_operations(OPERATIONS);
    output.extend(projection_support::replay_palette_projection(OWNED, 2630));
    let packet = std::fs::read("target/simulation-legend-palette-projection-native.bin").unwrap();
    output.extend(projection_support::replay_palette_projection(&packet, 276));
    let bytes: Vec<_> = output.into_iter().flat_map(u32::to_le_bytes).collect();
    let destination = std::env::var("BESSEMER_LEGEND_PALETTE_OUTPUT").unwrap();
    std::fs::write(destination, bytes).unwrap();
}
