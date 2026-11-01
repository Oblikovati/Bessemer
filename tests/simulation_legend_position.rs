//! Complete C847 raw factor views and source provenance, without pixel emulation (#166).
use bessemer::simulation::*;
#[path = "support/legend_position_edits.rs"]
mod edit_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/legend_position_projections.rs"]
mod projection_support;

#[test]
fn exported_position_ids_drive_isolated_typed_edits_with_literal_byte_expectations() {
    edit_support::assert_position_id_edits(OWNED);
}
const OWNED: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-position-projection.bin");

#[test]
fn all_owned_profiles_and_every_signed_exponent_class_match_independent_complete_views() {
    assert_eq!(
        projection_support::replay_position_projection(OWNED, 2480).len(),
        2480 * 56
    );
}

#[test]
fn eight_workers_match_serial_owned_results_and_all_public_types_are_send_sync() {
    assert_position_thread_traits();
    assert_eq!(
        projection_support::replay_position_parallel(OWNED, 2480).len(),
        2480 * 56
    );
}

fn assert_position_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendPositionAxis>();
    require::<SimulationLegendPositionFactor>();
    require::<SimulationLegendPositionFactors>();
    require::<SimulationLegendPositionCompletion>();
    require::<SimulationLegendPositionCoverage>();
    require::<SimulationLegendPositionDiagnostic>();
}

#[test]
#[ignore = "Requires fresh pinned complete native position-factor projections."]
fn all_native_contexts_preserve_raw_words_source_spans_history_and_parallel_results() {
    let packet = std::fs::read("target/simulation-legend-position-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_position_parallel(&packet, 276).len(),
        276 * 56
    );
}

#[test]
#[ignore = "Requires native projections; supports independent process replay."]
fn export_legend_position_for_fresh_process_replay() {
    let mut output = projection_support::replay_position_projection(OWNED, 2480);
    let packet = std::fs::read("target/simulation-legend-position-projection-native.bin").unwrap();
    output.extend(projection_support::replay_position_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_LEGEND_POSITION_OUTPUT").unwrap();
    std::fs::write(destination, output).unwrap();
}
