use super::load_projection::read_word;
use bessemer::simulation::*;

pub fn read_step_runtime(cursor: &mut &[u8]) -> SimulationLegendStepRuntime {
    let words: [u32; 6] = std::array::from_fn(|_| read_word(cursor));
    SimulationLegendStepRuntime {
        step_count_raw: words[0] as i32,
        plot_type_raw: words[1] as i32,
        analysis_type_raw: words[2] as i32,
        word_bd8_raw: words[3] as i32,
        word_be8_raw: words[4] as i32,
        word_bec_raw: words[5] as i32,
    }
}

fn assert_step_plan(runtime: SimulationLegendStepRuntime, plan: &SimulationLegendStepPlan) {
    assert_eq!(plan.history().runtime, runtime);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    let coverage = plan.coverage();
    assert_eq!(coverage.stored_words_read, 0);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert!(coverage.exact_signed32_policy);
    assert!(!coverage.runtime_plot_pointer_validated && !coverage.renderer_modeled);
    use SimulationLegendStepDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            RuntimePlotWordsExplicit,
            PointerSafetyNotEstablished,
            LayoutSwitchNotAvailability,
            RendererNotModeled
        ]
    );
}

fn step_result_words(runtime: SimulationLegendStepRuntime) -> [u32; 3] {
    let plan = runtime.step_availability();
    assert_step_plan(runtime, &plan);
    let completion = match plan.completion() {
        SimulationLegendStepCompletion::CompletedZeroReturn => 1,
        SimulationLegendStepCompletion::CompletedNonzeroReturn => 2,
    };
    [
        plan.availability_raw() as u32,
        plan.coverage().runtime_member_mask as u32,
        completion,
    ]
}

pub fn assert_step_case(cursor: &mut &[u8]) -> [u32; 3] {
    let runtime = read_step_runtime(cursor);
    let expected: [u32; 3] = std::array::from_fn(|_| read_word(cursor));
    let actual = step_result_words(runtime);
    assert_eq!(actual, expected, "{runtime:?}");
    actual
}

pub fn replay_step_operations(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 102272);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(assert_step_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
