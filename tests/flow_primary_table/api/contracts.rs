use super::*;

fn compact() -> PrimaryCase {
    compact_cases()
        .into_iter()
        .find(|case| case.version == 3)
        .unwrap()
}

fn compact_cases() -> Vec<PrimaryCase> {
    let mut selected = std::collections::BTreeSet::new();
    fixture::cases()
        .into_iter()
        .filter(|case| {
            let Some(item) = case.items.first() else {
                return false;
            };
            if !item.groups.iter().any(populated_group) {
                return false;
            }
            selected.insert((case.version, item.element_kind))
        })
        .collect()
}

fn populated_group(group: &PrimaryGroup) -> bool {
    group.body.is_some()
        && group.elements.as_ref().is_some_and(|elements| {
            elements
                .iter()
                .any(|element| !element.rows.is_empty() && !element.blocks.is_empty())
        })
}

#[test]
fn complete_primary_tables_reject_all_proper_prefixes_trailing_bytes_and_unsupported_versions() {
    for case in compact_cases() {
        check_malformed(&case);
    }
}

fn check_malformed(case: &PrimaryCase) {
    for end in 0..case.raw.len() {
        assert!(open(&case.raw[..end], options()).is_err(), "prefix {end}");
    }
    let mut extended = case.raw.clone();
    extended.push(0);
    assert!(open(&extended, options()).is_err());
    for version in [i32::MIN, -1, 4, i32::MAX] {
        let mut raw = case.raw.clone();
        raw[..4].copy_from_slice(&version.to_le_bytes());
        assert!(open(&raw, options()).is_err());
    }
}

#[test]
fn every_reached_count_rejects_negative_and_impossible_values_without_publishing_a_view() {
    for case in compact_cases() {
        check_counts(&case);
    }
}

fn check_counts(case: &PrimaryCase) {
    let table = open(&case.raw, options()).unwrap();
    for field in table.fields().filter(|field| {
        field.kind().is_structural() && field.kind() != FlowPrimaryFieldKind::Version
    }) {
        for count in [-1i32, i32::MAX] {
            let offset = field.element_offset(0).unwrap();
            let mut raw = case.raw.clone();
            raw[offset..offset + 4].copy_from_slice(&count.to_le_bytes());
            assert!(
                open(&raw, options()).is_err(),
                "{:?} at {offset}",
                field.kind()
            );
        }
    }
    assert_eq!(table.decoded_bytes(), case.raw);
}

#[test]
fn primary_edits_reject_structural_values_wrong_types_out_of_range_stale_and_layout_changes() {
    let case = compact();
    let table = open(&case.raw, options()).unwrap();
    for field in table.fields().filter(|field| field.kind().is_structural()) {
        assert!(
            table
                .replace_field(&field.id(), 0, field.value(0).unwrap())
                .is_err()
        );
    }
    check_invalid_members(&table);
    check_layout_change(&table);
    check_foreign_record(&case.raw);
    assert_eq!(table.decoded_bytes(), case.raw);
}

fn check_invalid_members(table: &FlowPrimaryTable) {
    let member = table
        .fields()
        .find(|field| field.kind() == FlowPrimaryFieldKind::Members)
        .unwrap();
    assert!(
        table
            .replace_field(&member.id(), 0, FlowPrimaryValue::Byte(1))
            .is_err()
    );
    assert!(
        table
            .replace_field(&member.id(), usize::MAX, FlowPrimaryValue::Signed32(1))
            .is_err()
    );
    let edit = table
        .replace_field(&member.id(), 0, FlowPrimaryValue::Signed32(42))
        .unwrap();
    assert!(edit.result().field(&member.id()).is_err());
}

fn check_layout_change(table: &FlowPrimaryTable) {
    let flag = table
        .fields()
        .find(|field| {
            field.kind() == FlowPrimaryFieldKind::Presence
                && field.value(0) != Some(FlowPrimaryValue::Byte(0))
        })
        .unwrap();
    assert!(
        table
            .replace_field(&flag.id(), 0, FlowPrimaryValue::Byte(0))
            .is_err()
    );
}

fn check_foreign_record(raw: &[u8]) {
    let source = carrier(raw);
    let wrong = source.latest(999).unwrap().unwrap().id;
    assert!(source.record_439_primary_table(&wrong, options()).is_err());
}

#[test]
fn every_primary_resource_axis_admits_exact_limits_and_rejects_one_below() {
    let case = compact();
    let table = open(&case.raw, options()).unwrap();
    let coverage = table.coverage();
    let exact = exact_options(coverage);
    assert!(open(&case.raw, exact).is_ok());
    for axis in 0..8 {
        assert!(
            open(&case.raw, reduced(exact, axis)).is_err(),
            "axis {axis}"
        );
    }
    let mut invalid = exact;
    invalid.maximum_decoded_bytes = 7;
    assert!(open(&case.raw, invalid).is_err());
    check_empty_budgets();
}

