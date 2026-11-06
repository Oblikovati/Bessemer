use super::*;
use std::io::Write;
use std::sync::Arc;
const BEGIN: &[u8] = b"BESSEMER_TEXT311_BEGIN\0";
const END: &[u8] = b"\0BESSEMER_TEXT311_END";
#[test]
#[ignore = "invoked twice by the fresh-process Source-verifying parent"]
fn text_slot_process_worker() {
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    for row in cases() {
        let view = check_case(&row);
        let projected = projection::check_projection(&view, &row);
        output.write_all(&edit::check_header(&view)).unwrap();
        append_state(&mut output, projected.result());
        append_edits(&mut output, &view, &row);
    }
    output.write_all(END).unwrap();
}
fn append_state(output: &mut impl Write, state: &FlowTextSlotState) {
    for index in 0..50 {
        let units = state.units(index).unwrap();
        output
            .write_all(&(units.len() as u64).to_le_bytes())
            .unwrap();
        for unit in units {
            output.write_all(&unit.to_le_bytes()).unwrap();
        }
    }
}
fn append_edits(output: &mut impl Write, view: &FlowTextSlots, row: &TextSlotCase) {
    for slot in &row.slots {
        for index in edit::indices(slot, row.native_header.is_some()) {
            output
                .write_all(&edit::check_unit(view, slot.ordinal as usize, index))
                .unwrap();
        }
    }
}
#[test]
fn two_fresh_processes_replay_all_reads_state_projections_and_selected_edits_byte_for_byte() {
    let worker = "api::replay::text_slot_process_worker";
    assert_eq!(
        state_process::transcript(worker, BEGIN, END),
        state_process::transcript(worker, BEGIN, END)
    );
}
fn operation(view: &FlowTextSlots, previous: &FlowTextSlotState, index: usize) -> Vec<u8> {
    assert!(view.replace_unit(50, index, 0).is_err());
    let edit = mixed_edit(view, index);
    state_carrier::check_history(edit.carrier_edit());
    let projected = edit.result().project(previous, 1_000_000).unwrap();
    assert!(edit.result().project(previous, 0).is_err());
    let mut output = edit.result().source().as_bytes().to_vec();
    append_state(&mut output, projected.result());
    output
}
fn mixed_edit(view: &FlowTextSlots, index: usize) -> FlowSimpleTableEdit<FlowTextSlots> {
    if index.is_multiple_of(2) {
        return view.replace_header(index as i32 ^ i32::MIN).unwrap();
    }
    let slot = view.slot(0).unwrap();
    let unit = index % (slot.stored_units() - 1);
    view.replace_unit(0, unit, index as u16 ^ u16::MAX).unwrap()
}

#[test]
fn eight_workers_match_serial_mixed_edits_projection_failures_and_retained_checkpoints() {
    let row = cases()
        .into_iter()
        .find(|row| row.name == "native-fld")
        .unwrap();
    let view = Arc::new(check_case(&row));
    let previous = Arc::new(projection::previous(&row));
    let serial: Vec<_> = (0..256)
        .map(|index| operation(&view, &previous, index))
        .collect();
    assert_eq!(concurrent(&view, &previous), serial);
    assert_eq!(view.decoded_bytes(), row.raw);
    let tail = view.source().records().nth(2).unwrap().id;
    let unrelated = view
        .source()
        .replace_payload(&tail, b"independent tail edit")
        .unwrap();
    drop(unrelated);
    assert_eq!(operation(&view, &previous, 7), serial[7]);
}
fn concurrent(view: &Arc<FlowTextSlots>, previous: &Arc<FlowTextSlotState>) -> Vec<Vec<u8>> {
    let workers: Vec<_> = (0..8)
        .map(|worker| {
            let view = Arc::clone(view);
            let previous = Arc::clone(previous);
            std::thread::spawn(move || {
                (0..32)
                    .map(|index| operation(&view, &previous, worker * 32 + index))
                    .collect::<Vec<_>>()
            })
        })
        .collect();
    workers
        .into_iter()
        .flat_map(|worker| worker.join().unwrap())
        .collect()
}
