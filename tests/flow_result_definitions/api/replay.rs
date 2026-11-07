use super::*;
use std::io::Write;
use std::sync::Arc;
const BEGIN: &[u8] = b"BESSEMER_IMS500_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_IMS500_END";
#[test]
#[ignore = "invoked twice by the fresh-process source-verifying parent"]
fn definition_process_worker() {
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
fn two_fresh_processes_match_all_source_checked_reads_coverage_and_selected_edits_byte_for_byte() {
    let worker = "api::replay::definition_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
fn operation(view: &FlowResultDefinitions, index: usize) -> Vec<u8> {
    let edit = match index % 3 {
        0 => view.replace_identifier(0, index as i32 ^ i32::MIN).unwrap(),
        1 => view.replace_allocation_byte(0, index as u8).unwrap(),
        _ => view
            .replace_pattern(0, KINDS[0], index, index as u32 ^ u32::MAX)
            .unwrap(),
    };
    assert!(view.replace_identifier(usize::MAX, 0).is_err());
    assert!(view.replace_pattern(0, KINDS[0], usize::MAX, 0).is_err());
    state_carrier::check_history(edit.carrier_edit());
    edit.result().source().as_bytes().to_vec()
}
#[test]
fn eight_workers_match_serial_mixed_edits_failures_and_immutable_snapshots() {
    let row = cases().into_iter().find(|row| row.native).unwrap();
    let view = Arc::new(check_case(&row));
    let serial: Vec<_> = (0..256).map(|index| operation(&view, index)).collect();
    assert_eq!(concurrent(&view), serial);
    assert_eq!(view.decoded_bytes(), row.decoded);
    let tail = view.source().records().nth(2).unwrap().id;
    let unrelated = view
        .source()
        .replace_payload(&tail, b"unrelated tail change")
        .unwrap();
    drop(unrelated);
    assert_eq!(operation(&view, 7), serial[7]);
}
fn concurrent(view: &Arc<FlowResultDefinitions>) -> Vec<Vec<u8>> {
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
fn caller_buffers_can_mutate_and_drop_without_affecting_retained_views_or_edits() {
    let row = cases().into_iter().find(|row| row.native).unwrap();
    let mut raw = state_carrier::carrier(500, &row.stored).as_bytes().to_vec();
    let source = snapshot(&raw);
    let id = source.records().nth(1).unwrap().id;
    let view = source.record_500_ims_definitions(&id, options()).unwrap();
    raw.fill(255);
    drop(raw);
    drop(source);
    let alias = view.clone();
    let edit = view.replace_identifier(0, -7).unwrap();
    assert_eq!(alias.decoded_bytes(), row.decoded);
    assert_eq!(edit.source().decoded_bytes(), row.decoded);
    assert_eq!(edit.result().definition(0).unwrap().identifier(), -7);
}

#[test]
fn exact_carrier_extent_rejects_growing_recompression_without_mutating_inputs() {
    let row = cases().into_iter().find(|row| row.native).unwrap();
    let raw = state_carrier::carrier(500, &row.stored).as_bytes().to_vec();
    let limits = FlowLimits {
        maximum_wire_bytes: raw.len(),
        maximum_payload_bytes: raw.len(),
        maximum_records: 3,
    };
    let source = FlowCarrier::from_bytes(&raw, limits).unwrap();
    let id = source.records().nth(1).unwrap().id;
    let view = source.record_500_ims_definitions(&id, options()).unwrap();
    let error = view.replace_identifier(0, -7).unwrap_err();
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
    assert_eq!(error.entity, "replacement payload");
    assert_eq!(source.as_bytes(), raw);
    assert_eq!(view.decoded_bytes(), row.decoded);
}
