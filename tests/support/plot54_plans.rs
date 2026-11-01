use super::read_word;
use bessemer::{FailureClass, simulation::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plot54FixtureCase {
    pub inputs: SimulationPlot54Inputs,
    pub runtime: SimulationPlot54Runtime,
}

fn plot54_known_word(tag: u32, raw: u32) -> Option<i32> {
    assert!(tag <= 1);
    (tag == 1).then_some(raw as i32)
}

fn plot54_known_bool(tag: u32) -> Option<bool> {
    assert!(tag <= 2);
    match tag {
        0 => None,
        1 => Some(false),
        _ => Some(true),
    }
}

pub fn plot54_inputs_from_words(words: [u32; 11]) -> SimulationPlot54Inputs {
    assert!(words[2] <= 1);
    let colors = (words[2] == 1).then(|| SimulationPlot54Colors {
        color_2ac: SimulationRgbaColor::from_bits(words[3..7].try_into().unwrap()),
        forwarded_2bc: SimulationRgbaColor::from_bits(words[7..11].try_into().unwrap()),
    });
    SimulationPlot54Inputs {
        plot_type_raw_code: plot54_known_word(words[0], words[1]),
        colors,
    }
}

pub fn plot54_runtime_from_words(words: [u32; 4]) -> SimulationPlot54Runtime {
    SimulationPlot54Runtime {
        mode_2c68_raw: plot54_known_word(words[0], words[1]),
        core_object_present: plot54_known_bool(words[2]),
        callback_key10_present: plot54_known_bool(words[3]),
    }
}

pub fn read_plot54_case(cursor: &mut &[u8]) -> Plot54FixtureCase {
    let words: [u32; 15] = std::array::from_fn(|_| read_word(cursor));
    Plot54FixtureCase {
        inputs: plot54_inputs_from_words(words[2..13].try_into().unwrap()),
        runtime: plot54_runtime_from_words([words[0], words[1], words[13], words[14]]),
    }
}

fn plot54_stage_word(stage: SimulationPlot54Stage) -> u32 {
    match stage {
        SimulationPlot54Stage::ModeBypassed => 0,
        SimulationPlot54Stage::PlotBypassed => 1,
        SimulationPlot54Stage::CoreRejected => 2,
        SimulationPlot54Stage::CallbackRejected => 3,
        SimulationPlot54Stage::ColorReadAdmitted => 4,
    }
}

fn assert_plot54_coverage(plan: &SimulationPlot54Plan) {
    let coverage = plan.coverage();
    assert_eq!(
        coverage.copied_color_arguments,
        2 * usize::from(plan.copied_arguments().is_some())
    );
    assert_eq!(
        coverage.renderer_components_read,
        4 * usize::from(plan.renderer_read_color().is_some())
    );
    assert!(coverage.local_color_only && coverage.exceptions_masked);
    assert!(!coverage.daz_enabled && !coverage.ftz_enabled);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert_eq!(
        plan.renderer_read_color().is_some(),
        plan.renderer_projected_color().is_some()
    );
    assert_plot54_completion(plan);
    assert_plot54_diagnostics(plan);
}

fn assert_plot54_completion(plan: &SimulationPlot54Plan) {
    let expected = if plan.renderer_read_color().is_some() {
        SimulationPlot54Completion::CompletedColorProjection
    } else {
        SimulationPlot54Completion::CompletedEmpty
    };
    assert_eq!(plan.completion(), expected);
}

fn assert_plot54_diagnostics(plan: &SimulationPlot54Plan) {
    assert_eq!(
        plan.diagnostics(),
        [
            SimulationPlot54Diagnostic::NativeNotExecuted,
            SimulationPlot54Diagnostic::RendererStateNotModeled,
            SimulationPlot54Diagnostic::CallbackGeometryUnknown,
            SimulationPlot54Diagnostic::Forwarded2bcPurposeUnknown
        ]
    );
}

fn assert_plot54_parents(case: Plot54FixtureCase, plan: &SimulationPlot54Plan) {
    assert_eq!(plan.source_inputs(), case.inputs);
    assert_eq!(plan.runtime_context(), case.runtime);
    assert_plot54_coverage(plan);
    if let Some(copied) = plan.copied_arguments() {
        assert_eq!(Some(copied), case.inputs.colors);
    }
    if let Some(read) = plan.renderer_read_color() {
        assert_eq!(read, case.inputs.colors.unwrap().color_2ac);
    }
}

fn plot54_copied_words(plan: &SimulationPlot54Plan) -> Vec<u32> {
    plan.copied_arguments().map_or(vec![0; 8], |colors| {
        [colors.color_2ac.bits(), colors.forwarded_2bc.bits()].concat()
    })
}

fn plot54_success_words(case: Plot54FixtureCase, plan: &SimulationPlot54Plan) -> Vec<u32> {
    assert_plot54_parents(case, plan);
    let mut words = vec![
        plot54_stage_word(plan.stage()),
        u32::from(plan.copied_arguments().is_some()),
        u32::from(plan.renderer_read_color().is_some()),
    ];
    words.extend(plot54_copied_words(plan));
    words.extend(
        plan.renderer_read_color()
            .map_or([0; 4], SimulationRgbaColor::bits),
    );
    words.extend(
        plan.renderer_projected_color()
            .map_or([0; 4], SimulationRgbaColor::bits),
    );
    words
}

pub fn plot54_result_words(case: Plot54FixtureCase) -> Vec<u32> {
    let original = case;
    let result = case.inputs.plan_masked_daz_ftz_off(case.runtime);
    assert_eq!(case, original);
    match result {
        Ok(plan) => plot54_success_words(case, &plan),
        Err(error) => {
            assert_eq!(error.operation, "plot54 color plan");
            assert_eq!(error.class, FailureClass::Missing);
            assert!(error.cause.contains("None") && error.cause.contains("expected"));
            let status = match error.entity.as_str() {
                "runtime2C68" => 10,
                "parent170" => 11,
                "stored color pair" => 12,
                "core2520" => 13,
                "callback key10" => 14,
                other => panic!("unexpected dependency {other}: {error:?}"),
            };
            [vec![status], vec![0; 18]].concat()
        }
    }
}

pub fn replay_plot54_cases(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 17236);
    let mut output = Vec::new();
    for _ in 0..count {
        let case = read_plot54_case(&mut cursor);
        let expected: [u32; 19] = std::array::from_fn(|_| read_word(&mut cursor));
        let actual = plot54_result_words(case);
        assert_eq!(actual, expected, "{case:?}");
        output.extend(actual);
    }
    assert!(cursor.is_empty());
    output
}
