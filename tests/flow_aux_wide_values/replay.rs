use super::*;
use std::io::Write;
use std::sync::Arc;
const BEGIN: &[u8] = b"BESSEMER_AUX444_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_AUX444_END";
#[test]
#[ignore = "invoked twice by the two-process Source-verifying parent"]
fn aux_wide_process_worker() {
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    for row in cases() {
        let view = check_case(&row);
        output.write_all(view.source().as_bytes()).unwrap();
        for selection in edit::selections(&row) {
            output
                .write_all(&edit::check_edit(&view, selection))
                .unwrap();
        }
    }
    output.write_all(END).unwrap();
}
#[test]
fn two_fresh_processes_match_all_source_checked_reads_and_selected_edits_byte_for_byte() {
    let worker = "api::replay::aux_wide_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
fn mixed(view: &FlowAuxWideValues, index: usize) -> Vec<u8> {
    let edit = if index.is_multiple_of(2) {
        view.replace_discarded_legacy(index as i32 ^ i32::MIN)
            .unwrap()
    } else {
        view.replace_pattern(index % view.stored_count(), index as u64 ^ u64::MAX)
            .unwrap()
    };
    assert!(view.replace_pattern(usize::MAX, 0).is_err());
    assert_eq!(edit.source().decoded_bytes(), view.decoded_bytes());
    state_carrier::check_history(edit.carrier_edit());
    edit.result().source().as_bytes().to_vec()
}
#[test]
fn eight_workers_match_serial_mixed_successes_failures_and_retained_snapshots() {
    let mut row = cases()
        .into_iter()
        .find(|row| row.name == "v0-count-32")
        .unwrap();
    let view = Arc::new(open(&row.stored, options()).unwrap());
    row.stored.fill(255);
    drop(row.stored);
    let serial: Vec<_> = (0..256).map(|index| mixed(&view, index)).collect();
    let concurrent = concurrent(&view);
    assert_eq!(concurrent, serial);
    assert_eq!(view.decoded_bytes(), row.decoded);
    check_retained(&view);
}
fn concurrent(view: &Arc<FlowAuxWideValues>) -> Vec<Vec<u8>> {
    let workers: Vec<_> = (0..8)
        .map(|worker| {
            let view = Arc::clone(view);
            std::thread::spawn(move || {
                (0..32)
                    .map(|index| mixed(&view, worker * 32 + index))
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    workers
        .into_iter()
        .flat_map(|worker| worker.join().unwrap())
        .collect()
}
fn check_retained(view: &FlowAuxWideValues) {
    let snapshot = view.clone();
    let original = view.source().as_bytes().to_vec();
    let tail = view.source().records().nth(2).unwrap().id;
    let unrelated = view
        .source()
        .replace_payload(&tail, b"independent opaque update")
        .unwrap();
    assert_ne!(unrelated.result().as_bytes(), original);
    drop(unrelated);
    assert_eq!(snapshot.source().as_bytes(), original);
    assert_eq!(mixed(&snapshot, 7), mixed(view, 7));
}