fn exact_options(coverage: FlowPrimaryCoverage) -> FlowPrimaryTableOptions {
    FlowPrimaryTableOptions {
        maximum_decoded_bytes: coverage.payload_bytes,
        maximum_items: coverage.items,
        maximum_elements: coverage.elements,
        maximum_rows: coverage.rows,
        maximum_wide_blocks: coverage.wide_blocks,
        maximum_scopes: coverage.scopes,
        maximum_fields: coverage.fields,
        maximum_values: coverage.values,
        ..options()
    }
}

fn check_empty_budgets() {
    let empty = [2, 0, 0, 0, 0, 0, 0, 0];
    let empty_options = FlowPrimaryTableOptions {
        maximum_items: 0,
        maximum_elements: 0,
        maximum_rows: 0,
        maximum_wide_blocks: 0,
        ..options()
    };
    assert_eq!(open(&empty, empty_options).unwrap().coverage().items, 0);
}

fn reduced(mut options: FlowPrimaryTableOptions, axis: usize) -> FlowPrimaryTableOptions {
    match axis {
        0 => options.maximum_decoded_bytes -= 1,
        1 => options.maximum_items -= 1,
        2 => options.maximum_elements -= 1,
        3 => options.maximum_rows -= 1,
        4 => options.maximum_wide_blocks -= 1,
        5 => options.maximum_scopes -= 1,
        6 => options.maximum_fields -= 1,
        7 => options.maximum_values -= 1,
        _ => panic!("Expected resource axis0..8"),
    }
    options
}

#[test]
fn immutable_primary_views_and_edits_are_send_sync() {
    assert_send_sync::<FlowPrimaryTable>();
    assert_send_sync::<FlowPrimaryTableEdit>();
    assert_send_sync::<FlowPrimaryTableOptions>();
    assert_send_sync::<FlowPrimaryTableProfile>();
    assert_send_sync::<FlowPrimaryFieldId>();
    assert_send_sync::<FlowPrimaryField<'static>>();
    assert_send_sync::<FlowPrimaryScope>();
    assert_send_sync::<FlowPrimaryScopeView>();
    assert_send_sync::<FlowPrimaryCoverage>();
    assert_send_sync::<FlowPrimaryFieldLayout>();
    assert_send_sync::<FlowPrimaryDiagnostic>();
    assert_send_sync::<FlowPrimaryFieldKind>();
    assert_send_sync::<FlowPrimaryEncoding>();
    assert_send_sync::<FlowPrimaryValue>();
}

#[test]
fn eight_primary_workers_match_complete_serial_field_results() {
    let case = compact();
    let table = open(&case.raw, options()).unwrap();
    let expected = transcript(&table);
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8).map(|_| scope.spawn(|| transcript(&table))).collect();
        for worker in workers {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn populated_unknown_factories_return_typed_errors_and_retained_views_isolate_mutated_inputs() {
    let mut case = compact();
    let table = open(&case.raw, options()).unwrap();
    let original = table.decoded_bytes().to_vec();
    let factory = case.items[0].start as usize + 1;
    for tag in [0, 5, 127, 255] {
        case.raw[factory] = tag;
        let error = open(&case.raw, options()).unwrap_err();
        assert_eq!(error.class, bessemer::FailureClass::Unsupported);
        assert_eq!(error.operation, "Flow legacy tables");
        assert!(error.entity.contains("439"));
        assert!(error.cause.contains(&tag.to_string()) && error.cause.contains("expected"));
    }
    case.raw.fill(0);
    drop(case);
    assert_eq!(table.decoded_bytes(), original);
}

#[test]
fn concurrent_primary_edits_replay_serial_bytes_without_mutating_the_shared_checkpoint() {
    let case = compact();
    let table = open(&case.raw, options()).unwrap();
    let serial: Vec<_> = (0..8).map(|worker| edited_bytes(&table, worker)).collect();
    let shared = &table;
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|worker| scope.spawn(move || edited_bytes(shared, worker)))
            .collect();
        for (worker, expected) in workers.into_iter().zip(serial) {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(table.decoded_bytes(), case.raw);
}

fn edited_bytes(table: &FlowPrimaryTable, worker: i32) -> Vec<u8> {
    let field = table
        .fields()
        .find(|field| field.kind() == FlowPrimaryFieldKind::Members)
        .unwrap();
    table
        .replace_field(&field.id(), 0, FlowPrimaryValue::Signed32(42 + worker))
        .unwrap()
        .result()
        .source()
        .as_bytes()
        .to_vec()
}

fn transcript(
    table: &FlowPrimaryTable,
) -> Vec<(
    FlowPrimaryScope,
    FlowPrimaryFieldKind,
    Vec<FlowPrimaryValue>,
)> {
    table
        .fields()
        .map(|field| {
            (
                field.scope(),
                field.kind(),
                (0..field.layout().element_count)
                    .map(|index| field.value(index).unwrap())
                    .collect(),
            )
        })
        .collect()
}
