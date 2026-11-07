//! Complete C863 composition, alias consistency and independent native-order outcomes (#166).
use bessemer::{FailureClass, simulation::*};
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/style_comparison_projections.rs"]
mod projection_support;
#[path = "support/style_comparison_results.rs"]
mod result_support;
#[path = "support/style_comparison_state.rs"]
mod state_support;
use SimulationStyleComparisonColor as Color;
use SimulationStyleComparisonScalar as Scalar;
use SimulationStyleComparisonWord as Word;
const OWNED: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/style-comparison-operations.bin");
const PROJECTIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/style-comparison-projection.bin");

fn complete_style_fixture_state() -> SimulationStyleComparisonState {
    let mut cursor = &OWNED[8..];
    state_support::read_style_state(&mut cursor)
}

#[test]
fn scalar_aliases_use_exactly_the_same_alpha_storage_as_their_colors() {
    let mut state = complete_style_fixture_state();
    for (scalar, color) in [
        (Scalar::Alpha22C, Color::Color220),
        (Scalar::Alpha2B8, Color::Color2AC),
    ] {
        state.set_color(color, Some(SimulationRgbaColor::from_bits([1, 2, 3, 4])));
        assert_eq!(state.scalar_bits(scalar), Some(4));
        state.set_scalar_bits(scalar, Some(0x80000000));
        assert_eq!(
            state.component_bits(color, SimulationColorChannel::Alpha),
            Some(0x80000000)
        );
        assert_eq!(
            state.component_bits(color, SimulationColorChannel::Red),
            Some(1)
        );
        state.set_color(color, None);
        assert_eq!(state.scalar_bits(scalar), None);
    }
}

#[test]
fn alpha_only_tessellation_facts_complete_when_the_rgb_branch_is_disabled() {
    let mut state = complete_style_fixture_state();
    state.set_word_bits(Word::Word21C, Some(1));
    state.set_word_bits(Word::Word230, Some(1));
    state.set_color(Color::Color220, None);
    state.set_scalar_bits(Scalar::Alpha22C, Some(0));
    let plan = state.compare_masked_exceptions_daz_off(&state).unwrap();
    assert_eq!(
        plan.outcome(),
        SimulationStyleComparisonOutcome::CompletedNoDifference
    );
    assert!(
        plan.checks()
            .contains(&SimulationStyleComparisonCheck::Scalar(Scalar::Alpha22C))
    );
    assert!(
        !plan
            .checks()
            .contains(&SimulationStyleComparisonCheck::Color(Color::Color220))
    );
}

#[test]
fn a_red_difference_completes_before_unknown_later_color_channels() {
    let mut left = complete_style_fixture_state();
    left.set_word_bits(Word::Word1B8, Some(1));
    left.set_component_bits(Color::Color1BC, SimulationColorChannel::Green, None);
    let mut right = left.clone();
    right.set_component_bits(
        Color::Color1BC,
        SimulationColorChannel::Red,
        Some(0x3f800000),
    );
    let plan = left.compare_masked_exceptions_daz_off(&right).unwrap();
    assert_mapping_color_difference(&plan);
}

fn assert_mapping_color_difference(plan: &SimulationStyleComparisonPlan) {
    assert_eq!(
        plan.outcome(),
        SimulationStyleComparisonOutcome::CompletedDifferent(
            SimulationStyleComparisonCheck::Color(Color::Color1BC)
        )
    );
    assert_eq!(
        plan.checks(),
        &[
            SimulationStyleComparisonCheck::Word(Word::Word1B8),
            SimulationStyleComparisonCheck::Color(Color::Color1BC)
        ]
    );
}

#[test]
fn the_bound_color_operator_reads_the_right_channel_before_the_left_channel() {
    let mut state = complete_style_fixture_state();
    state.set_word_bits(Word::Word1B8, Some(1));
    state.set_component_bits(Color::Color1BC, SimulationColorChannel::Red, None);
    let error = state.compare_masked_exceptions_daz_off(&state).unwrap_err();
    assert_eq!(error.class, FailureClass::Missing);
    assert_eq!(error.entity, "right.style.color_1bc.component_0");
}

