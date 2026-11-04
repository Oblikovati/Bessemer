use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlanInput {
    controls: SimulationAuxiliaryTriangleControls,
    context: SimulationAuxTriangleRuntime,
    groups: Vec<SimulationAuxTriangleGroup>,
    maximum: usize,
}

fn read_token(cursor: &mut &[u8]) -> u64 {
    let low = read_word(cursor);
    u64::from(low) | u64::from(read_word(cursor)) << 32
}

fn read_group(cursor: &mut &[u8]) -> SimulationAuxTriangleGroup {
    SimulationAuxTriangleGroup {
        triangle_count: read_word(cursor) as i32,
        payload_tokens: [read_token(cursor), read_token(cursor)],
        color: SimulationRgbaColor::from_bits(std::array::from_fn(|_| read_word(cursor))),
    }
}

fn read_plan_input(cursor: &mut &[u8]) -> PlanInput {
    let controls = read_controls(cursor);
    let context = read_runtime_context(cursor);
    let maximum = read_word(cursor) as usize;
    let count = read_word(cursor);
    assert!(count <= 3);
    let groups = (0..count).map(|_| read_group(cursor)).collect();
    PlanInput {
        controls,
        context,
        groups,
        maximum,
    }
}

fn read_runtime_context(cursor: &mut &[u8]) -> SimulationAuxTriangleRuntime {
    let core = read_word(cursor);
    let plot_word_170 = read_word(cursor) as i32;
    let group_count_raw = read_word(cursor);
    let array = read_word(cursor);
    let renderer_type_code = read_word(cursor) as i32;
    assert!(core <= 1 && array <= 1);
    SimulationAuxTriangleRuntime {
        core_object_present: core == 1,
        plot_word_170,
        group_count_raw,
        group_array_present: array == 1,
        renderer_type_code,
    }
}

fn stage_code(stage: SimulationAuxTriangleStage) -> u32 {
    match stage {
        SimulationAuxTriangleStage::CallerBypassed => 0,
        SimulationAuxTriangleStage::HelperEntered => 6,
        SimulationAuxTriangleStage::HelperRejected(reason) => match reason {
            SimulationAuxTriangleRejection::MissingCoreObject => 1,
            SimulationAuxTriangleRejection::ExcludedPlot => 2,
            SimulationAuxTriangleRejection::EmptyGroupCount => 3,
            SimulationAuxTriangleRejection::MissingGroupArray => 4,
            SimulationAuxTriangleRejection::ExcludedRenderer => 5,
        },
    }
}

fn project_call(call: &SimulationAuxTriangleCall) -> Vec<u32> {
    let source = match call.color_source {
        SimulationAuxTriangleColorSource::Supplied => 0,
        SimulationAuxTriangleColorSource::GroupRgbWithSuppliedAlpha => 1,
    };
    let [first, second] = call.payload_tokens;
    [
        vec![
            call.group_index,
            call.triangle_count as u32,
            first as u32,
            (first >> 32) as u32,
            second as u32,
            (second >> 32) as u32,
        ],
        call.color.bits().to_vec(),
        vec![source],
    ]
    .concat()
}

fn assert_plan_contract(plan: &SimulationAuxTrianglePlan, input: &PlanInput) {
    assert_eq!(plan.source_controls(), input.controls);
    assert_eq!(plan.runtime_context(), input.context);
    assert_eq!(
        plan.coverage().declared_group_count,
        input.context.group_count_raw
    );
    assert_eq!(plan.coverage().maximum_groups, input.maximum);
    assert!(plan.coverage().local_requests_only);
    assert_eq!(plan.coverage().achieved_pattern_error_bits, 0);
    assert_plan_exclusions(plan);
    assert_eq!(plan.clone(), *plan);
}

fn assert_plan_exclusions(plan: &SimulationAuxTrianglePlan) {
    assert_eq!(
        plan.diagnostics(),
        [
            SimulationAuxTriangleDiagnostic::NativeNotExecuted,
            SimulationAuxTriangleDiagnostic::PayloadNotValidated,
            SimulationAuxTriangleDiagnostic::SceneTraversalNotModeled
        ]
    );
    let completion = if plan.calls().is_empty() {
        SimulationAuxTriangleCompletion::CompletedEmpty
    } else {
        SimulationAuxTriangleCompletion::CompletedCalls
    };
    assert_eq!(plan.completion(), completion);
}

