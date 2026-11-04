//! C851 local integer layout with explicit post-helper state and native read order (#166).
use bessemer::{FailureClass, simulation::*};
#[path = "support/legend_step_layout_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_step_layout_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-step-layout-operations.bin");
const OWNED: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-step-layout-projection.bin");

fn layout_runtime() -> SimulationLegendStepLayoutRuntime {
    SimulationLegendStepLayoutRuntime {
        enable_raw: 1,
        point_x_raw: Some(17),
        point_y_raw: Some(100),
        size_y_raw: Some(20),
        word_12c_raw: Some(121),
        word_124_raw: Some(100),
        after_helpers: Some(SimulationLegendStepLayoutAfterHelpers {
            spacing_raw: Some(7),
            layout_switch_raw: Some(0),
            word_1b0_raw: Some(0),
            word_1b4_raw: Some(0),
        }),
    }
}

fn layout_plan(
    fc: Option<i32>,
    runtime: SimulationLegendStepLayoutRuntime,
) -> SimulationLegendStepLayoutPlan {
    SimulationLegendStepInputs {
        layout_switch_raw: fc,
    }
    .project_step_layout(runtime)
    .unwrap()
}

#[test]
fn explicit_zero_enable_completes_without_any_optional_fact_or_value_output() {
    let runtime = SimulationLegendStepLayoutRuntime {
        enable_raw: 0,
        point_x_raw: None,
        point_y_raw: None,
        size_y_raw: None,
        word_12c_raw: None,
        word_124_raw: None,
        after_helpers: None,
    };
    let plan = layout_plan(None, runtime);
    assert_eq!(plan.values(), None);
    assert_eq!(
        plan.completion(),
        SimulationLegendStepLayoutCompletion::CompletedDisabledEarlyReturn
    );
    assert_eq!(plan.coverage().entry_member_mask, 0);
    assert_eq!(plan.coverage().post_helper_member_mask, 0);
    assert_eq!(plan.coverage().native_text_helper_boundaries, 0);
    assert!(!plan.coverage().division_20_reached);
    assert_eq!(plan.history().runtime, runtime);
}

#[test]
fn entry_zero_ignores_range_facts_and_keeps_the_later_fc_read_independent() {
    let mut runtime = layout_runtime();
    runtime.word_12c_raw = None;
    runtime.word_124_raw = None;
    let initial = layout_plan(Some(0), runtime);
    assert_eq!(initial.values().unwrap().initial_y_raw, 80);
    assert_eq!(initial.values().unwrap().vertical_after_spacing_raw, 73);
    assert_eq!(initial.coverage().entry_member_mask, 57);
    let after = runtime.after_helpers.as_mut().unwrap();
    after.layout_switch_raw = Some(-1);
    after.word_1b0_raw = Some(42);
    after.word_1b4_raw = None;
    let changed = layout_plan(Some(0), runtime);
    assert_eq!(changed.values().unwrap().spacing_raw, 30);
    assert_eq!(changed.values().unwrap().vertical_after_spacing_raw, 50);
    assert_eq!(changed.coverage().post_helper_member_mask, 7);
}

#[test]
fn nonzero_entry_wraps_the_difference_before_truncating_signed_division() {
    let mut runtime = layout_runtime();
    runtime.point_y_raw = Some(0);
    runtime.size_y_raw = Some(0);
    for (upper, lower, expected) in [
        (19, 0, 0),
        (21, 0, 1),
        (-19, 0, 0),
        (-21, 0, -1),
        (i32::MIN, 1, 107374182),
        (i32::MAX, -1, -107374182),
    ] {
        runtime.word_12c_raw = Some(upper);
        runtime.word_124_raw = Some(lower);
        let plan = layout_plan(Some(i32::MIN), runtime);
        assert_eq!(plan.values().unwrap().initial_y_raw, expected);
        assert_eq!(plan.coverage().entry_member_mask, 63);
        assert!(plan.coverage().division_20_reached);
    }
}

