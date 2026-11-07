//! C892 local RGB-read guards and explicit endpoint/graphics exclusions (#166).
use bessemer::simulation::*;
#[allow(dead_code, reason = "Shared complete archive projection primitives.")]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};
#[path = "support/segment_rgb_plans.rs"]
mod plan_support;
const PLANS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/segment-rgb-plans.bin");
const VIEWS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/segment-rgb-projection.bin");
const PARENTS: SimulationPostDialogLimits = SimulationPostDialogLimits {
    maximum_bytes: 10000000,
    maximum_fields: 200000,
    maximum_entries: 10000000,
    maximum_text_bytes: 0x200000,
    embedded: bessemer::model_archives::ModelArchiveLimits {
        maximum_bytes: 1000000,
        maximum_fields: 100000,
        maximum_entries: 1000000,
        maximum_depth: 128,
        maximum_objects: 100000,
    },
};
const STYLES: SimulationPostStyleLimits = SimulationPostStyleLimits {
    maximum_bytes: 4096,
    maximum_fields: 116,
    maximum_entries: 512,
    maximum_text_bytes: 1024,
    maximum_characters: 512,
};

fn archive_segment_inputs(family: u32, version: u32, raw: &[u8]) -> SimulationSegmentColorInputs {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let inputs = parent.element_segment_color_inputs().unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.element_segment_color_inputs().unwrap(), inputs);
        return inputs;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let inputs = style.element_segment_color_inputs().unwrap();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.element_segment_color_inputs().unwrap(), inputs);
    inputs
}
fn segment_input_words(inputs: SimulationSegmentColorInputs) -> Vec<u32> {
    let mut words = vec![u32::from(inputs.color_1bc.is_some())];
    words.extend(inputs.color_1bc.map_or([0; 4], SimulationRgbaColor::bits));
    words
}
fn replay_segment_projection_case(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let length = read_word(cursor) as usize;
    let raw = take_projection(cursor, length);
    let expected: [u32; 5] = std::array::from_fn(|_| read_word(cursor));
    let inputs = archive_segment_inputs(family, version, raw);
    assert_eq!(inputs, plan_support::segment_inputs(expected));
    let mut output = segment_input_words(inputs);
    assert_eq!(output, expected);
    for _ in 0..4 {
        output.extend(assert_segment_scenario(cursor, inputs));
    }
    output
}
fn assert_segment_scenario(cursor: &mut &[u8], inputs: SimulationSegmentColorInputs) -> Vec<u32> {
    let runtime = plan_support::segment_runtime(std::array::from_fn(|_| read_word(cursor)));
    let expected: [u32; 9] = std::array::from_fn(|_| read_word(cursor));
    let actual =
        plan_support::segment_result_words(plan_support::SegmentRgbCase { inputs, runtime });
    assert_eq!(actual, expected);
    actual
}
fn replay_segment_projections(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let output = (0..count)
        .flat_map(|_| replay_segment_projection_case(&mut cursor))
        .collect();
    assert!(cursor.is_empty());
    output
}
#[test]
fn every_ordered_guard_unknown_and_raw_rgb_pattern_matches_independent_oracle() {
    assert_eq!(plan_support::replay_segment_cases(PLANS).len(), 25434 * 9);
}
#[test]
fn every_owned_context_preserves_legacy_absence_complete_rgba_and_archive_bytes() {
    assert_eq!(replay_segment_projections(VIEWS, 425).len(), 425 * 41);
}
fn find_segment_stage(stage: u32) -> plan_support::SegmentRgbCase {
    let mut cursor = PLANS;
    assert_eq!(read_word(&mut cursor), 1);
    for _ in 0..read_word(&mut cursor) {
        let case = plan_support::read_segment_case(&mut cursor);
        let expected: [u32; 9] = std::array::from_fn(|_| read_word(&mut cursor));
        if expected[0] == stage {
            return case;
        }
    }
    panic!("fixture lacks stage {stage}")
}
#[test]
fn completed_stages_retain_input_isolation_after_runtime_and_color_mutation() {
    for stage in 0..6 {
        let mut case = find_segment_stage(stage);
        let plan = case.inputs.plan_rgb_read(case.runtime).unwrap();
        let original = plan.clone();
        case.inputs.color_1bc = Some(SimulationRgbaColor::from_bits([42; 4]));
        case.runtime.mode_2c68_raw = None;
        assert_eq!(plan, original);
        assert_ne!(plan.source_inputs(), case.inputs);
        assert_ne!(plan.runtime_context(), case.runtime);
    }
}
#[test]
fn known_zero_rgba_remains_distinct_from_missing_and_alpha_never_changes_rgb() {
    let mut case = find_segment_stage(5);
    for alpha in [0, 0x80000000, 0x7fa00042, 0xffffffff] {
        case.inputs.color_1bc = Some(SimulationRgbaColor::from_bits([0, 0, 0, alpha]));
        let plan = case.inputs.plan_rgb_read(case.runtime).unwrap();
        assert_eq!(plan.rgb_bits(), Some([0; 3]));
        assert_eq!(plan.source_inputs(), case.inputs);
    }
    case.inputs.color_1bc = None;
    let error = case.inputs.plan_rgb_read(case.runtime).unwrap_err();
    assert_eq!(error.entity, "stored1BC");
    assert_eq!(error.class, bessemer::FailureClass::Missing);
}
#[test]
fn eight_workers_and_all_public_thread_traits_match_serial_read_plans() {
    assert_segment_thread_traits();
    let serial = plan_support::replay_segment_cases(PLANS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| plan_support::replay_segment_cases(PLANS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), serial);
        }
    });
}
fn assert_segment_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationSegmentColorInputs>();
    require::<SimulationSegmentColorRuntime>();
    require::<SimulationSegmentColorStage>();
    require::<SimulationSegmentColorCompletion>();
    require::<SimulationSegmentColorCoverage>();
    require::<SimulationSegmentColorDiagnostic>();
    require::<SimulationSegmentColorPlan>();
}
#[test]
#[ignore = "Requires freshly pinned complete native C892 fixtures."]
fn every_native_context_retains_unobserved_runtime_and_original_color_words() {
    replay_segment_projections(
        &std::fs::read("target/simulation-segment-rgb-projection-native.bin").unwrap(),
        276,
    );
}
#[test]
#[ignore = "Writes deterministic bytes for two fresh-process comparisons."]
fn export_segment_rgb_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-segment-rgb-projection-native.bin").unwrap();
    let output = [
        plan_support::replay_segment_cases(PLANS),
        replay_segment_projections(VIEWS, 425),
        replay_segment_projections(&native, 276),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_SEGMENT_RGB_OUTPUT") {
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
