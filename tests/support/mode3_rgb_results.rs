use super::{case_support::Mode3FixtureCase, read_word};
use bessemer::{FailureClass, simulation::*};

fn mode3_stage(stage: SimulationMode3RgbStage) -> u32 {
    match stage {
        SimulationMode3RgbStage::GroupHidden => 0,
        SimulationMode3RgbStage::GroupEmpty => 1,
        SimulationMode3RgbStage::PrimaryFiltered => 2,
        SimulationMode3RgbStage::ArrayRequested => 3,
    }
}
fn mode3_origin(origin: Option<SimulationMode3RgbOrigin>) -> u32 {
    match origin {
        None => 0,
        Some(SimulationMode3RgbOrigin::Stored1dc) => 1,
        Some(SimulationMode3RgbOrigin::GroupFirst) => 2,
        Some(SimulationMode3RgbOrigin::GroupOther) => 3,
        other => panic!("unexpected selected origin {other:?}"),
    }
}
fn assert_mode3_plan(case: &Mode3FixtureCase, plan: &SimulationMode3RgbPlan) {
    assert_eq!(plan.source_inputs(), case.inputs);
    assert_eq!(plan.source_group(), case.group);
    assert_eq!(Some(plan.copied_stored_rgba()), case.inputs.color_1dc);
    let origin = if case.inputs.mode_1b8_raw == Some(3) {
        SimulationMode3RgbOrigin::Stored1dc
    } else {
        SimulationMode3RgbOrigin::DefaultHalf
    };
    assert_eq!(plan.candidate_origin(), origin);
    assert_mode3_plan_coverage(plan);
    let completed = if plan.array_request_rgb().is_some() {
        SimulationMode3RgbCompletion::CompletedSelection
    } else {
        SimulationMode3RgbCompletion::CompletedEmpty
    };
    assert_eq!(plan.completion(), completed);
    assert_mode3_diagnostics(plan.diagnostics());
}
fn assert_mode3_diagnostics(actual: [SimulationMode3RgbDiagnostic; 4]) {
    assert_eq!(
        actual,
        [
            SimulationMode3RgbDiagnostic::NativeNotExecuted,
            SimulationMode3RgbDiagnostic::WholeCallerUnmodeled,
            SimulationMode3RgbDiagnostic::RendererStateUnmodeled,
            SimulationMode3RgbDiagnostic::AllocatorAndGeometryUnmodeled
        ]
    );
}
fn mode3_group_output(case: &Mode3FixtureCase, plan: &SimulationMode3RgbPlan) -> Vec<u32> {
    assert_mode3_plan(case, plan);
    let mut words = vec![mode3_stage(plan.stage())];
    words.extend(plan.candidate_rgb_bits());
    words.extend(plan.copied_stored_rgba().bits());
    let copied = plan.copied_group_colors();
    words.push(u32::from(copied.is_some()));
    words.extend(copied.map_or(vec![0; 8], |colors| {
        [colors[0].bits(), colors[1].bits()].concat()
    }));
    words.push(u32::from(plan.selected_rgb_bits().is_some()));
    words.extend(plan.selected_rgb_bits().unwrap_or([0; 3]));
    words.push(mode3_origin(plan.selected_origin()));
    assert_eq!(words.len(), 22);
    words
}
fn mode3_group_error(error: bessemer::BessemerError) -> u32 {
    assert_eq!(error.operation, "mode3 RGB operation");
    assert_eq!(error.class, FailureClass::Missing);
    assert!(error.cause.contains("None") && error.cause.contains("expected"));
    match error.entity.as_str() {
        "stored1B8" => 10,
        "stored1DC" => 11,
        "group status" => 12,
        "item test" => 13,
        "first group color" => 14,
        "other group color" => 15,
        "face" => 16,
        "parent170" => 17,
        "group selector" => 18,
        other => panic!("unexpected dependency {other}: {error:?}"),
    }
}
fn mode3_array_error(error: bessemer::BessemerError) -> u32 {
    assert_eq!(error.operation, "mode3 RGB operation");
    assert!(error.cause.contains("expected"));
    let (status, class) = match error.entity.as_str() {
        "array capacity" => (30, FailureClass::Missing),
        "existing destination" => (31, FailureClass::Missing),
        "growth result" if error.class == FailureClass::Missing => (32, FailureClass::Missing),
        "growth result" => (33, FailureClass::Invalid),
        "grown destination" => (34, FailureClass::Missing),
        "destination prefix" => (35, FailureClass::Invalid),
        other => panic!("unexpected dependency {other}: {error:?}"),
    };
    assert_eq!(error.class, class);
    status
}
fn assert_mode3_update(
    case: &Mode3FixtureCase,
    plan: &SimulationMode3RgbPlan,
    update: &SimulationMode3RgbUpdate,
) {
    assert_eq!(update.source_plan(), plan);
    assert_eq!(update.source_context(), &case.array);
    let mut expected = plan.coverage();
    expected.written_words = 3 * usize::from(update.destination().is_some());
    expected.destination_bounds_checked = update.destination().is_some();
    assert_eq!(update.coverage(), expected);
    let completion = if update.destination().is_some() {
        SimulationMode3RgbCompletion::CompletedArrayWrite
    } else {
        SimulationMode3RgbCompletion::CompletedEmpty
    };
    assert_eq!(update.completion(), completion);
    assert_mode3_diagnostics(update.diagnostics());
    if let Some(history) = update.history() {
        assert_mode3_history(case, plan, update, history);
    } else {
        assert!(update.destination().is_none());
    }
}
fn assert_mode3_history(
    case: &Mode3FixtureCase,
    plan: &SimulationMode3RgbPlan,
    update: &SimulationMode3RgbUpdate,
    history: SimulationMode3RgbWriteHistory,
) {
    let source = match history.parent {
        SimulationMode3RgbArrayParent::Existing => case.array.existing.as_ref().unwrap(),
        SimulationMode3RgbArrayParent::AfterGrowth => case.array.after_growth.as_ref().unwrap(),
    };
    let destination = update.destination().unwrap();
    assert_eq!(history.index, 5);
    assert_eq!(history.before, source.triples()[5]);
    assert_eq!(Some(history.after), plan.array_request_rgb());
    assert_eq!(destination.maximum_triples(), source.maximum_triples());
    let mut expected = source.triples().to_vec();
    expected[5] = history.after;
    assert_eq!(destination.triples(), expected);
}
fn mode3_array_output(
    case: &Mode3FixtureCase,
    plan: &SimulationMode3RgbPlan,
    update: &SimulationMode3RgbUpdate,
) -> Vec<u32> {
    assert_mode3_update(case, plan, update);
    let Some(destination) = update.destination() else {
        return vec![0; 32];
    };
    let history = update.history().unwrap();
    let parent = match history.parent {
        SimulationMode3RgbArrayParent::Existing => 1,
        SimulationMode3RgbArrayParent::AfterGrowth => 2,
    };
    let mut words = vec![parent, destination.triples().len() as u32];
    words.extend(destination.triples().iter().flatten().copied());
    words.resize(26, 0);
    words.extend(history.before);
    words.extend(history.after);
    assert_eq!(words.len(), 32);
    words
}
pub fn mode3_result_words(case: &Mode3FixtureCase) -> Vec<u32> {
    let original = case.clone();
    let result = match case.inputs.plan_group_rgb(case.group) {
        Err(error) => [vec![mode3_group_error(error)], vec![0; 53]].concat(),
        Ok(plan) => {
            let mut words = mode3_group_output(case, &plan);
            match plan.apply_local_array(&case.array) {
                Ok(update) => words.extend(mode3_array_output(case, &plan, &update)),
                Err(error) => words.extend([vec![mode3_array_error(error)], vec![0; 31]].concat()),
            };
            words
        }
    };
    assert_eq!(case, &original);
    result
}
pub fn replay_mode3_cases(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 15342);
    let mut output = Vec::new();
    for _ in 0..count {
        let case = super::case_support::read_mode3_case(&mut cursor);
        let expected: [u32; 54] = std::array::from_fn(|_| read_word(&mut cursor));
        let actual = mode3_result_words(&case);
        assert_eq!(actual, expected, "{case:?}");
        output.extend(actual);
    }
    assert!(cursor.is_empty());
    output
}

fn assert_mode3_plan_coverage(plan: &SimulationMode3RgbPlan) {
    let coverage = plan.coverage();
    assert_eq!(coverage.stored_words_copied, 4);
    assert_eq!(
        coverage.group_words_copied,
        8 * usize::from(plan.copied_group_colors().is_some())
    );
    assert_eq!(
        coverage.selected_words,
        3 * usize::from(plan.selected_rgb_bits().is_some())
    );
    assert_eq!(coverage.written_words, 0);
    assert!(!coverage.destination_bounds_checked && !coverage.geometry_validated);
    assert!(coverage.exact_word_checks);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
}
