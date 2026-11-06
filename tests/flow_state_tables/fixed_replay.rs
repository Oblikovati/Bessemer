use super::*;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_FIXED_STATE_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_FIXED_STATE_END";
fn assert_traits<T: Send + Sync>() {}
#[test]
fn fixed_public_types_are_send_sync() {
    assert_traits::<FlowFixedState>();
    assert_traits::<FlowFixedStateOptions>();
    assert_traits::<FlowFixedStateRow<'static>>();
    assert_traits::<FlowFixedStateWideMember>();
    assert_traits::<FlowFixedStateCoverage>();
    assert_traits::<FlowFixedStateDiagnostic>();
    assert_traits::<FlowFixedStateEdit>();
    assert_traits::<FlowBlockWordArray<'static>>();
    assert_traits::<FlowTableWideArray<'static>>();
    assert_traits::<FlowFixedSettingsKind>();
    assert_traits::<FlowFixedSlot>();
}
#[test]
fn fixed_states_isolate_source_mutation_failures_and_concurrent_edits_against_serial_carrier_bytes()
{
    let case = cases()
        .into_iter()
        .find(|case| case.name == "fixed-v4-n2")
        .unwrap();
    let mut raw = case.raw.clone();
    let state = open(&raw, options()).unwrap();
    raw.fill(255);
    drop(raw);
    check_concurrent_edits(&state);
    check_retained_state(&state, &case.raw);
}
fn check_concurrent_edits(state: &FlowFixedState) {
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
fn edits(state: &FlowFixedState, worker: u32) -> Vec<Vec<u8>> {
    (0..32)
        .map(|step| {
            let edit = edited(state, worker, step);
            state_carrier::check_history(edit.carrier_edit());
            edit.result().source().as_bytes().to_vec()
        })
        .collect()
}
fn edited(state: &FlowFixedState, worker: u32, step: u32) -> FlowFixedStateEdit {
    let row = worker as usize % 2;
    let value = worker * 32 + step;
    match step % 4 {
        0 => state.replace_wide_member(row, MEMBERS[step as usize % 2], u64::from(value)),
        1 => state.replace_setting(
            row,
            FlowFixedSettingsKind::Integer,
            step as usize % 100,
            FlowFixedSlot::Integer(value),
        ),
        2 => state.replace_setting(
            row,
            FlowFixedSettingsKind::Numeric,
            step as usize % 320,
            FlowFixedSlot::Numeric(0x7ff8000000000000 | u64::from(value)),
        ),
        _ => state.replace_final_member(row, value as i32),
    }
    .unwrap()
}
fn check_retained_state(state: &FlowFixedState, original: &[u8]) {
    assert!(
        state
            .replace_setting(
                usize::MAX,
                FlowFixedSettingsKind::Integer,
                0,
                FlowFixedSlot::Integer(0)
            )
            .is_err()
    );
    let changed = state
        .source()
        .replace_payload(state.record(), &[0; 4])
        .unwrap();
    assert_ne!(changed.result().as_bytes(), state.source().as_bytes());
    assert_eq!(state.decoded_bytes(), original);
}
#[test]
fn independent_fixed_processes_replay_every_source_checked_state_and_selected_edits_byte_for_byte()
{
    assert_eq!(process(), process());
}
fn process() -> Vec<u8> {
    crate::state_process::transcript("fixed_api::replay::fixed_process_worker", BEGIN, END)
}
#[test]
#[ignore = "The regular two-process test invokes this Source-verifying worker twice"]
fn fixed_process_worker() {
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
fn append_edits(transcript: &mut Vec<u8>, state: &FlowFixedState) {
    for row in state.rows() {
        append_row_edits(transcript, state, row);
    }
}
fn append_row_edits(transcript: &mut Vec<u8>, state: &FlowFixedState, row: FlowFixedStateRow<'_>) {
    for member in MEMBERS {
        let edit = state
            .replace_wide_member(row.ordinal(), member, !row.wide_member(member))
            .unwrap();
        transcript.extend_from_slice(edit.result().source().as_bytes());
    }
    for kind in [
        FlowFixedSettingsKind::Numeric,
        FlowFixedSettingsKind::Integer,
    ] {
        append_settings(transcript, state, row, kind);
    }
    append_final(transcript, state, row);
}
fn append_final(transcript: &mut Vec<u8>, state: &FlowFixedState, row: FlowFixedStateRow<'_>) {
    if let Some(old) = row.final_member() {
        transcript.extend_from_slice(
            state
                .replace_final_member(row.ordinal(), !old)
                .unwrap()
                .result()
                .source()
                .as_bytes(),
        );
    }
}
fn append_settings(
    transcript: &mut Vec<u8>,
    state: &FlowFixedState,
    row: FlowFixedStateRow<'_>,
    kind: FlowFixedSettingsKind,
) {
    if row.settings_wides().is_none() {
        return;
    }
    for index in [0, kind.slot_count() / 2, kind.slot_count() - 1] {
        let (_, _, next) = edit::setting_values(row, kind, index);
        let edit = state
            .replace_setting(row.ordinal(), kind, index, next)
            .unwrap();
        transcript.extend_from_slice(edit.result().source().as_bytes());
    }
}
