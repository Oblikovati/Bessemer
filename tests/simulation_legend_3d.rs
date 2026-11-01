//! C852 explicit phase facts and local request/background projections (#166).
use bessemer::{FailureClass, simulation::*};
#[path = "support/legend_3d_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_3d_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-3d-operations.bin");
const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/legend-3d-projection.bin");

fn body_runtime(argument: i32) -> SimulationLegend3DBodyRuntime {
    SimulationLegend3DBodyRuntime {
        cell_count_at_mapping_raw: Some(12),
        rendering_argument_raw: Some(argument),
    }
}

fn body_plan(entry: i32, later: i32, argument: i32) -> SimulationLegend3DBodyPlan {
    SimulationLegendHeightInputs {
        cell_count_raw: Some(entry),
    }
    .project_3d_body_requests(SimulationLegend3DBodyRuntime {
        cell_count_at_mapping_raw: Some(later),
        rendering_argument_raw: Some(argument),
    })
    .unwrap()
}

fn background_runtime() -> SimulationLegend3DBackgroundRuntime {
    SimulationLegend3DBackgroundRuntime {
        packed_color_raw: Some(0),
        precision_at_width_raw: Some(3),
        lighting_result_raw: Some(0),
    }
}

#[test]
fn entry_allocation_and_actual_later_mapping_count_remain_separate() {
    let plan = body_plan(10, 12, 2);
    let values = plan.values();
    assert_eq!(values.allocation_pairs_raw, 11);
    assert_eq!(values.vertex_float_count_raw, 66);
    assert_eq!(values.vertex_allocation_bytes_raw, 264);
    assert_eq!(values.mapping_request_count_raw, 13);
    assert_eq!(values.color_float_count_raw, 42);
    assert_eq!(values.color_allocation_bytes_raw, 168);
    assert_eq!(values.initial_shade_model_raw, 0x1d01);
    assert_eq!(values.begin_primitive_raw, 8);
    assert_eq!(values.later_smooth_argument_raw, 0x1d01);
    assert_eq!(plan.history().inputs.cell_count_raw, Some(10));
    assert_eq!(plan.history().runtime.cell_count_at_mapping_raw, Some(12));
}

#[test]
fn exactly_argument_two_selects_smooth_strip_without_assigning_a_stored_enum() {
    for argument in [i32::MIN, -1, 0, 1, 2, 3, i32::MAX] {
        assert_body_mode(body_plan(10, 12, argument), argument);
    }
}

fn assert_body_mode(plan: SimulationLegend3DBodyPlan, argument: i32) {
    let values = plan.values();
    assert_eq!(
        values.mapping_request_count_raw,
        if argument == 2 { 13 } else { 12 }
    );
    assert_eq!(
        values.initial_shade_model_raw,
        if argument == 2 { 0x1d01 } else { 0x1d00 }
    );
    assert_eq!(
        values.begin_primitive_raw,
        if argument == 2 { 8 } else { 7 }
    );
    assert_eq!(values.later_smooth_argument_raw, 0x1d01);
    assert_eq!(
        plan.history().runtime.rendering_argument_raw,
        Some(argument)
    );
}

#[test]
fn raw_allocation_requests_wrap_signed32_then_saturate_unsigned_multiplication() {
    let negative = body_plan(-2, -1, 2);
    assert_eq!(negative.values().vertex_float_count_raw, -6);
    assert_eq!(negative.values().vertex_allocation_bytes_raw, u64::MAX);
    assert_eq!(negative.values().mapping_request_count_raw, 0);
    assert_eq!(
        negative.completion(),
        SimulationLegend3DCompletion::CompletedZeroMappingRequest
    );
    let empty = body_plan(-1, -1, 0);
    assert_eq!(empty.values().vertex_allocation_bytes_raw, 0);
    assert_eq!(empty.values().color_allocation_bytes_raw, 0);
    assert_eq!(
        empty.completion(),
        SimulationLegend3DCompletion::CompletedNonzeroMappingRequest
    );
    let wrapped = body_plan(i32::MAX, i32::MAX, 2);
    assert_eq!(wrapped.values().allocation_pairs_raw, i32::MIN);
    assert_eq!(wrapped.values().vertex_float_count_raw, 0);
    assert_eq!(wrapped.values().mapping_request_count_raw, i32::MIN);
    assert_eq!(wrapped.values().color_allocation_bytes_raw, u64::MAX);
}

