use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "edit.rs"]
mod edit;
#[path = "replay.rs"]
mod replay;
fn context_options() -> FlowSelectionContextOptions {
    FlowSelectionContextOptions {
        assumption: FlowSelectionMapAssumption::FreshEmptyMap,
        maximum_entries: 100_000,
        maximum_selected_values: 1_000_000,
    }
}
fn options() -> FlowSelectedValueOptions {
    FlowSelectedValueOptions {
        maximum_payload_bytes: 2_000_000,
        maximum_values: 1_000_000,
    }
}
fn context(row: &SelectionCase) -> FlowSelectionContext {
    FlowSelectionContext::derive(
        &input_bindings::links(row),
        &input_bindings::states(row),
        context_options(),
    )
    .unwrap()
}
fn open(
    raw: &[u8],
    context: &FlowSelectionContext,
    options: FlowSelectedValueOptions,
) -> bessemer::Result<FlowSelectedValues> {
    let carrier = state_carrier::carrier(112, raw);
    let id = carrier.records().nth(1).unwrap().id;
    carrier.record_112_selected_values(&id, context, options)
}
fn check_context(row: &SelectionCase, context: &FlowSelectionContext) {
    assert_eq!(context.options(), context_options());
    assert_eq!(context.link_count(), row.expected.link_count as usize);
    assert_eq!(
        context.selected_count(),
        row.expected.selected_count as usize
    );
    assert_eq!(
        context.selected_ordinals(),
        row.expected
            .selected_ordinals
            .iter()
            .map(|x| *x as usize)
            .collect::<Vec<_>>()
    );
    assert_eq!(context.links().decoded_bytes(), row.inputs.links_decoded);
    assert_eq!(context.states().decoded_bytes(), row.inputs.states_decoded);
    check_dense_indices(row, context);
    check_context_coverage(row, context);
}
fn check_context_coverage(row: &SelectionCase, context: &FlowSelectionContext) {
    let proof = context.coverage();
    assert_eq!(
        proof.mode,
        FlowSelectionCheckMode::ExactFreshMapFromSerializedInputs
    );
    assert_eq!(proof.link_mode, context.links().coverage().mode);
    assert_eq!(proof.state_mode, FlowLegacyTableCheckMode::ExactZlibTable);
    assert_eq!(proof.links, row.inputs.entries.len());
    assert_eq!(proof.state_bytes, row.inputs.states_decoded.len() - 8);
    assert_eq!(proof.selected_values, row.expected.selected_count as usize);
    assert_eq!(proof.absent_entries, proof.links - proof.selected_values);
    assert_eq!(proof.assumption, context.options().assumption);
    assert_eq!(proof.achieved_error, 0);
    check_diagnostics(context.diagnostics());
}
fn check_diagnostics(actual: &[FlowSelectionDiagnostic]) {
    use FlowSelectionDiagnostic::*;
    assert_eq!(
        actual,
        &[
            FreshMapAssumed,
            NativeOwnerBindingNotVerified,
            NativeRuntimeScatterNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked
        ]
    );
}
fn check_case(row: &SelectionCase) -> FlowSelectedValues {
    let context = context(row);
    check_context(row, &context);
    let values = open(&row.expected.values, &context, options()).unwrap();
    assert_eq!(values.decoded_bytes(), row.expected.values);
    assert_eq!(values.options(), options());
    assert_eq!(
        values.context().selected_ordinals(),
        context.selected_ordinals()
    );
    check_values(row, &values);
    check_value_coverage(row, &values);
    values
}
fn check_values(row: &SelectionCase, values: &FlowSelectedValues) {
    let array = values.values();
    assert_eq!(array.as_bytes(), row.expected.values);
    assert_eq!(
        array.payload_interval(),
        (0, row.expected.value_end as usize)
    );
    assert_eq!(array.word_count(), row.expected.selected_count as usize);
    for (index, bytes) in row.expected.values.as_chunks::<8>().0.iter().enumerate() {
        let bits = u64::from_le_bytes(*bytes);
        let id = values.value_id(index).unwrap();
        assert_eq!(array.raw_bits(index), Some(bits));
        assert_eq!(values.value(&id).unwrap(), bits);
        check_id(values, &id, index);
    }
    for (ordinal, expected) in row.expected.entry_patterns.iter().enumerate() {
        check_entry_value(values, ordinal, *expected);
    }
    assert!(array.raw_bits(usize::MAX).is_none() && values.value_id(usize::MAX).is_err());
    assert!(values.entry_value(usize::MAX).is_err() && values.entry_value_id(usize::MAX).is_err());
}
fn check_id(values: &FlowSelectedValues, id: &FlowSelectedValueId, index: usize) {
    assert_eq!(id.index(), index);
    assert_eq!(id.record(), values.record());
    assert_eq!(
        id.context_records(),
        (
            values.context().links().record(),
            values.context().states().record()
        )
    );
    assert_eq!(id.assumption(), FlowSelectionMapAssumption::FreshEmptyMap);
}
fn check_entry_value(values: &FlowSelectedValues, ordinal: usize, expected: Option<u64>) {
    assert_eq!(values.entry_value(ordinal).unwrap(), expected);
    let id = values.entry_value_id(ordinal).unwrap();
    assert_eq!(id.as_ref().map(|id| values.value(id).unwrap()), expected);
}
fn check_value_coverage(row: &SelectionCase, values: &FlowSelectedValues) {
    let proof = values.coverage();
    assert_eq!(proof.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(proof.payload_bytes, row.expected.value_end as usize);
    assert_eq!(proof.links, row.expected.link_count as usize);
    assert_eq!(proof.selected_values, row.expected.parsed_count as usize);
    assert_eq!(proof.stored_values, proof.selected_values);
    assert_eq!(proof.absent_entries, proof.links - proof.selected_values);
    assert_eq!(proof.achieved_error, 0);
    check_diagnostics(values.diagnostics());
}
#[test]
fn every_original_fresh_context_raw_value_and_optional_projection_matches_the_typed_api() {
    for row in fixture::cases() {
        check_case(&row);
    }
}

fn check_dense_indices(row: &SelectionCase, context: &FlowSelectionContext) {
    for (ordinal, index) in row.expected.dense_indices.iter().enumerate() {
        assert_eq!(
            context.dense_index(ordinal).unwrap(),
            (*index >= 0).then_some(*index as usize)
        );
    }
    assert!(context.dense_index(usize::MAX).is_err());
}
