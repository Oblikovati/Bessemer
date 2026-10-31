//! Complete stored auxiliary rows, bounded reference fields and exclusive runtime query checks.
use bessemer::simulation::*;
use std::io::Read;
#[path = "common/body_range_packets.rs"]
mod checks;
#[path = "common/body_range_failures.rs"]
mod failures;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod packet;
use packet::Cursor;

fn inflate(bytes: &[u8]) -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(bytes)
        .read_to_end(&mut raw)
        .unwrap();
    raw
}

#[test]
fn complete_auxiliary_rows_fields_edits_and_checked_proposals_match_actual_bounded_helper() {
    let raw = inflate(include_bytes!(
        "fixtures/simulation-body-ranges/auxiliary.bin"
    ));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"BRA1");
    assert_eq!(cursor.word(), 144);
    for _ in 0..144 {
        owned_auxiliary(&mut cursor);
    }
    assert!(cursor.0.is_empty());
}

fn owned_auxiliary(cursor: &mut Cursor<'_>) {
    let raw = cursor.span();
    let profile = SimulationAuxiliaryBodyProfile::from_stride(raw.len()).unwrap();
    let row = SimulationAuxiliaryBodyRow::from_bytes(raw, profile).unwrap();
    checks::fields(&row, cursor.span());
    let golden = cursor.span();
    let edited_fields = cursor.span();
    let edit = checks::auxiliary_edit(&row, golden, edited_fields);
    checks::proposal(&row);
    assert_eq!(row.to_bytes(), raw);
    assert!(row.gen_parent().is_none());
    assert_auxiliary_replay(&row, &edit, raw);
}

#[test]
fn all_owned_exclusive_query_dispatches_and_rejections_match_actual_native_projection() {
    let raw = inflate(include_bytes!("fixtures/simulation-body-ranges/types.bin"));
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"BRT1");
    assert_eq!(cursor.word(), 3357);
    for _ in 0..3357 {
        let mesh =
            SimulationBodyQueryMesh::from_blocks(cursor.span(), cursor.span(), checks::LIMITS)
                .unwrap();
        let call = checks::type_call(&mut cursor);
        checks::type_query(&mesh, call, cursor.span());
    }
    assert_eq!(cursor.word(), 612);
    for _ in 0..612 {
        rejected_type_case(&mut cursor);
    }
    assert!(cursor.0.is_empty());
}

#[test]
#[ignore = "requires exported approved local native corpus"]
fn all_native_auxiliary_intervals_partition_elements_and_query_contexts_match_actual_readers() {
    let raw = std::fs::read("target/simulation-body-ranges-native.bin").unwrap();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"BRN1");
    assert_eq!(cursor.word(), 3);
    let mut exported = b"BRV1".to_vec();
    let mut totals = [0usize; 2];
    for _ in 0..3 {
        native_pair(&mut cursor, &mut exported, &mut totals);
    }
    assert_eq!(totals, [5, 25106]);
    assert!(cursor.0.is_empty());
    std::fs::write(
        "target/simulation-body-ranges-native-projections.bin",
        exported,
    )
    .unwrap();
}

fn native_pair(cursor: &mut Cursor<'_>, exported: &mut Vec<u8>, totals: &mut [usize; 2]) {
    let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
    let raw = cursor.span();
    let image = SimulationGenArchive::from_bytes(raw, &master).unwrap();
    let mesh = image.body_query_mesh(checks::LIMITS).unwrap();
    let mut expected = Cursor(cursor.span());
    let count = expected.word() as usize;
    assert_eq!(count, image.layout().counts[8] as usize);
    let mut coverage = vec![0u8; image.layout().counts[4] as usize];
    for index in 0..count {
        native_row(&image, &mesh, index, &mut expected, exported, &mut coverage);
    }
    assert!(coverage.iter().all(|count| *count == 1));
    assert!(image.auxiliary_body_row(count).is_err());
    assert!(image.auxiliary_body_row(usize::MAX).is_err());
    assert!(expected.0.is_empty());
    assert_eq!(image.to_bytes(), raw);
    totals[0] += count;
    totals[1] += coverage.len();
}