fn missing_3d(error: bessemer::BessemerError, entity: &str) {
    assert_eq!(error.operation, "legend 3D local projection");
    assert_eq!(error.class, FailureClass::Missing);
    assert_eq!(error.entity, entity);
    assert!(error.cause.contains("None") && error.cause.contains("expected Some"));
}

#[test]
fn body_requires_entry_then_explicit_later_count_then_plain_argument() {
    missing_body_facts(None, None, None, "allocation-entry count14");
    missing_body_facts(Some(0), None, None, "post-vertex count14");
    missing_body_facts(Some(0), Some(0), None, "rendering argument");
    assert_eq!(body_plan(0, 0, 0).values().mapping_request_count_raw, 0);
}

fn missing_body_facts(entry: Option<i32>, later: Option<i32>, argument: Option<i32>, entity: &str) {
    let inputs = SimulationLegendHeightInputs {
        cell_count_raw: entry,
    };
    let runtime = SimulationLegend3DBodyRuntime {
        cell_count_at_mapping_raw: later,
        rendering_argument_raw: argument,
    };
    missing_3d(
        inputs.project_3d_body_requests(runtime).unwrap_err(),
        entity,
    );
}

#[test]
fn background_constructs_packed_argument_without_stored_color_or_getter_output_claims() {
    let mut runtime = background_runtime();
    assert_eq!(background_color_bits(runtime), [0, 0, 0, 0x3f800000]);
    runtime.packed_color_raw = Some(0xff000000);
    assert_eq!(background_color_bits(runtime), [0, 0, 0, 0x3f800000]);
    runtime.packed_color_raw = Some(u32::MAX);
    let plan = runtime.project_background_nearest_even().unwrap();
    assert_eq!(plan.values().constructed_color.bits(), [0x3f800000; 4]);
    assert_eq!(plan.values().precision_factor_raw, 13);
    assert_eq!(plan.values().begin_primitive_raw, 9);
    assert_eq!(plan.values().lighting_capability_raw, 0xb50);
    assert_eq!(
        plan.completion(),
        SimulationLegend3DCompletion::CompletedBackgroundInputs
    );
}

fn background_color_bits(runtime: SimulationLegend3DBackgroundRuntime) -> [u32; 4] {
    runtime
        .project_background_nearest_even()
        .unwrap()
        .values()
        .constructed_color
        .bits()
}

#[test]
fn precision_factor_preserves_wrapping_and_every_raw_lighting_byte_is_retained() {
    let mut runtime = background_runtime();
    runtime.precision_at_width_raw = Some(i32::MAX);
    for raw in 0..=u8::MAX {
        runtime.lighting_result_raw = Some(raw);
        let plan = runtime.project_background_nearest_even().unwrap();
        assert_eq!(plan.values().precision_factor_raw, -2147483639);
        assert_eq!(plan.values().restore_lighting, raw != 0);
        assert_eq!(plan.history().runtime.lighting_result_raw, Some(raw));
    }
}

#[test]
fn background_requires_packed_argument_before_precision_and_lighting() {
    let mut runtime = SimulationLegend3DBackgroundRuntime {
        packed_color_raw: None,
        precision_at_width_raw: None,
        lighting_result_raw: None,
    };
    missing_3d(
        runtime.project_background_nearest_even().unwrap_err(),
        "packed background argument",
    );
    runtime.packed_color_raw = Some(0);
    missing_3d(
        runtime.project_background_nearest_even().unwrap_err(),
        "precision BC at width read",
    );
    runtime.precision_at_width_raw = Some(0);
    missing_3d(
        runtime.project_background_nearest_even().unwrap_err(),
        "lighting GLboolean result",
    );
}

#[test]
fn immutable_requests_keep_original_facts_and_explicit_native_exclusions() {
    let mut inputs = SimulationLegendHeightInputs {
        cell_count_raw: Some(10),
    };
    let mut runtime = body_runtime(2);
    let plan = inputs.project_3d_body_requests(runtime).unwrap();
    let before = plan;
    inputs.cell_count_raw = None;
    runtime.cell_count_at_mapping_raw = None;
    assert_eq!(plan, before);
    assert_ne!(plan.history().inputs, inputs);
    assert_ne!(plan.history().runtime, runtime);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    assert_3d_coverage(plan.coverage(), true);
    assert_body_diagnostics(&plan);
    let mut background = background_runtime();
    let background_plan = background.project_background_nearest_even().unwrap();
    background.packed_color_raw = None;
    assert_ne!(background_plan.history().runtime, background);
    assert_3d_coverage(background_plan.coverage(), false);
    assert_background_diagnostics(&background_plan);
}

