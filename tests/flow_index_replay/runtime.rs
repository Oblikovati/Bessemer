use super::*;
use std::io::Write;
const BEGIN: &[u8] = b"BESSEMER_LOGICAL_REPLAY_TRANSCRIPT_BEGIN\n";
const END: &[u8] = b"\nBESSEMER_LOGICAL_REPLAY_TRANSCRIPT_END";
#[test]
fn two_fresh_processes_compare_source_maps_controls_coverage_and_failures_byte_for_byte() {
    let first = process::transcript("runtime::logical_replay_process_worker", BEGIN, END);
    let second = process::transcript("runtime::logical_replay_process_worker", BEGIN, END);
    assert_eq!(first, second);
}
#[test]
#[ignore = "invoked twice by the parent process test"]
fn logical_replay_process_worker() {
    let mut transcript = Vec::new();
    for row in cases() {
        let source = oracles::carrier(&row);
        blob(&mut transcript, source.as_bytes());
        for forced in [false, true] {
            blob(&mut transcript, &replayed(&source, &row, forced));
        }
    }
    let mut output = std::io::stdout().lock();
    output.write_all(BEGIN).unwrap();
    output.write_all(&transcript).unwrap();
    output.write_all(END).unwrap();
}
fn replayed(source: &FlowCarrier, row: &ReplayCase, forced: bool) -> Vec<u8> {
    let result = source.logical_index(api::policy(forced), api::options());
    match &row.outcomes[usize::from(forced)] {
        ReplayOutcome::Completed(expected) => {
            let index = result.unwrap();
            api::check_projection(row, expected, &index);
            serialized(&index)
        }
        ReplayOutcome::Rejected { .. } => {
            let error = result.unwrap_err();
            assert_eq!(error.operation, "Flow logical index replay");
            format!("{error:?}").into_bytes()
        }
    }
}
fn serialized(index: &FlowLogicalIndex) -> Vec<u8> {
    let mut raw = Vec::new();
    serialize_header(index, &mut raw);
    for (identifier, span) in index.active_entries() {
        raw.extend_from_slice(&identifier.to_le_bytes());
        raw.extend_from_slice(&span.record_start.to_le_bytes());
        raw.extend_from_slice(&span.record_span.to_le_bytes());
    }
    for (key, value) in index.saved_entries() {
        raw.extend_from_slice(&key.to_le_bytes());
        raw.extend_from_slice(&value.to_le_bytes());
    }
    serialize_controls(index, &mut raw);
    raw
}
fn serialize_header(index: &FlowLogicalIndex, raw: &mut Vec<u8>) {
    blob(
        raw,
        format!(
            "{:?}{:?}{:?}{:?}",
            index.policy(),
            index.options(),
            index.coverage(),
            index.diagnostics()
        )
        .as_bytes(),
    );
}
fn serialize_controls(index: &FlowLogicalIndex, raw: &mut Vec<u8>) {
    if let Some(control) = index.terminal_control() {
        raw.extend_from_slice(&(control.record().ordinal() as u64).to_le_bytes());
        blob(raw, control.decoded_bytes());
        blob(raw, format!("{:?}", control.coverage()).as_bytes());
    }
    if let Some(control) = index.saved_control() {
        raw.extend_from_slice(&(control.record().ordinal() as u64).to_le_bytes());
        blob(raw, control.decoded_bytes());
        blob(raw, format!("{:?}", control.coverage()).as_bytes());
    }
}
fn blob(raw: &mut Vec<u8>, bytes: &[u8]) {
    raw.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    raw.extend_from_slice(bytes);
}
#[test]
fn eight_concurrent_lanes_match_serial_replay_with_failures_between_successes() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|row| row.raw.len() < 8192)
        .collect();
    let serial: Vec<_> = (0..8).map(|lane| lane_outputs(&rows, lane)).collect();
    std::thread::scope(|scope| {
        let workers: Vec<_> = (0..8)
            .map(|lane| {
                scope.spawn({
                    let rows = &rows;
                    move || lane_outputs(rows, lane)
                })
            })
            .collect();
        for (expected, worker) in serial.into_iter().zip(workers) {
            assert_eq!(expected, worker.join().unwrap());
        }
    });
}
fn lane_outputs(rows: &[ReplayCase], lane: usize) -> Vec<Vec<u8>> {
    let mut outputs = Vec::new();
    for iteration in 0..32 {
        let row = &rows[(lane * 31 + iteration * 17) % rows.len()];
        let source = oracles::carrier(row);
        outputs.push(replayed(&source, row, false));
        let invalid = FlowIndexOptions {
            maximum_active_entries: 0,
            maximum_saved_entries: 0,
            ..api::options()
        };
        let failure = source.logical_index(FlowIndexPolicy::ForcePhysicalScan, invalid);
        assert_eq!(failure.is_err(), source.records().len() != 0);
        outputs.push(replayed(&source, row, true));
    }
    outputs
}
#[test]
fn published_maps_and_exact_control_provenance_survive_caller_and_source_mutation() {
    for name in [
        "scan-8-duplicates-16",
        "terminal-8-duplicates-self-reserved",
        "saved-duplicate-last-wins",
    ] {
        let row = cases().into_iter().find(|row| row.name == name).unwrap();
        isolated_projection(row);
    }
}
fn isolated_projection(mut row: ReplayCase) {
    let source = oracles::carrier(&row);
    let index = source
        .logical_index(FlowIndexPolicy::PreferTerminalIndex, api::options())
        .unwrap();
    let before = serialized(&index);
    let checkpoint = source.as_bytes().to_vec();
    row.raw.fill(0xff);
    let edit = changed_source(&source);
    assert_ne!(edit.result().as_bytes(), checkpoint);
    assert_retained(&index, &checkpoint, &before);
    assert_eq!(serialized(&index.clone()), before);
    drop(edit);
    drop(source);
    assert_retained(&index, &checkpoint, &before);
}
fn changed_source(source: &FlowCarrier) -> FlowCarrierEdit {
    let invalid = FlowIndexOptions {
        maximum_active_entries: 0,
        maximum_saved_entries: 0,
        ..api::options()
    };
    assert!(
        source
            .logical_index(FlowIndexPolicy::ForcePhysicalScan, invalid)
            .is_err()
    );
    let record = source.records().next().unwrap();
    source.replace_payload(&record.id, b"changed").unwrap()
}
fn assert_retained(index: &FlowLogicalIndex, checkpoint: &[u8], before: &[u8]) {
    assert_eq!(index.source().as_bytes(), checkpoint);
    assert_eq!(serialized(index), before);
}
