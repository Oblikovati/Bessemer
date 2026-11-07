use super::{fixture, state_carrier};
use bessemer::flow::*;
#[path = "context_contracts.rs"]
mod contracts;
#[path = "context_edit.rs"]
mod edit;
#[path = "context_replay.rs"]
mod replay;
fn options(context: Option<i32>) -> FlowContextStateOptions {
    FlowContextStateOptions {
        index_count: context.map(|count| FlowSimpleLegacyCount::try_from(count as u32).unwrap()),
        maximum_payload_bytes: 1_000_000,
        maximum_index_words: 100_000,
    }
}
fn open(raw: &[u8], options: FlowContextStateOptions) -> bessemer::Result<FlowContextState> {
    let source = state_carrier::carrier(138, raw);
    let id = source.latest(138).unwrap().unwrap().id;
    source.record_138_context_state(&id, options)
}
fn cases() -> Vec<fixture::StateCase> {
    fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 138)
        .collect()
}
fn expected_field<'a>(
    case: &'a fixture::StateCase,
    path: &str,
) -> &'a crate::stored_fields::StoredField {
    case.fields.iter().find(|field| field.path == path).unwrap()
}
fn signed(raw: &[u8]) -> i32 {
    i32::from_le_bytes(raw.try_into().unwrap())
}
#[test]
fn all_complete_context_states_match_source_values_offsets_versions_and_applied_assumptions() {
    let cases = cases();
    assert_eq!(cases.len(), 86);
    for case in cases {
        check_case(&case);
    }
}
fn check_case(case: &fixture::StateCase) -> FlowContextState {
    let state = open(&case.raw, options(case.context)).unwrap();
    assert_eq!(state.decoded_bytes(), case.raw);
    assert_eq!(state.version(), case.version);
    assert_eq!(state.options(), options(case.context));
    assert_eq!(state.record().ordinal(), 1);
    assert_eq!(
        state.source().as_bytes(),
        state_carrier::carrier(138, &case.raw).as_bytes()
    );
    let objects = signed(&expected_field(case, "[\"object_count\"]").raw) as u32;
    assert_eq!(state.object_count(), objects);
    check_indices(&state, case);
    check_tail(&state, case);
    check_coverage(&state);
    state
}
fn check_indices(state: &FlowContextState, case: &fixture::StateCase) {
    let field = expected_field(case, "[\"index_patterns\"]");
    crate::state_arrays::check_words(state.indices(), field);
}
fn check_tail(state: &FlowContextState, case: &fixture::StateCase) {
    let field = case
        .fields
        .iter()
        .find(|field| field.path == "[\"tail_i32\"]");
    assert_eq!(state.tail(), field.map(|field| signed(&field.raw)));
}
fn check_coverage(state: &FlowContextState) {
    let coverage = state.coverage();
    assert_eq!(coverage.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(coverage.payload_bytes, state.decoded_bytes().len());
    assert_eq!(coverage.object_count, state.object_count());
    assert_eq!(coverage.index_words, state.indices().word_count());
    assert_eq!(coverage.tail_present, state.tail().is_some());
    assert_eq!(coverage.achieved_error, 0);
    let applied = state.version() < 5 && state.object_count() > 0;
    assert_eq!(state.context_is_applied(), applied);
    assert_eq!(
        coverage.applied_index_count,
        state.options().index_count.filter(|_| applied)
    );
    check_diagnostics(state, applied);
}
fn check_diagnostics(state: &FlowContextState, applied: bool) {
    let diagnostics = state.diagnostics();
    assert_eq!(
        diagnostics.contains(&FlowContextStateDiagnostic::NativeIndexCountAssumed),
        applied
    );
    for exclusion in [
        FlowContextStateDiagnostic::NativeRuntimeNotReproduced,
        FlowContextStateDiagnostic::EngineeringRolesNotResolved,
        FlowContextStateDiagnostic::NativeApplicationNotChecked,
    ] {
        assert!(diagnostics.contains(&exclusion));
    }
}
