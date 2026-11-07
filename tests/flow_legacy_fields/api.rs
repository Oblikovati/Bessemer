use super::*;
use bessemer::flow::*;
#[path = "api/contracts.rs"]
mod contracts;
#[path = "api/paths.rs"]
mod paths;
#[path = "api/replay.rs"]
mod replay;

fn word_options() -> FlowLegacyFieldWordOptions {
    FlowLegacyFieldWordOptions {
        maximum_payload_bytes: 2_000_000,
        maximum_patterns: 100_000,
    }
}
fn group_options() -> FlowConditionalItemOptions {
    FlowConditionalItemOptions {
        maximum_stored_bytes: 2_000_000,
        maximum_decoded_bytes: 2_000_000,
        maximum_entries: 100_000,
        maximum_items: 100_000,
        maximum_groups: 100_000,
        maximum_subrecords: 100_000,
        maximum_vector_values: 1_000_000,
        maximum_values: 1_000_000,
        maximum_scopes: 1_000_000,
        maximum_fields: 1_000_000,
    }
}
fn read_words(row: &fixture::WordCase) -> FlowLegacyWordFields {
    let source = carrier::carrier(row.identifier, &row.raw);
    let id = source.latest(row.identifier).unwrap().unwrap().id;
    match row.identifier {
        105 => source.record_105_fields(&id, word_options()).unwrap(),
        113 => source.record_113_fields(&id, word_options()).unwrap(),
        other => panic!("identifier {other}; expected 105 or 113"),
    }
}
#[test]
fn word_apis_match_every_frozen_native_and_synthetic_pattern_and_exact_limits() {
    for row in fixture::words() {
        let fields = read_words(&row);
        assert_eq!(fields.payload_bytes(), row.raw);
        assert_eq!(fields.stored_count(), row.patterns.len());
        assert_eq!(fields.profile().identifier(), row.identifier);
        assert_eq!(fields.source().records().len(), 3);
        check_word_values(&fields, &row);
        check_word_limits(&fields);
        check_word_edits(&fields);
    }
}
fn check_word_values(fields: &FlowLegacyWordFields, row: &fixture::WordCase) {
    for (index, expected) in row.patterns.iter().enumerate() {
        assert_eq!(fields.pattern(index).unwrap(), *expected);
        let (start, end) = fields.pattern_interval(index).unwrap();
        assert_eq!(&row.raw[start..end], &expected.to_le_bytes());
    }
    assert!(fields.pattern(row.patterns.len()).is_err());
    assert!(fields.pattern_interval(usize::MAX).is_err());
    assert_eq!(fields.coverage().patterns, row.patterns.len());
    assert_eq!(fields.coverage().achieved_error, 0);
    assert_eq!(
        fields.diagnostics().len(),
        if row.identifier == 105 { 5 } else { 3 }
    );
    if let Some(expected) = &row.query {
        check_query(fields.queries_105().unwrap(), expected);
    } else {
        assert!(fields.queries_105().is_err());
    }
}
fn reopen_words(
    fields: &FlowLegacyWordFields,
    options: FlowLegacyFieldWordOptions,
) -> bessemer::Result<FlowLegacyWordFields> {
    match fields.profile() {
        FlowLegacyWordProfile::Fixed105 => {
            fields.source().record_105_fields(fields.record(), options)
        }
        FlowLegacyWordProfile::Counted113 => {
            fields.source().record_113_fields(fields.record(), options)
        }
    }
}
fn check_word_limits(fields: &FlowLegacyWordFields) {
    let exact = FlowLegacyFieldWordOptions {
        maximum_payload_bytes: fields.payload_bytes().len(),
        maximum_patterns: fields.stored_count(),
    };
    assert!(reopen_words(fields, exact).is_ok());
    let mut reduced = exact;
    reduced.maximum_payload_bytes -= 1;
    assert!(reopen_words(fields, reduced).is_err());
    if exact.maximum_patterns != 0 {
        reduced = exact;
        reduced.maximum_patterns -= 1;
        assert!(reopen_words(fields, reduced).is_err());
    }
}

