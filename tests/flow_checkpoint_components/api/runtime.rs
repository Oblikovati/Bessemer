use super::*;
use std::io::{Read, Write};
const BEGIN: &[u8] = b"BESSEMER_CHECKPOINT_COMPONENT_PROCESS_BEGIN\n";
const END: &[u8] = b"\nBESSEMER_CHECKPOINT_COMPONENT_PROCESS_END";
#[test]
fn two_fresh_processes_match_lossless_source_and_selected_edit_transcripts_byte_for_byte() {
    let first = process::transcript("api::runtime::checkpoint_process_worker", BEGIN, END);
    let second = process::transcript("api::runtime::checkpoint_process_worker", BEGIN, END);
    assert_eq!(first, second);
    validate_stream(&first);
}
fn validate_stream(raw: &[u8]) {
    let mut decoder = flate2::read::ZlibDecoder::new(raw);
    let mut magic = [0; 4];
    decoder.read_exact(&mut magic).unwrap();
    assert_eq!(&magic, b"FCC2");
    let count = std::io::copy(&mut (&mut decoder).take(512_000_001), &mut std::io::sink()).unwrap();
    assert!(count > 14_582_647 && count <= 512_000_000);
    assert_eq!(decoder.total_in() as usize, raw.len());
}
#[test]
#[ignore = "the parent invokes this Source-verifying worker twice"]
fn checkpoint_process_worker() {
    let mut output = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
    output.write_all(b"FCC2").unwrap();
    let mut fields = 0;
    for row in cases() {
        fields += write_case(&mut output, &row);
    }
    blob(
        &mut output,
        format!("{{\"cases\":2899,\"selected_fields\":{fields}}}").as_bytes(),
    );
    let encoded = output.finish().unwrap();
    assert!(!encoded.windows(BEGIN.len()).any(|value| value == BEGIN));
    assert!(!encoded.windows(END.len()).any(|value| value == END));
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(BEGIN).unwrap();
    stdout.write_all(&encoded).unwrap();
    stdout.write_all(END).unwrap();
}
fn write_case<W: Write>(output: &mut W, row: &ComponentCase) -> usize {
    let view = component(row);
    write_header(output, row, &view);
    let fields = read::fields(row, &view);
    for field in &fields {
        write_edit(output, &view, field);
    }
    fields.len()
}
fn write_header<W: Write>(output: &mut W, row: &ComponentCase, view: &FlowCheckpointComponent) {
    blob(output, row.name.as_bytes());
    blob(output, view.as_bytes());
    blob(
        output,
        format!(
            "{:?}{:?}{:?}",
            view.options(),
            view.coverage(),
            view.diagnostics()
        )
        .as_bytes(),
    );
}
fn write_edit<W: Write>(output: &mut W, view: &FlowCheckpointComponent, field: &EditableField) {
    let changed = edit::check_edit(view, field, edit::replacement(field));
    blob(
        output,
        format!(
            "{:?}{:?}{:?}{:?}",
            field.id,
            field.value,
            changed.history(),
            changed.result().coverage()
        )
        .as_bytes(),
    );
    blob(output, changed.result().as_bytes());
}

fn blob<W: Write>(output: &mut W, raw: &[u8]) {
    output.write_all(&(raw.len() as u64).to_le_bytes()).unwrap();
    output.write_all(raw).unwrap();
}
#[test]
fn eight_lanes_share_immutable_components_and_match_serial_edits_with_failures_between_successes() {
    let rows: Vec<_> = cases()
        .into_iter()
        .filter(|row| row.raw.len() <= 512)
        .collect();
    let sources: Vec<_> = rows.iter().map(component).collect();
    let serial: Vec<_> = (0..8)
        .map(|lane| lane_outputs(&rows, &sources, lane))
        .collect();
    compare_parallel(&rows, &sources, serial);
}
fn compare_parallel(
    rows: &[ComponentCase],
    sources: &[FlowCheckpointComponent],
    serial: Vec<Vec<Vec<u8>>>,
) {
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..8)
            .map(|lane| {
                scope.spawn({
                    let rows = &rows;
                    let sources = &sources;
                    move || lane_outputs(rows, sources, lane)
                })
            })
            .collect();
        for (expected, handle) in serial.into_iter().zip(handles) {
            assert_eq!(expected, handle.join().unwrap());
        }
    });
}
fn lane_outputs(
    rows: &[ComponentCase],
    sources: &[FlowCheckpointComponent],
    lane: usize,
) -> Vec<Vec<u8>> {
    let mut outputs = Vec::new();
    for iteration in 0..32 {
        let index = (lane * 311 + iteration * 97) % rows.len();
        let row = &rows[index];
        let source = &sources[index];
        let fields = read::fields(row, source);
        failed_call(source);
        let mut output = format!("{:?}", source.coverage()).into_bytes();
        if let Some(field) = fields.get(iteration % fields.len().max(1)) {
            let changed = edit::check_edit(source, field, edit::replacement(field));
            output.extend_from_slice(changed.result().as_bytes());
            let foreign = component(row)
                .replace_field(&field.id, field.value)
                .unwrap_err();
            assert_eq!(foreign.entity, "field identity");
        }
        assert_eq!(source.as_bytes(), row.raw);
        outputs.push(output);
    }
    outputs
}
fn failed_call(source: &FlowCheckpointComponent) {
    let error =
        FlowCheckpointComponent::from_bytes(&[], source.context(), source.kind(), source.options())
            .unwrap_err();
    assert_eq!(error.operation, "Flow logical checkpoint components");
    assert_eq!(error.class, bessemer::FailureClass::Invalid);
}
