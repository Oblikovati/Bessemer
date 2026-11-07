use super::*;
#[test]
fn all_actual_context_and_value_bounds_accept_exact_counts_and_reject_one_below() {
    for row in fixture::cases() {
        check_bounds(&row);
    }
}
fn check_bounds(row: &SelectionCase) {
    let links = input_bindings::links(row);
    let states = input_bindings::states(row);
    let exact = FlowSelectionContextOptions {
        maximum_entries: row.inputs.entries.len(),
        maximum_selected_values: row.expected.selected_count as usize,
        ..context_options()
    };
    let context = FlowSelectionContext::derive(&links, &states, exact).unwrap();
    reject_reduced_context_bounds(&links, &states, exact);
    check_payload_bounds(row, &context);
}
fn check_payload_bounds(row: &SelectionCase, context: &FlowSelectionContext) {
    let exact = FlowSelectedValueOptions {
        maximum_payload_bytes: row.expected.values.len(),
        maximum_values: row.expected.selected_count as usize,
    };
    assert!(open(&row.expected.values, context, exact).is_ok());
    if exact.maximum_values == 0 {
        return;
    }
    let mut reduced = exact;
    reduced.maximum_values -= 1;
    assert!(open(&row.expected.values, context, reduced).is_err());
    reduced = exact;
    reduced.maximum_payload_bytes -= 1;
    assert!(open(&row.expected.values, context, reduced).is_err());
}
fn sample(name: &str) -> SelectionCase {
    fixture::cases()
        .into_iter()
        .find(|row| row.name == name)
        .unwrap()
}
#[test]
fn compact_payloads_reject_every_prefix_extension_and_mismatched_state_array() {
    let row = sample("selected-zero-distinct-from-absent");
    let context = context(&row);
    for end in 0..row.expected.values.len() {
        assert!(open(&row.expected.values[..end], &context, options()).is_err());
    }
    for suffix in [vec![0], vec![255; 8]] {
        let mut raw = row.expected.values.clone();
        raw.extend_from_slice(&suffix);
        assert!(open(&raw, &context, options()).is_err());
    }
    let other = sample("pattern-0");
    assert!(
        FlowSelectionContext::derive(
            &input_bindings::links(&row),
            &input_bindings::states(&other),
            context_options()
        )
        .is_err()
    );
}
#[test]
fn empty_selections_are_complete_headerless_results_with_zero_actual_budgets() {
    for name in ["empty-v0", "absent-states-v9", "grid-v9-code-ff"] {
        let row = sample(name);
        let context = context(&row);
        let values = open(
            &[],
            &context,
            FlowSelectedValueOptions {
                maximum_payload_bytes: 0,
                maximum_values: 0,
            },
        )
        .unwrap();
        assert!(values.values().as_bytes().is_empty());
        assert_eq!(values.coverage().payload_bytes, 0);
        assert!(values.value_id(0).is_err());
        assert_eq!(context.selected_count(), 0);
    }
}
#[test]
fn foreign_value_and_input_context_identities_are_rejected_without_mutation() {
    let row = sample("pattern-0");
    let values = check_case(&row);
    let id = values.value_id(0).unwrap();
    let other = check_case(&sample("pattern-1"));
    assert!(other.value(&id).is_err() && other.replace_value(&id, 7).is_err());
    let changed_states = values.context().states().replace_header(123).unwrap();
    let new_context = FlowSelectionContext::derive(
        values.context().links(),
        changed_states.result(),
        context_options(),
    )
    .unwrap();
    check_context_rejection(&values, &new_context, &id, "state revision matches false");
    assert_eq!(values.value(&id).unwrap(), 0);
    check_foreign_links(&row, &values, &id);
}
fn check_foreign_links(row: &SelectionCase, values: &FlowSelectedValues, id: &FlowSelectedValueId) {
    let links = values.context().links();
    assert_eq!(links.options().compression, FlowLegacyTableTransport::Raw);
    let mut raw = links.decoded_bytes().to_vec();
    let offset = row.inputs.entries.last().unwrap().end as usize;
    raw[offset..offset + 4].copy_from_slice(&17i32.to_le_bytes());
    let edit = links
        .source()
        .replace_payload(links.record(), &raw)
        .unwrap();
    let record = edit.result().records().nth(1).unwrap().id;
    let updated = edit
        .result()
        .record_402_links(&record, links.options())
        .unwrap();
    let context =
        FlowSelectionContext::derive(&updated, values.context().states(), context_options())
            .unwrap();
    check_context_rejection(values, &context, id, "link revision matches false");
    assert_eq!(links.decoded_bytes(), row.inputs.links_decoded);
}
#[test]
fn headerless_value_snapshots_survive_caller_mutation_and_input_drops() {
    let row = sample("selected-zero-distinct-from-absent");
    let context = context(&row);
    let mut raw = row.expected.values.clone();
    let values = open(&raw, &context, options()).unwrap();
    raw.fill(77);
    drop(raw);
    drop(context);
    assert_eq!(values.decoded_bytes(), row.expected.values);
    assert_eq!(values.entry_value(0).unwrap(), Some(0));
    assert_eq!(values.entry_value(1).unwrap(), None);
}
#[test]
fn wrong_foreign_and_stale_occurrences_fail_and_retained_inputs_survive_external_edits() {
    let row = sample("pattern-0");
    let values = check_case(&row);
    check_occurrences(&values);
    check_retained_inputs(&row, &values);
}
fn assert_shared<T: Send + Sync>() {}
#[test]
fn selected_public_types_and_reused_wide_edit_types_are_send_sync() {
    assert_shared::<FlowSelectionMapAssumption>();
    assert_shared::<FlowSelectionCheckMode>();
    assert_shared::<FlowSelectionDiagnostic>();
    assert_shared::<FlowSelectionContextOptions>();
    assert_shared::<FlowSelectionContext>();
    assert_shared::<FlowSelectionContextCoverage>();
    assert_shared::<FlowSelectedValueOptions>();
    assert_shared::<FlowSelectedValues>();
    assert_shared::<FlowSelectedValueId>();
    assert_shared::<FlowSelectedValueCoverage>();
    assert_shared::<FlowSimpleTableEdit<FlowSelectedValues>>();
    assert_shared::<FlowTableWideArray<'static>>();
}
#[test]
fn eight_workers_match_serial_context_derivation_raw_edits_and_failures() {
    let row = sample("grid-v9-code-00");
    let values = check_case(&row);
    let serial: Vec<_> = (0..8).map(|worker| worker_edits(&values, worker)).collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|worker| {
                let values = &values;
                scope.spawn(move || worker_edits(values, worker))
            })
            .collect();
        for (worker, expected) in workers.into_iter().zip(serial) {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
    assert_eq!(values.decoded_bytes(), row.expected.values);
}
fn worker_edits(values: &FlowSelectedValues, worker: usize) -> Vec<Vec<u8>> {
    (0..32)
        .map(|step| {
            let context = FlowSelectionContext::derive(
                values.context().links(),
                values.context().states(),
                context_options(),
            )
            .unwrap();
            assert_eq!(
                context.selected_ordinals(),
                values.context().selected_ordinals()
            );
            let edit = edit::changed(
                values,
                (worker * 32 + step) % values.context().selected_count(),
            );
            assert!(values.value_id(usize::MAX).is_err());
            edit.result().source().as_bytes().to_vec()
        })
        .collect()
}

