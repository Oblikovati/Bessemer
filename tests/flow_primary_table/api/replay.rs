use super::*;
use std::io::Write;
use std::process::Command;
const WORKER: &str = "api::replay::export_primary_process_transcript";
const BEGIN: &[u8] = b"BESSEMER_PRIMARY_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_PRIMARY_END";

#[test]
fn two_fresh_primary_processes_match_every_independent_source_field_byte_for_byte() {
    assert_eq!(run_worker(), run_worker());
}

fn run_worker() -> Vec<u8> {
    let result = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", WORKER, "--ignored", "--nocapture"])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stdout)
    );
    assert!(
        result.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    frame(&result.stdout).to_vec()
}

fn frame(raw: &[u8]) -> &[u8] {
    let start = raw
        .windows(BEGIN.len())
        .position(|bytes| bytes == BEGIN)
        .unwrap()
        + BEGIN.len();
    let end = raw
        .windows(END.len())
        .position(|bytes| bytes == END)
        .unwrap();
    &raw[start..end]
}

#[test]
#[ignore = "Subprocess worker; the regular two-process test invokes this twice and compares framed output"]
fn export_primary_process_transcript() {
    let mut transcript = Vec::new();
    for case in fixture::cases() {
        let table = open(&case.raw, options()).unwrap();
        assert_eq!(table.scopes().collect::<Vec<_>>(), expected::scopes(&case));
        let observed = actual_leaves(&table);
        assert_eq!(observed, expected::leaves(&case), "{}", case.name);
        transcript.extend_from_slice(
            format!("{}:{:?}:{observed:?}\n", case.name, table.coverage()).as_bytes(),
        );
    }
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}

fn actual_leaves(table: &FlowPrimaryTable) -> expected::Leaves {
    let mut result = expected::Leaves::new();
    for field in table.fields() {
        for index in 0..field.layout().element_count {
            let offset = field.element_offset(index).unwrap();
            assert!(
                result
                    .insert(
                        offset,
                        (field.value(index).unwrap(), field.scope(), field.kind())
                    )
                    .is_none()
            );
        }
    }
    result
}
