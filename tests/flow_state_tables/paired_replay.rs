use super::*;
use std::collections::BTreeSet;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_PAIRED_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_PAIRED_END";
fn assert_traits<T: Send + Sync>() {}
#[test]
fn paired_public_types_are_send_sync() {
    assert_traits::<FlowPairedState>();
    assert_traits::<FlowPairedStateOptions>();
    assert_traits::<FlowPairedStateGroup<'static>>();
    assert_traits::<FlowPairedStateCoverage>();
    assert_traits::<FlowPairedStateDiagnostic>();
    assert_traits::<FlowPairedStateEdit>();
    assert_traits::<FlowBlockWordArray<'static>>();
    assert_traits::<FlowTableWideArray<'static>>();
}
#[test]
fn paired_states_isolate_input_mutation_failure_and_concurrent_edits_against_serial_bytes() {
    let case = cases()
        .into_iter()
        .find(|case| case.name == "paired-(2, 3)-(3, 2)")
        .unwrap();
    let mut raw = case.raw.clone();
    let state = open(&raw, options()).unwrap();
    raw.fill(255);
    drop(raw);
    check_concurrent_edits(&state);
    check_retained_state(&state, &case.raw);
}
fn check_concurrent_edits(state: &FlowPairedState) {
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
fn check_retained_state(state: &FlowPairedState, original: &[u8]) {
    assert!(state.replace_wide(usize::MAX, 0, 1).is_err());
    let changed = state
        .source()
        .replace_payload(state.record(), &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    assert_ne!(changed.result().as_bytes(), state.source().as_bytes());
    assert_eq!(state.decoded_bytes(), original);
}
fn edits(state: &FlowPairedState, worker: u32) -> Vec<Vec<u8>> {
    (0..32)
        .map(|step| {
            let edit = if step % 2 == 0 {
                state.replace_index(0, step as usize % 2, worker * 32 + step)
            } else {
                state.replace_wide(1, step as usize % 2, u64::from(worker * 32 + step))
            }
            .unwrap();
            state_carrier::check_history(edit.carrier_edit());
            edit.result().source().as_bytes().to_vec()
        })
        .collect()
}
#[test]
fn independent_paired_processes_replay_every_source_checked_state_and_selected_edits_byte_for_byte()
{
    assert_eq!(process(), process());
}
fn process() -> Vec<u8> {
    crate::state_process::transcript("paired_api::replay::paired_process_worker", BEGIN, END)
}
#[test]
#[ignore = "The regular two-process test invokes this Source-verifying worker twice"]
fn paired_process_worker() {
    let mut transcript = Vec::new();
    for case in cases() {
        let state = check_case(&case);
        transcript.extend_from_slice(format!("{}:{:?}\n", case.name, state.coverage()).as_bytes());
        transcript.extend_from_slice(state.source().as_bytes());
        append_edits(&mut transcript, &state);
    }
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}
fn selected(count: usize) -> BTreeSet<usize> {
    [0, count / 2, count.saturating_sub(1)]
        .into_iter()
        .filter(|index| *index < count)
        .collect()
}
fn append_edits(transcript: &mut Vec<u8>, state: &FlowPairedState) {
    for group in state.groups() {
        append_group_edits(transcript, state, group);
    }
}
fn append_group_edits(
    transcript: &mut Vec<u8>,
    state: &FlowPairedState,
    group: FlowPairedStateGroup<'_>,
) {
    for index in selected(group.indices().word_count()) {
        let edit = state
            .replace_index(
                group.ordinal(),
                index,
                !group.indices().raw_word(index).unwrap(),
            )
            .unwrap();
        transcript.extend_from_slice(edit.result().source().as_bytes());
    }
    if let Some(array) = group.wides() {
        for index in selected(array.word_count()) {
            let edit = state
                .replace_wide(group.ordinal(), index, !array.raw_bits(index).unwrap())
                .unwrap();
            transcript.extend_from_slice(edit.result().source().as_bytes());
        }
    }
}
