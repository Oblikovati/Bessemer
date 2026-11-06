use super::*;

#[test]
fn all_complete_conditional_records_match_every_frozen_field_scope_value_and_codec_boundary() {
    for case in fixture::cases() {
        check_case(&case);
    }
}

fn check_fields(items: &FlowConditionalItems, case: &ConditionalCase) {
    assert_eq!(items.fields().len(), case.fields.len(), "{}", case.name);
    for (field, expected) in items.fields().zip(&case.fields) {
        assert_eq!(
            paths::field_path(items, field),
            expected.path,
            "{}",
            case.name
        );
        assert_eq!(field.layout().first_byte, expected.offset as usize);
        assert_eq!(field.layout().element_count, expected.count as usize);
        assert_eq!(field.encoding().width(), expected.width());
        assert_eq!(field.layout().stride_bytes, expected.width());
        check_field_identity(items, field);
        check_values(field, expected);
    }
    assert!(items.field_at(case.fields.len()).is_none());
}

fn check_values(field: FlowConditionalField<'_>, expected: &ConditionalField) {
    for (index, raw) in expected.raw.chunks_exact(expected.width()).enumerate() {
        assert_eq!(field.element_bytes(index).unwrap(), raw);
        assert_eq!(field.value(index), Some(typed(expected.encoding, raw)));
    }
    assert!(field.value(expected.count as usize).is_none());
    assert!(field.element_offset(usize::MAX).is_none());
    let mode = if field.kind() == FlowConditionalFieldKind::ItemMode {
        Some(expected.raw[0] != 0)
    } else {
        None
    };
    assert_eq!(field.native_mode_boolean(), mode);
}

pub(super) fn typed(encoding: u8, raw: &[u8]) -> FlowConditionalValue {
    match encoding {
        0 => FlowConditionalValue::Byte(raw[0]),
        1 => FlowConditionalValue::Signed32(i32::from_le_bytes(raw.try_into().unwrap())),
        2 => FlowConditionalValue::WordBits(u32::from_le_bytes(raw.try_into().unwrap())),
        3 => FlowConditionalValue::WideBits(u64::from_le_bytes(raw.try_into().unwrap())),
        other => panic!("Expected encoding0..3, got {other}"),
    }
}

fn check_scopes(items: &FlowConditionalItems, case: &ConditionalCase) {
    assert_eq!(items.scopes().len(), case.scopes.len(), "{}", case.name);
    for (scope, expected) in items.scopes().zip(&case.scopes) {
        assert_eq!(paths::scope_path(items, scope), expected.path);
        assert_eq!(
            scope.payload_interval(),
            (expected.start as usize, expected.end as usize)
        );
        check_scope_identity(items, scope);
        if let Some(parent) = scope.parent() {
            assert!(parent.ordinal() < scope.id().ordinal());
        }
    }
    assert!(items.scope_at(case.scopes.len()).is_none());
}

fn check_coverage(items: &FlowConditionalItems, case: &ConditionalCase) {
    let coverage = items.coverage();
    assert_eq!(coverage.stored_bytes, case.stored.len());
    assert_eq!(coverage.decoded_bytes, case.decoded.len());
    assert_eq!(
        coverage.stored_prefix_bytes,
        if case.identifier == 128 { 4 } else { 0 }
    );
    assert_eq!(
        coverage.wide_patterns_per_vector_element,
        case.scale as usize
    );
    assert_eq!(coverage.fields, case.fields.len());
    assert_eq!(coverage.scopes, case.scopes.len());
    assert_eq!(
        coverage.values,
        case.fields.iter().map(|field| field.count as usize).sum()
    );
    assert_eq!(coverage.achieved_error, 0);
    check_metadata(items, case);
    check_counts(items, case);
}

fn check_counts(items: &FlowConditionalItems, case: &ConditionalCase) {
    let scopes: Vec<_> = items.scopes().collect();
    let count = |kind| scopes.iter().filter(|scope| scope.kind() == kind).count();
    assert_eq!(
        items.coverage().entries,
        count(FlowConditionalScopeKind::Entry)
    );
    assert_eq!(
        items.coverage().items,
        count(FlowConditionalScopeKind::Item)
    );
    assert_eq!(
        items.coverage().groups,
        count(FlowConditionalScopeKind::HeaderGroup) + count(FlowConditionalScopeKind::ItemGroup)
    );
    assert_eq!(
        items.coverage().subrecords,
        count(FlowConditionalScopeKind::Subrecord)
    );
    check_count_fields(items, case);
}

fn check_field_identity(items: &FlowConditionalItems, field: FlowConditionalField<'_>) {
    assert_eq!(items.field(&field.id()).unwrap().kind(), field.kind());
    assert_eq!(field.id().record(), items.record());
    assert_eq!(
        items.field_at(field.id().ordinal()).unwrap().layout(),
        field.layout()
    );
}

fn check_scope_identity(items: &FlowConditionalItems, scope: FlowConditionalScope<'_>) {
    assert_eq!(items.scope(&scope.id()).unwrap().kind(), scope.kind());
    assert_eq!(scope.id().record(), items.record());
    assert_eq!(
        items
            .scope_at(scope.id().ordinal())
            .unwrap()
            .payload_interval(),
        scope.payload_interval()
    );
}

fn check_metadata(items: &FlowConditionalItems, case: &ConditionalCase) {
    let coverage = items.coverage();
    assert_eq!(items.record().ordinal(), 1);
    assert_eq!(items.source().records().len(), 3);
    assert_eq!(
        items.diagnostics().len(),
        if case.identifier == 126 { 4 } else { 5 }
    );
    let mode = match case.identifier {
        126 => FlowConditionalCheckMode::ExactRawPayload,
        315 => FlowConditionalCheckMode::ExactZlibPayload,
        _ => FlowConditionalCheckMode::ExactRawVersionZlibBody,
    };
    assert_eq!(coverage.mode, mode);
}

fn check_count_fields(items: &FlowConditionalItems, case: &ConditionalCase) {
    let actual: Vec<_> = items
        .fields()
        .filter(|field| {
            field.kind().is_structural() && field.kind() != FlowConditionalFieldKind::Version
        })
        .map(|field| field.layout().first_byte as u32)
        .collect();
    assert_eq!(actual, case.counts);
    let vectors: usize = case
        .fields
        .iter()
        .filter(|field| {
            field.path.ends_with("\"values\"]")
                || field.path.starts_with("[\"arrays\",")
                || field.path == "[\"extra_patterns\"]"
        })
        .map(|field| field.count as usize)
        .sum();
    assert_eq!(items.coverage().vector_values, vectors);
}

pub(super) fn check_case(case: &ConditionalCase) -> FlowConditionalItems {
    let items = open(case.identifier, &case.stored, options()).unwrap();
    assert_eq!(items.version(), case.version, "{}", case.name);
    assert_eq!(items.format().identifier(), case.identifier);
    assert_eq!(items.decoded_bytes(), case.decoded);
    assert_eq!(items.options(), options());
    check_fields(&items, case);
    check_scopes(&items, case);
    check_coverage(&items, case);
    items
}
