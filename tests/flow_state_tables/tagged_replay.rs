use super::*;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_TAGGED_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_TAGGED_END";
fn assert_traits<T: Send + Sync>() {}
#[test]
fn tagged_public_types_are_send_sync() {
    assert_traits::<FlowTaggedState>();
    assert_traits::<FlowTaggedStateOptions>();
    assert_traits::<FlowTaggedStateRow<'static>>();
    assert_traits::<FlowTaggedStateCoverage>();
    assert_traits::<FlowTaggedStateDiagnostic>();
    assert_traits::<FlowTaggedStateEdit>();
    assert_traits::<FlowTableWideArray<'static>>();
}
#[test]
fn tagged_states_isolate_input_mutation_failure_and_concurrent_edits_against_serial_bytes() {
    let case = cases()
        .into_iter()
        .find(|case| case.name == "tag-255-w3")
        .unwrap();
    let mut raw = case.raw.clone();
    let state = open(&raw, options()).unwrap();
    raw.fill(255);
    drop(raw);
    check_concurrent_edits(&state);
    check_retained_state(&state, &case.raw);
}
fn check_concurrent_edits(state: &FlowTaggedState) {
    let serial: Vec<_> = (0..8).map(|worker| edits(state, worker)).collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|worker| scope.spawn(move || edits(state, worker)))
            .collect();
        for (worker, expected) in workers.into_iter().zip(serial) {
            assert_eq!(worker.join().unwrap(), expected);
        }
    });
}
fn edits(state: &FlowTaggedState, worker: u32) -> Vec<Vec<u8>> {
    (0..32)
        .map(|step| {
            let value = worker * 32 + step;
            let edit = match step % 3 {
                0 => state.replace_tag(1, (value % 255 + 1) as u8),
                1 => {
                    state.replace_wide(2, step as usize % 4, 0x7ff8000000000000 | u64::from(value))
                }
                _ => state.replace_tail(value as i32),
            }
            .unwrap();
            state_carrier::check_history(edit.carrier_edit());
            edit.result().source().as_bytes().to_vec()
        })
        .collect()
}
fn check_retained_state(state: &FlowTaggedState, original: &[u8]) {
    assert!(state.replace_tag(1, 0).is_err());
    assert!(state.replace_wide(2, usize::MAX, 0).is_err());
    let changed = state
        .source()
        .replace_payload(state.record(), &[9, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    assert_ne!(changed.result().as_bytes(), state.source().as_bytes());
    assert_eq!(state.decoded_bytes(), original);
}
#[test]
fn independent_tagged_processes_replay_every_source_checked_read_and_selected_edits_byte_for_byte()
{
    assert_eq!(process(), process());
}
fn process() -> Vec<u8> {
    crate::state_process::transcript("tagged_api::replay::tagged_process_worker", BEGIN, END)
}
#[test]
#[ignore = "The regular two-process test invokes this Source-verifying worker twice"]
fn tagged_process_worker() {
    let mut transcript = Vec::new();
    for case in cases() {
        let state = check_case(&case);
        transcript.extend_from_slice(format!("{}:{:?}\n", case.name, state.coverage()).as_bytes());
        transcript.extend_from_slice(state.source().as_bytes());
        append_edits(&mut transcript, &state, &case);
    }
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}
fn append_edits(transcript: &mut Vec<u8>, state: &FlowTaggedState, case: &fixture::StateCase) {
    for ordinal in selected_rows(state) {
        append_row_edits(
            transcript,
            state,
            state.row(ordinal).unwrap(),
            case.name.starts_with("native-"),
        );
    }
    if let Some(old) = state.tail() {
        transcript.extend_from_slice(
            state
                .replace_tail(!old)
                .unwrap()
                .result()
                .source()
                .as_bytes(),
        );
    }
}
fn append_row_edits(
    transcript: &mut Vec<u8>,
    state: &FlowTaggedState,
    row: FlowTaggedStateRow<'_>,
    native: bool,
) {
    let next = if row.tag() == 0 {
        0
    } else {
        row.tag() % 255 + 1
    };
    let edit = state.replace_tag(row.ordinal(), next).unwrap();
    transcript.extend_from_slice(edit.result().source().as_bytes());
    let Some(array) = row.values() else {
        return;
    };
    for index in selected_elements(array.word_count(), native) {
        let edit = state
            .replace_wide(row.ordinal(), index, !array.raw_bits(index).unwrap())
            .unwrap();
        transcript.extend_from_slice(edit.result().source().as_bytes());
    }
}