fn reject_reduced_context_bounds(
    links: &FlowLinkRecords,
    states: &FlowByteStates,
    exact: FlowSelectionContextOptions,
) {
    for axis in 0..2 {
        let mut reduced = exact;
        let limit = if axis == 0 {
            &mut reduced.maximum_entries
        } else {
            &mut reduced.maximum_selected_values
        };
        if *limit == 0 {
            continue;
        }
        *limit -= 1;
        assert!(FlowSelectionContext::derive(links, states, reduced).is_err());
    }
}

fn check_occurrences(values: &FlowSelectedValues) {
    let source = values.source();
    let wrong = source.records().nth(2).unwrap().id;
    assert!(
        source
            .record_112_selected_values(&wrong, values.context(), options())
            .is_err()
    );
    let edit = edit::changed(values, 0);
    assert!(
        edit.result()
            .source()
            .record_112_selected_values(values.record(), values.context(), options())
            .is_err()
    );
}
fn check_retained_inputs(row: &SelectionCase, values: &FlowSelectedValues) {
    let changed_links = values
        .context()
        .links()
        .source()
        .replace_payload(values.context().links().record(), b"changed")
        .unwrap();
    assert_ne!(
        changed_links.result().as_bytes(),
        values.context().links().source().as_bytes()
    );
    assert_eq!(
        values.context().links().decoded_bytes(),
        row.inputs.links_decoded
    );
    assert_eq!(
        values.context().states().decoded_bytes(),
        row.inputs.states_decoded
    );
}

fn check_context_rejection(
    values: &FlowSelectedValues,
    context: &FlowSelectionContext,
    id: &FlowSelectedValueId,
    mismatch: &str,
) {
    let same_payload = values
        .source()
        .record_112_selected_values(values.record(), context, options())
        .unwrap();
    assert_eq!(same_payload.record(), values.record());
    let error = same_payload.value(id).unwrap_err();
    assert!(error.cause.contains("value revision matches true") && error.cause.contains(mismatch));
    assert!(error.cause.len() < 500);
    assert!(same_payload.replace_value(id, 0).is_err());
}
