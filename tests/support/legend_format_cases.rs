use super::load_projection::take_projection;
use super::read_word;
use bessemer::{FailureClass, simulation::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegendFixtureCase {
    pub words: [u32; 9],
    pub inputs: SimulationLegendFormatInputs,
    pub runtime: SimulationLegendFormatRuntime,
}

pub fn legend_case_from_words(words: [u32; 9]) -> LegendFixtureCase {
    assert!(words[0] <= 1 && words[1] < 16 && words[6] <= 2 && words[7] <= 1);
    let optional = |index: usize| (words[1] & (1 << index) != 0).then_some(words[index + 2] as i32);
    LegendFixtureCase {
        inputs: SimulationLegendFormatInputs {
            mode_raw: optional(0),
            precision_raw: optional(1),
            alternate_enabled_raw: optional(2),
            alternate_selector_raw: optional(3),
        },
        runtime: SimulationLegendFormatRuntime {
            output_pointer_present: match words[6] {
                0 => None,
                1 => Some(false),
                _ => Some(true),
            },
            value_bits: (words[7] == 1).then_some(words[8]),
        },
        words,
    }
}

pub fn read_legend_case(cursor: &mut &[u8]) -> LegendFixtureCase {
    legend_case_from_words(std::array::from_fn(|_| read_word(cursor)))
}

pub fn execute_legend_case(
    case: &LegendFixtureCase,
) -> bessemer::Result<SimulationLegendFormatPlan> {
    if case.words[0] == 0 {
        return case.inputs.primary_format();
    }
    case.inputs
        .alternate_format_masked_exceptions_daz_off(case.runtime)
}

fn legend_error_status(error: bessemer::BessemerError) -> u32 {
    assert_eq!(error.operation, "legend number format");
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("None") && error.cause.contains("expected"));
    match error.entity.as_str() {
        "output string pointer" => 10,
        "legend.word_b8" => 11,
        "legend.word_c4" => 12,
        "input value" => 13,
        "legend.word_c8" => 14,
        "legend.word_bc" => 15,
        other => panic!("unexpected {other}: {error:?}"),
    }
}

fn legend_selection_status(selection: SimulationLegendFormatSelection) -> u32 {
    match selection {
        SimulationLegendFormatSelection::Primary => 1,
        SimulationLegendFormatSelection::Alternate => 2,
        SimulationLegendFormatSelection::OutputPointerNull => 3,
        SimulationLegendFormatSelection::ModeNotZero => 4,
        SimulationLegendFormatSelection::AlternateDisabled => 5,
        SimulationLegendFormatSelection::MagnitudeOutsideInterval => 6,
    }
}

fn assert_legend_plan_history(case: &LegendFixtureCase, plan: &SimulationLegendFormatPlan) {
    let history = plan.history();
    assert_eq!(history.inputs, case.inputs);
    assert_eq!(
        history.runtime,
        (case.words[0] == 1).then_some(case.runtime)
    );
    assert_eq!(
        history.output_relation,
        plan.format_string()
            .map(|_| SimulationThermalRelation::Derived)
    );
    assert_eq!(
        plan.diagnostics(),
        [
            SimulationLegendFormatDiagnostic::NativeNotExecuted,
            SimulationLegendFormatDiagnostic::NativeFormattingNotModeled,
            SimulationLegendFormatDiagnostic::LocaleNotModeled,
            SimulationLegendFormatDiagnostic::RuntimeNotInferred,
        ]
    );
}

fn assert_legend_plan_coverage(case: &LegendFixtureCase, plan: &SimulationLegendFormatPlan) {
    let coverage = plan.coverage();
    let expected_mode = if case.words[0] == 0 {
        SimulationLegendFormatCheckMode::ExactWordArithmetic
    } else {
        SimulationLegendFormatCheckMode::ExactMaskedExceptionsDazOff
    };
    assert_eq!(coverage.check_mode, expected_mode);
    assert!(!coverage.wire_edited);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    let output = plan.format_string().unwrap_or("");
    assert!(output.is_ascii() && output.len() <= 32);
    assert_eq!(coverage.output_ascii_bytes, output.len());
}

fn legend_plan_packet(case: &LegendFixtureCase, plan: &SimulationLegendFormatPlan) -> Vec<u8> {
    assert_legend_plan_history(case, plan);
    assert_legend_plan_coverage(case, plan);
    let coverage = plan.coverage();
    let output = plan.format_string().unwrap_or("");
    let words = [
        legend_selection_status(plan.selection()),
        u32::from(coverage.consumed_member_mask),
        u32::from(coverage.pointer_checked),
        u32::from(coverage.value_checked),
        output.len() as u32,
    ];
    let mut packet: Vec<u8> = words.into_iter().flat_map(u32::to_le_bytes).collect();
    packet.extend_from_slice(output.as_bytes());
    packet.resize(52, 0);
    packet
}

pub fn legend_result_packet(case: &LegendFixtureCase) -> Vec<u8> {
    let before = case.clone();
    let result = execute_legend_case(case);
    assert_eq!(case, &before);
    match result {
        Ok(plan) => legend_plan_packet(case, &plan),
        Err(error) => {
            let mut packet = legend_error_status(error).to_le_bytes().to_vec();
            packet.resize(52, 0);
            packet
        }
    }
}

pub fn assert_legend_case(cursor: &mut &[u8]) -> Vec<u8> {
    let case = read_legend_case(cursor);
    let expected = take_projection(cursor, 52);
    let actual = legend_result_packet(&case);
    assert_eq!(actual, expected, "{:?}", case.words);
    actual
}

pub fn replay_legend_cases(mut cursor: &[u8]) -> Vec<u8> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    let mut output = Vec::new();
    for _ in 0..count {
        output.extend(assert_legend_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    output
}
