use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "replay.rs"]
mod replay;

fn options() -> FlowCountedByteOptions {
    FlowCountedByteOptions {
        maximum_payload_bytes: 2_000_000,
        maximum_values: 1_000_000,
    }
}
fn open(raw: &[u8], options: FlowCountedByteOptions) -> bessemer::Result<FlowCountedBytes> {
    let carrier = state_carrier::carrier(12, raw);
    let id = carrier.records().nth(1).unwrap().id;
    carrier.record_12_counted_bytes(&id, options)
}
fn check_case(row: &CountedCase) -> FlowCountedBytes {
    let values = open(&row.raw, options()).unwrap();
    assert_eq!(values.options(), options());
    assert_eq!(values.decoded_bytes(), row.raw);
    assert_eq!(values.raw_bytes(), row.values);
    assert_eq!(values.stored_count(), row.count as usize);
    assert_eq!(values.payload_interval(), (4, row.end as usize));
    for (index, expected) in row.values.iter().enumerate() {
        assert_eq!(values.byte(index), Some(*expected));
    }
    assert!(values.byte(values.stored_count()).is_none() && values.byte(usize::MAX).is_none());
    check_coverage(&values, row);
    values
}
fn check_coverage(values: &FlowCountedBytes, row: &CountedCase) {
    let proof = values.coverage();
    assert_eq!(proof.mode, FlowLegacyTableCheckMode::ExactRawTable);
    assert_eq!(proof.stored_bytes, row.raw.len());
    assert_eq!(proof.decoded_bytes, row.raw.len());
    assert_eq!(proof.raw_bytes, row.values.len());
    assert_eq!(proof.wire_rows, 0);
    assert_eq!(proof.wide_words, 0);
    assert_eq!(proof.raw_words, 0);
    assert_eq!(proof.achieved_error, 0);
    use FlowSimpleTableDiagnostic::*;
    assert_eq!(
        values.diagnostics(),
        &[
            EngineeringRolesNotResolved,
            NativeRuntimeNotReproduced,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn every_owned_schema_count_and_raw_byte_matches_the_complete_typed_view() {
    for row in cases() {
        check_case(&row);
    }
}
fn selected_indices(row: &CountedCase) -> BTreeSet<usize> {
    let count = row.values.len();
    if count <= 257 || row.native_count.is_some() {
        return (0..count).collect();
    }
    [0, count / 2, count.saturating_sub(1)]
        .into_iter()
        .filter(|index| *index < count)
        .collect()
}
#[test]
fn selected_existing_bytes_have_literal_patches_noops_history_and_fresh_whole_carrier_reversals() {
    for row in cases() {
        let values = check_case(&row);
        for index in selected_indices(&row) {
            check_edit(&values, index);
        }
    }
}
fn check_edit(values: &FlowCountedBytes, index: usize) {
    let old = values.byte(index).unwrap();
    let edit = values.replace_byte(index, !old).unwrap();
    let mut expected = values.decoded_bytes().to_vec();
    expected[4 + index] = !old;
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(
        edit.source().source().as_bytes(),
        values.source().as_bytes()
    );
    state_carrier::check_history(edit.carrier_edit());
    check_fresh_reverse(values, &edit, index, old);
    check_noop(values, index, old);
}
fn check_fresh_reverse(
    values: &FlowCountedBytes,
    edit: &FlowSimpleTableEdit<FlowCountedBytes>,
    index: usize,
    old: u8,
) {
    let fresh = edit
        .result()
        .source()
        .record_12_counted_bytes(edit.result().record(), options())
        .unwrap();
    let reverse = fresh.replace_byte(index, old).unwrap();
    assert_eq!(
        reverse.result().source().as_bytes(),
        values.source().as_bytes()
    );
}
fn check_noop(values: &FlowCountedBytes, index: usize, old: u8) {
    let noop = values.replace_byte(index, old).unwrap();
    assert_eq!(
        noop.result().source().as_bytes(),
        values.source().as_bytes()
    );
    state_carrier::check_history(noop.carrier_edit());
}