#[test]
fn ignored_rgb_payloads_remain_different_in_history_without_forcing_a_native_difference() {
    let left = complete_style_fixture_state();
    let mut right = left.clone();
    right.set_component_bits(
        Color::Color2AC,
        SimulationColorChannel::Red,
        Some(0x7fc12345),
    );
    let plan = left.compare_masked_exceptions_daz_off(&right).unwrap();
    assert_ne!(left, right);
    assert_eq!(
        plan.outcome(),
        SimulationStyleComparisonOutcome::CompletedNoDifference
    );
    assert_eq!(&plan.history().left, &left);
    assert_eq!(&plan.history().right, &right);
    assert_eq!(left.legend_state(), right.legend_state());
}

#[test]
fn unordered_alpha_and_signed_zero_follow_the_explicit_sse_policy() {
    let mut left = complete_style_fixture_state();
    left.set_word_bits(Word::Word21C, Some(1));
    left.set_word_bits(Word::Word230, Some(1));
    left.set_scalar_bits(Scalar::Alpha22C, Some(0x7f800001));
    let mut right = left.clone();
    right.set_scalar_bits(Scalar::Alpha22C, Some(0x3f800000));
    right.set_scalar_bits(Scalar::Alpha2B8, Some(0x80000000));
    assert_eq!(
        left.compare_masked_exceptions_daz_off(&right)
            .unwrap()
            .outcome(),
        SimulationStyleComparisonOutcome::CompletedNoDifference
    );
}

#[test]
fn successful_and_failed_comparisons_isolate_inputs_and_published_history() {
    let mut state = complete_style_fixture_state();
    let original = state.clone();
    let plan = state.compare_masked_exceptions_daz_off(&state).unwrap();
    state.set_word_bits(Word::Word1B8, None);
    let before_failure = state.clone();
    assert!(state.compare_masked_exceptions_daz_off(&original).is_err());
    assert_eq!(state, before_failure);
    assert_eq!(plan.history().left, original);
    assert_eq!(plan.history().right, original);
}

#[test]
fn all_public_style_comparison_types_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<SimulationStyleComparisonWord>();
    require::<SimulationStyleComparisonColor>();
    require::<SimulationStyleComparisonScalar>();
    require::<SimulationStyleComparisonCheck>();
    require::<SimulationStyleComparisonState>();
    require::<SimulationStyleComparisonOutcome>();
    require::<SimulationStyleComparisonCheckMode>();
    require::<SimulationStyleComparisonHistory>();
    require::<SimulationStyleComparisonCoverage>();
    require::<SimulationStyleComparisonDiagnostic>();
    require::<SimulationStyleComparisonPlan>();
}

#[test]
fn all_owned_native_program_outcomes_and_complete_parent_views_match() {
    assert_eq!(
        result_support::replay_style_cases(OWNED, 17125).len(),
        428125
    );
    assert_eq!(
        projection_support::replay_style_projections(PROJECTIONS, 425).len(),
        10625
    );
}

#[test]
fn eight_workers_reproduce_serial_comparison_outcomes_and_views() {
    let serial = result_support::replay_style_cases(OWNED, 17125);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    assert_eq!(
                        projection_support::replay_style_projections(PROJECTIONS, 425).len(),
                        10625
                    );
                    result_support::replay_style_cases(OWNED, 17125)
                })
            })
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
#[ignore = "Requires freshly frozen native comparison packets."]
fn native_style_comparisons_and_process_export() {
    let native = std::fs::read("target/simulation-style-comparison-operations-native.bin").unwrap();
    let parents =
        std::fs::read("target/simulation-style-comparison-projection-native.bin").unwrap();
    let mut words = result_support::replay_style_cases(OWNED, 17125);
    words.extend(result_support::replay_style_cases(&native, 8508));
    words.extend(projection_support::replay_style_projections(
        PROJECTIONS,
        425,
    ));
    words.extend(projection_support::replay_style_projections(&parents, 276));
    assert_eq!(words.len(), 658350);
    export_style_comparison_words(words);
}

fn export_style_comparison_words(words: Vec<u32>) {
    if let Some(path) = std::env::var_os("BESSEMER_STYLE_COMPARISON_OUTPUT") {
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
