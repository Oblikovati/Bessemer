use super::*;
use std::io::Write;
use std::sync::Arc;
const BEGIN: &[u8] = b"BESSEMER_IMS501_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_IMS501_END";
#[test]
#[ignore = "invoked twice by the fresh-process source-verifying parent"]
fn value_process_worker() {
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    for row in cases() {
        let view = check_case(&row);
        output
            .write_all(view.context().definitions().source().as_bytes())
            .unwrap();
        output.write_all(view.source().as_bytes()).unwrap();
        output
            .write_all(format!("{:?}\n", view.coverage()).as_bytes())
            .unwrap();
        for field in edit::fields(&row) {
            output
                .write_all(&edit::check_edit(&view, &row, field))
                .unwrap();
        }
    }
    output.write_all(END).unwrap();
}
#[test]
fn two_fresh_processes_match_all_reads_context_coverage_and_selected_edits_byte_for_byte() {
    let worker = "api::replay::value_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
fn operation(view: &FlowImsValues, index: usize) -> Vec<u8> {
    let edit = match index % 3 {
        0 => view
            .replace_flag(0, FlowImsValueFlag::Bl, index as u8 % 255 + 1)
            .unwrap(),
        1 => view
            .replace_shared_pattern(0, FlowImsValueArray::Main, index, index as u64 ^ u64::MAX)
            .unwrap(),
        _ => view
            .replace_bl_pattern(0, index, index % 18, index as u64)
            .unwrap(),
    };
    assert!(view.replace_flag(0, FlowImsValueFlag::Bl, 0).is_err());
    assert!(view.replace_bl_pattern(0, usize::MAX, 0, 0).is_err());
    state_carrier::check_history(edit.carrier_edit());
    assert_eq!(edit.result().context(), view.context());
    edit.result().source().as_bytes().to_vec()
}
#[test]
fn eight_workers_match_serial_mixed_edits_failures_and_retained_definition_snapshots() {
    let row = cases().into_iter().find(|row| row.native).unwrap();
    let view = Arc::new(check_case(&row));
    let serial: Vec<_> = (0..256).map(|index| operation(&view, index)).collect();
    assert_eq!(concurrent(&view), serial);
    assert_eq!(view.decoded_bytes(), row.frame.decoded);
    let tail = view.source().records().nth(2).unwrap().id;
    let unrelated = view
        .source()
        .replace_payload(&tail, b"unrelated opaque update")
        .unwrap();
    drop(unrelated);
    assert_eq!(operation(&view, 7), serial[7]);
}
fn concurrent(view: &Arc<FlowImsValues>) -> Vec<Vec<u8>> {
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
#[test]
fn caller_value_and_definition_buffers_can_mutate_drop_and_diverge_without_affecting_inputs() {
    let mut row = contracts::sample();
    let context = context(&row);
    let view = open(&row.frame.stored, &context, options()).unwrap();
    row.frame.stored.fill(255);
    row.context.stored.fill(255);
    let expected_values = row.frame.decoded.clone();
    let expected_definitions = row.context.decoded.clone();
    drop(row);
    let alias = view.clone();
    let changed = context.definitions().replace_identifier(0, -7).unwrap();
    drop(changed);
    drop(context);
    let edit = view.replace_flag(0, FlowImsValueFlag::Bl, 1).unwrap();
    assert_eq!(
        alias.context().definitions().decoded_bytes(),
        expected_definitions
    );
    assert_eq!(view.decoded_bytes(), expected_values);
    assert_eq!(edit.source().decoded_bytes(), expected_values);
}
