use super::{read_word, take_projection};
use bessemer::{BessemerError, FailureClass, simulation::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostTextFixtureCase {
    pub mode: u32,
    pub controls: SimulationPostTextControls,
    pub runtime: SimulationPostTextRuntime,
    pub maximum: usize,
}

pub fn read_post_text(cursor: &mut &[u8]) -> SimulationPostStyleText {
    let count = read_word(cursor) as usize;
    SimulationPostStyleText::from_payload(take_projection(cursor, count * 2), count * 2).unwrap()
}

fn read_optional_post_text(cursor: &mut &[u8]) -> Option<SimulationPostStyleText> {
    let tag = read_word(cursor);
    assert!(tag <= 1);
    (tag == 1).then(|| read_post_text(cursor))
}

pub fn read_post_text_case(mut cursor: &[u8]) -> PostTextFixtureCase {
    let mode = read_word(&mut cursor);
    assert!(mode <= 2);
    let primary_enabled_raw = read_word(&mut cursor) as i32;
    let seven_lines_enabled_raw = read_word(&mut cursor) as i32;
    let maximum = read_word(&mut cursor) as usize;
    let controls = SimulationPostTextControls {
        primary_enabled_raw,
        seven_lines_enabled_raw,
        stored_text: read_post_text(&mut cursor),
    };
    let runtime = read_post_text_runtime(&mut cursor);
    assert!(cursor.is_empty());
    PostTextFixtureCase {
        mode,
        controls,
        runtime,
        maximum,
    }
}

fn read_post_text_runtime(cursor: &mut &[u8]) -> SimulationPostTextRuntime {
    SimulationPostTextRuntime {
        primary: read_optional_post_text(cursor),
        lines: std::array::from_fn(|_| read_optional_post_text(cursor)),
    }
}

pub fn append_post_words(output: &mut Vec<u8>, words: &[u32]) {
    output.extend(words.iter().flat_map(|word| word.to_le_bytes()));
}

pub fn append_post_frame(output: &mut Vec<u8>, text: &SimulationPostStyleText) {
    output.extend(text.to_bytes());
}

fn post_completion_word(completion: SimulationPostTextCompletion) -> u32 {
    match completion {
        SimulationPostTextCompletion::CompletedEmpty => 0,
        SimulationPostTextCompletion::CompletedText => 1,
    }
}

fn post_assignment_kind(kind: SimulationPostTextAssignmentKind) -> u32 {
    match kind {
        SimulationPostTextAssignmentKind::AssignedPrefix => 1,
        SimulationPostTextAssignmentKind::PreservedPrevious => 2,
    }
}

fn post_runtime_line_error(entity: &str) -> u32 {
    let index = entity
        .strip_prefix("runtime line[")
        .unwrap()
        .strip_suffix("] CString")
        .unwrap()
        .parse::<u32>()
        .unwrap();
    assert!(index < 7);
    14 + index
}

fn post_error_status(case: &PostTextFixtureCase, error: BessemerError) -> Vec<u8> {
    assert!(matches!(
        error.operation,
        "post text assignment" | "post text selection"
    ));
    assert!(error.cause.contains("expected"));
    let status = match (error.class, error.entity.as_str()) {
        (FailureClass::Missing, "previous primary CString") => 11,
        (FailureClass::Missing, "runtime primary CString") => 13,
        (FailureClass::Missing, entity) => post_runtime_line_error(entity),
        (FailureClass::Invalid, "code-unit budget")
            if case.mode != 1 && case.controls.stored_text.character_count() > case.maximum =>
        {
            10
        }
        (FailureClass::Invalid, "code-unit budget") => 12,
        other => panic!("unexpected {other:?}: {error:?}"),
    };
    status.to_le_bytes().to_vec()
}

fn assert_post_coverage(case: &PostTextFixtureCase, coverage: SimulationPostTextCoverage) {
    assert_eq!(
        coverage.retained_wire_code_units,
        case.controls.stored_text.character_count()
    );
    assert_eq!(coverage.maximum_code_units, case.maximum);
    assert!(coverage.local_units_only);
    assert_eq!(coverage.achieved_pattern_error_units, 0);
    assert!(coverage.result_code_units <= coverage.maximum_code_units);
}

fn assert_post_diagnostics(diagnostics: [SimulationPostTextDiagnostic; 4]) {
    assert_eq!(
        diagnostics,
        [
            SimulationPostTextDiagnostic::NativeNotExecuted,
            SimulationPostTextDiagnostic::UnicodeNotValidated,
            SimulationPostTextDiagnostic::LayoutNotModeled,
            SimulationPostTextDiagnostic::RuntimeProductionNotInferred
        ]
    );
}

fn assert_post_assignment(case: &PostTextFixtureCase, assigned: &SimulationPostTextAssignment) {
    assert_eq!(assigned.source_wire_text(), &case.controls.stored_text);
    assert_eq!(assigned.previous_primary(), case.runtime.primary.as_ref());
    let coverage = assigned.coverage();
    assert_post_coverage(case, coverage);
    assert_eq!(
        coverage.result_code_units,
        assigned.primary_after().character_count()
    );
    assert_eq!(
        coverage.required_runtime_strings,
        usize::from(assigned.kind() == SimulationPostTextAssignmentKind::PreservedPrevious)
    );
    assert_post_diagnostics(assigned.diagnostics());
}

fn post_result_header(
    status: u32,
    coverage: SimulationPostTextCoverage,
    completion: SimulationPostTextCompletion,
    kind: u32,
) -> Vec<u8> {
    let mut output = Vec::new();
    append_post_words(
        &mut output,
        &[
            status,
            coverage.result_code_units as u32,
            coverage.required_runtime_strings as u32,
            post_completion_word(completion),
            kind,
        ],
    );
    output
}

fn post_assignment_bytes(
    case: &PostTextFixtureCase,
    assignment: &SimulationPostTextAssignment,
) -> Vec<u8> {
    assert_post_assignment(case, assignment);
    let mut output = post_result_header(
        0,
        assignment.coverage(),
        assignment.completion(),
        post_assignment_kind(assignment.kind()),
    );
    append_post_frame(&mut output, assignment.primary_after());
    output
}

fn assert_post_plan_context(case: &PostTextFixtureCase, plan: &SimulationPostTextPlan) {
    assert_eq!(plan.source_controls(), &case.controls);
    let mut runtime = case.runtime.clone();
    if case.mode == 2 {
        let assignment = plan.assignment().unwrap();
        assert_post_assignment(case, assignment);
        runtime.primary = Some(assignment.primary_after().clone());
    } else {
        assert!(plan.assignment().is_none());
    }
    assert_eq!(plan.runtime_context(), &runtime);
    let coverage = plan.coverage();
    assert_post_coverage(case, coverage);
    let required = usize::from(case.controls.primary_enabled_raw != 0)
        + 7 * usize::from(case.controls.seven_lines_enabled_raw != 0);
    assert_eq!(coverage.required_runtime_strings, required);
    assert_post_diagnostics(plan.diagnostics());
}

fn append_post_selection(output: &mut Vec<u8>, selected: &SimulationPostTextSelection) {
    let source = match selected.origin {
        SimulationPostTextOrigin::EmptyLiteral => 0,
        SimulationPostTextOrigin::RuntimePrimary => 1,
        SimulationPostTextOrigin::RuntimeLine(index) => 2 + u32::from(index),
    };
    append_post_words(output, &[source]);
    append_post_frame(output, &selected.text);
}

fn post_plan_bytes(case: &PostTextFixtureCase, plan: &SimulationPostTextPlan) -> Vec<u8> {
    assert_post_plan_context(case, plan);
    let kind = plan
        .assignment()
        .map_or(0, |assignment| post_assignment_kind(assignment.kind()));
    let mut output = post_result_header(case.mode, plan.coverage(), plan.completion(), kind);
    if let Some(assignment) = plan.assignment() {
        append_post_frame(&mut output, assignment.primary_after());
    }
    append_post_selection(&mut output, plan.primary_selection());
    for line in plan.line_selections() {
        append_post_selection(&mut output, line);
    }
    output
}

pub fn post_operation_bytes(case: &PostTextFixtureCase) -> Vec<u8> {
    let original = case.clone();
    let output = if case.mode == 0 {
        case.controls
            .assign_primary(case.runtime.primary.as_ref(), case.maximum)
            .map(|assignment| post_assignment_bytes(case, &assignment))
    } else {
        let result = if case.mode == 1 {
            case.controls.plan_selection(&case.runtime, case.maximum)
        } else {
            case.controls
                .plan_after_assignment(&case.runtime, case.maximum)
        };
        result.map(|plan| post_plan_bytes(case, &plan))
    };
    assert_eq!(*case, original);
    output.unwrap_or_else(|error| post_error_status(case, error))
}

pub fn replay_post_text_cases(mut cursor: &[u8]) -> Vec<u8> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 18200);
    let mut output = Vec::new();
    for _ in 0..count {
        let length = read_word(&mut cursor) as usize;
        let case = read_post_text_case(take_projection(&mut cursor, length));
        let length = read_word(&mut cursor) as usize;
        let expected = take_projection(&mut cursor, length);
        let actual = post_operation_bytes(&case);
        assert_eq!(actual, expected, "{case:?}");
        output.extend(actual);
    }
    assert!(cursor.is_empty());
    output
}
