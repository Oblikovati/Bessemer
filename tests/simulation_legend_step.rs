//! C851 stored FC is independent of explicit runtime availability (#166).
use bessemer::simulation::*;
#[path = "support/legend_step_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_step_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-step-operations.bin");
const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/legend-step-projection.bin");

fn step_runtime() -> SimulationLegendStepRuntime {
    SimulationLegendStepRuntime {
        step_count_raw: 2,
        plot_type_raw: 0,
        analysis_type_raw: 8,
        word_bd8_raw: 0,
        word_be8_raw: 2,
        word_bec_raw: -3,
    }
}

#[test]
fn plot21_override_is_count_qualified_and_keeps_the_earlier_bd8_read() {
    let mut runtime = step_runtime();
    runtime.plot_type_raw = 21;
    runtime.word_bd8_raw = i32::MIN;
    let plan = runtime.step_availability();
    assert_eq!(plan.availability_raw(), 1);
    assert_eq!(plan.coverage().runtime_member_mask, 11);
    for count in [i32::MIN, -1, 0, 1] {
        runtime.step_count_raw = count;
        let plan = runtime.step_availability();
        assert_eq!(plan.availability_raw(), 0);
        assert_eq!(plan.coverage().runtime_member_mask, 3);
    }
}

#[test]
fn ordinary_gate_and_analysis_branches_keep_exact_reached_read_masks() {
    let mut runtime = step_runtime();
    runtime.word_bd8_raw = -1;
    let plan = runtime.step_availability();
    assert_eq!(plan.availability_raw(), 0);
    assert_eq!(plan.coverage().runtime_member_mask, 11);
    runtime.word_bd8_raw = 0;
    runtime.analysis_type_raw = 1;
    let plan = runtime.step_availability();
    assert_eq!(plan.availability_raw(), 1);
    assert_eq!(plan.coverage().runtime_member_mask, 15);
    runtime.analysis_type_raw = 8;
    runtime.word_be8_raw = -2;
    let plan = runtime.step_availability();
    assert_eq!(plan.availability_raw(), 1);
    assert_eq!(plan.coverage().runtime_member_mask, 31);
}

#[test]
fn raw_bec_return_preserves_negative_zero_and_nonboolean_patterns() {
    let mut runtime = step_runtime();
    for raw in [i32::MIN, -3, -1, 0, 1, 2, i32::MAX] {
        runtime.word_bec_raw = raw;
        let plan = runtime.step_availability();
        assert_eq!(plan.availability_raw(), raw);
        assert_eq!(plan.coverage().runtime_member_mask, 63);
        let completion = if raw == 0 {
            SimulationLegendStepCompletion::CompletedZeroReturn
        } else {
            SimulationLegendStepCompletion::CompletedNonzeroReturn
        };
        assert_eq!(plan.completion(), completion);
    }
}

#[test]
fn fc_layout_role_preserves_absence_and_cannot_supply_runtime_availability() {
    let mut inputs = SimulationLegendStepInputs {
        layout_switch_raw: None,
    };
    assert_eq!(inputs.step_layout_nonzero(), None);
    let runtime = step_runtime();
    let before = runtime.step_availability();
    for raw in [i32::MIN, -1, 0, 1, 42, i32::MAX] {
        inputs.layout_switch_raw = Some(raw);
        assert_eq!(inputs.step_layout_nonzero(), Some(raw != 0));
        assert_eq!(inputs.layout_switch_raw, Some(raw));
        assert_eq!(runtime.step_availability(), before);
    }
}

#[test]
fn immutable_availability_keeps_all_supplied_parents_and_local_qualifications() {
    let mut runtime = step_runtime();
    let plan = runtime.step_availability();
    let before = plan;
    assert_eq!(plan.history().runtime, runtime);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    runtime.word_bec_raw = 0;
    assert_ne!(plan.history().runtime, runtime);
    assert_eq!(plan, before);
    assert_step_coverage(&plan);
    assert_step_diagnostics(&plan);
}

fn assert_step_coverage(plan: &SimulationLegendStepPlan) {
    let coverage = plan.coverage();
    assert_eq!(coverage.stored_words_read, 0);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(coverage.exact_signed32_policy);
    assert!(!coverage.runtime_plot_pointer_validated);
    assert!(!coverage.renderer_modeled);
}

fn assert_step_diagnostics(plan: &SimulationLegendStepPlan) {
    use SimulationLegendStepDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            RuntimePlotWordsExplicit,
            PointerSafetyNotEstablished,
            LayoutSwitchNotAvailability,
            RendererNotModeled
        ]
    );
}

#[test]
fn all_public_step_types_are_send_sync_and_parallel_evaluations_match_serial() {
    assert_step_thread_traits();
    let serial = step_runtime().step_availability();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| step_runtime().step_availability()))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

fn assert_step_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendStepInputs>();
    require::<SimulationLegendStepRuntime>();
    require::<SimulationLegendStepPlan>();
    require::<SimulationLegendStepHistory>();
    require::<SimulationLegendStepCoverage>();
    require::<SimulationLegendStepCompletion>();
    require::<SimulationLegendStepDiagnostic>();
}

#[test]
fn all_source_runtime_cases_and_reached_reads_match_on_eight_workers() {
    let serial = case_support::replay_step_operations(OPERATIONS);
    assert_eq!(serial.len(), 102272 * 3);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_step_operations(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
fn complete_owned_parents_and_fc_mutants_preserve_raw_presence_and_independent_runtime_results() {
    assert_eq!(
        projection_support::replay_step_projection(OWNED, 693).len(),
        693 * 26
    );
}

#[test]
fn all_owned_fc_views_and_availability_plans_match_across_eight_workers() {
    assert_eq!(
        projection_support::replay_step_parallel(OWNED, 693).len(),
        693 * 26
    );
}

#[test]
#[ignore = "Requires fresh complete pinned native step projections."]
fn complete_native_fc_views_and_hypothetical_runtime_policies_preserve_source_results() {
    let packet = std::fs::read("target/simulation-legend-step-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_step_parallel(&packet, 276).len(),
        276 * 26
    );
}

#[test]
#[ignore = "Requires native projections; supports independent process replay."]
fn export_legend_step_for_fresh_process_replay() {
    let mut words = case_support::replay_step_operations(OPERATIONS);
    words.extend(projection_support::replay_step_projection(OWNED, 693));
    let packet = std::fs::read("target/simulation-legend-step-projection-native.bin").unwrap();
    words.extend(projection_support::replay_step_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_LEGEND_STEP_OUTPUT").unwrap();
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    std::fs::write(destination, bytes).unwrap();
}