fn check_word_edits(fields: &FlowLegacyWordFields) {
    let count = fields.stored_count();
    let indices: Vec<_> = if count <= 3368 {
        (0..count).collect()
    } else {
        vec![0, count / 2, count - 1]
    };
    for index in indices {
        let before = fields.pattern(index).unwrap();
        let edit = fields.replace_pattern(index, before ^ u64::MAX).unwrap();
        assert_eq!(edit.result().pattern(index).unwrap(), before ^ u64::MAX);
        assert_eq!(edit.source().pattern(index).unwrap(), before);
        carrier::check_history(edit.carrier_edit());
        let inverse = edit.result().replace_pattern(index, before).unwrap();
        assert_eq!(
            inverse.result().source().as_bytes(),
            fields.source().as_bytes()
        );
    }
    assert!(fields.replace_pattern(count, 0).is_err());
}
fn check_query(actual: Flow105ScalarQueries, expected: &fixture::ScalarQuery) {
    assert_eq!(actual.slot10_nonzero, expected.nonzero);
    assert_eq!(actual.slot7.would_call_virtual_78(), expected.call);
    assert_eq!(actual.slot7.precheck_status(), expected.status);
    assert_eq!(actual.native_fp_faults_reproduced, expected.faults);
    assert_eq!(
        actual.native_virtual_result_reproduced,
        expected.virtual_result
    );
}
#[test]
fn scalar_api_matches_all_90601_frozen_comparison_pairs_without_float_conversion() {
    let mut raw = vec![0; 2560];
    for row in fixture::queries() {
        raw[56..64].copy_from_slice(&row.slot7.to_le_bytes());
        raw[80..88].copy_from_slice(&row.slot10.to_le_bytes());
        let source = carrier::carrier(105, &raw);
        let id = source.latest(105).unwrap().unwrap().id;
        let fields = source.record_105_fields(&id, word_options()).unwrap();
        check_query(fields.queries_105().unwrap(), &row.query);
    }
}
#[test]
fn raw155_api_matches_all_1986_complete_frozen_field_and_scope_ledgers() {
    for row in fixture::groups() {
        let source = carrier::carrier(155, &row.stored);
        let id = source.latest(155).unwrap().unwrap().id;
        let fields = source.record_155_fields(&id, group_options()).unwrap();
        assert_eq!(fields.payload_bytes(), row.decoded, "{}", row.name);
        assert_eq!(fields.version(), 0);
        assert_eq!(fields.record(), &id);
        assert_eq!(fields.options(), group_options());
        assert_eq!(fields.diagnostics().len(), 5);
        check_group_fields(&fields, &row);
        check_group_scopes(&fields, &row);
    }
}
fn check_group_fields(fields: &FlowLegacyGroup155, row: &ConditionalCase) {
    assert_eq!(fields.fields().len(), row.fields.len());
    for (field, expected) in fields.fields().zip(&row.fields) {
        assert_eq!(paths::field_path(fields, field), expected.path);
        assert_eq!(field.layout().first_byte, expected.offset as usize);
        assert_eq!(field.layout().element_count, expected.count as usize);
        assert_eq!(field.layout().stride_bytes, expected.width());
        assert_eq!(field.encoding().width(), expected.width());
        check_group_field_identity(fields, field);
        check_group_values(field, expected);
        assert!(field.value(expected.count as usize).is_none());
    }
    assert!(fields.field_at(row.fields.len()).is_none());
    assert_eq!(
        fields.coverage().values,
        row.fields
            .iter()
            .map(|field| field.count as usize)
            .sum::<usize>()
    );
}
fn check_group_field_identity(fields: &FlowLegacyGroup155, field: FlowConditionalField<'_>) {
    assert_eq!(fields.field(&field.id()).unwrap().id(), field.id());
    assert_eq!(
        fields.field_at(field.id().ordinal()).unwrap().id(),
        field.id()
    );
}
fn check_group_values(field: FlowConditionalField<'_>, expected: &ConditionalField) {
    for (index, raw) in expected.raw.chunks_exact(expected.width()).enumerate() {
        assert_eq!(field.element_bytes(index).unwrap(), raw);
        assert_eq!(
            field.value(index),
            Some(typed_value(expected.encoding, raw))
        );
    }
}
fn typed_value(encoding: u8, raw: &[u8]) -> FlowConditionalValue {
    match encoding {
        0 => FlowConditionalValue::Byte(raw[0]),
        1 => FlowConditionalValue::Signed32(i32::from_le_bytes(raw.try_into().unwrap())),
        2 => FlowConditionalValue::WordBits(u32::from_le_bytes(raw.try_into().unwrap())),
        3 => FlowConditionalValue::WideBits(u64::from_le_bytes(raw.try_into().unwrap())),
        other => panic!("encoding {other}; expected 0..=3"),
    }
}
fn check_group_scopes(fields: &FlowLegacyGroup155, row: &ConditionalCase) {
    assert_eq!(fields.scopes().len(), row.scopes.len());
    for (scope, expected) in fields.scopes().zip(&row.scopes) {
        assert_eq!(paths::scope_path(fields, scope), expected.path);
        assert_eq!(
            scope.payload_interval(),
            (expected.start as usize, expected.end as usize)
        );
        assert_eq!(fields.scope(&scope.id()).unwrap().id(), scope.id());
        assert_eq!(
            fields.scope_at(scope.id().ordinal()).unwrap().id(),
            scope.id()
        );
    }
    assert!(fields.scope_at(row.scopes.len()).is_none());
    assert_eq!(fields.coverage().fields, row.fields.len());
    assert_eq!(fields.coverage().scopes, row.scopes.len());
    assert_eq!(fields.coverage().stored_bytes, row.stored.len());
    assert_eq!(fields.coverage().decoded_bytes, row.decoded.len());
    assert_eq!(fields.coverage().achieved_error, 0);
}
fn selection_source(records: &[(u32, Vec<u8>)]) -> FlowCarrier {
    let mut raw = 8u16.to_le_bytes().to_vec();
    for (identifier, payload) in records {
        raw.extend_from_slice(&identifier.to_le_bytes());
        raw.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        raw.extend_from_slice(payload);
    }
    FlowCarrier::from_bytes(
        &raw,
        FlowLimits {
            maximum_wire_bytes: 100_000,
            maximum_payload_bytes: 100_000,
            maximum_records: 10,
        },
    )
    .unwrap()
}
#[test]
fn selection_api_preserves_all_400_frozen_preference_latest_empty_and_failure_cases() {
    for row in fixture::selections() {
        let source = selection_source(&row.records);
        let selected = source.legacy_field_container_latest(word_options(), group_options());
        assert_eq!(selected.is_ok(), row.completed, "{}", row.name);
        if let Ok(selected) = selected {
            check_selected_record(&source, selected, row.identifier);
        }
    }
}
fn check_selected_record(
    source: &FlowCarrier,
    selected: Option<FlowLegacyFieldSelection>,
    identifier: Option<u32>,
) {
    assert_eq!(
        selected.as_ref().map(|value| value.identifier()),
        identifier
    );
    if let Some(selected) = selected {
        assert_eq!(
            selected.record(),
            &source.latest(selected.identifier()).unwrap().unwrap().id
        );
    }
}
#[test]
fn complete_legacy_results_options_and_projections_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<FlowLegacyWordProfile>();
    check::<FlowLegacyFieldWordOptions>();
    check::<FlowLegacyWordFields>();
    check::<FlowLegacyWordCoverage>();
    check::<FlowLegacyFieldDiagnostic>();
    check::<Flow105Slot7Decision>();
    check::<Flow105ScalarQueries>();
    check::<FlowLegacyGroup155>();
    check::<FlowLegacyFieldSelection>();
    check::<FlowSimpleTableEdit<FlowLegacyWordFields>>();
    check::<FlowSimpleTableEdit<FlowLegacyGroup155>>();
}
