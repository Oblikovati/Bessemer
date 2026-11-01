use super::load_projection::{read_word, take_projection};
use bessemer::{FailureClass, simulation::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaletteFixtureCase {
    pub words: [u32; 10],
    pub inputs: SimulationLegendPaletteInputs,
    pub runtime: SimulationLegendPaletteRuntime,
}

pub fn palette_case_from_words(words: [u32; 10]) -> PaletteFixtureCase {
    assert!(words[0] < 16 && words[5] <= 1 && words[7] <= 2 && words[8] <= 1);
    let optional = |index: usize| (words[0] & (1 << index) != 0).then_some(words[index + 1] as i32);
    PaletteFixtureCase {
        inputs: SimulationLegendPaletteInputs {
            mapping_mode_raw: optional(0),
            reverse_base_palette_raw: optional(1),
            base_palette_count_raw: optional(2),
            gray_above_yield_raw: optional(3),
        },
        runtime: SimulationLegendPaletteRuntime {
            gray_helper_return_raw: (words[5] == 1).then_some(words[6] as i32),
            plot_pointer_present: match words[7] {
                0 => None,
                1 => Some(false),
                _ => Some(true),
            },
            plot_word18_raw: (words[8] == 1).then_some(words[9] as i32),
        },
        words,
    }
}

pub fn read_palette_case(cursor: &mut &[u8]) -> PaletteFixtureCase {
    palette_case_from_words(std::array::from_fn(|_| read_word(cursor)))
}

fn palette_error_status(error: bessemer::BessemerError) -> u32 {
    assert_eq!(error.operation, "legend palette dispatch");
    assert!(error.cause.contains("expected"));
    let (status, class) = match error.entity.as_str() {
        "legend.word_d4" => (10, FailureClass::Missing),
        "gray helper return" => (11, FailureClass::Missing),
        "legend.word_8" => (12, FailureClass::Missing),
        "plot pointer" if error.class == FailureClass::Missing => (13, FailureClass::Missing),
        "plot pointer" => (14, FailureClass::Unsupported),
        "plot.word_18" => (15, FailureClass::Missing),
        other => panic!("unexpected {other}: {error:?}"),
    };
    assert_eq!(error.class, class);
    status
}

fn palette_status(dispatch: SimulationLegendPaletteDispatch) -> u32 {
    match dispatch {
        SimulationLegendPaletteDispatch::ReturnedAfterGrayHelper => 1,
        SimulationLegendPaletteDispatch::RegularMapper => 2,
        SimulationLegendPaletteDispatch::ShellMaterialMapper => 3,
        SimulationLegendPaletteDispatch::ShellThicknessMapper => 4,
    }
}

fn assert_palette_plan(case: PaletteFixtureCase, plan: &SimulationLegendPalettePlan) {
    let history = plan.history();
    assert_eq!(history.inputs, case.inputs);
    assert_eq!(history.runtime, case.runtime);
    assert_eq!(history.relation, SimulationThermalRelation::Derived);
    let completion = match plan.dispatch() {
        SimulationLegendPaletteDispatch::ReturnedAfterGrayHelper => {
            SimulationLegendPaletteCompletion::CompletedEarlyReturnSelection
        }
        SimulationLegendPaletteDispatch::RegularMapper => {
            SimulationLegendPaletteCompletion::CompletedRegularSelection
        }
        _ => SimulationLegendPaletteCompletion::CompletedTailSelection,
    };
    assert_eq!(plan.completion(), completion);
    assert_palette_qualifications(plan);
}

fn assert_palette_qualifications(plan: &SimulationLegendPalettePlan) {
    let coverage = plan.coverage();
    assert!(coverage.exact_local_dispatch);
    assert!(
        !coverage.native_count_validated
            && !coverage.selected_mapper_modeled
            && !coverage.wire_edited
    );
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert_eq!(
        plan.diagnostics(),
        [
            SimulationLegendPaletteDiagnostic::NativeNotExecuted,
            SimulationLegendPaletteDiagnostic::MappingAlgorithmsNotModeled,
            SimulationLegendPaletteDiagnostic::GrayOutputNotEstablished,
            SimulationLegendPaletteDiagnostic::RuntimeProductionNotInferred,
            SimulationLegendPaletteDiagnostic::NativeCountValidityUnknown
        ]
    );
}

fn palette_diagnostics(inputs: SimulationLegendPaletteInputs) -> Vec<u32> {
    [
        inputs.reverse_base_palette(),
        inputs.gray_above_yield_enabled(),
        inputs.base_count_within_nine_slots(),
    ]
    .into_iter()
    .flat_map(|value| {
        [
            u32::from(value.is_some()),
            u32::from(value.unwrap_or(false)),
        ]
    })
    .collect()
}

pub fn palette_result_words(case: PaletteFixtureCase) -> Vec<u32> {
    let before = case;
    let result = case.inputs.dispatch_palette(case.runtime);
    assert_eq!(case, before);
    let mut output = match result {
        Ok(plan) => {
            assert_palette_plan(case, &plan);
            let coverage = plan.coverage();
            vec![
                palette_status(plan.dispatch()),
                u32::from(coverage.consumed_member_mask),
                u32::from(coverage.gray_helper_result_read),
                u32::from(coverage.plot_pointer_read),
                u32::from(coverage.plot_word_read),
            ]
        }
        Err(error) => vec![palette_error_status(error), 0, 0, 0, 0],
    };
    output.extend(palette_diagnostics(case.inputs));
    output
}

pub fn assert_palette_case(cursor: &mut &[u8]) -> Vec<u32> {
    let case = read_palette_case(cursor);
    let expected: Vec<_> = (0..11).map(|_| read_word(cursor)).collect();
    let actual = palette_result_words(case);
    assert_eq!(actual, expected, "{:?}", case.words);
    actual
}

pub fn replay_palette_operations(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    let mut output = Vec::new();
    for _ in 0..count {
        output.extend(assert_palette_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    output
}

pub fn find_palette_case(mut cursor: &[u8], status: u32) -> PaletteFixtureCase {
    assert_eq!(read_word(&mut cursor), 1);
    for _ in 0..read_word(&mut cursor) {
        let case = read_palette_case(&mut cursor);
        let expected = take_projection(&mut cursor, 44);
        if u32::from_le_bytes(expected[..4].try_into().unwrap()) == status {
            return case;
        }
    }
    panic!("fixture lacks palette status{status}")
}
