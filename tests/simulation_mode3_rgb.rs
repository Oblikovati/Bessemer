//! Complete C893 local group selection and fixed array-write regression matrix (#166).
use bessemer::simulation::*;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::read_word;
#[path = "support/mode3_rgb_cases.rs"]
mod case_support;
#[path = "support/mode3_rgb_results.rs"]
mod result_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/mode3-rgb-operations.bin");

#[test]
fn complete_group_and_array_matrix_matches_independent_expected_words() {
    assert_eq!(
        result_support::replay_mode3_cases(OPERATIONS).len(),
        15342 * 54
    );
}
#[test]
fn eight_workers_match_serial_raw_words_and_all_public_types_are_send_sync() {
    assert_mode3_thread_traits();
    let serial = result_support::replay_mode3_cases(OPERATIONS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| result_support::replay_mode3_cases(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}
fn assert_mode3_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationMode3RgbInputs>();
    require::<SimulationMode3RgbPass>();
    require::<SimulationMode3RgbGroup>();
    require::<SimulationMode3RgbOrigin>();
    require::<SimulationMode3RgbStage>();
    require::<SimulationMode3RgbCompletion>();
    require::<SimulationMode3RgbCoverage>();
    require::<SimulationMode3RgbDiagnostic>();
    require::<SimulationMode3RgbArrayContext>();
    require::<SimulationMode3RgbArrayParent>();
    require::<SimulationMode3RgbWriteHistory>();
    require::<SimulationMode3RgbTriples>();
    require::<SimulationMode3RgbPlan>();
    require::<SimulationMode3RgbUpdate>();
}
#[test]
fn constructor_budget_and_borrowed_buffer_mutation_preserve_snapshot_isolation() {
    let mut source = [[1, 2, 3]; 8];
    let snapshot = SimulationMode3RgbTriples::from_slice(&source, 8).unwrap();
    source[5] = [42; 3];
    assert_eq!(snapshot.triples(), &[[1, 2, 3]; 8]);
    let error = SimulationMode3RgbTriples::from_slice(&source, 7).unwrap_err();
    assert_eq!(error.entity, "triple budget");
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert!(
        error.cause.contains('8') && error.cause.contains('7') && error.cause.contains("expected")
    );
    assert_eq!(snapshot.maximum_triples(), 8);
    assert_eq!(snapshot.triples(), &[[1, 2, 3]; 8]);
}
fn find_mode3_case(stage: u32, array_status: u32) -> case_support::Mode3FixtureCase {
    let mut cursor = OPERATIONS;
    assert_eq!(read_word(&mut cursor), 1);
    for _ in 0..read_word(&mut cursor) {
        let case = case_support::read_mode3_case(&mut cursor);
        let expected: [u32; 54] = std::array::from_fn(|_| read_word(&mut cursor));
        if expected[0] == stage && expected[22] == array_status {
            return case;
        }
    }
    panic!("missing fixture stage{stage}, array{array_status}")
}
#[test]
fn all_completed_stages_and_both_array_parents_are_isolated_from_later_mutations() {
    for (stage, array) in [(0, 0), (1, 0), (2, 0), (3, 1), (3, 2)] {
        let mut case = find_mode3_case(stage, array);
        let plan = case.inputs.plan_group_rgb(case.group).unwrap();
        let update = plan.apply_local_array(&case.array).unwrap();
        let original = update.clone();
        case.inputs.color_1dc = None;
        case.group.face_raw = Some(-12345);
        case.array.existing = None;
        case.array.after_growth = None;
        assert_eq!(update, original);
        assert_ne!(update.source_plan().source_inputs(), case.inputs);
        assert_ne!(update.source_plan().source_group(), case.group);
        if array > 0 {
            assert_ne!(update.source_context(), &case.array);
        }
    }
}

#[path = "support/mode3_rgb_projections.rs"]
mod projection_support;
const VIEWS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/mode3-rgb-projection.bin");
#[test]
fn all_owned_contexts_preserve_optional_mode_rgba_parent_and_complete_archive_bytes() {
    assert_eq!(
        projection_support::replay_mode3_projections(VIEWS, 425).len(),
        425 * 225
    );
}
#[test]
#[ignore = "Requires complete freshly pinned native C893 projections."]
fn every_native_context_preserves_original_file_facts_and_unobserved_runtime() {
    projection_support::replay_mode3_projections(
        &std::fs::read("target/simulation-mode3-rgb-projection-native.bin").unwrap(),
        276,
    );
}
#[test]
#[ignore = "Writes deterministic output for independent fresh-process comparison."]
fn export_mode3_rgb_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-mode3-rgb-projection-native.bin").unwrap();
    let output = [
        result_support::replay_mode3_cases(OPERATIONS),
        projection_support::replay_mode3_projections(VIEWS, 425),
        projection_support::replay_mode3_projections(&native, 276),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_MODE3_RGB_OUTPUT") {
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

#[test]
fn zero_budget_admits_only_known_empty_storage_without_a_default_unlimited_fallback() {
    let empty = SimulationMode3RgbTriples::from_slice(&[], 0).unwrap();
    assert!(empty.triples().is_empty());
    assert_eq!(empty.maximum_triples(), 0);
    let error = SimulationMode3RgbTriples::from_slice(&[[0; 3]], 0).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert_eq!(error.entity, "triple budget");
    assert!(error.cause.contains("1 supplied triples") && error.cause.contains("at most 0"));
    assert!(empty.triples().is_empty());
}