#[test]
fn post_helper_zero_fc_does_not_double_spacing_even_after_nonzero_entry() {
    let plan = layout_plan(Some(-1), layout_runtime());
    let values = plan.values().unwrap();
    assert_eq!(values.initial_y_raw, 81);
    assert_eq!(values.spacing_raw, 7);
    assert_eq!(values.vertical_after_spacing_raw, 74);
    assert_eq!(values.point_x_raw, 17);
    assert_eq!(plan.coverage().native_text_helper_boundaries, 3);
}

#[test]
fn spacing_doubling_minus_four_and_tripling_use_native_low32_patterns() {
    for (spacing, expected) in [(i32::MIN, -12), (i32::MAX, -18)] {
        let mut runtime = layout_runtime();
        let after = runtime.after_helpers.as_mut().unwrap();
        after.spacing_raw = Some(spacing);
        after.layout_switch_raw = Some(1);
        after.word_1b0_raw = Some(0);
        after.word_1b4_raw = Some(-1);
        let plan = layout_plan(Some(0), runtime);
        assert_eq!(plan.values().unwrap().spacing_raw, expected);
        assert_eq!(
            plan.values().unwrap().vertical_after_spacing_raw,
            80 - expected
        );
        assert_eq!(plan.coverage().post_helper_member_mask, 15);
    }
}

fn missing_layout_fact(fc: Option<i32>, runtime: SimulationLegendStepLayoutRuntime, entity: &str) {
    let inputs = SimulationLegendStepInputs {
        layout_switch_raw: fc,
    };
    let before = (inputs, runtime);
    let error = inputs.project_step_layout(runtime).unwrap_err();
    assert_eq!(error.operation, "legend step layout projection");
    assert_eq!(error.class, FailureClass::Missing);
    assert_eq!(error.entity, entity);
    assert!(error.cause.contains("None") && error.cause.contains("expected Some"));
    assert_eq!((inputs, runtime), before);
}

#[test]
fn zero_entry_requires_point_before_size_then_point_x_and_post_helper_state() {
    let mut runtime = layout_runtime();
    runtime.point_y_raw = None;
    runtime.size_y_raw = None;
    missing_layout_fact(None, runtime, "entry legend.word_fc");
    missing_layout_fact(Some(0), runtime, "pointY");
    runtime.point_y_raw = Some(0);
    missing_layout_fact(Some(0), runtime, "sizeY");
    runtime.size_y_raw = Some(0);
    runtime.point_x_raw = None;
    runtime.after_helpers = None;
    missing_layout_fact(Some(0), runtime, "pointX");
    runtime.point_x_raw = Some(0);
    missing_layout_fact(Some(0), runtime, "post-helper state");
}

#[test]
fn nonzero_entry_requires_range_words_then_size_before_point() {
    let mut runtime = layout_runtime();
    runtime.word_12c_raw = None;
    runtime.word_124_raw = None;
    runtime.size_y_raw = None;
    runtime.point_y_raw = None;
    missing_layout_fact(Some(1), runtime, "runtime word12C");
    runtime.word_12c_raw = Some(0);
    missing_layout_fact(Some(1), runtime, "runtime word124");
    runtime.word_124_raw = Some(0);
    missing_layout_fact(Some(1), runtime, "sizeY");
    runtime.size_y_raw = Some(0);
    missing_layout_fact(Some(1), runtime, "pointY");
}

#[test]
fn post_helper_spacing_fc_and_flags_are_required_in_native_order() {
    let mut runtime = layout_runtime();
    let after = runtime.after_helpers.as_mut().unwrap();
    after.spacing_raw = None;
    after.layout_switch_raw = None;
    after.word_1b0_raw = None;
    after.word_1b4_raw = None;
    missing_layout_fact(Some(0), runtime, "post-helper spacing");
    runtime.after_helpers.as_mut().unwrap().spacing_raw = Some(0);
    missing_layout_fact(Some(0), runtime, "post-helper legend.word_fc");
    runtime.after_helpers.as_mut().unwrap().layout_switch_raw = Some(0);
    missing_layout_fact(Some(0), runtime, "runtime word1B0");
    runtime.after_helpers.as_mut().unwrap().word_1b0_raw = Some(0);
    missing_layout_fact(Some(0), runtime, "runtime word1B4");
    runtime.after_helpers.as_mut().unwrap().word_1b0_raw = Some(1);
    assert_eq!(
        layout_plan(Some(0), runtime).values().unwrap().spacing_raw,
        0
    );
}

