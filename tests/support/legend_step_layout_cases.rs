use super::load_projection::read_word;
use bessemer::{FailureClass, simulation::*};

#[derive(Debug, Clone, Copy)]
pub struct LayoutFixtureCase {
    pub inputs: SimulationLegendStepInputs,
    pub runtime: SimulationLegendStepLayoutRuntime,
}

fn optional_layout_word(words: &[u32; 12], bit: u32, index: usize) -> Option<i32> {
    (words[0] & (1 << bit) != 0).then_some(words[index] as i32)
}

fn layout_runtime_words(words: &[u32; 12]) -> SimulationLegendStepLayoutRuntime {
    SimulationLegendStepLayoutRuntime {
        enable_raw: words[1] as i32,
        point_x_raw: optional_layout_word(words, 1, 3),
        point_y_raw: optional_layout_word(words, 2, 4),
        size_y_raw: optional_layout_word(words, 3, 5),
        word_12c_raw: optional_layout_word(words, 4, 6),
        word_124_raw: optional_layout_word(words, 5, 7),
        after_helpers: (words[0] & 64 != 0).then(|| layout_after_words(words)),
    }
}

fn layout_after_words(words: &[u32; 12]) -> SimulationLegendStepLayoutAfterHelpers {
    SimulationLegendStepLayoutAfterHelpers {
        spacing_raw: optional_layout_word(words, 7, 8),
        layout_switch_raw: optional_layout_word(words, 8, 9),
        word_1b0_raw: optional_layout_word(words, 9, 10),
        word_1b4_raw: optional_layout_word(words, 10, 11),
    }
}

pub fn read_layout_case(cursor: &mut &[u8]) -> LayoutFixtureCase {
    let words: [u32; 12] = std::array::from_fn(|_| read_word(cursor));
    assert!(words[0] < 2048);
    LayoutFixtureCase {
        inputs: SimulationLegendStepInputs {
            layout_switch_raw: optional_layout_word(&words, 0, 2),
        },
        runtime: layout_runtime_words(&words),
    }
}

fn layout_error_status(error: bessemer::BessemerError) -> u32 {
    assert_eq!(error.operation, "legend step layout projection");
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("None") && error.cause.contains("expected Some"));
    match error.entity.as_str() {
        "entry legend.word_fc" => 10,
        "pointY" => 11,
        "sizeY" => 12,
        "runtime word12C" => 13,
        "runtime word124" => 14,
        "pointX" => 15,
        "post-helper state" => 16,
        "post-helper spacing" => 17,
        "post-helper legend.word_fc" => 18,
        "runtime word1B0" => 19,
        "runtime word1B4" => 20,
        other => panic!("unexpected {other}: {error:?}"),
    }
}

fn assert_local_layout_plan(case: LayoutFixtureCase, plan: &SimulationLegendStepLayoutPlan) {
    assert_eq!(plan.history().inputs, case.inputs);
    assert_eq!(plan.history().runtime, case.runtime);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    let coverage = plan.coverage();
    assert!(coverage.exact_wrapping_signed32);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(!coverage.renderer_modeled && !coverage.pixel_coordinates_validated);
    let divided = case.runtime.enable_raw != 0 && case.inputs.layout_switch_raw != Some(0);
    assert_eq!(coverage.division_20_reached, divided);
    use SimulationLegendStepLayoutDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            AfterHelperFactsExplicit,
            TextHelpersNotModeled,
            CompleteRendererNotModeled,
            PixelValidityNotEstablished
        ]
    );
}

fn completed_layout_words(plan: &SimulationLegendStepLayoutPlan) -> [u32; 8] {
    let Some(values) = plan.values() else {
        assert_eq!(
            plan.completion(),
            SimulationLegendStepLayoutCompletion::CompletedDisabledEarlyReturn
        );
        return [0; 8];
    };
    assert_eq!(
        plan.completion(),
        SimulationLegendStepLayoutCompletion::CompletedIntegerLayout
    );
    layout_scalar_words(values, plan.coverage())
}

fn layout_scalar_words(
    values: SimulationLegendStepLayoutValues,
    coverage: SimulationLegendStepLayoutCoverage,
) -> [u32; 8] {
    [
        1,
        values.point_x_raw as u32,
        values.initial_y_raw as u32,
        values.spacing_raw as u32,
        values.vertical_after_spacing_raw as u32,
        coverage.entry_member_mask as u32,
        coverage.post_helper_member_mask as u32,
        coverage.native_text_helper_boundaries as u32,
    ]
}

fn layout_result_words(case: LayoutFixtureCase) -> [u32; 8] {
    match case.inputs.project_step_layout(case.runtime) {
        Ok(plan) => {
            assert_local_layout_plan(case, &plan);
            completed_layout_words(&plan)
        }
        Err(error) => {
            let mut words = [0; 8];
            words[0] = layout_error_status(error);
            words
        }
    }
}

pub fn assert_layout_case(cursor: &mut &[u8]) -> [u32; 8] {
    let case = read_layout_case(cursor);
    let expected: [u32; 8] = std::array::from_fn(|_| read_word(cursor));
    let actual = layout_result_words(case);
    assert_eq!(actual, expected, "{case:?}");
    actual
}

pub fn replay_layout_operations(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 35468);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(assert_layout_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
