//! Signed diagnostic runs never restrict shared element references.
use bessemer::simulation::*;
use std::io::Read;
#[path = "common/node_group_packets.rs"]
mod checks;
#[allow(dead_code)]
#[path = "common/simulation_images.rs"]
mod packet;
use packet::Cursor;

fn owned_cases() -> Vec<u8> {
    let mut raw = Vec::new();
    flate2::read::ZlibDecoder::new(
        include_bytes!("fixtures/simulation-node-groups/groups.bin").as_slice(),
    )
    .read_to_end(&mut raw)
    .unwrap();
    raw
}

#[test]
fn entire_signed_domain_and_contiguous_runs_match_independent_and_actual_readers() {
    let raw = owned_cases();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"SNG1");
    assert_eq!(cursor.word(), 6);
    let mut nodes = 0;
    for _ in 0..6 {
        nodes += owned_case(&mut cursor);
    }
    assert_eq!(nodes, 81_933);
    assert!(cursor.0.is_empty());
}

fn owned_case(cursor: &mut Cursor<'_>) -> usize {
    let raw = cursor.span();
    let image = checks::owned_image(raw);
    assert_eq!(image.block(5).unwrap(), raw);
    assert_eq!(checks::node_bytes(&image), cursor.span());
    let projection = image.node_group_runs().unwrap();
    checks::assert_projection(&image, &projection);
    assert_eq!(checks::run_bytes(&projection), cursor.span());
    raw.len() / 52
}

#[test]
#[ignore = "requires exported approved local native corpus"]
fn every_native_signed_tag_run_and_shared_reference_matches_actual_complete_readers() {
    let raw = std::fs::read("target/simulation-node-groups-native.bin").unwrap();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"SNGN");
    assert_eq!(cursor.word(), 3);
    let mut output = b"SNGV".to_vec();
    let mut totals = [0usize; 3];
    for _ in 0..3 {
        native_pair(&mut cursor, &mut output, &mut totals);
    }
    assert!(cursor.0.is_empty());
    assert_eq!(totals, [45_246, 5, 152]);
    std::fs::write(
        "target/simulation-node-groups-native-projections.bin",
        output,
    )
    .unwrap();
}

fn native_pair(cursor: &mut Cursor<'_>, output: &mut Vec<u8>, totals: &mut [usize; 3]) {
    let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
    let raw = cursor.span();
    let image = SimulationGenArchive::from_bytes(raw, &master).unwrap();
    let mut expected = Cursor(cursor.span());
    let projection = image.node_group_runs().unwrap();
    checks::assert_projection(&image, &projection);
    let nodes = checks::node_bytes(&image);
    let runs = checks::run_bytes(&projection);
    assert_eq!(nodes, expected.span());
    assert_eq!(runs, expected.span());
    checks::append_span(output, &nodes);
    checks::append_span(output, &runs);
    native_queries(&image, &mut expected, output, totals);
    totals[0] += projection.checked_nodes;
    assert!(expected.0.is_empty());
    assert_eq!(image.to_bytes(), raw);
}

fn native_queries(
    image: &SimulationGenArchive,
    expected: &mut Cursor<'_>,
    output: &mut Vec<u8>,
    totals: &mut [usize; 3],
) {
    let mesh = checks::mesh(image);
    let count = expected.word();
    checks::append_words(output, &[count]);
    for _ in 0..count {
        let mut query = Cursor(expected.span());
        let actual = checks::query_bytes(&mesh, &mut query, &mut totals[2]);
        checks::append_span(output, &actual);
        totals[1] += 1;
    }
    assert_eq!(mesh.gen_revision(), Some(image.revision()));
}

#[test]
fn inactive_and_separated_runs_are_continuous_across_allocation_chunk_boundaries() {
    let mut nodes = vec![0u8; 52 * 16_387];
    for raw in nodes.as_chunks_mut::<52>().0.iter_mut().take(16_385) {
        raw[22..24].copy_from_slice(&65535u16.to_le_bytes());
    }
    let image = checks::owned_image(&nodes);
    let projection = image.node_group_runs().unwrap();
    assert_eq!(projection.runs.len(), 2);
    assert_run(&projection.runs[0], (-1, 1, 16_385));
    assert_run(&projection.runs[1], (0, 16_386, 16_387));
    assert!(
        image
            .block(5)
            .unwrap()
            .as_chunks::<52>()
            .0
            .iter()
            .all(|row| row[0] == 0)
    );
    checks::assert_projection(&image, &projection);
}

