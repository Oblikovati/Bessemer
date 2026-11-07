use super::*;
use std::io::Write;
use std::sync::Arc;
const BEGIN: &[u8] = b"BESSEMER_OWNER122434_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_OWNER122434_END";
#[test]
#[ignore = "invoked twice by the fresh-process Source-verifying parent"]
fn owner_state_process_worker() {
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    let (records, selections) = cases();
    for row in records {
        let view = check_case(&row);
        output.write_all(view.source().as_bytes()).unwrap();
        for field in edit::fields(&view) {
            output.write_all(&edit::check_edit(&view, field)).unwrap();
        }
    }
    for row in selections {
        output.write_all(&selection::check_case(&row)).unwrap();
    }
    output.write_all(END).unwrap();
}
#[test]
fn two_fresh_processes_match_all_reads_edits_projections_and_selection_outcomes_byte_for_byte() {
    let worker = "api::replay::owner_state_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
fn operation(view: &FlowOwnerState, index: usize) -> Vec<u8> {
    assert!(view.replace_word(usize::MAX, 0).is_err());
    let edit = if view.state_byte().is_some() && index.is_multiple_of(2) {
        view.replace_state_byte(index as u8).unwrap()
    } else {
        view.replace_word(index % view.words().word_count(), index as u32 ^ u32::MAX)
            .unwrap()
    };
    assert_eq!(edit.source().decoded_bytes(), view.decoded_bytes());
    state_carrier::check_history(edit.carrier_edit());
    let identifier = if view.kind() == FlowOwnerStateKind::Legacy122 {
        122
    } else {
        434
    };
    assert_eq!(
        projection(edit.result()),
        expected_updates(identifier, edit.result().decoded_bytes())
    );
    edit.result().source().as_bytes().to_vec()
}
#[test]
fn eight_workers_match_serial_gate_member_byte_edits_failures_and_retained_snapshots() {
    for identifier in [122, 434] {
        let mut row = cases()
            .0
            .into_iter()
            .find(|row| row.identifier == identifier)
            .unwrap();
        let view = Arc::new(open(identifier, &row.raw, options()).unwrap());
        let original = row.raw.clone();
        row.raw.fill(255);
        drop(row);
        let serial: Vec<_> = (0..256).map(|index| operation(&view, index)).collect();
        assert_eq!(concurrent(&view), serial);
        assert_eq!(view.decoded_bytes(), original);
        check_retained(&view);
    }
}
fn concurrent(view: &Arc<FlowOwnerState>) -> Vec<Vec<u8>> {
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
fn check_retained(view: &FlowOwnerState) {
    let retained = view.clone();
    let before = view.source().as_bytes().to_vec();
    let tail = view.source().records().nth(2).unwrap().id;
    let unrelated = view
        .source()
        .replace_payload(&tail, b"modified unrelated tail")
        .unwrap();
    drop(unrelated);
    assert_eq!(retained.source().as_bytes(), before);
    assert_eq!(operation(&retained, 11), operation(view, 11));
}
