use super::*;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_SELECTED_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_SELECTED_END";
#[test]
fn two_fresh_processes_match_every_source_checked_context_value_and_selected_edit() {
    let worker = "api::replay::selected_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
#[test]
#[ignore = "The regular two-process test invokes this Source-verifying worker twice"]
fn selected_process_worker() {
    let mut transcript = Vec::new();
    for row in fixture::cases() {
        let values = check_case(&row);
        transcript.extend_from_slice(
            format!(
                "{}:{:?}:{:?}\n",
                row.name,
                values.context().coverage(),
                values.coverage()
            )
            .as_bytes(),
        );
        transcript.extend_from_slice(values.source().as_bytes());
        for index in edit::indices(&row) {
            let edit = edit::changed(&values, index);
            transcript.extend_from_slice(edit.result().source().as_bytes());
        }
    }
    emit_transcript(&transcript);
}

fn emit_transcript(transcript: &[u8]) {
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(transcript).unwrap();
    output.write_all(END).unwrap();
}
