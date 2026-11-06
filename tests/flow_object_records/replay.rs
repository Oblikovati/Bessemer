use super::*;
use std::io::Write;
use std::sync::Arc;
const BEGIN: &[u8] = b"BESSEMER_OBJECT304_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_OBJECT304_END";
fn replay(output: &mut impl Write) {
    for row in cases() {
        let view = check_case(&row);
        output.write_all(view.decoded_bytes()).unwrap();
        for location in edit::locations(&view) {
            output
                .write_all(&edit::check_edit(&view, location))
                .unwrap();
        }
    }
}
#[test]
#[ignore = "invoked twice by the fresh-process replay parent"]
fn object_replay_worker() {
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    replay(&mut output);
    output.write_all(END).unwrap();
}
#[test]
fn two_processes_replay_complete_reads_and_selected_edits_byte_for_byte() {
    let first = state_process::transcript("api::replay::object_replay_worker", BEGIN, END);
    let second = state_process::transcript("api::replay::object_replay_worker", BEGIN, END);
    assert_eq!(first, second);
    assert!(!first.is_empty());
}
fn operation(view: &FlowObjectRecords, index: usize) -> Vec<u8> {
    let row = view.row(index % view.stored_count()).unwrap();
    let id = row.field_id(FlowObjectField::Member10).unwrap();
    assert!(view.row(usize::MAX).is_err());
    let edit = view.replace_signed(&id, index as i32 ^ i32::MIN).unwrap();
    assert_eq!(view.signed(&id).unwrap(), row.member_10());
    assert!(edit.result().signed(&id).is_err());
    state_carrier::check_history(edit.carrier_edit());
    edit.result().source().as_bytes().to_vec()
}
#[test]
fn eight_workers_match_serial_edits_failures_and_immutable_reads() {
    let source = cases()
        .into_iter()
        .find(|row| row.version == 7 && row.count == 32)
        .unwrap();
    let view = Arc::new(open(&source.raw, options()).unwrap());
    let serial: Vec<_> = (0..256).map(|index| operation(&view, index)).collect();
    let concurrent = concurrent_operations(&view);
    assert_eq!(concurrent, serial);
    assert_eq!(view.decoded_bytes(), source.raw);
}

fn concurrent_operations(view: &Arc<FlowObjectRecords>) -> Vec<Vec<u8>> {
    let workers: Vec<_> = (0..8)
        .map(|worker| {
            let view = Arc::clone(view);
            std::thread::spawn(move || {
                (0..32)
                    .map(|index| operation(&view, worker * 32 + index))
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    workers
        .into_iter()
        .flat_map(|worker| worker.join().unwrap())
        .collect()
}
