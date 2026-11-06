use super::*;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_CONTEXT_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_CONTEXT_END";
fn assert_traits<T: Send + Sync>() {}
#[test]
fn context_public_types_are_send_sync() {
    assert_traits::<FlowContextState>();
    assert_traits::<FlowContextStateOptions>();
    assert_traits::<FlowContextStateCoverage>();
    assert_traits::<FlowContextStateDiagnostic>();
    assert_traits::<FlowContextStateEdit>();
    assert_traits::<FlowSimpleLegacyCount>();
    assert_traits::<FlowBlockWordArray<'static>>();
}
#[test]
fn shared_context_states_isolate_source_mutation_failures_and_concurrent_edits() {
    let case = cases()
        .into_iter()
        .find(|case| case.name == "context-v3-n1-c3")
        .unwrap();
    let mut raw = case.raw.clone();
    let state = open(&raw, options(case.context)).unwrap();
    raw.fill(255);
    drop(raw);
    check_concurrent_context(&state);
    check_retained_context(&state);
}
fn check_concurrent_context(state: &FlowContextState) {
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
fn check_retained_context(state: &FlowContextState) {
    assert!(state.replace_index(usize::MAX, 1).is_err());
    let original = state.decoded_bytes().to_vec();
    let changed = state
        .source()
        .replace_payload(state.record(), &[5, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    assert_ne!(changed.result().as_bytes(), state.source().as_bytes());
    assert_eq!(state.decoded_bytes(), original);
}
fn edits(state: &FlowContextState, worker: u32) -> Vec<Vec<u8>> {
    (0..32)
        .map(|step| {
            let edit = state
                .replace_index(step as usize % 3, worker * 32 + step)
                .unwrap();
            edit::check_history(edit.carrier_edit());
            edit.result().source().as_bytes().to_vec()
        })
        .collect()
}
#[test]
fn independent_context_processes_replay_every_source_checked_state_and_edit_byte_for_byte() {
    assert_eq!(process(), process());
}
fn process() -> Vec<u8> {
    crate::state_process::transcript("context_api::replay::context_process_worker", BEGIN, END)
}
#[test]
#[ignore = "The regular two-process test invokes this Source-verifying worker twice"]
fn context_process_worker() {
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
fn append_edits(transcript: &mut Vec<u8>, state: &FlowContextState) {
    for index in 0..state.indices().word_count() {
        let edit = state
            .replace_index(index, !state.indices().raw_word(index).unwrap())
            .unwrap();
        transcript.extend_from_slice(edit.result().source().as_bytes());
    }
    if let Some(tail) = state.tail() {
        transcript.extend_from_slice(
            state
                .replace_tail(!tail)
                .unwrap()
                .result()
                .source()
                .as_bytes(),
        );
    }
}
