use super::load_projection::read_word;
use bessemer::{FailureClass, simulation::*};

#[derive(Debug, Clone, Copy)]
pub struct YieldFixtureCase {
    pub inputs: SimulationLegendYieldInputs,
    pub runtime: SimulationLegendYieldRuntime,
}

pub fn read_yield_case(cursor: &mut &[u8]) -> YieldFixtureCase {
    let words: [u32; 11] = std::array::from_fn(|_| read_word(cursor));
    let mask = words[0];
    assert!(mask < 16);
    YieldFixtureCase {
        inputs: SimulationLegendYieldInputs {
            gray_base_color: (mask & 1 != 0)
                .then(|| SimulationRgbaColor::from_bits(words[1..5].try_into().unwrap())),
            reverse_base_palette_raw: (mask & 2 != 0).then_some(words[5] as i32),
        },
        runtime: SimulationLegendYieldRuntime {
            requested_output_count_raw: words[7] as i32,
            split_index_raw: (mask & 4 != 0).then_some(words[6] as i32),
            after_suffix: prefix_fixture_state(&words),
        },
    }
}

fn prefix_fixture_state(words: &[u32; 11]) -> Option<SimulationLegendYieldPrefixState> {
    (words[0] & 8 != 0).then_some(SimulationLegendYieldPrefixState {
        reverse_base_palette_raw: words[8] as i32,
        base_palette_count_raw: words[9] as i32,
        split_index_raw: words[10] as i32,
    })
}

fn yield_error_status(error: bessemer::BessemerError) -> u32 {
    assert_eq!(error.operation, "legend yield call plan");
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("None") && error.cause.contains("expected"));
    match error.entity.as_str() {
        "legend.color_d8" => 10,
        "runtime split index178" => 11,
        "legend.word_c" => 12,
        "post-suffix reread state" => 13,
        other => panic!("unexpected {other}: {error:?}"),
    }
}

fn assert_yield_plan(case: YieldFixtureCase, plan: &SimulationLegendYieldPlan) {
    assert_eq!(plan.history().inputs, case.inputs);
    assert_eq!(plan.history().runtime, case.runtime);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    assert_eq!(
        Some(plan.copied_base_pair()[0]),
        case.inputs.gray_base_color
    );
    assert_eq!(plan.copied_base_pair()[1].bits(), [0x3f800000; 4]);
    let coverage = plan.coverage();
    assert!(coverage.stored_color_copied && coverage.exact_local_integer_plan);
    assert!(!coverage.mapping_helper_modeled && !coverage.output_memory_validated);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert_yield_diagnostics(plan);
}

fn assert_yield_diagnostics(plan: &SimulationLegendYieldPlan) {
    use SimulationLegendYieldDiagnostic::*;
    assert_eq!(
        plan.diagnostics(),
        [
            NativeNotExecuted,
            TickAdjustmentNotModeled,
            MappingHelperNotModeled,
            OutputMemoryNotValidated,
            PostSuffixFactsExplicit,
            NearestEvenWhiteConstruction
        ]
    );
}

fn yield_call_words(call: &SimulationLegendYieldCall) -> [u32; 6] {
    let raw = call.destination_byte_offset as u64;
    let base = match call.base {
        SimulationLegendYieldBase::CopiedYieldPair => 0,
        SimulationLegendYieldBase::OrdinaryLegendArea => 1,
    };
    [
        raw as u32,
        (raw >> 32) as u32,
        call.output_count_raw as u32,
        call.base_count_raw as u32,
        call.reversal_raw as u32,
        base,
    ]
}

fn completed_yield_words(plan: &SimulationLegendYieldPlan) -> Vec<u32> {
    let count = plan.calls().len() as u32;
    let coverage = plan.coverage();
    let mut words = vec![
        count,
        count,
        plan.return_raw_if_calls_complete() as u32,
        coverage.entry_reversal_read as u32,
        coverage.post_suffix_state_read as u32,
    ];
    for call in plan.calls() {
        words.extend(yield_call_words(call));
    }
    words.resize(17, 0);
    words
}

pub fn yield_result_words(case: YieldFixtureCase) -> Vec<u32> {
    match case.inputs.plan_gray_calls_nearest_even(case.runtime) {
        Ok(plan) => {
            assert_yield_plan(case, &plan);
            completed_yield_words(&plan)
        }
        Err(error) => {
            let mut words = vec![0; 17];
            words[0] = yield_error_status(error);
            words
        }
    }
}

pub fn assert_yield_case(cursor: &mut &[u8]) -> Vec<u32> {
    let case = read_yield_case(cursor);
    let expected: Vec<_> = (0..17).map(|_| read_word(cursor)).collect();
    let actual = yield_result_words(case);
    assert_eq!(actual, expected, "{case:?}");
    actual
}

pub fn replay_yield_operations(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 13632);
    let mut words = Vec::new();
    for _ in 0..count {
        words.extend(assert_yield_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    words
}
