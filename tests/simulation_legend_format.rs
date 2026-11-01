//! C846 bounded format construction and explicit alternate-call dependencies (#166).
use bessemer::simulation::*;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::read_word;
#[path = "support/legend_format_cases.rs"]
mod case_support;
#[path = "support/legend_format_projections.rs"]
mod projection_support;
const OPERATIONS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-format-operations.bin");
const PARENTS: &[u8] =
    include_bytes!("fixtures/simulation-post-dialog/legend-format-projection.bin");

#[test]
fn all_word_presence_wrap_guards_and_binary32_classes_match_independent_expected_results() {
    let mut cursor = OPERATIONS;
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor) as usize;
    assert_eq!(
        case_support::replay_legend_cases(OPERATIONS).len(),
        count * 52
    );
}

#[test]
fn eight_workers_match_serial_and_all_public_types_are_send_sync() {
    assert_legend_thread_traits();
    let serial = case_support::replay_legend_cases(OPERATIONS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| case_support::replay_legend_cases(OPERATIONS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}

fn assert_legend_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationLegendFormatInputs>();
    require::<SimulationLegendFormatRuntime>();
    require::<SimulationLegendFormatSelection>();
    require::<SimulationLegendFormatCheckMode>();
    require::<SimulationLegendFormatCoverage>();
    require::<SimulationLegendFormatHistory>();
    require::<SimulationLegendFormatDiagnostic>();
    require::<SimulationLegendFormatPlan>();
}

fn find_legend_case(status: u32) -> case_support::LegendFixtureCase {
    let mut cursor = OPERATIONS;
    assert_eq!(read_word(&mut cursor), 1);
    for _ in 0..read_word(&mut cursor) {
        let case = case_support::read_legend_case(&mut cursor);
        let expected = load_projection::take_projection(&mut cursor, 52);
        if u32::from_le_bytes(expected[..4].try_into().unwrap()) == status {
            return case;
        }
    }
    panic!("fixture lacks legend format status{status}")
}

#[test]
fn every_completed_branch_keeps_all_parents_after_source_mutation() {
    for status in 1..=6 {
        let mut case = find_legend_case(status);
        let plan = case_support::execute_legend_case(&case).unwrap();
        let original = plan.clone();
        case.inputs.mode_raw = Some(42);
        case.inputs.precision_raw = None;
        case.runtime.output_pointer_present = None;
        case.runtime.value_bits = Some(42);
        case.words.fill(42);
        assert_eq!(plan, original);
        assert_ne!(plan.history().inputs, case.inputs);
    }
}

#[test]
fn all_owned_contexts_keep_presence_raw_words_and_complete_archives() {
    assert_eq!(
        projection_support::replay_legend_projection(PARENTS, 425).len(),
        425 * 7 * 52
    );
}

#[test]
fn owned_archives_project_identically_on_eight_workers() {
    assert_eq!(
        projection_support::replay_legend_projection_parallel(PARENTS, 425).len(),
        425 * 7 * 52
    );
}

#[test]
#[ignore = "Requires fresh pinned complete native legend-format projections."]
fn all_native_contexts_keep_raw_controls_absence_and_complete_archives() {
    let packet = std::fs::read("target/simulation-legend-format-projection-native.bin").unwrap();
    assert_eq!(
        projection_support::replay_legend_projection_parallel(&packet, 276).len(),
        276 * 7 * 52
    );
}

#[test]
#[ignore = "Requires native projections; supports independent fresh-process replay."]
fn export_legend_format_for_fresh_process_replay() {
    let mut output = case_support::replay_legend_cases(OPERATIONS);
    output.extend(projection_support::replay_legend_projection(PARENTS, 425));
    let packet = std::fs::read("target/simulation-legend-format-projection-native.bin").unwrap();
    output.extend(projection_support::replay_legend_projection(&packet, 276));
    let destination = std::env::var("BESSEMER_LEGEND_FORMAT_OUTPUT").unwrap();
    std::fs::write(destination, output).unwrap();
}
