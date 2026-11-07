use super::*;
use std::io::Write;
use std::sync::Arc;
const BEGIN: &[u8] = b"BESSEMER_VIEWER121_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_VIEWER121_END";
#[test]
#[ignore = "invoked twice by the fresh-process Source-verifying parent"]
fn viewer_prefix_process_worker() {
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    for row in cases() {
        let view = check_case(&row);
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
fn two_fresh_processes_match_all_owned_reads_prefix_coverage_and_selected_edits_byte_for_byte() {
    let worker = "api::replay::viewer_prefix_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
fn mixed_edit(view: &FlowViewerPrefix, index: usize) -> FlowViewerPrefixEdit {
    match index % 3 {
        0 => view
            .replace_signed(FlowViewerSignedField::StoredSecond, index as i32)
            .unwrap(),
        1 => view
            .replace_array_pattern(
                0,
                index % view.array(0).unwrap().word_count(),
                index as u64 ^ u64::MAX,
            )
            .unwrap(),
        _ => view
            .replace_discarded_pattern(index % 3, index as u64)
            .unwrap(),
    }
}
fn operation(view: &FlowViewerPrefix, index: usize) -> Vec<u8> {
    let edit = mixed_edit(view, index);
    assert!(view.replace_array_pattern(3, index, 0).is_err());
    state_carrier::check_history(edit.carrier_edit());
    assert_eq!(edit.result().opaque_tail(), view.opaque_tail());
    assert_eq!(
        edit.coverage().body_validation,
        FlowViewerBodyValidation::Incomplete
    );
    let mut output = edit.result().source().as_bytes().to_vec();
    output.extend_from_slice(format!("{:?}", edit.result().native_metadata()).as_bytes());
    output
}
#[test]
fn eight_workers_match_serial_mixed_field_edits_failures_and_immutable_snapshots() {
    let mut row = cases()
        .into_iter()
        .find(|row| row.frame.native_version.is_some())
        .unwrap();
    let view = Arc::new(check_case(&row));
    let original = row.frame.decoded.clone();
    row.frame.stored.fill(255);
    drop(row);
    let serial: Vec<_> = (0..256).map(|index| operation(&view, index)).collect();
    assert_eq!(concurrent(&view), serial);
    assert_eq!(view.decoded_body(), original);
    check_retained(&view);
}
fn concurrent(view: &Arc<FlowViewerPrefix>) -> Vec<Vec<u8>> {
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
fn check_retained(view: &FlowViewerPrefix) {
    let clone = view.clone();
    let before = view.source().as_bytes().to_vec();
    let tail = view.source().records().nth(2).unwrap().id;
    let unrelated = view
        .source()
        .replace_payload(&tail, b"independent opaque record update")
        .unwrap();
    drop(unrelated);
    assert_eq!(clone.source().as_bytes(), before);
    assert_eq!(operation(&clone, 11), operation(view, 11));
}
