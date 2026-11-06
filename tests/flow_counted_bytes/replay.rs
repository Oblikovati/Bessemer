use super::*;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_COUNTED_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_COUNTED_END";
#[test]
fn two_fresh_processes_match_complete_source_checked_views_and_selected_edits() {
    let worker = "api::replay::counted_byte_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
#[test]
#[ignore = "The regular two-process test invokes this Source-verifying worker twice"]
fn counted_byte_process_worker() {
    let mut transcript = Vec::new();
    for row in cases() {
        let values = check_case(&row);
        transcript.extend_from_slice(format!("{}:{:?}\n", row.name, values.coverage()).as_bytes());
        transcript.extend_from_slice(values.source().as_bytes());
        for index in selected_indices(&row) {
            append_edit(&mut transcript, &values, index);
        }
    }
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}
fn append_edit(transcript: &mut Vec<u8>, values: &FlowCountedBytes, index: usize) {
    let edit = values
        .replace_byte(index, !values.byte(index).unwrap())
        .unwrap();
    let mut expected = values.decoded_bytes().to_vec();
    expected[4 + index] = !values.byte(index).unwrap();
    assert_eq!(edit.result().decoded_bytes(), expected);
    state_carrier::check_history(edit.carrier_edit());
    transcript.extend_from_slice(edit.result().source().as_bytes());
}
