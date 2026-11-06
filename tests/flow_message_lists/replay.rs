use super::*;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_MESSAGE_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_MESSAGE_END";
#[test]
fn two_fresh_processes_replay_source_checked_messages_and_edits_byte_for_byte() {
    let worker = "api::replay::message_process_worker";
    assert_eq!(
        crate::state_process::transcript(worker, BEGIN, END),
        crate::state_process::transcript(worker, BEGIN, END)
    );
}
#[test]
#[ignore = "The regular two-process test invokes this Source-verifying worker twice"]
fn message_process_worker() {
    let mut transcript = Vec::new();
    for case in fixture::cases() {
        let messages = open(&case.raw, options()).unwrap();
        check_header(&case, &messages);
        for (expected, text) in case.texts.iter().zip(messages.texts()) {
            check_string(&case, expected, text);
        }
        transcript
            .extend_from_slice(format!("{}:{:?}\n", case.name, messages.coverage()).as_bytes());
        transcript.extend_from_slice(messages.source().as_bytes());
        append_edits(&mut transcript, &messages, &case);
    }
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}
fn append_edits(transcript: &mut Vec<u8>, messages: &FlowMessageLists, case: &MessageCase) {
    for item in messages.items() {
        check_item(case, item);
        let edit = messages
            .replace_member(item.ordinal(), !item.member_pattern())
            .unwrap();
        state_carrier::check_history(edit.carrier_edit());
        transcript.extend_from_slice(edit.result().source().as_bytes());
    }
    for text in messages.texts() {
        append_text(transcript, messages, text);
    }
}
fn append_text(transcript: &mut Vec<u8>, messages: &FlowMessageLists, text: FlowMessageText<'_>) {
    let count = text.stored_count();
    let indices: BTreeSet<_> = [0, count / 2, count.saturating_sub(1)]
        .into_iter()
        .filter(|index| *index < count)
        .collect();
    for index in indices {
        let edit = changed_text(messages, text, index);
        state_carrier::check_history(edit.carrier_edit());
        transcript.extend_from_slice(edit.result().source().as_bytes());
    }
}
