use super::*;
use std::io::Write;
const WORKER: &str = "api::replay::load_state_process_worker";
const BEGIN: &[u8] = b"BESSEMER_LOAD_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_LOAD_END";

#[test]
fn load_state_public_types_are_send_sync() {
    assert_traits::<FlowLoadMode>();
    assert_traits::<FlowLoadStateOptions>();
    assert_traits::<FlowLoadState>();
    assert_traits::<FlowLoadScope>();
    assert_traits::<FlowLoadScopeView>();
    assert_traits::<FlowLoadFieldKind>();
    assert_traits::<FlowLoadEncoding>();
    assert_traits::<FlowLoadValue>();
    assert_traits::<FlowLoadMemberOffset>();
    assert_traits::<FlowLoadFieldId>();
    assert_traits::<FlowLoadField<'static>>();
    assert_traits::<FlowLoadText<'static>>();
    assert_traits::<FlowLoadCheckMode>();
    assert_traits::<FlowLoadCoverage>();
    assert_traits::<FlowLoadDiagnostic>();
    assert_traits::<FlowLoadStateEdit>();
    assert_traits::<FlowLoadFieldLayout>();
}
fn assert_traits<T: Send + Sync>() {}
#[test]
fn concurrent_load_state_edits_match_serial_bytes_and_preserve_shared_inputs() {
    for case in contracts::compact_cases() {
        let state = read::check_case(&case);
        let serial: Vec<_> = (0..8).map(|index| edited(&state, index)).collect();
        let shared = &state;
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..8)
                .map(|index| scope.spawn(move || edited(shared, index)))
                .collect();
            for (worker, expected) in workers.into_iter().zip(serial) {
                assert_eq!(worker.join().unwrap(), expected);
            }
        });
        assert_eq!(state.decoded_bytes(), case.raw);
    }
}
fn edited(state: &FlowLoadState, index: i32) -> Vec<u8> {
    state
        .replace_field(
            &member_field(state).id(),
            0,
            FlowLoadValue::Scalar(FlowPrimaryValue::Signed32(42 + index)),
        )
        .unwrap()
        .result()
        .source()
        .as_bytes()
        .to_vec()
}
#[test]
fn independent_load_state_processes_replay_every_source_checked_value_and_prefix_byte_for_byte() {
    assert_eq!(process(), process());
}
fn process() -> Vec<u8> {
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", WORKER, "--ignored", "--nocapture"])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stdout)
    );
    assert!(result.stderr.is_empty());
    let start = marker(&result.stdout, BEGIN) + BEGIN.len();
    let end = marker(&result.stdout, END);
    result.stdout[start..end].to_vec()
}
fn marker(raw: &[u8], pattern: &[u8]) -> usize {
    raw.windows(pattern.len())
        .position(|bytes| bytes == pattern)
        .unwrap()
}
#[test]
#[ignore = "The regular separate-process test invokes this Source-verifying worker twice"]
fn load_state_process_worker() {
    let mut transcript = Vec::new();
    for case in fixture::cases() {
        let state = read::check_case(&case);
        transcript.extend_from_slice(format!("{}:{:?}\n", case.name, state.coverage()).as_bytes());
        append_state(&mut transcript, &state);
    }
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}
#[test]
fn retained_load_state_views_isolate_input_mutation_destruction_success_and_failure() {
    for case in contracts::compact_cases() {
        check_isolation(case);
    }
}

fn append_state(transcript: &mut Vec<u8>, state: &FlowLoadState) {
    for field in state.fields() {
        transcript.extend_from_slice(
            format!(
                "{}:{:?}:{:?}\n",
                field_path(field),
                field.encoding(),
                field.layout()
            )
            .as_bytes(),
        );
        for index in 0..field.layout().element_count {
            transcript.extend_from_slice(&edit::encoded(field.value(index).unwrap()));
        }
    }
    for text in state.texts() {
        transcript.extend_from_slice(
            format!("{:?}:{}\n", text.scope(), text.native_prefix_units()).as_bytes(),
        );
        transcript.extend_from_slice(text.native_prefix_bytes());
    }
}

fn check_isolation(mut case: LoadCase) {
    let state = read::check_case(&case);
    let original = case.raw.clone();
    let field = member_field(&state);
    let edit = state
        .replace_field(
            &field.id(),
            0,
            FlowLoadValue::Scalar(FlowPrimaryValue::Signed32(42)),
        )
        .unwrap();
    assert!(
        state
            .replace_field(&field.id(), 0, FlowLoadValue::Utf16Unit(9))
            .is_err()
    );
    case.raw.fill(0);
    drop(case);
    assert_eq!(state.decoded_bytes(), original);
    assert_eq!(edit.source().decoded_bytes(), original);
    assert_ne!(edit.result().decoded_bytes(), original);
}
