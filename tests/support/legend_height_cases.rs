use super::load_projection::{read_word, take_projection};
use bessemer::{FailureClass, simulation::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeightFixtureCase {
    pub words: [u32; 4],
    pub inputs: SimulationLegendHeightInputs,
    pub cell: Option<i32>,
}

pub fn read_height_case(cursor: &mut &[u8]) -> HeightFixtureCase {
    let words: [u32; 4] = std::array::from_fn(|_| read_word(cursor));
    assert!(words[0] <= 1 && words[2] <= 1);
    HeightFixtureCase {
        inputs: SimulationLegendHeightInputs {
            cell_count_raw: (words[0] == 1).then_some(words[1] as i32),
        },
        cell: (words[2] == 1).then_some(words[3] as i32),
        words,
    }
}

fn height_error_status(error: bessemer::BessemerError) -> u32 {
    assert_eq!(error.operation, "legend height product");
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("None") && error.cause.contains("expected"));
    match error.entity.as_str() {
        "cell-height helper result" => 10,
        "legend.word_14" => 11,
        other => panic!("unexpected {other}: {error:?}"),
    }
}

fn assert_height_plan(case: HeightFixtureCase, plan: &SimulationLegendHeightPlan) {
    assert_eq!(plan.history().inputs, case.inputs);
    assert_eq!(Some(plan.history().cell_height_raw), case.cell);
    assert_eq!(plan.history().relation, SimulationThermalRelation::Derived);
    let expected = if plan.total_height_raw() == 0 {
        SimulationLegendHeightCompletion::CompletedZeroProduct
    } else {
        SimulationLegendHeightCompletion::CompletedNonzeroProduct
    };
    assert_eq!(plan.completion(), expected);
    let coverage = plan.coverage();
    assert!(coverage.exact_wrapping_signed32);
    assert!(
        !coverage.cell_helper_modeled
            && !coverage.pixel_coordinates_validated
            && !coverage.wire_edited
    );
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert_height_diagnostics(plan);
}

fn assert_height_diagnostics(plan: &SimulationLegendHeightPlan) {
    assert_eq!(
        plan.diagnostics(),
        [
            SimulationLegendHeightDiagnostic::NativeNotExecuted,
            SimulationLegendHeightDiagnostic::CellHeightHelperNotModeled,
            SimulationLegendHeightDiagnostic::RuntimeFontMetricsNotInferred,
            SimulationLegendHeightDiagnostic::TitleAndTicksNotModeled,
            SimulationLegendHeightDiagnostic::PixelValidityNotEstablished
        ]
    );
}

pub fn height_result_words(case: HeightFixtureCase) -> Vec<u32> {
    let before = case;
    let result = case.inputs.total_height_from_cell_height(case.cell);
    assert_eq!(case, before);
    match result {
        Ok(plan) => {
            assert_height_plan(case, &plan);
            let status = if plan.total_height_raw() == 0 { 1 } else { 2 };
            vec![
                status,
                plan.total_height_raw() as u32,
                plan.coverage().stored_words_read as u32,
                plan.coverage().runtime_scalars_read as u32,
            ]
        }
        Err(error) => vec![height_error_status(error), 0, 0, 0],
    }
}

pub fn assert_height_case(cursor: &mut &[u8]) -> Vec<u32> {
    let case = read_height_case(cursor);
    let expected: Vec<_> = (0..4).map(|_| read_word(cursor)).collect();
    let actual = height_result_words(case);
    assert_eq!(actual, expected, "{:?}", case.words);
    actual
}

pub fn replay_height_operations(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    let mut output = Vec::new();
    for _ in 0..count {
        output.extend(assert_height_case(&mut cursor));
    }
    assert!(cursor.is_empty());
    output
}

pub fn find_height_case(mut cursor: &[u8], status: u32) -> HeightFixtureCase {
    assert_eq!(read_word(&mut cursor), 1);
    for _ in 0..read_word(&mut cursor) {
        let case = read_height_case(&mut cursor);
        let expected = take_projection(&mut cursor, 16);
        if u32::from_le_bytes(expected[..4].try_into().unwrap()) == status {
            return case;
        }
    }
    panic!("fixture lacks height status{status}")
}
