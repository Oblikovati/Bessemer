use super::*;
pub(super) fn options() -> FlowIndexOptions {
    control_limits(2_000_000, 100_000)
}
fn control_limits(decoded: usize, entries: usize) -> FlowIndexOptions {
    FlowIndexOptions {
        maximum_active_entries: 100_000,
        maximum_saved_entries: 100_000,
        controls: FlowControlOptions {
            maximum_decoded_bytes: decoded,
            maximum_entries: entries,
        },
    }
}
pub(super) fn policy(forced: bool) -> FlowIndexPolicy {
    if forced {
        FlowIndexPolicy::ForcePhysicalScan
    } else {
        FlowIndexPolicy::PreferTerminalIndex
    }
}
#[test]
fn all_2140_requested_api_outcomes_match_frozen_maps_and_recognition_errors() {
    let mut outcomes = [0; 3];
    for row in cases() {
        let source = oracles::carrier(&row);
        for (forced, expected) in [false, true].into_iter().zip(&row.outcomes) {
            let actual = source.logical_index(policy(forced), options());
            compare(&row, expected, actual, &mut outcomes);
            assert_eq!(source.as_bytes(), row.raw);
        }
    }
    assert_eq!(outcomes, [1474, 649, 17]);
}
fn compare(
    row: &ReplayCase,
    expected: &ReplayOutcome,
    actual: bessemer::Result<FlowLogicalIndex>,
    totals: &mut [usize; 3],
) {
    match expected {
        ReplayOutcome::Rejected { .. } => {
            let error = actual.unwrap_err();
            assert_eq!(error.operation, "Flow logical index replay");
            assert!(
                !error.entity.is_empty() && !error.cause.is_empty(),
                "{}",
                row.name
            );
            totals[2] += 1;
        }
        ReplayOutcome::Completed(expected) => {
            let actual = actual.unwrap_or_else(|error| panic!("{}: {error}", row.name));
            check_projection(row, expected, &actual);
            totals[usize::from(expected.terminal_adopted)] += 1;
        }
    }
}
pub(super) fn check_projection(
    row: &ReplayCase,
    expected: &ReplayProjection,
    actual: &FlowLogicalIndex,
) {
    check_maps(row, expected, actual);
    assert_eq!(actual.policy(), policy(expected.forced));
    assert_eq!(actual.options(), options());
    assert_eq!(actual.source().as_bytes(), row.raw);
    assert_eq!(actual.source().version(), row.version);
    assert_eq!(
        actual.terminal_control().is_some(),
        expected.terminal_adopted
    );
    check_coverage(row, expected, actual);
    check_provenance(actual);
}
fn check_maps(row: &ReplayCase, expected: &ReplayProjection, actual: &FlowLogicalIndex) {
    let active: Vec<_> = actual
        .active_entries()
        .iter()
        .map(|(id, span)| RecordSpan {
            identifier: *id,
            start: span.record_start,
            span: span.record_span,
        })
        .collect();
    assert_eq!(active, expected.active, "{}", row.name);
    assert_eq!(
        actual
            .saved_entries()
            .iter()
            .map(|(key, value)| (*key, *value))
            .collect::<Vec<_>>(),
        expected.saved,
        "{}",
        row.name
    );
}
fn check_coverage(row: &ReplayCase, expected: &ReplayProjection, actual: &FlowLogicalIndex) {
    let coverage = actual.coverage();
    let method = if expected.terminal_adopted {
        FlowIndexMethod::TerminalIndex
    } else {
        FlowIndexMethod::PhysicalScan
    };
    assert_eq!(actual.method(), method);
    assert_eq!(coverage.method, method);
    assert_eq!(coverage.physical_wire_bytes, row.raw.len());
    assert_eq!(coverage.physical_records, row.physical.len());
    assert_eq!(coverage.active_entries, expected.active.len());
    assert_eq!(coverage.saved_entries, expected.saved.len());
    check_control_coverage(expected, actual);
    check_qualifications(actual);
}
fn check_control_coverage(expected: &ReplayProjection, actual: &FlowLogicalIndex) {
    check_terminal_selection(expected, actual);
    let coverage = actual.coverage();
    assert_eq!(
        coverage.terminal_decoded_bytes,
        expected
            .terminal
            .as_ref()
            .map_or(0, |control| control.decoded.len())
    );
    assert_eq!(
        coverage.saved_decoded_bytes,
        actual
            .saved_control()
            .map_or(0, |control| control.decoded_bytes().len())
    );
}
fn check_terminal_selection(expected: &ReplayProjection, actual: &FlowLogicalIndex) {
    let Some(expected) = &expected.terminal else {
        return;
    };
    let control = actual.terminal_control().unwrap();
    assert_eq!(control.record().ordinal(), expected.ordinal as usize);
    assert_eq!(control.decoded_bytes(), expected.decoded);
    assert_eq!(control.stored_count(), expected.count as usize);
    assert_eq!(control.length_width(), FlowControlLengthWidth::EightBytes);
    assert_eq!(control.backward_distance(), expected.distance);
    assert_eq!(control.footer_identifier(), expected.identifier);
    assert_eq!(
        control.footer_interval(),
        (expected.footer_start as usize, expected.stored as usize)
    );
}
fn check_qualifications(actual: &FlowLogicalIndex) {
    let coverage = actual.coverage();
    assert!(
        !coverage.ordinary_payloads_decoded
            && !coverage.indexed_ordinary_targets_validated
            && !coverage.complete_document_schema
    );
    assert_eq!(coverage.achieved_byte_error, 0);
    assert_eq!(actual.diagnostics().len(), 7);
    assert!(
        actual
            .diagnostics()
            .contains(&FlowIndexDiagnostic::FreshEmptyMapsAssumed)
    );
}
fn check_provenance(actual: &FlowLogicalIndex) {
    if let Some(terminal) = actual.terminal_control() {
        let record = actual
            .source()
            .records()
            .nth(terminal.record().ordinal())
            .unwrap();
        assert_eq!(terminal.record(), &record.id);
        assert_eq!(terminal.source().as_bytes(), actual.source().as_bytes());
    }
    check_saved_provenance(actual);
}
fn check_saved_provenance(actual: &FlowLogicalIndex) {
    if let Some(control) = actual.saved_control() {
        let record = actual
            .source()
            .records()
            .nth(control.record().ordinal())
            .unwrap();
        assert_eq!(control.record(), &record.id);
        assert_eq!(record.identifier, SAVED);
        let span = actual.active_entries().get(&SAVED).unwrap();
        assert_eq!(span.record_start, record.header_offset as u64);
        assert_eq!(span.record_span, 12 + record.payload.len() as u64);
        assert_eq!(control.source().as_bytes(), actual.source().as_bytes());
    }
}
#[test]
fn exact_distinct_map_limits_admit_duplicates_and_reject_each_new_key_before_publication() {
    for row in cases() {
        let source = oracles::carrier(&row);
        for expected in &row.outcomes {
            let ReplayOutcome::Completed(expected) = expected else {
                continue;
            };
            check_limits(&source, expected);
        }
    }
}
fn check_limits(source: &FlowCarrier, expected: &ReplayProjection) {
    let mut exact = options();
    exact.maximum_active_entries = expected.active.len();
    exact.maximum_saved_entries = expected.saved.len();
    let actual = source
        .logical_index(policy(expected.forced), exact)
        .unwrap();
    assert_eq!(actual.active_entries().len(), exact.maximum_active_entries);
    assert_eq!(actual.saved_entries().len(), exact.maximum_saved_entries);
    check_lower_limits(source, expected.forced, exact);
}
fn check_lower_limits(source: &FlowCarrier, forced: bool, exact: FlowIndexOptions) {
    if exact.maximum_active_entries > 0 {
        let below = FlowIndexOptions {
            maximum_active_entries: exact.maximum_active_entries - 1,
            ..exact
        };
        assert_bound_failure(source, forced, below, "active map");
    }
    if exact.maximum_saved_entries > 0 {
        let below = FlowIndexOptions {
            maximum_saved_entries: exact.maximum_saved_entries - 1,
            ..exact
        };
        assert_bound_failure(source, forced, below, "saved map");
    }
}
fn assert_bound_failure(
    source: &FlowCarrier,
    forced: bool,
    options: FlowIndexOptions,
    entity: &str,
) {
    let before = source.as_bytes().to_vec();
    let error = source.logical_index(policy(forced), options).unwrap_err();
    assert_eq!(error.operation, "Flow logical index replay");
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert_eq!(error.entity, entity);
    assert!(error.cause.contains("configured maximum"));
    assert_eq!(source.as_bytes(), before);
}
#[test]
fn control_budgets_apply_only_to_adopted_controls_and_do_not_break_opaque_forced_scanning() {
    for row in cases().into_iter().filter(|row| row.raw.len() < 8192) {
        let source = oracles::carrier(&row);
        let tiny = FlowIndexOptions {
            controls: FlowControlOptions {
                maximum_decoded_bytes: 0,
                maximum_entries: 0,
            },
            ..options()
        };
        source
            .logical_index(FlowIndexPolicy::ForcePhysicalScan, tiny)
            .unwrap();
        let ReplayOutcome::Completed(expected) = &row.outcomes[0] else {
            continue;
        };
        let result = source.logical_index(FlowIndexPolicy::PreferTerminalIndex, tiny);
        assert_eq!(result.is_ok(), !expected.terminal_adopted, "{}", row.name);
    }
}
#[test]
fn all_public_replay_types_are_send_and_sync() {
    fn traits<T: Send + Sync>() {}
    traits::<FlowIndexPolicy>();
    traits::<FlowIndexOptions>();
    traits::<FlowIndexedSpan>();
    traits::<FlowIndexMethod>();
    traits::<FlowLogicalIndex>();
    traits::<FlowIndexCoverage>();
    traits::<FlowIndexDiagnostic>();
}
#[test]
fn exact_control_body_and_stored_entry_budgets_are_checked_independently() {
    for row in cases() {
        let source = oracles::carrier(&row);
        let Ok(index) = source.logical_index(FlowIndexPolicy::PreferTerminalIndex, options())
        else {
            continue;
        };
        let Some(terminal) = index.terminal_control() else {
            continue;
        };
        let saved = index.saved_control();
        let decoded = terminal
            .decoded_bytes()
            .len()
            .max(saved.map_or(0, |view| view.decoded_bytes().len()));
        let entries = terminal
            .stored_count()
            .max(saved.map_or(0, |view| view.stored_count()));
        check_control_bounds(&source, decoded, entries);
    }
}
fn check_control_bounds(source: &FlowCarrier, decoded: usize, entries: usize) {
    let exact = control_limits(decoded, entries);
    let index = source
        .logical_index(FlowIndexPolicy::PreferTerminalIndex, exact)
        .unwrap();
    let before = source.as_bytes().to_vec();
    let smaller = control_limits(decoded - 1, entries);
    control_rejected(source, smaller);
    if entries > 0 {
        let smaller = control_limits(decoded, entries - 1);
        control_rejected(source, smaller);
    }
    assert_eq!(source.as_bytes(), before);
    assert_eq!(index.source().as_bytes(), before);
}
fn control_rejected(source: &FlowCarrier, options: FlowIndexOptions) {
    let failure = source
        .logical_index(FlowIndexPolicy::PreferTerminalIndex, options)
        .unwrap_err();
    assert_eq!(failure.operation, "Flow logical index replay");
    assert!(!failure.entity.is_empty() && !failure.cause.is_empty());
}
