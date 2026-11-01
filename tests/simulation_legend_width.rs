//! C854 explicit nearest-even numerical policy without ambient SSE state (#166).
use bessemer::simulation::*;
#[path = "support/legend_width_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_width_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-width-operations.bin");
const OWNED: &[u8] = include_bytes!("fixtures/simulation-post-dialog/legend-width-projection.bin");

fn width_runtime(font: i32, selector: i32) -> SimulationLegendWidthRuntime {
    SimulationLegendWidthRuntime {
        font_size_at_entry_raw: Some(font),
        selector_at_entry_raw: Some(selector),
    }
}

#[test]
fn one_font_width_keeps_each_independent_stage_and_native_branch() {
    let result = case_support::width_result_words(width_runtime(1, 0));
    assert_eq!(
        result,
        [2, 0x3f800000, 0x3a83126f, 0x3a23d70b, 0x3b23d70b, 2, 2, 3]
    );
    assert_eq!(
        case_support::width_result_words(width_runtime(1, 1))[4..6],
        [0x3aa3d70b, 1]
    );
    for selector in [i32::MIN, -1, 2, 42, i32::MAX] {
        assert_eq!(
            case_support::width_result_words(width_runtime(1, selector))[4..6],
            [0x3a23d70b, 0]
        );
    }
}

#[test]
fn conversion_midpoints_round_to_even_before_width_multiplication() {
    for (font, expected) in [
        (16777217, 0x4b800000),
        (16777219, 0x4b800002),
        (-16777217, 0xcb800000),
        (-16777219, 0xcb800002),
        (i32::MAX, 0x4f000000),
        (i32::MIN, 0xcf000000),
    ] {
        assert_eq!(
            case_support::width_result_words(width_runtime(font, 2))[1],
            expected
        );
    }
}

#[test]
fn zero_font_still_requires_b4_and_missing_font_is_first_failure() {
    let mut runtime = SimulationLegendWidthRuntime {
        font_size_at_entry_raw: None,
        selector_at_entry_raw: None,
    };
    assert_eq!(
        case_support::width_result_words(runtime),
        [10, 0, 0, 0, 0, 0, 0, 0]
    );
    runtime.font_size_at_entry_raw = Some(0);
    assert_eq!(
        case_support::width_result_words(runtime),
        [11, 0, 0, 0, 0, 0, 0, 0]
    );
    runtime.selector_at_entry_raw = Some(0);
    assert_eq!(
        case_support::width_result_words(runtime),
        [1, 0, 0, 0, 0, 2, 1, 3]
    );
}

#[test]
fn signed_font_results_are_symmetric_and_do_not_validate_physical_sizes() {
    for font in [1, 2, 3, 16777217, i32::MAX] {
        let positive = width_runtime(font, 0).project_width_nearest_even().unwrap();
        let negative = width_runtime(-font, 0)
            .project_width_nearest_even()
            .unwrap();
        assert_eq!(
            positive.values().selected_width_bits ^ negative.values().selected_width_bits,
            0x80000000
        );
        assert!(!negative.coverage().pointer_geometry_renderer_validated);
    }
}

#[test]
fn published_width_plan_isolated_from_success_failure_and_source_mutation() {
    let mut runtime = width_runtime(i32::MIN, 0);
    let plan = runtime.project_width_nearest_even().unwrap();
    let before = plan;
    runtime.selector_at_entry_raw = None;
    assert!(runtime.project_width_nearest_even().is_err());
    assert_eq!(runtime.selector_at_entry_raw, None);
    runtime.font_size_at_entry_raw = Some(0);
    assert_ne!(plan.history().runtime, runtime);
    assert_eq!(plan, before);
    assert_eq!(plan.history().runtime, width_runtime(i32::MIN, 0));
}

#[test]
fn all_public_width_types_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendWidthInputs>();
    require::<SimulationLegendWidthRuntime>();
    require::<SimulationLegendWidthOperation>();
    require::<SimulationLegendWidthValues>();
    require::<SimulationLegendWidthCompletion>();
    require::<SimulationLegendWidthHistory>();
    require::<SimulationLegendWidthCoverage>();
    require::<SimulationLegendWidthDiagnostic>();
    require::<SimulationLegendWidthPlan>();
}

#[test]
fn all_independent_rational_width_cases_match_on_eight_workers() {
    let serial = case_support::replay_width_operations(OPERATIONS);
    assert_eq!(serial.len(), 18040 * 8);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_width_operations(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
fn complete_owned_and_mutant_width_parents_keep_optional_raw_selectors() {
    assert_eq!(
        projection_support::replay_width_projection(OWNED, 560).len(),
        560 * 66
    );
}

#[test]
fn all_owned_width_projections_match_serial_results_on_eight_workers() {
    assert_eq!(
        projection_support::replay_width_parallel(OWNED, 560).len(),
        560 * 66
    );
}

#[test]
#[ignore = "Requires fresh complete pinned native width projections."]
fn complete_native_width_parents_keep_explicit_font_and_selector_contexts() {
    let packet = std::fs::read("target/simulation-legend-width-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_width_parallel(&packet, 276).len(),
        276 * 66
    );
}

#[test]
#[ignore = "Requires native projections; supports independent process replay."]
fn export_legend_width_for_fresh_process_replay() {
    let mut words = case_support::replay_width_operations(OPERATIONS);
    words.extend(projection_support::replay_width_projection(OWNED, 560));
    let packet = std::fs::read("target/simulation-legend-width-projection-native.bin").unwrap();
    words.extend(projection_support::replay_width_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_LEGEND_WIDTH_OUTPUT").unwrap();
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    std::fs::write(destination, bytes).unwrap();
}
