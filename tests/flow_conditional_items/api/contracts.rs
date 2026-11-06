use super::*;

fn compact_cases() -> Vec<ConditionalCase> {
    fixture::cases()
        .into_iter()
        .filter(|case| case.name.starts_with("item-v") && case.name.contains("-active1-"))
        .collect()
}

#[test]
fn conditional_framing_rejects_all_compact_prefixes_extensions_and_reached_invalid_counts() {
    let cases = compact_cases();
    assert_eq!(cases.len(), 24);
    for case in cases {
        check_malformed(&case);
    }
}

fn check_malformed(case: &ConditionalCase) {
    for end in 0..case.decoded.len() {
        assert!(
            open_decoded(case, &case.decoded[..end]).is_err(),
            "{}:{end}",
            case.name
        );
    }
    let mut extended = case.decoded.clone();
    extended.push(0);
    assert!(open_decoded(case, &extended).is_err());
    check_invalid_counts(case);
}

fn open_decoded(case: &ConditionalCase, decoded: &[u8]) -> bessemer::Result<FlowConditionalItems> {
    open(
        case.identifier,
        &stored(case.identifier, case.version, decoded),
        options(),
    )
}

fn stored(identifier: u32, version: i32, decoded: &[u8]) -> Vec<u8> {
    use std::io::Write;
    if identifier == 126 {
        return decoded.to_vec();
    }
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(decoded).unwrap();
    let compressed = encoder.finish().unwrap();
    if identifier == 315 {
        return compressed;
    }
    let mut result = version.to_le_bytes().to_vec();
    result.extend_from_slice(&compressed);
    result
}

#[test]
fn every_independent_conditional_budget_accepts_exact_totals_and_rejects_one_below() {
    for case in compact_cases() {
        let items = open(case.identifier, &case.stored, options()).unwrap();
        let exact = exact_options(items.coverage());
        assert!(open(case.identifier, &case.stored, exact).is_ok());
        for axis in 0..10 {
            if limit(exact, axis) == 0 {
                continue;
            }
            assert!(
                open(case.identifier, &case.stored, reduced(exact, axis)).is_err(),
                "{}:{axis}",
                case.name
            );
        }
    }
}

fn exact_options(coverage: FlowConditionalCoverage) -> FlowConditionalItemOptions {
    FlowConditionalItemOptions {
        maximum_stored_bytes: coverage.stored_bytes,
        maximum_decoded_bytes: coverage.decoded_bytes,
        maximum_entries: coverage.entries,
        maximum_items: coverage.items,
        maximum_groups: coverage.groups,
        maximum_subrecords: coverage.subrecords,
        maximum_vector_values: coverage.vector_values,
        maximum_values: coverage.values,
        maximum_scopes: coverage.scopes,
        maximum_fields: coverage.fields,
    }
}

fn limit(options: FlowConditionalItemOptions, axis: usize) -> usize {
    [
        options.maximum_stored_bytes,
        options.maximum_decoded_bytes,
        options.maximum_entries,
        options.maximum_items,
        options.maximum_groups,
        options.maximum_subrecords,
        options.maximum_vector_values,
        options.maximum_values,
        options.maximum_scopes,
        options.maximum_fields,
    ][axis]
}

fn reduced(mut options: FlowConditionalItemOptions, axis: usize) -> FlowConditionalItemOptions {
    match axis {
        0 => options.maximum_stored_bytes -= 1,
        1 => options.maximum_decoded_bytes -= 1,
        2 => options.maximum_entries -= 1,
        3 => options.maximum_items -= 1,
        4 => options.maximum_groups -= 1,
        5 => options.maximum_subrecords -= 1,
        6 => options.maximum_vector_values -= 1,
        7 => options.maximum_values -= 1,
        8 => options.maximum_scopes -= 1,
        9 => options.maximum_fields -= 1,
        _ => panic!("Expected budget0..10"),
    }
    options
}

#[test]
fn conditional_edits_reject_structural_wrong_type_out_of_range_and_foreign_or_stale_ids() {
    for case in compact_cases() {
        let items = open(case.identifier, &case.stored, options()).unwrap();
        check_invalid_edits(&items, &case);
        assert_eq!(items.decoded_bytes(), case.decoded);
    }
}

#[test]
fn conditional_public_views_are_send_sync() {
    assert_send_sync::<FlowConditionalItems>();
    assert_send_sync::<FlowConditionalItemEdit>();
    assert_send_sync::<FlowConditionalItemOptions>();
    assert_send_sync::<FlowConditionalFormat>();
    assert_send_sync::<FlowConditionalFieldId>();
    assert_send_sync::<FlowConditionalScopeId>();
    assert_send_sync::<FlowConditionalField<'static>>();
    assert_send_sync::<FlowConditionalScope<'static>>();
    assert_send_sync::<FlowConditionalCoverage>();
    assert_send_sync::<FlowConditionalFieldKind>();
    assert_send_sync::<FlowConditionalScopeKind>();
    assert_send_sync::<FlowConditionalCheckMode>();
    assert_send_sync::<FlowConditionalDiagnostic>();
    assert_send_sync::<FlowConditionalValue>();
    assert_send_sync::<FlowConditionalEncoding>();
    assert_send_sync::<FlowConditionalFieldLayout>();
}

