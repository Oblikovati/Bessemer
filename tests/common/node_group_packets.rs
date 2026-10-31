use super::packet::{self, Cursor};
use bessemer::simulation::*;

pub fn owned_image(nodes: &[u8]) -> SimulationGenArchive {
    let raw = packet::owned();
    let mut cursor = Cursor(&raw);
    assert_eq!(cursor.take(4), b"SIMO");
    assert_eq!(cursor.word(), 9);
    let master = SimulationMasterArchive::from_bytes(cursor.span()).unwrap();
    let base = SimulationGenArchive::from_bytes(cursor.span(), &master).unwrap();
    let mut parts = base.parts();
    parts.counts[3] = (nodes.len() / 52) as u32;
    parts.blocks[5] = nodes.into();
    SimulationGenArchive::from_parts(parts, &master).unwrap()
}

pub fn append_words(output: &mut Vec<u8>, values: &[u32]) {
    output.extend(values.iter().flat_map(|value| value.to_le_bytes()));
}

pub fn append_span(output: &mut Vec<u8>, bytes: &[u8]) {
    append_words(output, &[bytes.len() as u32]);
    output.extend_from_slice(bytes);
}

pub fn node_bytes(image: &SimulationGenArchive) -> Vec<u8> {
    let count = image.layout().counts[3] as usize;
    let mut output = Vec::new();
    append_words(&mut output, &[count as u32]);
    for index in 0..count {
        let id = image.node_id(index).unwrap();
        let view = image.node(&id).unwrap();
        append_words(
            &mut output,
            &[
                u32::from(view.activity),
                view.group_tag as u32,
                u32::from(view.group_tag as u16),
            ],
        );
    }
    output
}

pub fn run_bytes(projection: &SimulationNodeGroupProjection) -> Vec<u8> {
    let mut output = Vec::new();
    append_words(&mut output, &[projection.runs.len() as u32]);
    for run in projection.runs.iter() {
        append_words(
            &mut output,
            &[
                run.tag as u32,
                run.first_ordinal as u32,
                run.last_ordinal as u32,
            ],
        );
    }
    output
}

pub fn assert_projection(image: &SimulationGenArchive, result: &SimulationNodeGroupProjection) {
    let count = image.layout().counts[3] as usize;
    assert_eq!(&result.parent, image.revision());
    assert_eq!(result.source.revision(), image.revision());
    assert_eq!(result.source.to_bytes(), image.to_bytes());
    assert_eq!(
        result.check_mode,
        SimulationNodeGroupCheckMode::ExactAllStoredTagsAndContiguousRuns
    );
    assert_eq!(result.checked_nodes, count);
    assert_eq!(result.checked_tag_bytes, count * 2);
    assert_eq!(result.tag_pattern_error_bits, 0);
    assert_completion(result, count);
    assert_partition(result, count);
}

fn assert_partition(result: &SimulationNodeGroupProjection, count: usize) {
    let mut previous = None;
    let mut next_ordinal = 1;
    for run in result.runs.iter() {
        assert_eq!(run.first_ordinal, next_ordinal);
        assert!(run.last_ordinal >= run.first_ordinal && run.last_ordinal <= count);
        assert_ne!(previous, Some(run.tag));
        assert_endpoints(&result.source, run);
        for ordinal in run.first_ordinal..=run.last_ordinal {
            let id = result.source.node_id(ordinal - 1).unwrap();
            assert_eq!(result.source.node(&id).unwrap().group_tag, run.tag);
        }
        next_ordinal = run.last_ordinal + 1;
        previous = Some(run.tag);
    }
    assert_eq!(next_ordinal, count + 1);
}

pub fn mesh(image: &SimulationGenArchive) -> SimulationBodyQueryMesh {
    image
        .body_query_mesh(SimulationBodyQueryLimits {
            maximum_nodes: 100_000,
            maximum_elements: 100_000,
            maximum_write_visits: 1_000_000,
        })
        .unwrap()
}

pub fn selected_nodes(mesh: &SimulationBodyQueryMesh, body: u32) -> Vec<usize> {
    let context = SimulationBodyQueryContext::new(
        SimulationBodySelectionId::from_pattern(body),
        3,
        Some(-1),
        Some((0, mesh.element_bytes().len() as i32 / 140 - 1)),
    )
    .unwrap();
    let result = mesh.project_body_nodes(context, None).unwrap();
    result
        .marks
        .iter()
        .enumerate()
        .filter_map(|(slot, mark)| mark.map(|_| slot + 1))
        .collect()
}

pub fn query_bytes(
    mesh: &SimulationBodyQueryMesh,
    cursor: &mut Cursor<'_>,
    cross_total: &mut usize,
) -> Vec<u8> {
    let body = cursor.word();
    let selected = selected_nodes(mesh, body);
    let crossed = cross_tag_nodes(mesh, body, &selected);
    assert_ordinals(cursor, &selected);
    assert_ordinals(cursor, &crossed);
    assert!(cursor.0.is_empty());
    let mut output = Vec::new();
    append_words(&mut output, &[body]);
    append_ordinals(&mut output, &selected);
    append_ordinals(&mut output, &crossed);
    *cross_total += crossed.len();
    output
}

fn assert_completion(result: &SimulationNodeGroupProjection, count: usize) {
    let expected = if count == 0 {
        SimulationNodeGroupCompletion::CompletedEmpty
    } else {
        SimulationNodeGroupCompletion::CompletedRuns
    };
    assert_eq!(result.completion, expected);
    assert_eq!(
        result.diagnostics,
        [
            SimulationNodeGroupDiagnostic::ExclusiveBodyOwnershipUnassigned,
            SimulationNodeGroupDiagnostic::NativeElementDrivenTraversalNotExecuted
        ]
    );
}

fn assert_endpoints(image: &SimulationGenArchive, run: &SimulationNodeGroupRun) {
    assert_eq!(
        run.first_node,
        image.node_id(run.first_ordinal - 1).unwrap()
    );
    assert_eq!(run.last_node, image.node_id(run.last_ordinal - 1).unwrap());
}

fn cross_tag_nodes(mesh: &SimulationBodyQueryMesh, body: u32, selected: &[usize]) -> Vec<usize> {
    selected
        .iter()
        .copied()
        .filter(|ordinal| {
            let offset = (ordinal - 1) * 52 + 22;
            let tag = i16::from_le_bytes(mesh.node_bytes()[offset..offset + 2].try_into().unwrap());
            i32::from(tag) != body as i32
        })
        .collect()
}

fn assert_ordinals(cursor: &mut Cursor<'_>, ordinals: &[usize]) {
    assert_eq!(ordinals.len(), cursor.word() as usize);
    for ordinal in ordinals {
        assert_eq!(*ordinal, cursor.word() as usize);
    }
}

fn append_ordinals(output: &mut Vec<u8>, ordinals: &[usize]) {
    append_words(output, &[ordinals.len() as u32]);
    for ordinal in ordinals {
        append_words(output, &[*ordinal as u32]);
    }
}