fn assert_3d_coverage(coverage: SimulationLegend3DCoverage, body: bool) {
    assert!(coverage.exact_integer_patterns);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(
        !coverage.actual_allocation_performed
            && !coverage.renderer_modeled
            && !coverage.pointer_geometry_validated
    );
    assert_eq!(coverage.stored_snapshot_words_read, usize::from(body));
    assert_eq!(coverage.runtime_scalars_read, if body { 2 } else { 3 });
    assert_eq!(coverage.allocation_requests_computed, body);
    assert_eq!(coverage.nearest_even_constructor, !body);
}

fn assert_body_diagnostics(plan: &SimulationLegend3DBodyPlan) {
    use SimulationLegend3DDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            AllocationAndGeometryNotValidated,
            RendererHelpersNotModeled,
            LaterFactsExplicit,
            StoredArgumentIdentityNotInferred
        ]
    );
}

fn assert_background_diagnostics(plan: &SimulationLegend3DBackgroundPlan) {
    use SimulationLegend3DDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            LaterFactsExplicit,
            StoredArgumentIdentityNotInferred,
            BackgroundFloatingOutputNotModeled,
            CompleteGlStateNotEstablished
        ]
    );
}

#[test]
fn all_public_3d_types_are_send_sync_and_eight_workers_match_serial_projections() {
    assert_3d_thread_traits();
    let serial = paired_3d_plans();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(paired_3d_plans)).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

fn paired_3d_plans() -> (SimulationLegend3DBodyPlan, SimulationLegend3DBackgroundPlan) {
    (
        body_plan(10, 12, 2),
        background_runtime()
            .project_background_nearest_even()
            .unwrap(),
    )
}

fn assert_3d_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegend3DBodyRuntime>();
    require::<SimulationLegend3DBodyValues>();
    require::<SimulationLegend3DBodyHistory>();
    require::<SimulationLegend3DBodyPlan>();
    require::<SimulationLegend3DBackgroundRuntime>();
    require::<SimulationLegend3DBackgroundValues>();
    require::<SimulationLegend3DBackgroundHistory>();
    require::<SimulationLegend3DBackgroundPlan>();
    require::<SimulationLegend3DCompletion>();
    require::<SimulationLegend3DCoverage>();
    require::<SimulationLegend3DDiagnostic>();
}

#[test]
fn all_independent_3d_body_background_cases_match_on_eight_workers() {
    let serial = case_support::replay_3d_operations(OPERATIONS);
    assert_eq!(serial.len(), 26784 * 12);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_3d_operations(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
fn complete_owned_count_precision_and_mutant_parents_keep_explicit_phase_results() {
    assert_eq!(
        projection_support::replay_3d_projection(OWNED, 877).len(),
        877 * 192
    );
}

#[test]
fn all_owned_3d_local_projections_match_serial_results_on_eight_workers() {
    assert_eq!(
        projection_support::replay_3d_parallel(OWNED, 877).len(),
        877 * 192
    );
}

#[test]
#[ignore = "Requires fresh complete pinned native 3D consumer projections."]
fn complete_native_3d_local_projections_preserve_raw_fields_and_hypothetical_facts() {
    let packet = std::fs::read("target/simulation-legend-3d-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_3d_parallel(&packet, 276).len(),
        276 * 192
    );
}

#[test]
#[ignore = "Requires native projections; supports independent process replay."]
fn export_legend_3d_for_fresh_process_replay() {
    let mut words = case_support::replay_3d_operations(OPERATIONS);
    words.extend(projection_support::replay_3d_projection(OWNED, 877));
    let packet = std::fs::read("target/simulation-legend-3d-projection-native.bin").unwrap();
    words.extend(projection_support::replay_3d_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_LEGEND_3D_OUTPUT").unwrap();
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    std::fs::write(destination, bytes).unwrap();
}