#[test]
fn malformed_parts_fail_without_changing_the_source_or_published_ranges() {
    let image = checks::owned_image(&[0; 104]);
    let result = image.node_group_runs().unwrap();
    let original = image.to_bytes();
    for length in (1..52).chain([53, 103, 105]) {
        let mut parts = image.parts();
        parts.blocks[5] = vec![0; length].into();
        assert!(SimulationGenArchive::from_parts(parts, &base_master()).is_err());
    }
    assert_eq!(image.to_bytes(), original);
    assert_eq!(result.source.to_bytes(), original);
    assert_eq!(result.runs.len(), 1);
}

fn base_master() -> SimulationMasterArchive {
    let raw = packet::owned();
    let mut cursor = Cursor(&raw);
    cursor.take(8);
    SimulationMasterArchive::from_bytes(cursor.span()).unwrap()
}

#[test]
fn captured_ranges_survive_source_drop_returned_byte_mutation_and_coordinate_edits() {
    let result = retained_projection();
    let original = result.source.to_bytes();
    let mut returned = result.source.to_bytes();
    returned.fill(0);
    let id = result.runs[0].first_node.clone();
    let edited = result.source.edit_node_coordinates(&id, [1, 2, 3]).unwrap();
    assert_ne!(edited.archive.revision(), &result.parent);
    assert!(edited.archive.node(&id).is_err());
    assert_eq!(result.source.to_bytes(), original);
    checks::assert_projection(&result.source, &result);
}

fn retained_projection() -> SimulationNodeGroupProjection {
    let mut nodes = vec![0; 104];
    nodes[22..24].copy_from_slice(&32768u16.to_le_bytes());
    let image = checks::owned_image(&nodes);
    let result = image.node_group_runs().unwrap();
    nodes.fill(255);
    assert_ne!(image.block(5).unwrap(), nodes);
    result
}

#[test]
fn node_group_public_types_are_send_sync_and_parallel_complete_bytes_match_serial() {
    assert_send_sync::<SimulationNodeGroupRun>();
    assert_send_sync::<SimulationNodeGroupProjection>();
    assert_send_sync::<SimulationNodeGroupCompletion>();
    assert_send_sync::<SimulationNodeGroupCheckMode>();
    assert_send_sync::<SimulationNodeGroupDiagnostic>();
    let image = retained_projection().source;
    let serial = checks::run_bytes(&image.node_group_runs().unwrap());
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let (image, serial) = (&image, &serial);
            scope.spawn(move || repeat_projection(image, serial));
        }
    });
}

fn assert_send_sync<T: Send + Sync>() {}

fn repeat_projection(image: &SimulationGenArchive, serial: &[u8]) {
    for _ in 0..32 {
        let result = image.node_group_runs().unwrap();
        assert_eq!(checks::run_bytes(&result), serial);
        checks::assert_projection(image, &result);
    }
}

fn assert_run(run: &SimulationNodeGroupRun, expected: (i16, usize, usize)) {
    assert_eq!((run.tag, run.first_ordinal, run.last_ordinal), expected);
}

#[test]
fn signed_negative_tag_and_other_group_tag_never_exclude_shared_element_references() {
    let mesh = shared_reference_mesh();
    let image = checks::owned_image(mesh.node_bytes());
    let result = image.node_group_runs().unwrap();
    assert_run(&result.runs[0], (-1, 1, 1));
    assert_run(&result.runs[1], (2, 2, 2));
    assert_ne!(i32::from(result.runs[0].tag), 65535);
    assert_eq!(checks::selected_nodes(&mesh, 65535), [1, 2]);
    assert!(checks::selected_nodes(&mesh, 2).is_empty());
    assert_eq!(image.block(5).unwrap(), mesh.node_bytes());
}

fn shared_reference_mesh() -> SimulationBodyQueryMesh {
    let mut nodes = [0u8; 104];
    nodes[0] = 1;
    nodes[52] = 255;
    nodes[22..24].copy_from_slice(&65535u16.to_le_bytes());
    nodes[74..76].copy_from_slice(&2u16.to_le_bytes());
    let element = shared_element();
    SimulationBodyQueryMesh::from_blocks(
        &nodes,
        &element,
        SimulationBodyQueryLimits {
            maximum_nodes: 2,
            maximum_elements: 1,
            maximum_write_visits: 2,
        },
    )
    .unwrap()
}

fn shared_element() -> [u8; 140] {
    let mut element = [0u8; 140];
    element[0] = 1;
    element[20] = 4;
    element[23] = 2;
    element[28..30].copy_from_slice(&65535u16.to_le_bytes());
    element[96..100].copy_from_slice(&1i32.to_le_bytes());
    element[100..104].copy_from_slice(&2i32.to_le_bytes());
    element
}
