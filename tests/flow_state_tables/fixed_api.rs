use super::{fixture, state_carrier};
use bessemer::flow::*;
#[path = "fixed_contracts.rs"]
mod contracts;
#[path = "fixed_edit.rs"]
mod edit;
#[path = "fixed_replay.rs"]
mod replay;
const MEMBERS: [FlowFixedStateWideMember; 2] = [
    FlowFixedStateWideMember::Member600,
    FlowFixedStateWideMember::Discarded,
];
fn options() -> FlowFixedStateOptions {
    FlowFixedStateOptions {
        maximum_payload_bytes: 1_000_000,
        maximum_rows: 10_000,
    }
}
fn open(raw: &[u8], selected: FlowFixedStateOptions) -> bessemer::Result<FlowFixedState> {
    let source = state_carrier::carrier(139, raw);
    let id = source.latest(139).unwrap().unwrap().id;
    source.record_139_fixed_state(&id, selected)
}
fn cases() -> Vec<fixture::StateCase> {
    fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 139)
        .collect()
}
fn expected_field<'a>(
    case: &'a fixture::StateCase,
    row: usize,
    role: &str,
) -> &'a crate::stored_fields::StoredField {
    let path = format!("[\"items\",{row},\"{role}\"]");
    case.fields.iter().find(|field| field.path == path).unwrap()
}
fn signed(raw: &[u8]) -> i32 {
    i32::from_le_bytes(raw.try_into().unwrap())
}
fn wide(raw: &[u8]) -> u64 {
    u64::from_le_bytes(raw.try_into().unwrap())
}
fn member_role(member: FlowFixedStateWideMember) -> &'static str {
    match member {
        FlowFixedStateWideMember::Member600 => "member_600_pattern",
        FlowFixedStateWideMember::Discarded => "discarded_wide_pattern",
    }
}
#[test]
fn fixed_states_match_every_complete_source_row_scope_field_value_absence_and_version() {
    let cases = cases();
    assert_eq!(cases.len(), 562);
    for case in cases {
        check_case(&case);
    }
}
fn check_case(case: &fixture::StateCase) -> FlowFixedState {
    let state = open(&case.raw, options()).unwrap();
    check_metadata(case, &state);
    assert_eq!(state.rows().len(), state.row_count());
    for row in state.rows() {
        check_row(case, row);
    }
    assert!(state.row(state.row_count()).is_none());
    assert!(state.row(usize::MAX).is_none());
    check_coverage(case, &state);
    state
}
fn check_metadata(case: &fixture::StateCase, state: &FlowFixedState) {
    assert_eq!(state.version(), case.version);
    assert_eq!(state.decoded_bytes(), case.raw);
    assert_eq!(state.options(), options());
    assert_eq!(state.record().ordinal(), 1);
    assert_eq!(
        state.source().as_bytes(),
        state_carrier::carrier(139, &case.raw).as_bytes()
    );
    let count = case
        .fields
        .iter()
        .find(|field| field.path == "[\"count\"]")
        .map(|field| signed(&field.raw) as u32);
    assert_eq!(state.declared_count(), count);
    assert_eq!(state.row_count(), count.unwrap_or(0) as usize);
}
fn check_row(case: &fixture::StateCase, row: FlowFixedStateRow<'_>) {
    let ordinal = row.ordinal();
    let path = format!("[\"items\",{ordinal}]");
    let scope = case.scopes.iter().find(|(key, _, _)| key == &path).unwrap();
    assert_eq!(row.payload_interval(), (scope.1 as usize, scope.2 as usize));
    for member in MEMBERS {
        let field = expected_field(case, ordinal, member_role(member));
        assert_eq!(row.wide_member(member), wide(&field.raw));
        assert_eq!(
            field.offset as usize,
            row.payload_interval().0 + member.relative_offset()
        );
    }
    check_arrays(case, row);
    check_final(case, row);
}
fn check_arrays(case: &fixture::StateCase, row: FlowFixedStateRow<'_>) {
    assert_eq!(row.settings_wides().is_some(), case.version >= 2);
    assert_eq!(row.settings_words().is_some(), case.version >= 2);
    if let Some(array) = row.settings_wides() {
        crate::state_arrays::check_wides(
            array,
            expected_field(case, row.ordinal(), "settings_wide_patterns"),
        );
        assert_eq!(array.word_count(), 320);
    }
    if let Some(array) = row.settings_words() {
        crate::state_arrays::check_words(
            array,
            expected_field(case, row.ordinal(), "settings_i32_patterns"),
        );
        assert_eq!(array.word_count(), 100);
    }
}
fn check_final(case: &fixture::StateCase, row: FlowFixedStateRow<'_>) {
    let expected = if case.version >= 3 {
        Some(signed(
            &expected_field(case, row.ordinal(), "final_member_e0_i32").raw,
        ))
    } else {
        None
    };
    assert_eq!(row.final_member(), expected);
}
fn check_coverage(case: &fixture::StateCase, state: &FlowFixedState) {
    let coverage = state.coverage();
    assert_eq!(coverage.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(coverage.payload_bytes, case.raw.len());
    assert_eq!(coverage.declared_count, state.declared_count());
    assert_eq!(coverage.rows, state.row_count());
    let width = match case.version {
        0 => None,
        1 => Some(16),
        2 => Some(2976),
        _ => Some(2980),
    };
    assert_eq!(coverage.row_bytes, width);
    assert_eq!(
        coverage.stored_values,
        case.fields.iter().map(|field| field.count as usize).sum()
    );
    assert_eq!(coverage.achieved_error, 0);
    check_totals(case, coverage);
    check_diagnostics(state);
}
fn check_totals(case: &fixture::StateCase, coverage: FlowFixedStateCoverage) {
    let wides: usize = case
        .fields
        .iter()
        .filter(|field| field.encoding == 3)
        .map(|field| field.count as usize)
        .sum();
    let words: usize = case
        .fields
        .iter()
        .filter(|field| field.path.ends_with("\"settings_i32_patterns\"]"))
        .map(|field| field.count as usize)
        .sum();
    let finals = case
        .fields
        .iter()
        .filter(|field| field.path.ends_with("\"final_member_e0_i32\"]"))
        .count();
    assert_eq!(coverage.wide_patterns, wides);
    assert_eq!(coverage.settings_words, words);
    assert_eq!(coverage.final_members, finals);
}
fn check_diagnostics(state: &FlowFixedState) {
    assert_eq!(
        state.diagnostics(),
        &[
            FlowFixedStateDiagnostic::NativeSettingsApplicationNotReproduced,
            FlowFixedStateDiagnostic::EngineeringRolesNotResolved,
            FlowFixedStateDiagnostic::NativeApplicationNotChecked,
        ]
    );
}
