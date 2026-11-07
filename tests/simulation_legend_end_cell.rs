//! C853 stored fields never supply implicit native read-time defaults (#166).
use bessemer::simulation::*;
#[path = "support/legend_end_cell_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_end_cell_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-end-cell-operations.bin");
const OWNED: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-end-cell-projection.bin");

fn end_cell_runtime() -> SimulationLegendEndCellRuntime {
    SimulationLegendEndCellRuntime {
        plot_word_524_raw: Some(-1),
        enabled_at_gate_raw: Some(i32::MIN),
        color_at_rgb_copy: Some(SimulationRgbaColor::from_bits([
            0x7f800001, 0x80000000, 0xff800000, 0x7fa12345,
        ])),
    }
}

#[test]
fn runtime_zero_skips_missing_e8_and_ec_before_stored_gate_is_reached() {
    let runtime = SimulationLegendEndCellRuntime {
        plot_word_524_raw: Some(0),
        enabled_at_gate_raw: None,
        color_at_rgb_copy: None,
    };
    assert_eq!(
        case_support::end_cell_result_words(runtime),
        [1, 1, 0, 0, 0, 1]
    );
}

#[test]
fn zero_e8_skips_missing_rgb_after_nonzero_runtime_gate() {
    let runtime = SimulationLegendEndCellRuntime {
        plot_word_524_raw: Some(i32::MIN),
        enabled_at_gate_raw: Some(0),
        color_at_rgb_copy: None,
    };
    assert_eq!(
        case_support::end_cell_result_words(runtime),
        [1, 3, 0, 0, 0, 1]
    );
}

#[test]
fn reached_unknowns_fail_in_native_read_order_without_successful_partial_rgb() {
    let mut runtime = end_cell_runtime();
    runtime.plot_word_524_raw = None;
    runtime.enabled_at_gate_raw = None;
    runtime.color_at_rgb_copy = None;
    assert_end_cell_missing(runtime, 10);
    runtime.plot_word_524_raw = Some(1);
    assert_end_cell_missing(runtime, 11);
    runtime.enabled_at_gate_raw = Some(-1);
    assert_end_cell_missing(runtime, 12);
}

fn assert_end_cell_missing(runtime: SimulationLegendEndCellRuntime, status: u32) {
    assert_eq!(
        case_support::end_cell_result_words(runtime),
        [status, 0, 0, 0, 0, 0]
    );
}

#[test]
fn rgb_copy_preserves_nan_infinity_and_signed_zero_without_consuming_alpha() {
    let runtime = end_cell_runtime();
    let plan = runtime.project_end_cell().unwrap();
    assert_eq!(plan.rgb_words(), Some([0x7f800001, 0x80000000, 0xff800000]));
    assert_eq!(plan.coverage().reached_read_mask, 7);
    assert_eq!(
        plan.completion(),
        SimulationLegendEndCellCompletion::CompletedRgbCopy
    );
    for alpha in [0, 1, 0x80000000, 0x7fa12345, u32::MAX] {
        assert_end_cell_alpha(runtime, plan, alpha);
    }
}

fn assert_end_cell_alpha(
    runtime: SimulationLegendEndCellRuntime,
    plan: SimulationLegendEndCellPlan,
    alpha: u32,
) {
    let changed = SimulationLegendEndCellRuntime {
        color_at_rgb_copy: Some(SimulationRgbaColor::from_bits([
            0x7f800001, 0x80000000, 0xff800000, alpha,
        ])),
        ..runtime
    }
    .project_end_cell()
    .unwrap();
    assert_eq!(changed.rgb_words(), plan.rgb_words());
    assert_eq!(
        changed.history().runtime.color_at_rgb_copy.unwrap().bits()[3],
        alpha
    );
}

#[test]
fn published_plan_keeps_all_parents_isolated_from_source_mutation() {
    let mut runtime = end_cell_runtime();
    let plan = runtime.project_end_cell().unwrap();
    let before = plan;
    assert_eq!(plan.history().runtime, runtime);
    runtime.enabled_at_gate_raw = Some(0);
    runtime.color_at_rgb_copy = None;
    assert_ne!(runtime, plan.history().runtime);
    assert_eq!(plan, before);
    assert_ne!(runtime.project_end_cell().unwrap(), plan);
}

#[test]
fn every_public_end_cell_type_is_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendEndCellInputs>();
    require::<SimulationLegendEndCellRuntime>();
    require::<SimulationLegendEndCellCompletion>();
    require::<SimulationLegendEndCellHistory>();
    require::<SimulationLegendEndCellCoverage>();
    require::<SimulationLegendEndCellDiagnostic>();
    require::<SimulationLegendEndCellPlan>();
}

#[test]
fn all_independent_end_cell_cases_match_on_eight_workers() {
    let serial = case_support::replay_end_cell_operations(OPERATIONS);
    assert_eq!(serial.len(), 7232 * 6);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_end_cell_operations(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
fn complete_owned_and_mutant_end_cell_parents_preserve_optional_raw_views() {
    assert_eq!(
        projection_support::replay_end_cell_projection(OWNED, 609).len(),
        609 * 54
    );
}

#[test]
fn all_owned_end_cell_projections_match_serial_results_on_eight_workers() {
    assert_eq!(
        projection_support::replay_end_cell_parallel(OWNED, 609).len(),
        609 * 54
    );
}

#[test]
#[ignore = "Requires fresh complete pinned native end-cell projections."]
fn complete_native_end_cell_parents_keep_explicit_hypothetical_runtime_facts() {
    let packet = std::fs::read("target/simulation-legend-end-cell-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_end_cell_parallel(&packet, 276).len(),
        276 * 54
    );
}

#[test]
#[ignore = "Requires native projections; supports independent process replay."]
fn export_legend_end_cell_for_fresh_process_replay() {
    let mut words = case_support::replay_end_cell_operations(OPERATIONS);
    words.extend(projection_support::replay_end_cell_projection(OWNED, 609));
    let packet = std::fs::read("target/simulation-legend-end-cell-projection-native.bin").unwrap();
    words.extend(projection_support::replay_end_cell_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_LEGEND_END_CELL_OUTPUT").unwrap();
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    std::fs::write(destination, bytes).unwrap();
}
