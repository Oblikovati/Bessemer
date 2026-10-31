//! Exact family-mask gates and complete supplied-context body node query references.
use bessemer::simulation::*;
use std::io::Read;
#[path = "common/selection_packets.rs"]
mod checks;
#[path = "common/selection_failures.rs"]
mod failures;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod packet;
use packet::Cursor;

fn inflate(bytes: &[u8]) -> Vec<u8> {
    let mut decoded = Vec::new();
    flate2::read::ZlibDecoder::new(bytes)
        .read_to_end(&mut decoded)
        .unwrap();
    decoded
}

#[test]
fn complete_predicate_cross_products_match_actual_readers_and_independent_expectations() {
    let bytes = inflate(include_bytes!(
        "fixtures/simulation-selection/predicates.bin"
    ));
    let mut cursor = Cursor(&bytes);
    assert_eq!(cursor.take(4), b"SEP1");
    assert_eq!(cursor.word(), 88800);
    for _ in 0..88800 {
        predicate_case(&mut cursor, true);
    }
    assert_eq!(cursor.word(), 9472);
    for _ in 0..9472 {
        predicate_case(&mut cursor, false);
    }
    assert!(cursor.0.is_empty());
}

fn predicate_case(cursor: &mut Cursor<'_>, scalar: bool) {
    let raw = cursor.span();
    let record = SimulationElementRecord::from_bytes(raw).unwrap();
    let mask = cursor.word();
    let key = if scalar { cursor.word() as i32 } else { 0 };
    checks::predicate(&record, mask, key, cursor.span());
    assert_eq!(record.to_bytes().as_slice(), raw);
}

#[test]
fn complete_owned_queries_match_reference_marks_flags_callbacks_and_all_parent_histories() {
    let bytes = inflate(include_bytes!("fixtures/simulation-selection/queries.bin"));
    let mut cursor = Cursor(&bytes);
    assert_eq!(cursor.take(4), b"SEQ1");
    assert_eq!(cursor.word(), 1117);
    for _ in 0..1117 {
        let mesh =
            SimulationBodyQueryMesh::from_blocks(cursor.span(), cursor.span(), checks::LIMITS)
                .unwrap();
        let inputs = checks::query_inputs(&mut cursor);
        checks::query(&mesh, &inputs, cursor.span());
    }
    assert!(cursor.0.is_empty());
}

#[test]
#[ignore = "requires exported approved local native corpus"]
fn complete_installed_selection_and_all_documented_query_contexts_match_actual_readers() {
    let bytes = std::fs::read("target/simulation-selection-native.bin").unwrap();
    let mut cursor = Cursor(&bytes);
    assert_eq!(cursor.take(4), b"SEN1");
    assert_eq!(cursor.word(), 3);
    let mut exported = b"SEV1".to_vec();
    let mut totals = [0usize; 4];
    for _ in 0..3 {
        native_pair(&mut cursor, &mut exported, &mut totals);
    }
    assert_eq!(totals, [25106, 11, 209204, 45398]);
    assert!(cursor.0.is_empty());
    std::fs::write(
        "target/simulation-selection-native-projections.bin",
        exported,
    )
    .unwrap();
}

fn native_pair(cursor: &mut Cursor<'_>, exported: &mut Vec<u8>, totals: &mut [usize; 4]) {
    let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
    let raw = cursor.span();
    let image = SimulationGenArchive::from_bytes(raw, &master).unwrap();
    let mesh = image.body_query_mesh(checks::LIMITS).unwrap();
    assert_eq!(mesh.gen_revision(), Some(image.revision()));
    let mut expected = Cursor(cursor.span());
    let count = expected.word();
    assert_eq!(count, image.layout().counts[4]);
    exported.extend_from_slice(&count.to_le_bytes());
    for index in 0..count as usize {
        native_predicate(&image, index, &mut expected, exported);
    }
    totals[0] += count as usize;
    native_queries(&mesh, &mut expected, exported, totals);
    assert!(expected.0.is_empty());
    assert_eq!(image.to_bytes(), raw);
}

fn native_predicate(
    image: &SimulationGenArchive,
    index: usize,
    expected: &mut Cursor<'_>,
    exported: &mut Vec<u8>,
) {
    let record = image.element_record(index).unwrap();
    exported.extend(checks::predicate(&record, 0, 0, expected.span()));
    exported.extend(checks::predicate(&record, u32::MAX, -1, expected.span()));
    assert_eq!(
        record.to_bytes().as_slice(),
        &image.block(6).unwrap()[index * 140..(index + 1) * 140]
    );
}

fn native_queries(
    mesh: &SimulationBodyQueryMesh,
    expected: &mut Cursor<'_>,
    exported: &mut Vec<u8>,
    totals: &mut [usize; 4],
) {
    let count = expected.word();
    exported.extend_from_slice(&count.to_le_bytes());
    for _ in 0..count {
        let inputs = checks::query_inputs(expected);
        let bytes = checks::query(mesh, &inputs, expected.span());
        exported.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        exported.extend(bytes);
        totals[1] += 1;
        accumulate_primary(mesh, &inputs, totals);
    }
}

fn accumulate_primary(
    mesh: &SimulationBodyQueryMesh,
    inputs: &checks::QueryInputs,
    totals: &mut [usize; 4],
) {
    if inputs.context.mode() != 3 {
        return;
    }
    let query = mesh.project_body_nodes(inputs.context, None).unwrap();
    totals[2] += query.writes.len();
    totals[3] += query.marks.iter().filter(|v| v.is_some()).count();
}
