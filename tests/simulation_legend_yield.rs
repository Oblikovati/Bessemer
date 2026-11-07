//! C850 call ordering and signed32 arithmetic with explicit post-helper facts (#166).
use bessemer::FailureClass;
use bessemer::simulation::*;
#[path = "support/legend_yield_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_yield_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-yield-operations.bin");
const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/legend-yield-projection.bin");

fn yield_inputs() -> SimulationLegendYieldInputs {
    SimulationLegendYieldInputs {
        gray_base_color: Some(SimulationRgbaColor::from_bits([
            0x7fc00001, 0x80000000, 0x7f800000, 0xff800001,
        ])),
        reverse_base_palette_raw: Some(i32::MIN),
    }
}

fn yield_runtime(index: i32, count: i32) -> SimulationLegendYieldRuntime {
    SimulationLegendYieldRuntime {
        requested_output_count_raw: count,
        split_index_raw: Some(index),
        after_suffix: Some(SimulationLegendYieldPrefixState {
            reverse_base_palette_raw: 42,
            base_palette_count_raw: -19,
            split_index_raw: -71,
        }),
    }
}

fn yield_plan(index: i32, count: i32) -> SimulationLegendYieldPlan {
    yield_inputs()
        .plan_gray_calls_nearest_even(yield_runtime(index, count))
        .unwrap()
}

#[test]
fn sentinel_forwards_every_signed_count_and_raw_reversal_to_one_copied_pair_call() {
    for count in [i32::MIN, -2, -1, 0, 1, i32::MAX] {
        let plan = yield_plan(-2, count);
        assert_eq!(plan.return_raw_if_calls_complete(), 1);
        assert_eq!(plan.calls().len(), 1);
        assert_eq!(plan.calls()[0].destination_byte_offset, 0);
        assert_eq!(plan.calls()[0].output_count_raw, count);
        assert_eq!(plan.calls()[0].base_count_raw, 2);
        assert_eq!(plan.calls()[0].reversal_raw, i32::MIN);
        assert_eq!(
            plan.calls()[0].base,
            SimulationLegendYieldBase::CopiedYieldPair
        );
        assert!(!plan.coverage().post_suffix_state_read);
    }
}

#[test]
fn complete_fallback_still_constructs_and_copies_all_four_arbitrary_color_words() {
    for (index, count) in [(-3, 10), (-1, 10), (0, 0), (1, 1), (1, -9)] {
        let mut inputs = yield_inputs();
        inputs.reverse_base_palette_raw = None;
        let plan = inputs
            .plan_gray_calls_nearest_even(yield_runtime(index, count))
            .unwrap();
        assert!(plan.calls().is_empty());
        assert_eq!(plan.return_raw_if_calls_complete(), 0);
        assert_eq!(
            plan.completion(),
            SimulationLegendYieldCompletion::CompletedFallbackPlan
        );
        assert_eq!(plan.copied_base_pair()[0], inputs.gray_base_color.unwrap());
        assert_eq!(plan.copied_base_pair()[1].bits(), [0x3f800000; 4]);
        assert!(plan.coverage().stored_color_copied);
        assert!(!plan.coverage().entry_reversal_read);
    }
}

#[test]
fn partial_suffix_precedes_prefix_and_retains_explicit_changed_rereads_even_at_zero() {
    for index in [0, 1, 4] {
        let plan = yield_plan(index, 5);
        assert_eq!(plan.calls().len(), 2);
        assert_eq!(plan.calls()[0].output_count_raw, 5 - index);
        assert_eq!(
            plan.calls()[0].destination_byte_offset,
            i64::from(index) * 12
        );
        assert_changed_yield_prefix(&plan);
    }
}

fn assert_changed_yield_prefix(plan: &SimulationLegendYieldPlan) {
    let prefix = SimulationLegendYieldCall {
        destination_byte_offset: 0,
        output_count_raw: -71,
        base_count_raw: -19,
        reversal_raw: 42,
        base: SimulationLegendYieldBase::OrdinaryLegendArea,
    };
    assert_eq!(plan.calls()[1], prefix);
    assert_eq!(
        plan.completion(),
        SimulationLegendYieldCompletion::CompletedMappingCallPlan
    );
    assert!(plan.coverage().post_suffix_state_read);
}

#[test]
fn unchanged_zero_index_still_emits_the_zero_count_prefix_request() {
    let mut runtime = yield_runtime(0, 7);
    runtime.after_suffix.as_mut().unwrap().split_index_raw = 0;
    let plan = yield_inputs()
        .plan_gray_calls_nearest_even(runtime)
        .unwrap();
    assert_eq!(plan.calls().len(), 2);
    assert_eq!(plan.calls()[1].output_count_raw, 0);
}

#[test]
fn large_indices_use_low32_triple_then_signed_byte_displacement() {
    for index in [715827882, 715827883, 1431655765, 1431655766, i32::MAX - 1] {
        let plan = yield_plan(index, i32::MAX);
        assert_yield_offset(&plan, index);
        assert_eq!(plan.calls()[0].output_count_raw, i32::MAX - index);
    }
    assert_eq!(
        yield_plan(715827883, i32::MAX).calls()[0].destination_byte_offset,
        -8589934588
    );
    assert_eq!(
        yield_plan(1431655766, i32::MAX).calls()[0].destination_byte_offset,
        8
    );
}

