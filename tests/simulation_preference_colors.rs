//! C907 typed preference routes with explicit context, exact RGB and raw alpha provenance (#166).
use bessemer::simulation::*;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::read_word;
#[path = "support/preference_color_cases.rs"]
mod case_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/preference-color-operations.bin");

#[test]
fn every_context_route_bound_rounding_and_raw_alpha_matches_the_independent_source_model() {
    assert_eq!(
        case_support::replay_preference_cases(OPERATIONS).len(),
        17400 * 18
    );
}
#[test]
fn eight_workers_match_serial_results_and_all_public_types_are_send_sync() {
    assert_preference_thread_traits();
    let serial = case_support::replay_preference_cases(OPERATIONS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_preference_cases(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}
fn assert_preference_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationStylePreferenceColorRoute>();
    require::<SimulationStylePreferenceColorRuntime>();
    require::<SimulationStylePreferenceAlphaOrigin>();
    require::<SimulationStylePreferenceColorHistory>();
    require::<SimulationStylePreferenceColorCompletion>();
    require::<SimulationStylePreferenceColorCoverage>();
    require::<SimulationStylePreferenceColorDiagnostic>();
    require::<SimulationStylePreferenceColorPlan>();
}
fn find_preference_case(status: u32) -> case_support::PreferenceFixtureCase {
    let mut cursor = OPERATIONS;
    assert_eq!(read_word(&mut cursor), 1);
    for _ in 0..read_word(&mut cursor) {
        let case = case_support::read_preference_case(&mut cursor);
        let expected: [u32; 18] = std::array::from_fn(|_| read_word(&mut cursor));
        if expected[0] == status {
            return case;
        }
    }
    panic!("fixture lacks preference status{status}")
}
#[test]
fn both_completed_stages_keep_original_inputs_and_consumed_words_after_source_mutation() {
    for status in [0, 1] {
        let mut case = find_preference_case(status);
        let route = SimulationStylePreferenceColorRoute::from_style_member(case.words[0]).unwrap();
        let plan = case
            .original
            .project_style_preference_nearest_even(route, case.source.as_deref(), case.runtime)
            .unwrap();
        let original = plan.clone();
        if let Some(source) = &mut case.source {
            source.fill(0xff);
        }
        case.original = SimulationRgbaColor::from_bits([42; 4]);
        case.runtime.object_pointer_present = None;
        assert_eq!(plan, original);
        assert_ne!(plan.original(), case.original);
        assert_ne!(plan.runtime_context(), case.runtime);
    }
}
#[test]
fn null_pointer_ignores_cached_type73_but_nonnull_unknown_type_remains_missing() {
    let null = SimulationStylePreferenceColorRuntime {
        object_pointer_present: Some(false),
        pointed_type_raw: Some(73),
    };
    assert!(null.local_block_applies().unwrap());
    let unknown = SimulationStylePreferenceColorRuntime {
        object_pointer_present: Some(true),
        pointed_type_raw: None,
    };
    let error = unknown.local_block_applies().unwrap_err();
    assert_eq!(error.entity, "pointed type10");
    assert_eq!(error.class, bessemer::FailureClass::Missing);
    let unknown_pointer = SimulationStylePreferenceColorRuntime {
        object_pointer_present: None,
        pointed_type_raw: Some(73),
    };
    assert_eq!(
        unknown_pointer.local_block_applies().unwrap_err().entity,
        "style2D0 pointer"
    );
}
#[test]
fn reached_projection_that_matches_original_bits_is_still_a_completed_projection() {
    let route = SimulationStylePreferenceColorRoute::Style1bc;
    let color = SimulationRgbaColor::from_bits([0, 0, 0, 0x7fa00042]);
    let source = vec![0; route.required_preference_bytes()];
    let context = SimulationStylePreferenceColorRuntime {
        object_pointer_present: Some(false),
        pointed_type_raw: None,
    };
    let plan = color
        .project_style_preference_nearest_even(route, Some(&source), context)
        .unwrap();
    assert_eq!(plan.projected(), color);
    assert_eq!(
        plan.completion(),
        SimulationStylePreferenceColorCompletion::CompletedProjection
    );
    assert_eq!(plan.history().packed_preference_word, Some(0));
    assert_eq!(plan.coverage().rgb_words_projected, 3);
}

#[path = "support/preference_color_projections.rs"]
mod projection_support;
const VIEWS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/preference-color-projection.bin");
#[test]
fn every_owned_context_preserves_conditional_route_absence_original_rgba_and_wire_bytes() {
    assert_eq!(
        projection_support::replay_preference_projections(VIEWS, 425).len(),
        425 * 8 * 6 + 2457 * 4 * 18
    );
}
#[test]
#[ignore = "Requires complete freshly pinned native preference-color projections."]
fn every_native_context_retains_unobserved_applicability_and_original_route_colors() {
    assert_eq!(
        projection_support::replay_preference_projections(
            &std::fs::read("target/simulation-preference-color-projection-native.bin").unwrap(),
            276
        )
        .len(),
        276 * 8 * 6 + 1698 * 4 * 18
    );
}
#[test]
#[ignore = "Writes deterministic output for independent fresh-process comparisons."]
fn export_preference_colors_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-preference-color-projection-native.bin").unwrap();
    let output = [
        case_support::replay_preference_cases(OPERATIONS),
        projection_support::replay_preference_projections(VIEWS, 425),
        projection_support::replay_preference_projections(&native, 276),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_PREFERENCE_COLORS_OUTPUT") {
        std::fs::write(
            path,
            output
                .into_iter()
                .flat_map(u32::to_le_bytes)
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }
}
