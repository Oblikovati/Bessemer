use super::read_word;
use bessemer::{FailureClass, simulation::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreferenceFixtureCase {
    pub words: [u32; 16],
    pub original: SimulationRgbaColor,
    pub runtime: SimulationStylePreferenceColorRuntime,
    pub source: Option<Vec<u8>>,
}
pub fn preference_runtime(words: [u32; 3]) -> SimulationStylePreferenceColorRuntime {
    assert!(words[0] <= 2 && words[1] <= 1);
    SimulationStylePreferenceColorRuntime {
        object_pointer_present: match words[0] {
            0 => None,
            1 => Some(false),
            _ => Some(true),
        },
        pointed_type_raw: (words[1] == 1).then_some(words[2] as i32),
    }
}
fn preference_source(words: &[u32; 16]) -> Option<Vec<u8>> {
    assert!(words[8] <= 1);
    if words[8] == 0 {
        return None;
    }
    let mut source = vec![words[12] as u8; words[9] as usize];
    patch_preference_word(&mut source, words[13] as usize, words[10]);
    if words[14] == 1 {
        patch_preference_word(&mut source, words[15] as usize, words[11]);
    }
    Some(source)
}
fn patch_preference_word(source: &mut [u8], offset: usize, word: u32) {
    for (index, value) in word.to_le_bytes().into_iter().enumerate() {
        if let Some(byte) = source.get_mut(offset + index) {
            *byte = value;
        }
    }
}
pub fn preference_case_from_words(words: [u32; 16]) -> PreferenceFixtureCase {
    PreferenceFixtureCase {
        original: SimulationRgbaColor::from_bits(words[1..5].try_into().unwrap()),
        runtime: preference_runtime(words[5..8].try_into().unwrap()),
        source: preference_source(&words),
        words,
    }
}
pub fn read_preference_case(cursor: &mut &[u8]) -> PreferenceFixtureCase {
    preference_case_from_words(std::array::from_fn(|_| read_word(cursor)))
}
fn assert_preference_route(
    case: &PreferenceFixtureCase,
    route: SimulationStylePreferenceColorRoute,
) {
    assert_eq!(route.style_member(), case.words[0]);
    assert_eq!(
        route.field_path(),
        format!("style.color_{:x}", case.words[0])
    );
    assert_eq!(route.packed_preference_offset(), case.words[13] as usize);
    assert_eq!(
        route.alpha_preference_offset(),
        (case.words[14] == 1).then_some(case.words[15] as usize)
    );
    let last = if case.words[14] == 1 {
        case.words[13].max(case.words[15])
    } else {
        case.words[13]
    };
    assert_eq!(route.required_preference_bytes(), last as usize + 4);
}
fn preference_error_status(error: bessemer::BessemerError) -> u32 {
    assert_eq!(error.operation, "style preference color");
    assert!(error.cause.contains("expected"));
    let (status, expected) = match error.entity.as_str() {
        "style member" => (10, FailureClass::Unsupported),
        "style2D0 pointer" => (11, FailureClass::Missing),
        "pointed type10" => (12, FailureClass::Missing),
        "preference bytes" if error.class == FailureClass::Missing => (13, FailureClass::Missing),
        "preference bytes" => (14, FailureClass::Invalid),
        other => panic!("unexpected {other}: {error:?}"),
    };
    assert_eq!(error.class, expected);
    status
}
fn assert_preference_plan(case: &PreferenceFixtureCase, plan: &SimulationStylePreferenceColorPlan) {
    assert_eq!(plan.original(), case.original);
    assert_eq!(plan.runtime_context(), case.runtime);
    assert_eq!(plan.route().style_member(), case.words[0]);
    assert_eq!(
        plan.preference_source_length(),
        case.source.as_ref().map(Vec::len)
    );
    assert_preference_coverage(plan);
    assert_preference_diagnostics(plan);
}
fn assert_preference_coverage(plan: &SimulationStylePreferenceColorPlan) {
    let history = plan.history();
    let applied = usize::from(history.packed_preference_word.is_some());
    let alpha = usize::from(history.alpha_preference_word.is_some());
    let coverage = plan.coverage();
    assert_eq!(coverage.preference_words_read, applied + alpha);
    assert_eq!(coverage.rgb_words_projected, 3 * applied);
    assert_eq!(coverage.alpha_words_replaced, alpha);
    assert!(coverage.legend_call_precedes_guard && coverage.exact_nearest_even_model);
    assert!(!coverage.wire_edited);
    assert_eq!(coverage.achieved_pattern_error_bits, 0);
    let completion = if applied == 1 {
        SimulationStylePreferenceColorCompletion::CompletedProjection
    } else {
        SimulationStylePreferenceColorCompletion::CompletedSkipped
    };
    assert_eq!(plan.completion(), completion);
}
fn assert_preference_diagnostics(plan: &SimulationStylePreferenceColorPlan) {
    assert_eq!(
        plan.diagnostics(),
        [
            SimulationStylePreferenceColorDiagnostic::NativeNotExecuted,
            SimulationStylePreferenceColorDiagnostic::LegendCallNotModeled,
            SimulationStylePreferenceColorDiagnostic::WholeSetterNotModeled,
            SimulationStylePreferenceColorDiagnostic::NativeFloatingEnvironmentUnknown
        ]
    );
}
fn preference_success_words(
    case: &PreferenceFixtureCase,
    plan: &SimulationStylePreferenceColorPlan,
) -> Vec<u32> {
    assert_preference_plan(case, plan);
    let history = plan.history();
    let applied = u32::from(history.packed_preference_word.is_some());
    let (alpha_origin, alpha_offset) = match history.alpha_origin {
        SimulationStylePreferenceAlphaOrigin::Original => (0, 0),
        SimulationStylePreferenceAlphaOrigin::PreferenceWord { byte_offset } => {
            (1, byte_offset as u32)
        }
    };
    let mut words = vec![applied];
    words.extend(plan.original().bits());
    words.extend(plan.projected().bits());
    words.extend(preference_history_words(plan, alpha_origin, alpha_offset));
    assert_eq!(words.len(), 18);
    words
}
pub fn preference_result_words(case: &PreferenceFixtureCase) -> Vec<u32> {
    let original = case.clone();
    let result =
        SimulationStylePreferenceColorRoute::from_style_member(case.words[0]).and_then(|route| {
            assert_preference_route(case, route);
            case.original.project_style_preference_nearest_even(
                route,
                case.source.as_deref(),
                case.runtime,
            )
        });
    let words = match result {
        Ok(plan) => preference_success_words(case, &plan),
        Err(error) => [vec![preference_error_status(error)], vec![0; 17]].concat(),
    };
    assert_eq!(case, &original);
    words
}
pub fn replay_preference_cases(mut cursor: &[u8]) -> Vec<u32> {
    assert_eq!(read_word(&mut cursor), 1);
    let count = read_word(&mut cursor);
    assert_eq!(count, 17400);
    let mut output = Vec::new();
    for _ in 0..count {
        let case = read_preference_case(&mut cursor);
        let expected: [u32; 18] = std::array::from_fn(|_| read_word(&mut cursor));
        let actual = preference_result_words(&case);
        assert_eq!(actual, expected, "{case:?}");
        output.extend(actual);
    }
    assert!(cursor.is_empty());
    output
}

fn preference_history_words(
    plan: &SimulationStylePreferenceColorPlan,
    alpha_origin: u32,
    alpha_offset: u32,
) -> [u32; 9] {
    let history = plan.history();
    [
        u32::from(history.packed_preference_word.is_some()),
        history.packed_preference_word.unwrap_or(0),
        u32::from(history.alpha_preference_word.is_some()),
        history.alpha_preference_word.unwrap_or(0),
        alpha_origin,
        alpha_offset,
        u32::from(plan.preference_source_length().is_some()),
        plan.preference_source_length().unwrap_or(0) as u32,
        plan.route().required_preference_bytes() as u32,
    ]
}
