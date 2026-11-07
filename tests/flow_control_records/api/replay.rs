use super::*;
use std::io::Write;
use std::sync::Arc;
const BEGIN: &[u8] = b"BESSEMER_CONTROL_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_CONTROL_END";
#[test]
#[ignore = "invoked twice by the fresh-process source-verifying parent"]
fn control_process_worker() {
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    for row in cases() {
        replay_case(&mut output, &row);
    }
    output.write_all(END).unwrap();
}
fn replay_case(output: &mut impl Write, row: &ControlCase) {
    if row.identifier == SAVED {
        replay_saved(output, row);
        return;
    }
    let view = check_index(row);
    output.write_all(view.source().as_bytes()).unwrap();
    output
        .write_all(format!("{:?}\n", view.coverage()).as_bytes())
        .unwrap();
    for field in edit::fields(row) {
        output
            .write_all(&edit::check_index_edit(&view, row, field))
            .unwrap();
    }
}
fn replay_saved(output: &mut impl Write, row: &ControlCase) {
    let view = check_saved(row);
    output.write_all(view.source().as_bytes()).unwrap();
    output
        .write_all(format!("{:?}\n", view.coverage()).as_bytes())
        .unwrap();
    for field in edit::fields(row) {
        output
            .write_all(&edit::check_saved_edit(&view, row, field))
            .unwrap();
    }
}
#[test]
fn two_fresh_processes_match_all_owned_reads_coverage_and_selected_edits_byte_for_byte() {
    let worker = "api::replay::control_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
fn operation(saved: &FlowSavedSpans, index: &FlowTerminalIndex, step: usize) -> Vec<u8> {
    assert!(saved.replace_value(usize::MAX, 0).is_err());
    assert!(index.replace_record_span(usize::MAX, 0).is_err());
    let saved_edit = saved
        .replace_key(step % saved.stored_count(), step as u64 ^ u64::MAX)
        .unwrap();
    let index_edit = index
        .replace_record_start(step % index.stored_count(), step as u64)
        .unwrap();
    state_carrier::check_history(saved_edit.carrier_edit());
    state_carrier::check_history(index_edit.carrier_edit());
    edit::check_recomputed_footer(index_edit.result());
    let mut raw = saved_edit.result().source().as_bytes().to_vec();
    raw.extend_from_slice(index_edit.result().source().as_bytes());
    raw
}
#[test]
fn eight_workers_match_serial_mixed_control_edits_failures_and_both_footer_widths() {
    let saved_row = contracts::sample(SAVED, 0);
    let saved = Arc::new(check_saved(&saved_row));
    for width_bytes in [4, 8] {
        let row = contracts::sample(INDEX, width_bytes);
        let index = Arc::new(check_index(&row));
        let serial: Vec<_> = (0..256)
            .map(|step| operation(&saved, &index, step))
            .collect();
        assert_eq!(concurrent(&saved, &index), serial);
        assert_eq!(index.decoded_bytes(), row.decoded);
    }
    assert_eq!(saved.decoded_bytes(), saved_row.decoded);
}
fn concurrent(saved: &Arc<FlowSavedSpans>, index: &Arc<FlowTerminalIndex>) -> Vec<Vec<u8>> {
    let workers: Vec<_> = (0..8)
        .map(|worker| {
            let saved = Arc::clone(saved);
            let index = Arc::clone(index);
            std::thread::spawn(move || {
                (0..32)
                    .map(|step| operation(&saved, &index, worker * 32 + step))
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    workers
        .into_iter()
        .flat_map(|worker| worker.join().unwrap())
        .collect()
}
#[test]
fn caller_buffers_and_unrelated_carrier_edits_cannot_change_retained_control_views() {
    let mut row = contracts::sample(SAVED, 0);
    let saved = check_saved(&row);
    row.stored.fill(255);
    assert_eq!(saved.decoded_bytes(), row.decoded);
    let tail = saved.source().records().nth(2).unwrap().id;
    let unrelated = saved
        .source()
        .replace_payload(&tail, b"unrelated opaque edit")
        .unwrap();
    drop(unrelated);
    let mut row = contracts::sample(INDEX, 8);
    let index = check_index(&row);
    row.stored.fill(255);
    assert_eq!(index.decoded_bytes(), row.decoded);
    let before = operation(&saved, &index, 7);
    drop(row);
    assert_eq!(operation(&saved.clone(), &index.clone(), 7), before);
}
