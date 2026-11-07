use super::*;
use bessemer::flow::*;
#[path = "contracts.rs"]
mod contracts;
#[path = "replay.rs"]
mod replay;

fn options() -> FlowByteStateOptions {
    FlowByteStateOptions {
        maximum_decoded_bytes: 2_000_000,
        maximum_values: 1_000_000,
    }
}
fn open(stored: &[u8], options: FlowByteStateOptions) -> bessemer::Result<FlowByteStates> {
    let carrier = state_carrier::carrier(316, stored);
    let id = carrier.records().nth(1).unwrap().id;
    carrier.record_316_byte_states(&id, options)
}
fn check_case(row: &ByteStateCase) -> FlowByteStates {
    let states = open(&row.stored, options()).unwrap();
    assert_eq!(states.options(), options());
    assert_eq!(states.header(), row.header);
    assert_eq!(states.stored_count(), row.count as usize);
    assert_eq!(states.decoded_bytes(), row.decoded);
    assert_eq!(states.raw_bytes(), row.values);
    assert_eq!(states.payload_interval(), (8, row.end as usize));
    for (index, value) in row.values.iter().enumerate() {
        assert_eq!(states.byte(index), Some(*value));
    }
    assert!(states.byte(row.values.len()).is_none() && states.byte(usize::MAX).is_none());
    check_coverage(&states, row);
    states
}
fn check_coverage(states: &FlowByteStates, row: &ByteStateCase) {
    let proof = states.coverage();
    assert_eq!(proof.mode, FlowLegacyTableCheckMode::ExactZlibTable);
    assert_eq!(proof.stored_bytes, row.stored.len());
    assert_eq!(proof.decoded_bytes, row.decoded.len());
    assert_eq!(proof.raw_bytes, row.values.len());
    assert_eq!(proof.wire_rows, 0);
    assert_eq!(proof.wide_words, 0);
    assert_eq!(proof.raw_words, 0);
    assert_eq!(proof.achieved_error, 0);
    use FlowSimpleTableDiagnostic::*;
    assert_eq!(
        states.diagnostics(),
        &[
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked
        ]
    );
}
#[test]
fn every_complete_byte_state_matches_all_source_header_count_and_raw_values() {
    for row in cases() {
        check_case(&row);
    }
}
fn selected_indices(row: &ByteStateCase) -> BTreeSet<usize> {
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
fn every_header_and_selected_byte_has_independent_patch_noop_history_and_fresh_decoded_reverse() {
    for row in cases() {
        let states = check_case(&row);
        let header_edit = changed_header(&states, !row.header);
        check_header_reverse(&states, &header_edit);
        for index in selected_indices(&row) {
            check_byte_edit(&states, index);
        }
    }
}
fn changed_header(states: &FlowByteStates, header: i32) -> FlowSimpleTableEdit<FlowByteStates> {
    let edit = states.replace_header(header).unwrap();
    check_patch(states, &edit, 0, &header.to_le_bytes());
    edit
}
fn changed_byte(states: &FlowByteStates, index: usize) -> FlowSimpleTableEdit<FlowByteStates> {
    let value = !states.byte(index).unwrap();
    let edit = states.replace_byte(index, value).unwrap();
    check_patch(states, &edit, 8 + index, &[value]);
    edit
}
fn check_patch(
    states: &FlowByteStates,
    edit: &FlowSimpleTableEdit<FlowByteStates>,
    offset: usize,
    bytes: &[u8],
) {
    let mut expected = states.decoded_bytes().to_vec();
    expected[offset..offset + bytes.len()].copy_from_slice(bytes);
    assert_eq!(edit.result().decoded_bytes(), expected);
    assert_eq!(
        edit.source().source().as_bytes(),
        states.source().as_bytes()
    );
    assert_eq!(edit.result().options(), states.options());
    assert_eq!(edit.result().stored_count(), states.stored_count());
    state_carrier::check_history(edit.carrier_edit());
}
fn fresh(edit: &FlowSimpleTableEdit<FlowByteStates>) -> FlowByteStates {
    edit.result()
        .source()
        .record_316_byte_states(edit.result().record(), edit.result().options())
        .unwrap()
}
fn check_header_reverse(states: &FlowByteStates, edit: &FlowSimpleTableEdit<FlowByteStates>) {
    let reverse = fresh(edit).replace_header(states.header()).unwrap();
    assert_eq!(reverse.result().decoded_bytes(), states.decoded_bytes());
    state_carrier::check_history(reverse.carrier_edit());
    let noop = states.replace_header(states.header()).unwrap();
    assert_eq!(
        noop.result().source().as_bytes(),
        states.source().as_bytes()
    );
    state_carrier::check_history(noop.carrier_edit());
}
fn check_byte_edit(states: &FlowByteStates, index: usize) {
    let edit = changed_byte(states, index);
    let reverse = fresh(&edit)
        .replace_byte(index, states.byte(index).unwrap())
        .unwrap();
    assert_eq!(reverse.result().decoded_bytes(), states.decoded_bytes());
    state_carrier::check_history(reverse.carrier_edit());
    let noop = states
        .replace_byte(index, states.byte(index).unwrap())
        .unwrap();
    assert_eq!(
        noop.result().source().as_bytes(),
        states.source().as_bytes()
    );
    state_carrier::check_history(noop.carrier_edit());
}