fn assert_yield_offset(plan: &SimulationLegendYieldPlan, index: i32) {
    let unsigned = (i128::from(index) * 3) % (1_i128 << 32);
    let signed = if unsigned >= 1_i128 << 31 {
        unsigned - (1_i128 << 32)
    } else {
        unsigned
    };
    assert_eq!(
        i128::from(plan.calls()[0].destination_byte_offset),
        signed * 4
    );
}

fn missing_yield_fact(
    inputs: SimulationLegendYieldInputs,
    runtime: SimulationLegendYieldRuntime,
    entity: &str,
) {
    let original = inputs;
    let error = inputs.plan_gray_calls_nearest_even(runtime).unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    assert_eq!(error.operation, "legend yield call plan");
    assert_eq!(error.entity, entity);
    assert!(error.cause.contains("None"));
    assert!(error.cause.contains("expected Some"));
    assert_eq!(inputs, original);
}

#[test]
fn missing_dependencies_follow_unconditional_setup_and_reached_call_order() {
    let mut inputs = yield_inputs();
    let mut runtime = yield_runtime(-1, 0);
    inputs.gray_base_color = None;
    runtime.split_index_raw = None;
    missing_yield_fact(inputs, runtime, "legend.color_d8");
    inputs = yield_inputs();
    missing_yield_fact(inputs, runtime, "runtime split index178");
    runtime = yield_runtime(0, 1);
    runtime.after_suffix = None;
    inputs.reverse_base_palette_raw = None;
    missing_yield_fact(inputs, runtime, "legend.word_c");
    missing_yield_fact(yield_inputs(), runtime, "post-suffix reread state");
    assert!(
        yield_inputs()
            .plan_gray_calls_nearest_even(yield_runtime(-1, 0))
            .is_ok()
    );
}

#[test]
fn immutable_plan_retains_all_parents_coverage_and_explicit_qualifications() {
    let mut inputs = yield_inputs();
    let mut runtime = yield_runtime(0, 10);
    let plan = inputs.plan_gray_calls_nearest_even(runtime).unwrap();
    let before = plan.clone();
    assert_eq!(plan.history().inputs, inputs);
    assert_eq!(plan.history().runtime, runtime);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    inputs.gray_base_color = None;
    runtime.after_suffix = None;
    assert_eq!(plan, before);
    assert_ne!(plan.history().inputs, inputs);
    assert_ne!(plan.history().runtime, runtime);
    assert_eq!(plan.coverage().achieved_pattern_error_bits, 0);
    assert!(plan.coverage().exact_local_integer_plan);
    assert!(!plan.coverage().mapping_helper_modeled);
    assert!(!plan.coverage().output_memory_validated);
    assert_eq!(plan.diagnostics().len(), 6);
}

#[test]
fn all_public_yield_types_are_send_sync_and_parallel_plans_equal_serial_results() {
    assert_yield_thread_traits();
    let serial: Vec<_> = (-4..=11).map(|index| yield_plan(index, 10)).collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    (-4..=11)
                        .map(|index| yield_plan(index, 10))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

fn assert_yield_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendYieldInputs>();
    require::<SimulationLegendYieldRuntime>();
    require::<SimulationLegendYieldPrefixState>();
    require::<SimulationLegendYieldBase>();
    require::<SimulationLegendYieldCall>();
    require::<SimulationLegendYieldCompletion>();
    require::<SimulationLegendYieldHistory>();
    require::<SimulationLegendYieldCoverage>();
    require::<SimulationLegendYieldDiagnostic>();
    require::<SimulationLegendYieldPlan>();
}

#[test]
fn independent_generated_call_expectations_match_on_eight_workers() {
    let serial = case_support::replay_yield_operations(OPERATIONS);
    assert_eq!(serial.len(), 13632 * 17);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_yield_operations(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
fn complete_owned_parents_and_arbitrary_rgba_reversal_mutants_match_source() {
    assert_eq!(
        projection_support::replay_yield_projection(OWNED, 635).len(),
        635 * 8 * 17
    );
}

#[test]
fn complete_owned_parent_plans_are_identical_across_eight_workers() {
    assert_eq!(
        projection_support::replay_yield_parallel(OWNED, 635).len(),
        635 * 8 * 17
    );
}

#[test]
#[ignore = "Requires fresh complete pinned native yield-color projections."]
fn complete_native_parents_retain_raw_colors_absence_and_explicit_runtime_facts() {
    let packet = std::fs::read("target/simulation-legend-yield-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_yield_parallel(&packet, 276).len(),
        276 * 8 * 17
    );
}

#[test]
#[ignore = "Requires native projections; supports independent process replay."]
fn export_legend_yield_for_fresh_process_replay() {
    let mut words = case_support::replay_yield_operations(OPERATIONS);
    words.extend(projection_support::replay_yield_projection(OWNED, 635));
    let packet = std::fs::read("target/simulation-legend-yield-projection-native.bin").unwrap();
    words.extend(projection_support::replay_yield_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_LEGEND_YIELD_OUTPUT").unwrap();
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    std::fs::write(destination, bytes).unwrap();
}