fn project_success(plan: &SimulationAuxTrianglePlan, input: &PlanInput) -> Vec<u32> {
    assert_plan_contract(plan, input);
    let initial = plan.initial_renderer_color();
    let mut words = vec![
        stage_code(plan.stage()),
        plan.coverage().inspected_groups as u32,
        u32::from(initial.is_some()),
    ];
    words.extend(initial.map_or([0; 4], SimulationRgbaColor::bits));
    words.push(plan.calls().len() as u32);
    for call in plan.calls() {
        words.extend(project_call(call));
    }
    words
}

fn project_failure(error: &bessemer::BessemerError, input: &PlanInput) -> Vec<u32> {
    [vec![failure_status(error, input)], vec![0; 7]].concat()
}

fn failure_status(error: &bessemer::BessemerError, input: &PlanInput) -> u32 {
    assert_eq!(error.operation, "auxiliary triangle call plan");
    match error.class {
        FailureClass::Invalid => {
            assert_failure_detail(
                error,
                "group_count",
                &format!("maximum_groups {}", input.maximum),
            );
            7
        }
        FailureClass::Missing => {
            assert_failure_detail(
                error,
                "group snapshots",
                &format!("{} provided groups", input.groups.len()),
            );
            8
        }
        class => panic!("unexpected failure class {class:?}: {error:?}"),
    }
}

fn assert_failure_detail(error: &bessemer::BessemerError, entity: &str, detail: &str) {
    assert_eq!(error.entity, entity);
    assert!(error.cause.contains(detail));
}

fn read_expected_plan(cursor: &mut &[u8]) -> Vec<u32> {
    let header: [u32; 8] = std::array::from_fn(|_| read_word(cursor));
    assert!(header[0] <= 8 && header[7] <= 3);
    let mut words = header.to_vec();
    for _ in 0..header[7] {
        words.extend((0..11).map(|_| read_word(cursor)));
    }
    words
}

fn assert_plan_case(cursor: &mut &[u8]) -> Vec<u32> {
    let input = read_plan_input(cursor);
    let original = input.clone();
    let expected = read_expected_plan(cursor);
    let result = input
        .controls
        .plan_masked_exceptions(input.context, &input.groups, input.maximum);
    let actual = match result {
        Ok(plan) => project_success(&plan, &input),
        Err(error) => project_failure(&error, &input),
    };
    assert_eq!(actual, expected, "plan input {input:?}");
    assert_eq!(input, original);
    actual
}

pub(super) fn replay_plans(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    assert_eq!(read_word(&mut cursor), 6628);
    let words = (0..6628)
        .flat_map(|_| assert_plan_case(&mut cursor))
        .collect();
    assert!(cursor.is_empty());
    words
}

fn first_nonempty_plan(
    mut cursor: &[u8],
    per_group: bool,
) -> (SimulationAuxTrianglePlan, PlanInput) {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    for _ in 0..count {
        let input = read_plan_input(&mut cursor);
        read_expected_plan(&mut cursor);
        let Ok(plan) = input_plan(&input) else {
            continue;
        };
        if plan.calls().is_empty() || input.controls.per_group_rgb_requested() != per_group {
            continue;
        }
        return (plan, input);
    }
    panic!("expected a successful nonempty plan for source-isolation verification");
}

fn input_plan(input: &PlanInput) -> bessemer::Result<SimulationAuxTrianglePlan> {
    input
        .controls
        .plan_masked_exceptions(input.context, &input.groups, input.maximum)
}

pub(super) fn assert_source_isolation(cursor: &[u8]) {
    for per_group in [false, true] {
        assert_isolated_mode(cursor, per_group);
    }
}

fn assert_isolated_mode(cursor: &[u8], per_group: bool) {
    let (plan, mut input) = first_nonempty_plan(cursor, per_group);
    let snapshot = plan.clone();
    for group in &mut input.groups {
        group.triangle_count = 0;
        group.color = SimulationRgbaColor::from_bits([0; 4]);
        group.payload_tokens = [0; 2];
    }
    input.controls.color = SimulationRgbaColor::from_bits([u32::MAX; 4]);
    assert_eq!(plan, snapshot);
}