fn native_row(
    image: &SimulationGenArchive,
    mesh: &SimulationBodyQueryMesh,
    index: usize,
    expected: &mut Cursor<'_>,
    exported: &mut Vec<u8>,
    coverage: &mut [u8],
) {
    let row = image.auxiliary_body_row(index).unwrap();
    let raw = expected.span();
    assert_eq!(row.to_bytes(), raw);
    assert_eq!(row.gen_parent(), Some((image.revision(), index)));
    exported.extend(checks::fields(&row, expected.span()));
    let edit = checks::auxiliary_edit(&row, expected.span(), expected.span());
    append_span(exported, &edit.row.to_bytes());
    exported.extend(checks::field_bytes(edit.row.fields()));
    let inputs = checks::proposal(&row).unwrap();
    append_inputs(exported, inputs);
    native_type_queries(
        mesh,
        inputs,
        image.layout().counts[4] as i32,
        expected,
        exported,
    );
    crosscheck_interval(image, &row, coverage);
    assert_eq!(row.to_bytes(), raw);
}

fn crosscheck_interval(
    image: &SimulationGenArchive,
    row: &SimulationAuxiliaryBodyRow,
    coverage: &mut [u8],
) {
    let fields = row.fields();
    assert!(1 <= fields.stored_low && fields.stored_low <= fields.stored_high);
    assert!((fields.stored_high as usize) <= coverage.len());
    for ordinal in fields.stored_low..=fields.stored_high {
        let index = ordinal as usize - 1;
        coverage[index] += 1;
        assert_eq!(
            u32::from(
                image
                    .element_record(index)
                    .unwrap()
                    .selection()
                    .body_selection_id
            ),
            fields.body_id.pattern()
        );
    }
}

fn append_span(output: &mut Vec<u8>, bytes: &[u8]) {
    output.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
    output.extend_from_slice(bytes);
}
fn append_inputs(output: &mut Vec<u8>, inputs: SimulationBodyTypeInputs) {
    output.extend_from_slice(&inputs.body_id.pattern().to_le_bytes());
    output.extend_from_slice(&inputs.start.to_le_bytes());
    output.extend_from_slice(&inputs.exclusive_upper.to_le_bytes());
}

fn assert_auxiliary_replay(
    row: &SimulationAuxiliaryBodyRow,
    edit: &SimulationAuxiliaryBodyEdit,
    raw: &[u8],
) {
    let fields = row.fields();
    let noop = row
        .edit_fields(fields.body_id, fields.stored_low, fields.stored_high)
        .unwrap();
    assert_eq!(noop.parent, noop.result);
    let expected = bessemer::display_lists::DisplayEditCompletion::Unchanged;
    assert_eq!(noop.completion, expected);
    let reverted = edit
        .row
        .edit_fields(fields.body_id, fields.stored_low, fields.stored_high)
        .unwrap();
    assert_eq!(reverted.row.to_bytes(), raw);
    assert_eq!(reverted.parent, edit.result);
    let independent = SimulationAuxiliaryBodyRow::from_bytes(raw, row.profile()).unwrap();
    assert_ne!(row.revision(), independent.revision());
}

fn rejected_type_case(cursor: &mut Cursor<'_>) {
    let mesh =
        SimulationBodyQueryMesh::from_blocks(cursor.span(), cursor.span(), checks::LIMITS).unwrap();
    let call = checks::type_call(cursor);
    assert!(
        mesh.project_body_type(call.inputs, call.mode, call.callback)
            .is_err()
    );
}

fn native_type_queries(
    mesh: &SimulationBodyQueryMesh,
    inputs: SimulationBodyTypeInputs,
    count: i32,
    expected: &mut Cursor<'_>,
    exported: &mut Vec<u8>,
) {
    let whole = SimulationBodyTypeInputs {
        body_id: inputs.body_id,
        start: 0,
        exclusive_upper: count,
    };
    let call = checks::TypeCall {
        inputs: whole,
        mode: 3,
        callback: Some(-1),
    };
    append_span(exported, &checks::type_query(mesh, call, expected.span()));
    let call = checks::TypeCall {
        inputs,
        mode: 3,
        callback: Some(-1),
    };
    append_span(exported, &checks::type_query(mesh, call, expected.span()));
}
