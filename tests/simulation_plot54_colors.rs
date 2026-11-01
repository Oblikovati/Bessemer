//! Complete bounded C891 colors with explicit absent/unknown and callback/geometry exclusions (#166).
use bessemer::simulation::*;
#[allow(
    dead_code,
    reason = "Shared support also serves complete field/history packets."
)]
#[path = "support/binary_load_projection.rs"]
mod load_projection;
use load_projection::{read_word, take_projection};
#[path = "support/plot54_plans.rs"]
mod plan_support;
const PLANS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/plot54-plans.bin");
const VIEWS: &[u8] = include_bytes!("fixtures/simulation-post-dialog/plot54-projection.bin");
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

fn archive_plot54_inputs(family: u32, version: u32, raw: &[u8]) -> SimulationPlot54Inputs {
    let version = SimulationPersistVersion::new(version as i32).unwrap();
    if family == 0 {
        let parent = SimulationPostDialog::from_bytes(raw, version, PARENTS).unwrap();
        let inputs = parent.plot54_color_inputs().unwrap();
        assert_eq!(parent.to_bytes(), raw);
        assert_eq!(parent.plot54_color_inputs().unwrap(), inputs);
        return inputs;
    }
    assert_eq!(family, 2);
    let style = SimulationPostStyle::from_bytes(raw, version, STYLES).unwrap();
    let inputs = style.plot54_color_inputs().unwrap();
    assert_eq!(style.to_bytes(), raw);
    assert_eq!(style.plot54_color_inputs().unwrap(), inputs);
    inputs
}

fn plot54_input_words(inputs: SimulationPlot54Inputs) -> Vec<u32> {
    let mut output = vec![
        u32::from(inputs.plot_type_raw_code.is_some()),
        inputs.plot_type_raw_code.unwrap_or(0) as u32,
        u32::from(inputs.colors.is_some()),
    ];
    output.extend(inputs.colors.map_or(vec![0; 8], |colors| {
        [colors.color_2ac.bits(), colors.forwarded_2bc.bits()].concat()
    }));
    output
}

fn replay_plot54_projection_case(cursor: &mut &[u8]) -> Vec<u32> {
    let family = read_word(cursor);
    let version = read_word(cursor);
    let length = read_word(cursor) as usize;
    let raw = take_projection(cursor, length);
    let expected_words: [u32; 11] = std::array::from_fn(|_| read_word(cursor));
    let inputs = archive_plot54_inputs(family, version, raw);
    assert_eq!(
        inputs,
        plan_support::plot54_inputs_from_words(expected_words)
    );
    let mut output = plot54_input_words(inputs);
    assert_eq!(output, expected_words);
    for _ in 0..4 {
        output.extend(assert_plot54_scenario(cursor, inputs));
    }
    output
}

fn assert_plot54_scenario(cursor: &mut &[u8], inputs: SimulationPlot54Inputs) -> Vec<u32> {
    let runtime =
        plan_support::plot54_runtime_from_words(std::array::from_fn(|_| read_word(cursor)));
    let expected: [u32; 19] = std::array::from_fn(|_| read_word(cursor));
    let actual =
        plan_support::plot54_result_words(plan_support::Plot54FixtureCase { inputs, runtime });
    assert_eq!(actual, expected);
    actual
}

fn replay_plot54_projections(mut cursor: &[u8], count: u32) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), count);
    let output = (0..count)
        .flat_map(|_| replay_plot54_projection_case(&mut cursor))
        .collect();
    assert!(cursor.is_empty());
    output
}

#[test]
fn complete_copy_guard_and_comiss_flag_matrix_matches_independent_oracle() {
    assert!(!plan_support::replay_plot54_cases(PLANS).is_empty());
}

#[test]
fn every_owned_context_preserves_old_absence_unknown_parent_and_complete_source_bytes() {
    assert_eq!(replay_plot54_projections(VIEWS, 425).len(), 425 * 87);
}

fn find_plot54_stage(stage: u32) -> plan_support::Plot54FixtureCase {
    let mut cursor = PLANS;
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    for _ in 0..count {
        let case = plan_support::read_plot54_case(&mut cursor);
        let expected: [u32; 19] = std::array::from_fn(|_| read_word(&mut cursor));
        if expected[0] == stage {
            return case;
        }
    }
    panic!("fixture lacks local stage {stage}")
}

#[test]
fn every_completed_stage_is_isolated_from_later_input_fact_and_color_mutations() {
    for stage in 0..5 {
        let mut case = find_plot54_stage(stage);
        let plan = case.inputs.plan_masked_daz_ftz_off(case.runtime).unwrap();
        let original = plan.clone();
        case.inputs.plot_type_raw_code = Some(123);
        case.inputs.colors = Some(SimulationPlot54Colors {
            color_2ac: SimulationRgbaColor::from_bits([0; 4]),
            forwarded_2bc: SimulationRgbaColor::from_bits([1; 4]),
        });
        case.runtime.mode_2c68_raw = Some(-1);
        case.runtime.core_object_present = None;
        case.runtime.callback_key10_present = None;
        assert_eq!(plan, original);
        assert_ne!(plan.source_inputs(), case.inputs);
        assert_ne!(plan.runtime_context(), case.runtime);
    }
}

#[test]
fn raw_zero_color_pair_remains_known_and_distinct_from_a_missing_pair() {
    let mut case = find_plot54_stage(4);
    case.inputs.colors = Some(SimulationPlot54Colors {
        color_2ac: SimulationRgbaColor::from_bits([0; 4]),
        forwarded_2bc: SimulationRgbaColor::from_bits([0; 4]),
    });
    let plan = case.inputs.plan_masked_daz_ftz_off(case.runtime).unwrap();
    assert_eq!(
        plan.renderer_read_color(),
        Some(SimulationRgbaColor::from_bits([0; 4]))
    );
    assert_eq!(plan.renderer_projected_color(), plan.renderer_read_color());
    case.inputs.colors = None;
    let error = case
        .inputs
        .plan_masked_daz_ftz_off(case.runtime)
        .unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Missing);
    assert_eq!(error.entity, "stored color pair");
}

#[test]
fn eight_workers_and_all_public_thread_traits_match_serial_requests() {
    assert_plot54_thread_traits();
    let expected = plan_support::replay_plot54_cases(PLANS);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| plan_support::replay_plot54_cases(PLANS)))
            .collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}

fn assert_plot54_thread_traits() {
    fn require<T: Send + Sync>() {}
    require::<SimulationPlot54Colors>();
    require::<SimulationPlot54Inputs>();
    require::<SimulationPlot54Runtime>();
    require::<SimulationPlot54Stage>();
    require::<SimulationPlot54Completion>();
    require::<SimulationPlot54Coverage>();
    require::<SimulationPlot54Diagnostic>();
    require::<SimulationPlot54Plan>();
}

#[test]
#[ignore = "Requires complete freshly pinned native plot54 input fixtures."]
fn every_native_context_preserves_raw_codes_pairs_and_unobserved_runtime_facts() {
    replay_plot54_projections(
        &std::fs::read("target/simulation-plot54-projection-native.bin").unwrap(),
        276,
    );
}

#[test]
#[ignore = "Writes deterministic output for independent process comparison."]
fn export_plot54_results_for_fresh_process_replay() {
    let native = std::fs::read("target/simulation-plot54-projection-native.bin").unwrap();
    let output = [
        plan_support::replay_plot54_cases(PLANS),
        replay_plot54_projections(VIEWS, 425),
        replay_plot54_projections(&native, 276),
    ]
    .concat();
    if let Some(path) = std::env::var_os("BESSEMER_PLOT54_OUTPUT") {
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
