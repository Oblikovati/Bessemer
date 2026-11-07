//! C860 typed local zero tests preserve arbitrary source words and reached-read order (#166).
use bessemer::{FailureClass, simulation::*};
#[path = "support/legend_inverse_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_inverse_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-inverse-operations.bin");
const OWNED: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-inverse-projection.bin");

fn inverse_runtime(
    plot_context: Option<SimulationLegendPreferencePlot>,
    word: Option<i32>,
) -> SimulationLegendPreferenceRuntime {
    SimulationLegendPreferenceRuntime {
        plot_context,
        preference_word_1528_raw: word,
    }
}

#[test]
fn zero_test_is_not_bitwise_complement_or_a_lossless_inverse() {
    for word in [i32::MIN, -1, 0, 1, 2, i32::MAX] {
        let inputs = SimulationLegendPreferenceInputs {
            stored_a8_raw: Some(word),
        };
        let plan = inputs.project_preference_word().unwrap();
        assert_eq!(
            plan.effect(),
            SimulationLegendPreferenceEffect::PreferenceWord0(i32::from(word == 0))
        );
        assert_eq!(
            plan.history().source,
            SimulationLegendPreferenceSource::StoredA8(inputs)
        );
    }
}

#[test]
fn applying_the_exported_preference_is_lossy_for_nonbinary_original_words() {
    let first = SimulationLegendPreferenceInputs {
        stored_a8_raw: Some(2),
    }
    .project_preference_word()
    .unwrap();
    let SimulationLegendPreferenceEffect::PreferenceWord0(word) = first.effect() else {
        panic!("expected exported word")
    };
    let second = inverse_runtime(Some(SimulationLegendPreferencePlot::NullPlot), Some(word))
        .project_local_a8_assignment()
        .unwrap();
    assert_eq!(
        second.effect(),
        SimulationLegendPreferenceEffect::LegendWordA8(1)
    );
}

#[test]
fn only_exact_plot_types_skip_and_skip_does_not_require_preference1528() {
    for plot in [22, 57, 73] {
        let runtime = inverse_runtime(
            Some(SimulationLegendPreferencePlot::PresentType(plot)),
            None,
        );
        let plan = runtime.project_local_a8_assignment().unwrap();
        assert_eq!(
            plan.effect(),
            SimulationLegendPreferenceEffect::NoAssignment
        );
        assert_eq!(plan.route(), SimulationLegendPreferenceRoute::SkipPlotType);
        assert_eq!(
            plan.completion(),
            SimulationLegendPreferenceCompletion::CompletedNoAssignment
        );
        assert!(!plan.coverage().source_word_read);
        assert_eq!(plan.coverage().runtime_words_assigned, 0);
    }
}

const NONSKIP_PLOT_TYPES: [i32; 15] = [
    i32::MIN,
    -42,
    -7,
    -1,
    9,
    21,
    23,
    56,
    58,
    72,
    74,
    86,
    121,
    137,
    i32::MAX,
];

#[test]
fn nearby_and_wrapped_plot_codes_do_not_alias_the_bounded_native_mask() {
    for plot in NONSKIP_PLOT_TYPES {
        assert_preference_word_required(plot);
    }
}

fn assert_preference_word_required(plot: i32) {
    let error = inverse_runtime(
        Some(SimulationLegendPreferencePlot::PresentType(plot)),
        None,
    )
    .project_local_a8_assignment()
    .unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    assert_eq!(error.entity, "actual preference1528");
}

#[test]
fn unknown_plot_precedes_missing_preference_and_known_null_is_distinct() {
    let error = inverse_runtime(None, None)
        .project_local_a8_assignment()
        .unwrap_err();
    assert_eq!(error.entity, "actual plot context");
    let null = inverse_runtime(Some(SimulationLegendPreferencePlot::NullPlot), Some(-1))
        .project_local_a8_assignment()
        .unwrap();
    assert_eq!(null.route(), SimulationLegendPreferenceRoute::ApplyOrdinary);
    assert_eq!(
        null.effect(),
        SimulationLegendPreferenceEffect::LegendWordA8(0)
    );
    assert!(!null.coverage().plot_type_word_read);
}

#[test]
fn type70_keeps_its_distinct_local_assignment_branch() {
    let special = inverse_runtime(
        Some(SimulationLegendPreferencePlot::PresentType(70)),
        Some(0),
    )
    .project_local_a8_assignment()
    .unwrap();
    assert_eq!(
        special.route(),
        SimulationLegendPreferenceRoute::ApplyType70
    );
    assert_eq!(
        special.effect(),
        SimulationLegendPreferenceEffect::LegendWordA8(1)
    );
}

#[test]
fn original_unused_words_and_published_results_survive_input_mutation() {
    let mut runtime = inverse_runtime(
        Some(SimulationLegendPreferencePlot::PresentType(22)),
        Some(i32::MIN),
    );
    let original = runtime;
    let plan = runtime.project_local_a8_assignment().unwrap();
    runtime.plot_context = None;
    runtime.preference_word_1528_raw = None;
    assert!(runtime.project_local_a8_assignment().is_err());
    assert_eq!(
        plan.history().source,
        SimulationLegendPreferenceSource::RuntimePreference(original)
    );
    assert_eq!(plan, original.project_local_a8_assignment().unwrap());
}

#[test]
fn all_public_inverse_types_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendPreferenceInputs>();
    require::<SimulationLegendPreferencePlot>();
    require::<SimulationLegendPreferenceRuntime>();
    require::<SimulationLegendPreferenceEffect>();
    require::<SimulationLegendPreferenceRoute>();
    require::<SimulationLegendPreferenceSource>();
    require::<SimulationLegendPreferenceHistory>();
    require::<SimulationLegendPreferenceCompletion>();
    require::<SimulationLegendPreferenceCoverage>();
    require::<SimulationLegendPreferenceDiagnostic>();
    require::<SimulationLegendPreferencePlan>();
}

#[test]
fn all_frozen_cases_and_complete_owned_parents_match_independent_expectations() {
    assert_eq!(case_support::replay_inverse_cases(OPERATIONS).len(), 190170);
    assert_eq!(
        projection_support::replay_inverse_projection(OWNED, 565).len(),
        8475
    );
}

#[test]
fn eight_workers_reproduce_serial_inverse_cases_and_complete_parent_views() {
    let serial = projection_support::replay_inverse_projection(OWNED, 565);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    assert_eq!(case_support::replay_inverse_cases(OPERATIONS).len(), 190170);
                    projection_support::replay_inverse_projection(OWNED, 565)
                })
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
#[ignore = "Requires freshly frozen native reference packets."]
fn native_inverse_contexts_and_process_export() {
    let native = std::fs::read("target/simulation-legend-inverse-projection-native.bin").unwrap();
    let mut words = case_support::replay_inverse_cases(OPERATIONS);
    words.extend(projection_support::replay_inverse_projection(OWNED, 565));
    words.extend(projection_support::replay_inverse_projection(&native, 276));
    assert_eq!(words.len(), 202785);
    if let Some(path) = std::env::var_os("BESSEMER_LEGEND_INVERSE_OUTPUT") {
        std::fs::write(
            path,
            words
                .into_iter()
                .flat_map(u32::to_le_bytes)
                .collect::<Vec<_>>(),
        )
        .unwrap();
    }
}
