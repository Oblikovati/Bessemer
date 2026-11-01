use super::read_word;
use bessemer::{BessemerError, FailureClass, simulation::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriangleColorFixtureCase {
    pub controls: SimulationTriangleColorControls,
    pub runtime: SimulationTriangleColorRuntime,
    pub groups: Vec<SimulationTriangleColorGroup>,
    pub maximum: usize,
}

pub fn read_triangle_color_case(cursor: &mut &[u8]) -> TriangleColorFixtureCase {
    let words: [u32; 17] = std::array::from_fn(|_| read_word(cursor));
    let controls = fixture_triangle_controls(words);
    TriangleColorFixtureCase {
        controls,
        runtime: triangle_color_runtime(words),
        groups: (0..words[16])
            .map(|_| read_triangle_color_group(cursor))
            .collect(),
        maximum: words[15] as usize,
    }
}

fn fixture_triangle_controls(words: [u32; 17]) -> SimulationTriangleColorControls {
    let color = SimulationRgbaColor::from_bits(words[3..7].try_into().unwrap());
    if words[0] == 0 {
        SimulationTriangleColorControls::HiddenParts {
            selector_raw: words[1] as i32,
            color,
        }
    } else {
        SimulationTriangleColorControls::ModelTessellation {
            override_raw: words[2] as i32,
            selector_raw: words[1] as i32,
            color,
        }
    }
}

fn triangle_color_runtime(words: [u32; 17]) -> SimulationTriangleColorRuntime {
    let caller = if words[0] == 0 {
        SimulationTriangleColorCaller::HiddenParts {
            option_result: known_triangle_word(words[7], words[8]),
            core_678_raw: known_triangle_word(words[9], words[10]),
        }
    } else {
        SimulationTriangleColorCaller::ModelTessellation {
            renderer_type_code: words[10] as i32,
            precheck_result: known_triangle_word(words[7], words[8]),
        }
    };
    SimulationTriangleColorRuntime {
        caller,
        core_object_present: words[11] != 0,
        group_count_raw: words[12],
        group_array_present: words[13] != 0,
        selection_flag_raw: words[14] as i32,
    }
}

fn known_triangle_word(tag: u32, raw: u32) -> Option<i32> {
    assert!(tag <= 1);
    (tag == 1).then_some(raw as i32)
}

fn triangle_payload(cursor: &mut &[u8]) -> u64 {
    let low = u64::from(read_word(cursor));
    low | (u64::from(read_word(cursor)) << 32)
}

fn read_triangle_color_group(cursor: &mut &[u8]) -> SimulationTriangleColorGroup {
    let class_raw = read_word(cursor) as i32;
    let triangle_count = read_word(cursor) as i32;
    let payload_tokens = std::array::from_fn(|_| triangle_payload(cursor));
    let color_c = SimulationRgbaColor::from_bits(std::array::from_fn(|_| read_word(cursor)));
    let color_1c = SimulationRgbaColor::from_bits(std::array::from_fn(|_| read_word(cursor)));
    let lookup_key_raw = read_word(cursor);
    let tag = read_word(cursor);
    assert!(tag <= 2);
    SimulationTriangleColorGroup {
        class_raw,
        triangle_count,
        payload_tokens,
        color_c,
        color_1c,
        lookup_key_raw,
        lookup_matches_end: if tag == 0 { None } else { Some(tag == 2) },
    }
}

fn triangle_stage_word(stage: SimulationTriangleColorStage) -> u32 {
    use SimulationTriangleColorStage::*;
    match stage {
        HiddenDisabled => 0,
        HiddenSelectorBypassed => 1,
        HiddenOtherPath => 2,
        ModelRendererExcluded => 3,
        ModelPrecheckRejected => 4,
        ModelOtherPath => 5,
        HelperCoreMissing => 6,
        HelperCountZero => 7,
        HelperArrayMissing => 8,
        HelperEntered => 9,
    }
}

fn triangle_error_words(error: BessemerError) -> Vec<u32> {
    assert_eq!(error.operation, "triangle color call plan");
    assert!(error.cause.contains("expected"));
    let code = match (error.class, error.entity.as_str()) {
        (FailureClass::Invalid, "group_count") => 10,
        (FailureClass::Missing, "group snapshots") => 11,
        (FailureClass::Missing, entity) if entity.starts_with("group[") => 13,
        (FailureClass::Missing, _) => 12,
        other => panic!("unexpected fixture error {other:?}: {error:?}"),
    };
    vec![code, 0, 0, 0, 0, 0, 0, 0, 0]
}

pub fn triangle_result_words(case: &TriangleColorFixtureCase) -> Vec<u32> {
    let original = case.clone();
    let result = case
        .controls
        .plan_masked_daz_ftz_off(case.runtime, &case.groups, case.maximum);
    assert_eq!(*case, original);
    match result {
        Ok(plan) => triangle_success_words(case, &plan),
        Err(error) => triangle_error_words(error),
    }
}

fn triangle_success_words(
    case: &TriangleColorFixtureCase,
    plan: &SimulationTriangleColorPlan,
) -> Vec<u32> {
    let coverage = plan.coverage();
    assert_eq!(plan.source_controls(), case.controls);
    assert_eq!(plan.runtime_context(), case.runtime);
    assert_eq!(
        plan.source_groups(),
        &case.groups[..coverage.inspected_groups]
    );
    assert_triangle_coverage(plan, case.maximum);
    let mut output = triangle_plan_header(plan);
    for call in plan.calls() {
        output.extend(triangle_call_words(*call));
    }
    output
}

fn triangle_plan_header(plan: &SimulationTriangleColorPlan) -> Vec<u32> {
    let coverage = plan.coverage();
    let mut output = vec![
        triangle_stage_word(plan.stage()),
        coverage.inspected_groups as u32,
        coverage.required_lookups as u32,
        u32::from(plan.initial_renderer_color().is_some()),
    ];
    output.extend(
        plan.initial_renderer_color()
            .map_or([0; 4], SimulationRgbaColor::bits),
    );
    output.push(plan.calls().len() as u32);
    output
}

fn assert_triangle_coverage(plan: &SimulationTriangleColorPlan, maximum: usize) {
    let coverage = plan.coverage();
    assert_eq!(
        coverage.declared_group_count,
        plan.runtime_context().group_count_raw
    );
    assert_eq!(coverage.maximum_groups, maximum);
    assert!(coverage.local_requests_only && coverage.exceptions_masked);
    assert!(!coverage.daz_enabled && !coverage.ftz_enabled);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    assert_eq!(plan.diagnostics().len(), 4);
    let completed = if plan.calls().is_empty() {
        SimulationTriangleColorCompletion::CompletedEmpty
    } else {
        SimulationTriangleColorCompletion::CompletedCalls
    };
    assert_eq!(plan.completion(), completed);
}

fn triangle_call_words(call: SimulationTriangleColorCall) -> Vec<u32> {
    let mut words = vec![call.group_index, call.triangle_count as u32];
    for payload in call.payload_tokens {
        words.extend([payload as u32, (payload >> 32) as u32]);
    }
    words.extend(call.color.bits());
    words.push(match call.origin {
        SimulationTriangleColorOrigin::Supplied => 0,
        SimulationTriangleColorOrigin::GroupC => 1,
        SimulationTriangleColorOrigin::Group1C => 2,
    });
    words
}

pub fn replay_triangle_color_cases(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 25436);
    let mut output = Vec::new();
    for _ in 0..count {
        let case = read_triangle_color_case(&mut cursor);
        let header: [u32; 9] = std::array::from_fn(|_| read_word(&mut cursor));
        let mut expected = header.to_vec();
        expected.extend((0..11 * header[8]).map(|_| read_word(&mut cursor)));
        let actual = triangle_result_words(&case);
        assert_eq!(actual, expected, "{case:?}");
        output.extend(actual);
    }
    assert!(cursor.is_empty());
    output
}
