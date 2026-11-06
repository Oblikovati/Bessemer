use super::*;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_STORED_STATE_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_STORED_STATE_END";

#[test]
fn independent_processes_replay_every_source_verified_state_and_edit_byte_for_byte() {
    assert_eq!(process(), process());
}
fn process() -> Vec<u8> {
    crate::state_process::transcript(
        "stored_api::replay::stored_state_process_worker",
        BEGIN,
        END,
    )
}
#[test]
#[ignore = "The regular two-process test invokes this Source-verifying worker twice"]
fn stored_state_process_worker() {
    let mut transcript = Vec::new();
    for case in fixture::cases()
        .into_iter()
        .filter(|case| case.identifier == 432)
    {
        let state = open(&case.raw, 20).unwrap();
        for member in MEMBERS {
            check_member(&state, &case, member);
        }
        check_coverage(&state);
        transcript.extend_from_slice(format!("{}:{:?}\n", case.name, state.coverage()).as_bytes());
        append_edits(&mut transcript, &state);
    }
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}
fn append_edits(transcript: &mut Vec<u8>, state: &FlowStoredState) {
    transcript.extend_from_slice(state.source().as_bytes());
    for member in MEMBERS {
        if let Some(old) = state.member(member) {
            let edit = state.replace_member(member, !old).unwrap();
            check_carrier_history(edit.carrier_edit());
            transcript.extend_from_slice(edit.result().source().as_bytes());
        }
    }
}
