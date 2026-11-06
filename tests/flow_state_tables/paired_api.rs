use super::{fixture, state_carrier};
use bessemer::flow::*;
#[path = "paired_contracts.rs"]
mod contracts;
#[path = "paired_edit.rs"]
mod edit;
#[path = "paired_replay.rs"]
mod replay;
fn options() -> FlowPairedStateOptions {
    FlowPairedStateOptions {
        maximum_payload_bytes: 1_000_000,
        maximum_index_words: 100_000,
        maximum_wide_words: 100_000,
    }
}
fn open(raw: &[u8], selected: FlowPairedStateOptions) -> bessemer::Result<FlowPairedState> {
    let source = state_carrier::carrier(417, raw);
    let id = source.latest(417).unwrap().unwrap().id;
    source.record_417_paired_state(&id, selected)
}
fn cases() -> Vec<fixture::StateCase> {
    fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 417)
        .collect()
}
fn expected_field<'a>(
    case: &'a fixture::StateCase,
    group: usize,
    role: &str,
) -> &'a crate::stored_fields::StoredField {
    let path = format!("[\"groups\",{group},\"{role}\"]");
    case.fields.iter().find(|field| field.path == path).unwrap()
}
#[test]
fn paired_states_match_every_source_scope_count_raw_value_and_optional_native_quotient() {
    let cases = cases();
    assert_eq!(cases.len(), 237);
    for case in cases {
        check_case(&case);
    }
}
fn check_case(case: &fixture::StateCase) -> FlowPairedState {
    let state = open(&case.raw, options()).unwrap();
    assert_eq!(state.version(), 1);
    assert_eq!(state.decoded_bytes(), case.raw);
    assert_eq!(state.options(), options());
    assert_eq!(state.record().ordinal(), 1);
    assert_eq!(
        state.source().as_bytes(),
        state_carrier::carrier(417, &case.raw).as_bytes()
    );
    assert_eq!(state.groups().len(), 2);
    for group in state.groups() {
        check_group(case, group);
    }
    assert!(state.group(2).is_none());
    assert!(state.group(usize::MAX).is_none());
    check_coverage(&state);
    state
}
fn check_group(case: &fixture::StateCase, group: FlowPairedStateGroup<'_>) {
    let scope = check_group_scope(case, group);
    let ordinal = group.ordinal();
    crate::state_arrays::check_words(
        group.indices(),
        expected_field(case, ordinal, "index_patterns"),
    );
    check_wides(case, group);
    let quotient = case
        .quotients
        .iter()
        .find(|(path, _)| path == scope)
        .unwrap()
        .1;
    assert_eq!(
        group.native_quotient(),
        quotient.map(|value| value as usize)
    );
}
fn check_group_scope<'a>(
    case: &'a fixture::StateCase,
    group: FlowPairedStateGroup<'_>,
) -> &'a String {
    let ordinal = group.ordinal();
    let scope = case
        .scopes
        .iter()
        .find(|(path, _, _)| path == &format!("[\"groups\",{ordinal}]"))
        .unwrap();
    assert_eq!(
        group.payload_interval(),
        (scope.1 as usize, scope.2 as usize)
    );
    &scope.0
}
fn check_wides(case: &fixture::StateCase, group: FlowPairedStateGroup<'_>) {
    let ordinal = group.ordinal();
    let Some(array) = group.wides() else {
        assert_eq!(group.indices().word_count(), 0);
        return;
    };
    let field = expected_field(case, ordinal, "wide_patterns");
    assert_eq!(array.as_bytes(), field.raw);
    assert_eq!(array.word_count(), field.count as usize);
    let origin = field.offset as usize;
    assert_eq!(array.payload_interval(), (origin, origin + field.raw.len()));
    let (chunks, remainder) = field.raw.as_chunks::<8>();
    assert!(remainder.is_empty());
    for (index, raw) in chunks.iter().enumerate() {
        assert_eq!(array.raw_bits(index), Some(u64::from_le_bytes(*raw)));
    }
    assert_eq!(array.raw_bits(array.word_count()), None);
    assert_eq!(array.raw_bits(usize::MAX), None);
}
fn check_coverage(state: &FlowPairedState) {
    let coverage = state.coverage();
    assert_eq!(coverage.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(coverage.payload_bytes, state.decoded_bytes().len());
    assert_eq!(coverage.groups, 2);
    assert_eq!(coverage.achieved_error, 0);
    check_array_totals(state, coverage);
    check_diagnostics(state);
}
fn check_array_totals(state: &FlowPairedState, coverage: FlowPairedStateCoverage) {
    let indices: usize = state
        .groups()
        .map(|group| group.indices().word_count())
        .sum();
    let wides: usize = state
        .groups()
        .filter_map(|group| group.wides())
        .map(|wide| wide.word_count())
        .sum();
    let present = state
        .groups()
        .filter(|group| group.wides().is_some())
        .count();
    assert_eq!(coverage.index_words, indices);
    assert_eq!(coverage.wide_words, wides);
    assert_eq!(coverage.present_wide_arrays, present);
}
fn check_diagnostics(state: &FlowPairedState) {
    assert_eq!(
        state.diagnostics(),
        &[
            FlowPairedStateDiagnostic::EngineeringRolesNotResolved,
            FlowPairedStateDiagnostic::NativeRuntimeNotReproduced,
            FlowPairedStateDiagnostic::NativeApplicationNotChecked,
        ]
    );
}