#[test]
fn conditional_concurrent_edits_match_serial_bytes() {
    for case in compact_cases()
        .into_iter()
        .filter(|case| case.name.starts_with("item-v5-"))
    {
        check_concurrency(&case);
    }
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn conditional_profile_constraints_and_encoded_boundaries_reject_invalid_fields_without_mutation() {
    for case in compact_cases() {
        let items = open(case.identifier, &case.stored, options()).unwrap();
        for field in items.fields() {
            let invalid = invalid_profile_value(&items, field);
            if let Some(value) = invalid {
                let mut changed = case.decoded.clone();
                let start = field.layout().first_byte;
                changed[start..start + value.len()].copy_from_slice(&value);
                assert!(
                    open_decoded(&case, &changed).is_err(),
                    "{}:{:?}",
                    case.name,
                    field.kind()
                );
            }
        }
        check_bad_transport(&case);
        assert_eq!(items.decoded_bytes(), case.decoded);
    }
}

fn invalid_profile_value(
    items: &FlowConditionalItems,
    field: FlowConditionalField<'_>,
) -> Option<Vec<u8>> {
    use FlowConditionalFieldKind as Kind;
    match field.kind() {
        Kind::Version | Kind::Dimension | Kind::Mode | Kind::Protocol | Kind::InnerWord => {
            Some(99i32.to_le_bytes().to_vec())
        }
        Kind::ItemMode if item_version(items, field) == 1 => Some(vec![1]),
        _ => None,
    }
}

fn check_bad_transport(case: &ConditionalCase) {
    if case.identifier == 126 {
        return;
    }
    let mut trailing = case.stored.clone();
    trailing.push(0);
    assert!(open(case.identifier, &trailing, options()).is_err());
    assert!(
        open(
            case.identifier,
            &case.stored[..case.stored.len() - 1],
            options()
        )
        .is_err()
    );
    let mut corrupt = case.stored.clone();
    let last = corrupt.len() - 1;
    corrupt[last] ^= 255;
    assert!(open(case.identifier, &corrupt, options()).is_err());
    check_bad_outer_version(case);
}

fn check_concurrency(case: &ConditionalCase) {
    let items = open(case.identifier, &case.stored, options()).unwrap();
    let serial: Vec<_> = (0..8).map(|index| edited_bytes(&items, index)).collect();
    let shared = &items;
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|index| scope.spawn(move || edited_bytes(shared, index)))
            .collect();
        for (worker, expected) in workers.into_iter().zip(serial) {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(items.decoded_bytes(), case.decoded);
}

fn edited_bytes(items: &FlowConditionalItems, index: i32) -> Vec<u8> {
    let field = items
        .fields()
        .find(|field| field.kind() == FlowConditionalFieldKind::Member84)
        .unwrap();
    items
        .replace_field(&field.id(), 0, FlowConditionalValue::Signed32(42 + index))
        .unwrap()
        .result()
        .source()
        .as_bytes()
        .to_vec()
}

fn check_invalid_counts(case: &ConditionalCase) {
    for &offset in &case.counts {
        for value in [-1i32, i32::MAX] {
            let mut changed = case.decoded.clone();
            changed[offset as usize..offset as usize + 4].copy_from_slice(&value.to_le_bytes());
            assert!(
                open_decoded(case, &changed).is_err(),
                "{}:{offset}",
                case.name
            );
        }
    }
}

fn check_invalid_edits(items: &FlowConditionalItems, case: &ConditionalCase) {
    check_readonly(items);
    let field = items
        .fields()
        .find(|field| field.kind() == FlowConditionalFieldKind::Member84)
        .unwrap();
    check_wrong_values(items, field);
    let edit = items
        .replace_field(&field.id(), 0, FlowConditionalValue::Signed32(42))
        .unwrap();
    assert!(edit.result().field(&field.id()).is_err());
    assert!(edit.result().scope(&field.scope()).is_err());
    let foreign = open(case.identifier, &case.stored, options()).unwrap();
    assert!(foreign.field(&field.id()).is_err());
}

fn check_readonly(items: &FlowConditionalItems) {
    for field in items.fields().filter(|field| !field.is_editable()) {
        assert!(
            items
                .replace_field(&field.id(), 0, field.value(0).unwrap())
                .is_err()
        );
    }
}

fn check_wrong_values(items: &FlowConditionalItems, field: FlowConditionalField<'_>) {
    assert!(
        items
            .replace_field(&field.id(), usize::MAX, FlowConditionalValue::Signed32(42))
            .is_err()
    );
    assert!(
        items
            .replace_field(&field.id(), 0, FlowConditionalValue::Byte(1))
            .is_err()
    );
}

fn check_bad_outer_version(case: &ConditionalCase) {
    if case.identifier == 128 {
        let mut corrupt = case.stored.clone();
        corrupt[..4].copy_from_slice(&5i32.to_le_bytes());
        let error = open(128, &corrupt, options()).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
        assert!(error.cause.contains('5') && error.cause.contains("expected"));
    }
}
