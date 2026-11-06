use super::*;
use std::io::Write;
const WORKER: &str = "api::replay::conditional_replay_worker";
const START: &[u8] = b"BESSEMER_CONDITIONAL_START\0";
const END: &[u8] = b"\0BESSEMER_CONDITIONAL_END";

#[test]
fn separate_conditional_processes_match_complete_source_verified_field_transcripts_byte_for_byte() {
    assert_eq!(replay_process(), replay_process());
}

fn replay_process() -> Vec<u8> {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", WORKER, "--ignored", "--nocapture"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let begin = marker(&output.stdout, START) + START.len();
    let end = marker(&output.stdout, END);
    output.stdout[begin..end].to_vec()
}

fn marker(raw: &[u8], pattern: &[u8]) -> usize {
    raw.windows(pattern.len())
        .position(|bytes| bytes == pattern)
        .unwrap()
}

#[test]
#[ignore = "The regular separate-process test invokes this Source-verifying worker twice"]
fn conditional_replay_worker() {
    let mut transcript = Vec::new();
    for case in fixture::cases() {
        let items = read::check_case(&case);
        transcript.extend_from_slice(format!("{}:{:?}\n", case.name, items.coverage()).as_bytes());
        append_fields(&mut transcript, &items);
    }
    let mut output = std::io::stdout().lock();
    output.write_all(START).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}

#[test]
fn conditional_views_retain_original_input_after_success_failure_source_edit_and_input_destruction()
{
    for identifier in [126, 315, 128] {
        let case = fixture::cases()
            .into_iter()
            .find(|case| case.identifier == identifier && case.name.starts_with("item-v5-active1-"))
            .unwrap();
        check_input_isolation(case);
    }
}

fn append_fields(transcript: &mut Vec<u8>, items: &FlowConditionalItems) {
    for field in items.fields() {
        transcript.extend_from_slice(
            format!(
                "{}:{:?}:{:?}\n",
                paths::field_path(items, field),
                field.encoding(),
                field.layout()
            )
            .as_bytes(),
        );
        for index in 0..field.layout().element_count {
            transcript.extend_from_slice(format!("{:?}\n", field.value(index).unwrap()).as_bytes());
        }
    }
}

fn check_input_isolation(mut case: ConditionalCase) {
    let items = read::check_case(&case);
    let original = case.decoded.clone();
    let field = items
        .fields()
        .find(|field| field.kind() == FlowConditionalFieldKind::Member84)
        .unwrap();
    let edit = items
        .replace_field(&field.id(), 0, FlowConditionalValue::Signed32(42))
        .unwrap();
    assert!(
        items
            .replace_field(&field.id(), 0, FlowConditionalValue::Byte(7))
            .is_err()
    );
    case.stored.fill(0);
    case.decoded.fill(0);
    drop(case);
    assert_eq!(items.decoded_bytes(), original);
    assert_ne!(edit.result().decoded_bytes(), original);
    assert_eq!(edit.source().decoded_bytes(), original);
}