#[test]
fn immutable_local_values_keep_all_parents_and_exactness_qualifications() {
    let mut runtime = layout_runtime();
    let plan = layout_plan(Some(0), runtime);
    let before = plan;
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    assert_eq!(plan.history().inputs.layout_switch_raw, Some(0));
    assert_eq!(plan.history().runtime, runtime);
    runtime.after_helpers = None;
    assert_eq!(plan, before);
    assert_ne!(plan.history().runtime, runtime);
    let coverage = plan.coverage();
    assert!(coverage.exact_wrapping_signed32);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(!coverage.renderer_modeled && !coverage.pixel_coordinates_validated);
    assert_eq!(
        plan.completion(),
        SimulationLegendStepLayoutCompletion::CompletedIntegerLayout
    );
    assert_layout_diagnostics(&plan);
}

fn assert_layout_diagnostics(plan: &SimulationLegendStepLayoutPlan) {
    use SimulationLegendStepLayoutDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            AfterHelperFactsExplicit,
            TextHelpersNotModeled,
            CompleteRendererNotModeled,
            PixelValidityNotEstablished
        ]
    );
}

#[test]
fn all_public_layout_types_are_send_sync_and_parallel_projections_equal_serial() {
    assert_layout_thread_traits();
    let serial = layout_plan(Some(-1), layout_runtime());
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| layout_plan(Some(-1), layout_runtime())))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

fn assert_layout_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendStepLayoutRuntime>();
    require::<SimulationLegendStepLayoutAfterHelpers>();
    require::<SimulationLegendStepLayoutValues>();
    require::<SimulationLegendStepLayoutPlan>();
    require::<SimulationLegendStepLayoutCompletion>();
    require::<SimulationLegendStepLayoutHistory>();
    require::<SimulationLegendStepLayoutCoverage>();
    require::<SimulationLegendStepLayoutDiagnostic>();
}

#[test]
fn independent_presence_boundary_and_generated_layout_cases_match_on_eight_workers() {
    let serial = case_support::replay_layout_operations(OPERATIONS);
    assert_eq!(serial.len(), 35468 * 8);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_layout_operations(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
fn complete_owned_and_fc_mutant_parents_keep_hypothetical_post_helper_results() {
    assert_eq!(
        projection_support::replay_layout_projection(OWNED, 693).len(),
        693 * 64
    );
}

#[test]
fn all_owned_layout_projections_match_serial_results_on_eight_workers() {
    assert_eq!(
        projection_support::replay_layout_parallel(OWNED, 693).len(),
        693 * 64
    );
}

#[test]
#[ignore = "Requires fresh complete pinned native step-layout projections."]
fn complete_native_parent_layout_projections_match_independent_results_and_parallel_workers() {
    let packet =
        std::fs::read("target/simulation-legend-step-layout-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_layout_parallel(&packet, 276).len(),
        276 * 64
    );
}

#[test]
#[ignore = "Requires native projections; supports independent process replay."]
fn export_legend_step_layout_for_fresh_process_replay() {
    let mut words = case_support::replay_layout_operations(OPERATIONS);
    words.extend(projection_support::replay_layout_projection(OWNED, 693));
    let packet =
        std::fs::read("target/simulation-legend-step-layout-projection-native.bin").unwrap();
    words.extend(projection_support::replay_layout_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_LEGEND_STEP_LAYOUT_OUTPUT").unwrap();
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    std::fs::write(destination, bytes).unwrap();
}
