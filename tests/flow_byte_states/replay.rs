use super::*;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_BYTE_STATE_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_BYTE_STATE_END";
#[test]
fn two_fresh_processes_match_all_source_checked_views_and_selected_compressed_edits() {
    let worker = "api::replay::byte_state_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
#[test]
#[ignore = "The regular two-process test invokes this Source-verifying worker twice"]
fn byte_state_process_worker() {
    let mut transcript = Vec::new();
    for row in cases() {
        let states = check_case(&row);
        transcript.extend_from_slice(format!("{}:{:?}\n", row.name, states.coverage()).as_bytes());
        transcript.extend_from_slice(states.source().as_bytes());
        let edit = changed_header(&states, !row.header);
        transcript.extend_from_slice(edit.result().source().as_bytes());
        for index in selected_indices(&row) {
            let edit = changed_byte(&states, index);
            transcript.extend_from_slice(edit.result().source().as_bytes());
        }
    }
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}
