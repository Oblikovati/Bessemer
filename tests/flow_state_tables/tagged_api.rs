use super::{fixture, state_carrier};
use bessemer::flow::*;
#[path = "tagged_contracts.rs"]
mod contracts;
#[path = "tagged_edit.rs"]
mod edit;
#[path = "tagged_replay.rs"]
mod replay;
use std::collections::{BTreeMap, BTreeSet};
fn options() -> FlowTaggedStateOptions {
    FlowTaggedStateOptions {
        maximum_payload_bytes: 1_000_000,
        maximum_rows: 100_000,
        maximum_wide_words: 1_000_000,
    }
}
fn open(raw: &[u8], selected: FlowTaggedStateOptions) -> bessemer::Result<FlowTaggedState> {
    let source = state_carrier::carrier(129, raw);
    let id = source.latest(129).unwrap().unwrap().id;
    source.record_129_tagged_state(&id, selected)
}
fn cases() -> Vec<fixture::StateCase> {
    fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 129)
        .collect()
}
fn expected_field<'a>(
    case: &'a fixture::StateCase,
    path: &str,
) -> &'a crate::stored_fields::StoredField {
    case.fields.iter().find(|field| field.path == path).unwrap()
}
fn row_field<'a>(
    case: &'a fixture::StateCase,
    row: usize,
    role: &str,
) -> &'a crate::stored_fields::StoredField {
    let path = format!("[\"rows\",{row},\"{role}\"]");
    expected_field(case, &path)
}
fn signed(raw: &[u8]) -> i32 {
    i32::from_le_bytes(raw.try_into().unwrap())
}
fn selected_rows(state: &FlowTaggedState) -> BTreeSet<usize> {
    if state.row_count() <= 1024 {
        return (0..state.row_count()).collect();
    }
    let mut shapes = BTreeMap::new();
    for row in state.rows() {
        shapes
            .entry((row.tag(), row.declared_width()))
            .or_insert(row.ordinal());
    }
    let mut rows: BTreeSet<_> = shapes.into_values().collect();
    rows.extend([0, state.row_count() / 2, state.row_count() - 1]);
    rows
}
fn selected_elements(count: usize, native: bool) -> BTreeSet<usize> {
    if !native {
        return (0..count).collect();
    }
    [0, count / 2, count.saturating_sub(1)]
        .into_iter()
        .filter(|index| *index < count)
        .collect()
}
#[test]
fn tagged_states_match_every_source_row_body_tag_byte_width_value_and_tail() {
    let cases = cases();
    assert_eq!(cases.len(), 773);
    for case in cases {
        check_case(&case);
    }
}
fn check_case(case: &fixture::StateCase) -> FlowTaggedState {
    let state = open(&case.raw, options()).unwrap();
    check_metadata(case, &state);
    for row in state.rows() {
        check_row(case, row);
    }
    assert!(state.row(state.row_count()).is_none());
    assert!(state.row(usize::MAX).is_none());
    check_coverage(case, &state);
    state
}
fn check_metadata(case: &fixture::StateCase, state: &FlowTaggedState) {
    assert_eq!(state.version(), 9);
    assert_eq!(state.decoded_bytes(), case.raw);
    assert_eq!(state.options(), options());
    assert_eq!(state.record().ordinal(), 1);
    assert_eq!(
        state.source().as_bytes(),
        state_carrier::carrier(129, &case.raw).as_bytes()
    );
    let count = signed(&expected_field(case, "[\"count\"]").raw) as usize;
    assert_eq!(state.row_count(), count);
    assert_eq!(state.rows().len(), count);
    assert_eq!(
        state.tags_bytes(),
        expected_field(case, "[\"tags_raw\"]").raw
    );
    check_tail(case, state);
}
fn check_tail(case: &fixture::StateCase, state: &FlowTaggedState) {
    let tail = case
        .fields
        .iter()
        .find(|field| field.path == "[\"tail_i32\"]")
        .map(|field| signed(&field.raw));
    assert_eq!(state.tail(), tail);
}
fn check_row(case: &fixture::StateCase, row: FlowTaggedStateRow<'_>) {
    let ordinal = row.ordinal();
    let path = format!("[\"rows\",{ordinal}]");
    let scope = case.scopes.iter().find(|(key, _, _)| key == &path).unwrap();
    assert_eq!(row.payload_interval(), (scope.1 as usize, scope.2 as usize));
    assert_eq!(
        row.tag(),
        expected_field(case, "[\"tags_raw\"]").raw[ordinal]
    );
    assert_eq!(
        row.tag_offset(),
        expected_field(case, "[\"tags_raw\"]").offset as usize + ordinal
    );
    assert_eq!(row.values().is_none(), row.tag() == 0);
    check_row_values(case, row);
}
fn check_row_values(case: &fixture::StateCase, row: FlowTaggedStateRow<'_>) {
    let ordinal = row.ordinal();
    let width_path = format!("[\"rows\",{ordinal},\"width\"]");
    let width = case
        .fields
        .iter()
        .find(|field| field.path == width_path)
        .map(|field| signed(&field.raw) as usize);
    assert_eq!(row.declared_width(), width);
    if let Some(array) = row.values() {
        crate::state_arrays::check_wides(array, row_field(case, ordinal, "value_patterns"));
    } else {
        assert_eq!(row.payload_interval().0, row.payload_interval().1);
    }
}
fn check_coverage(case: &fixture::StateCase, state: &FlowTaggedState) {
    let coverage = state.coverage();
    assert_eq!(coverage.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(coverage.payload_bytes, case.raw.len());
    assert_eq!(coverage.rows, state.row_count());
    assert_eq!(coverage.tag_bytes, state.row_count());
    assert_eq!(coverage.tail_present, state.tail().is_some());
    let present = state.rows().filter(|row| row.values().is_some()).count();
    assert_eq!(coverage.present_rows, present);
    check_value_totals(case, coverage);
    assert_eq!(coverage.achieved_error, 0);
    check_diagnostics(state);
}
fn check_value_totals(case: &fixture::StateCase, coverage: FlowTaggedStateCoverage) {
    let wides: usize = case
        .fields
        .iter()
        .filter(|field| field.encoding == 3)
        .map(|field| field.count as usize)
        .sum();
    assert_eq!(coverage.wide_words, wides);
    assert_eq!(
        coverage.stored_values,
        case.fields.iter().map(|field| field.count as usize).sum()
    );
}
fn check_diagnostics(state: &FlowTaggedState) {
    assert_eq!(
        state.diagnostics(),
        &[
            FlowTaggedStateDiagnostic::NativeContextValidationNotReproduced,
            FlowTaggedStateDiagnostic::NativeRuntimeStateNotReproduced,
            FlowTaggedStateDiagnostic::EngineeringColumnsNotAssigned,
            FlowTaggedStateDiagnostic::NativeApplicationNotChecked,
        ]
    );
}
