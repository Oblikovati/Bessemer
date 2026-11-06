use super::*;
use std::io::Write;
use std::sync::Arc;
const BEGIN: &[u8] = b"BESSEMER_VECTOR148_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_VECTOR148_END";
#[test]
#[ignore = "invoked twice by the fresh-process Source-verifying parent"]
fn wide_vector_process_worker() {
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    for row in cases() {
        let view = check_case(&row);
        output.write_all(view.source().as_bytes()).unwrap();
        append_edits(&mut output, &view, &row);
    }
    output.write_all(END).unwrap();
}
fn append_edits(output: &mut impl Write, view: &FlowWideVectors, row: &VectorCase) {
    for (ordinal, array) in row.arrays.iter().enumerate() {
        for index in edit::selected_indices(array) {
            output
                .write_all(&edit::check_edit(view, ordinal, index))
                .unwrap();
        }
    }
}
#[test]
fn two_fresh_processes_replay_all_reads_and_selected_edits_byte_for_byte() {
    let worker = "api::replay::wide_vector_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
fn operation(view: &FlowWideVectors, index: usize) -> Vec<u8> {
    assert!(view.replace_pattern(usize::MAX, index, 0).is_err());
    let edit = view
        .replace_pattern(
            0,
            index % view.vector(0).unwrap().word_count(),
            index as u64 ^ u64::MAX,
        )
        .unwrap();
    state_carrier::check_history(edit.carrier_edit());
    assert_eq!(edit.source().decoded_bytes(), view.decoded_bytes());
    edit.result().source().as_bytes().to_vec()
}
#[test]
fn eight_workers_match_serial_successes_failures_and_retained_snapshots() {
    let mut row = cases()
        .into_iter()
        .find(|row| row.name == "mixed-bit-vectors")
        .unwrap();
    let view = Arc::new(open(&row.raw, options()).unwrap());
    let original = row.raw.clone();
    row.raw.fill(255);
    drop(row);
    let serial: Vec<_> = (0..256).map(|index| operation(&view, index)).collect();
    assert_eq!(concurrent(&view), serial);
    assert_eq!(view.decoded_bytes(), original);
    check_retained(&view);
}
fn concurrent(view: &Arc<FlowWideVectors>) -> Vec<Vec<u8>> {
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
fn check_retained(view: &FlowWideVectors) {
    let retained = view.clone();
    let original = view.source().as_bytes().to_vec();
    let tail = view.source().records().nth(2).unwrap().id;
    let unrelated = view
        .source()
        .replace_payload(&tail, b"changed independent tail")
        .unwrap();
    drop(unrelated);
    assert_eq!(retained.source().as_bytes(), original);
    assert_eq!(operation(&retained, 19), operation(view, 19));
}
