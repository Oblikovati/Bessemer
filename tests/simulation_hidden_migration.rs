//! C856 runtime pairs preserve original stored words and exact load scope (#166).
use bessemer::{FailureClass, simulation::*};
#[path = "support/hidden_migration_cases.rs"]
mod case_support;
#[path = "support/post_legend_limits.rs"]
mod legend_limits;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
#[path = "support/hidden_migration_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/hidden-migration-operations.bin");
const OWNED: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/hidden-migration-projection.bin");

fn hidden_inputs(version: i32, word: Option<i32>) -> SimulationHiddenPartsMigrationInputs {
    SimulationHiddenPartsMigrationInputs {
        version: SimulationPersistVersion::new(version).unwrap(),
        stored_word_1f0_raw: word,
    }
}

#[test]
fn original_one_two_three_use_a_single_qword_store_and_clear_adjacent_runtime_word() {
    for original in 1..=3 {
        let inputs = hidden_inputs(700006, Some(original));
        let plan = inputs.project_legacy_hidden_parts().unwrap();
        assert_eq!(plan.values().hidden_option_raw, 1);
        assert_eq!(plan.values().runtime_word_1f0_raw, 0);
        assert_eq!(
            plan.values().store,
            SimulationHiddenPartsStore::WriteAdjacentPairOne
        );
        assert_eq!(plan.coverage().runtime_store_bytes, 8);
        assert_eq!(plan.coverage().native_store_instructions, 1);
        assert_eq!(plan.history().inputs.stored_word_1f0_raw, Some(original));
    }
}

#[test]
fn zero_and_other_signed_patterns_retain_distinct_store_sequences() {
    assert_eq!(
        case_support::hidden_result_words(hidden_inputs(840007, Some(0))),
        [1, 0, 0, 0, 1, 4, 1, 1]
    );
    for original in [i32::MIN, -2, -1, 4, 5, i32::MAX] {
        assert_eq!(
            case_support::hidden_result_words(hidden_inputs(840007, Some(original))),
            [2, 1, 1, 2, 2, 8, 2, 2]
        );
    }
}

#[test]
fn version_scope_is_checked_before_missing_stored_word_without_later_fallback() {
    for version in [0, 700005, 840008, i32::MAX] {
        let error = hidden_inputs(version, None)
            .project_legacy_hidden_parts()
            .unwrap_err();
        assert_eq!(error.class, FailureClass::Unsupported);
        assert!(error.cause.contains(&version.to_string()));
        assert_eq!(
            case_support::hidden_result_words(hidden_inputs(version, Some(2))),
            [10, 0, 0, 0, 0, 0, 0, 0]
        );
    }
    assert_eq!(
        case_support::hidden_result_words(hidden_inputs(700006, None)),
        [11, 0, 0, 0, 0, 0, 0, 0]
    );
}

#[test]
fn published_migration_keeps_original_inputs_after_success_failure_and_mutation() {
    let mut inputs = hidden_inputs(840007, Some(i32::MIN));
    let plan = inputs.project_legacy_hidden_parts().unwrap();
    let before = plan;
    inputs.stored_word_1f0_raw = None;
    assert!(inputs.project_legacy_hidden_parts().is_err());
    assert_eq!(inputs.stored_word_1f0_raw, None);
    inputs.version = SimulationPersistVersion::new(840008).unwrap();
    assert_ne!(inputs, plan.history().inputs);
    assert_eq!(plan, before);
    assert_eq!(plan.history().inputs, hidden_inputs(840007, Some(i32::MIN)));
}

#[test]
fn all_public_hidden_migration_types_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<SimulationHiddenPartsMigrationInputs>();
    require::<SimulationHiddenPartsStore>();
    require::<SimulationHiddenPartsMigrationValues>();
    require::<SimulationHiddenPartsMigrationCompletion>();
    require::<SimulationHiddenPartsMigrationHistory>();
    require::<SimulationHiddenPartsMigrationCoverage>();
    require::<SimulationHiddenPartsMigrationDiagnostic>();
    require::<SimulationHiddenPartsMigrationPlan>();
}

#[test]
fn all_independent_hidden_migration_cases_match_on_eight_workers() {
    let serial = case_support::replay_hidden_operations(OPERATIONS);
    assert_eq!(serial.len(), 4336 * 8);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_hidden_operations(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

#[test]
fn complete_owned_and_mutant_hidden_parents_preserve_original_words_and_wire_bytes() {
    assert_eq!(
        projection_support::replay_hidden_projection(OWNED, 565).len(),
        565 * 66
    );
}

#[test]
fn all_owned_hidden_projections_match_serial_results_on_eight_workers() {
    assert_eq!(
        projection_support::replay_hidden_parallel(OWNED, 565).len(),
        565 * 66
    );
}

#[test]
#[ignore = "Requires fresh complete pinned native hidden migration projections."]
fn complete_native_hidden_parents_keep_migration_and_later_contexts_distinct() {
    let packet = std::fs::read("target/simulation-hidden-migration-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_hidden_parallel(&packet, 276).len(),
        276 * 66
    );
}

#[test]
#[ignore = "Requires native projections; supports independent process replay."]
fn export_hidden_migration_for_fresh_process_replay() {
    let mut words = case_support::replay_hidden_operations(OPERATIONS);
    words.extend(projection_support::replay_hidden_projection(OWNED, 565));
    let packet = std::fs::read("target/simulation-hidden-migration-projection-native.bin").unwrap();
    words.extend(projection_support::replay_hidden_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_HIDDEN_MIGRATION_OUTPUT").unwrap();
    let bytes: Vec<_> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    std::fs::write(destination, bytes).unwrap();
}
