use super::read_word;
use bessemer::{FailureClass, simulation::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentRgbCase {
    pub inputs: SimulationSegmentColorInputs,
    pub runtime: SimulationSegmentColorRuntime,
}

fn segment_known_bits(tag: u32, raw: u32) -> Option<u32> {
    assert!(tag <= 1);
    (tag == 1).then_some(raw)
}
pub fn segment_inputs(words: [u32; 5]) -> SimulationSegmentColorInputs {
    assert!(words[0] <= 1);
    SimulationSegmentColorInputs {
        color_1bc: (words[0] == 1)
            .then(|| SimulationRgbaColor::from_bits(words[1..5].try_into().unwrap())),
    }
}
pub fn segment_runtime(words: [u32; 8]) -> SimulationSegmentColorRuntime {
    SimulationSegmentColorRuntime {
        mode_2c68_raw: segment_known_bits(words[0], words[1]).map(|value| value as i32),
        first_element_274c: segment_known_bits(words[2], words[3]).map(|value| value as i32),
        last_element_2750: segment_known_bits(words[4], words[5]).map(|value| value as i32),
        flags_da4_raw: segment_known_bits(words[6], words[7]),
    }
}
pub fn read_segment_case(cursor: &mut &[u8]) -> SegmentRgbCase {
    let words: [u32; 13] = std::array::from_fn(|_| read_word(cursor));
    SegmentRgbCase {
        runtime: segment_runtime(words[..8].try_into().unwrap()),
        inputs: segment_inputs(words[8..].try_into().unwrap()),
    }
}
fn segment_stage_word(stage: SimulationSegmentColorStage) -> u32 {
    match stage {
        SimulationSegmentColorStage::ModeBypassed => 0,
        SimulationSegmentColorStage::FirstIndexRejected => 1,
        SimulationSegmentColorStage::LastIndexRejected => 2,
        SimulationSegmentColorStage::RangeRejected => 3,
        SimulationSegmentColorStage::FlagsRejected => 4,
        SimulationSegmentColorStage::RgbReadAdmitted => 5,
    }
}
fn assert_segment_contract(case: SegmentRgbCase, plan: &SimulationSegmentColorPlan) {
    assert_eq!(plan.source_inputs(), case.inputs);
    assert_eq!(plan.runtime_context(), case.runtime);
    let read = plan.rgb_bits();
    assert_eq!(
        plan.coverage().components_read,
        3 * usize::from(read.is_some())
    );
    assert!(!plan.coverage().geometry_validated);
    assert_eq!(plan.coverage().achieved_pattern_error_bits, 0);
    let completion = if read.is_some() {
        SimulationSegmentColorCompletion::CompletedRgbRead
    } else {
        SimulationSegmentColorCompletion::CompletedEmpty
    };
    assert_eq!(plan.completion(), completion);
    assert_segment_diagnostics(plan);
}
fn segment_success_words(case: SegmentRgbCase, plan: &SimulationSegmentColorPlan) -> Vec<u32> {
    assert_segment_contract(case, plan);
    let mut words = vec![
        segment_stage_word(plan.stage()),
        u32::from(plan.rgb_bits().is_some()),
    ];
    words.extend(if plan.rgb_bits().is_some() {
        case.inputs.color_1bc.unwrap().bits()
    } else {
        [0; 4]
    });
    words.extend(plan.rgb_bits().unwrap_or([0; 3]));
    words
}
pub fn segment_result_words(case: SegmentRgbCase) -> Vec<u32> {
    let original = case;
    let result = case.inputs.plan_rgb_read(case.runtime);
    assert_eq!(case, original);
    match result {
        Ok(plan) => segment_success_words(case, &plan),
        Err(error) => segment_error_words(error),
    }
}
fn segment_error_words(error: bessemer::BessemerError) -> Vec<u32> {
    assert_eq!(error.operation, "element-segment RGB read");
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("None") && error.cause.contains("expected"));
    let status = match error.entity.as_str() {
        "runtime2C68" => 10,
        "first274C" => 11,
        "last2750" => 12,
        "flagsDA4" => 13,
        "stored1BC" => 14,
        other => panic!("unexpected dependency {other}: {error:?}"),
    };
    [vec![status], vec![0; 8]].concat()
}
pub fn replay_segment_cases(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 25434);
    let mut output = Vec::new();
    for _ in 0..count {
        let case = read_segment_case(&mut cursor);
        let expected: [u32; 9] = std::array::from_fn(|_| read_word(&mut cursor));
        let actual = segment_result_words(case);
        assert_eq!(actual, expected, "{case:?}");
        output.extend(actual);
    }
    assert!(cursor.is_empty());
    output
}

fn assert_segment_diagnostics(plan: &SimulationSegmentColorPlan) {
    assert_eq!(
        plan.diagnostics(),
        [
            SimulationSegmentColorDiagnostic::NativeNotExecuted,
            SimulationSegmentColorDiagnostic::PreReadStateUnmodeled,
            SimulationSegmentColorDiagnostic::EndpointGeometryUnknown,
            SimulationSegmentColorDiagnostic::StoredAlphaPurposeUnknown
        ]
    );
}
