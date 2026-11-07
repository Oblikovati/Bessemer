use super::checks::{self, LIMITS};
use bessemer::simulation::*;
use std::sync::Arc;

fn blocks() -> (Vec<u8>, Vec<u8>) {
    let mut nodes = vec![0xabu8; 3 * 52];
    nodes[0] = 1;
    nodes[52] = 0;
    nodes[104] = 255;
    let mut elements = vec![0x5a; 2 * 140];
    for (index, family) in [4, 8].into_iter().enumerate() {
        let row = &mut elements[index * 140..(index + 1) * 140];
        row[0] = 1;
        row[20] = family;
        row[23] = if index == 0 { 3 } else { 2 };
        row[28..30].copy_from_slice(&1u16.to_le_bytes());
        for (slot, id) in [1i32, 0, 3].into_iter().enumerate() {
            row[96 + slot * 4..100 + slot * 4].copy_from_slice(&id.to_le_bytes());
        }
    }
    (nodes, elements)
}

fn context(
    body: u32,
    mode: i32,
    callback: Option<i16>,
    range: Option<(i32, i32)>,
) -> SimulationBodyQueryContext {
    SimulationBodyQueryContext::new(
        SimulationBodySelectionId::from_pattern(body),
        mode,
        callback,
        range,
    )
    .unwrap()
}

#[test]
fn whole_records_source_budgets_and_initial_shapes_are_required_before_publication() {
    let (nodes, elements) = blocks();
    assert_block_shapes(&nodes, &elements);
    assert_source_budgets(&nodes, &elements);
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &elements, LIMITS).unwrap();
    let ctx = context(1, 3, Some(-1), Some((0, 1)));
    for length in [0, 2, 4] {
        assert!(
            mesh.project_body_nodes(ctx, Some(&vec![None; length]))
                .is_err()
        );
    }
    assert_eq!(mesh.node_bytes(), nodes);
    assert_eq!(mesh.element_bytes(), elements);
}

fn assert_block_shapes(nodes: &[u8], elements: &[u8]) {
    for size in [1, 51, 53, 155] {
        assert!(SimulationBodyQueryMesh::from_blocks(&nodes[..size], elements, LIMITS).is_err());
    }
    for size in [1, 139, 141, 279] {
        assert!(SimulationBodyQueryMesh::from_blocks(nodes, &elements[..size], LIMITS).is_err());
    }
}

fn assert_source_budgets(nodes: &[u8], elements: &[u8]) {
    for limits in [
        SimulationBodyQueryLimits {
            maximum_nodes: 2,
            ..LIMITS
        },
        SimulationBodyQueryLimits {
            maximum_elements: 1,
            ..LIMITS
        },
    ] {
        assert!(SimulationBodyQueryMesh::from_blocks(nodes, elements, limits).is_err());
    }
}

#[test]
fn callback_and_range_guards_preserve_mode_one_bypass_and_empty_range_rules() {
    let body = SimulationBodySelectionId::from_pattern(1);
    for mode in [i32::MIN, -1, 0, 2, 3, 4, 5, 6, i32::MAX] {
        assert!(SimulationBodyQueryContext::new(body, mode, None, None).is_err());
    }
    for mode in 2..=5 {
        assert!(SimulationBodyQueryContext::new(body, mode, Some(-1), None).is_err());
        for range in [(i32::MAX, 0), (0, i32::MAX), (i32::MAX, i32::MAX)] {
            assert!(SimulationBodyQueryContext::new(body, mode, Some(-1), Some(range)).is_err());
        }
    }
    let bypass = context(u32::MAX, 1, None, Some((i32::MAX, i32::MAX)));
    assert_eq!(bypass.callback_result(), None);
    assert_eq!(bypass.mode(), 1);
    assert_eq!(bypass.body_id().pattern(), u32::MAX);
}

#[test]
fn nonempty_ranges_reject_unsafe_endpoints_while_empty_ranges_retain_prior_marks() {
    let (nodes, elements) = blocks();
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &elements, LIMITS).unwrap();
    for range in [(-1, 0), (0, 2), (i32::MIN, -1)] {
        assert!(
            mesh.project_body_nodes(context(1, 3, Some(-1), Some(range)), None)
                .is_err()
        );
    }
    for range in [(100, 99), (-1, -2), (0, -1), (i32::MAX - 1, i32::MAX - 2)] {
        assert_empty_range(&mesh, range);
    }
}

#[test]
fn selected_count_and_reference_errors_do_not_publish_partial_marks_or_mutate_inputs() {
    let (nodes, elements) = blocks();
    for count in [12, 255] {
        let mut changed = elements.clone();
        changed[23] = count;
        assert_selected_failure(&nodes, &changed);
    }
    for id in [i32::MIN, -1, 4, i32::MAX] {
        let mut changed = elements.clone();
        changed[100..104].copy_from_slice(&id.to_le_bytes());
        assert_selected_failure(&nodes, &changed);
    }
}

fn assert_selected_failure(nodes: &[u8], elements: &[u8]) {
    let mesh = SimulationBodyQueryMesh::from_blocks(nodes, elements, LIMITS).unwrap();
    assert!(mesh.gen_revision().is_none());
    let initial = [Some(7), None, Some(9)];
    let result = mesh.project_body_nodes(context(1, 3, Some(-1), Some((0, 1))), Some(&initial));
    assert!(result.is_err());
    assert_eq!(mesh.node_bytes(), nodes);
    assert_eq!(mesh.element_bytes(), elements);
    assert_eq!(initial, [Some(7), None, Some(9)]);
    let unmatched = mesh
        .project_body_nodes(context(2, 3, Some(-1), Some((0, 1))), None)
        .unwrap();
    assert!(unmatched.writes.is_empty());
    let bypass = mesh
        .project_body_nodes(context(u32::MAX, 1, None, None), None)
        .unwrap();
    assert_eq!(
        bypass.marks.as_ref(),
        [Some(u32::MAX), None, Some(u32::MAX)]
    );
}

#[test]
fn actual_write_budget_counts_duplicate_visits_and_preserves_exact_boundary_success() {
    let (nodes, elements) = blocks();
    for maximum in [0, 1, 2] {
        assert_write_budget_failure(&nodes, &elements, maximum);
    }
    let limits = SimulationBodyQueryLimits {
        maximum_write_visits: 3,
        ..LIMITS
    };
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &elements, limits).unwrap();
    let result = mesh
        .project_body_nodes(context(1, 3, Some(-1), Some((0, 1))), None)
        .unwrap();
    assert_eq!(result.writes.len(), 3);
    assert_eq!(result.writes[2].previous_pattern, Some(1));
    assert_eq!(result.coverage.visited_references, 5);
}

#[test]
fn unknown_family_first_success_resolves_minus_one_and_callback_overrides_it() {
    let (nodes, mut elements) = blocks();
    elements[20] = 7;
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &elements, LIMITS).unwrap();
    let result = mesh
        .project_body_nodes(context(1, 3, Some(-1), Some((0, 1))), None)
        .unwrap();
    assert_eq!(result.type_flag, -1);
    assert!(result.type_flag_resolved);
    for callback in [i16::MIN, 0, i16::MAX] {
        let result = mesh
            .project_body_nodes(context(1, 3, Some(callback), Some((0, 1))), None)
            .unwrap();
        assert_eq!(result.type_flag, 16);
        assert!(result.type_flag_resolved);
    }
    assert_unknown_fallback(&nodes, &elements);
}

fn assert_unknown_fallback(nodes: &[u8], original: &[u8]) {
    let mut elements = original.to_vec();
    elements[20] = 4;
    let fallback = SimulationBodyQueryMesh::from_blocks(nodes, &elements, LIMITS).unwrap();
    let result = fallback
        .project_body_nodes(context(1, 7, Some(-1), None), None)
        .unwrap();
    assert!(!result.writes.is_empty());
    assert_eq!(result.type_flag, -1);
    assert!(!result.type_flag_resolved);
}

#[test]
fn source_and_prior_mark_mutation_drop_and_result_clone_do_not_change_published_results() {
    let (mut nodes, mut elements) = blocks();
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &elements, LIMITS).unwrap();
    let mut initial = [Some(9), Some(8), None];
    let result = mesh
        .project_body_nodes(context(1, 3, Some(-1), Some((0, 1))), Some(&initial))
        .unwrap();
    nodes.fill(0);
    elements.fill(0);
    initial.fill(None);
    assert_ne!(result.source.node_bytes(), nodes);
    assert_ne!(result.source.element_bytes(), elements);
    assert_eq!(result.initial_marks.as_ref(), [Some(9), Some(8), None]);
    assert_eq!(result.marks.as_ref(), [Some(1), Some(8), Some(1)]);
    let mut copied = result.clone();
    Arc::make_mut(&mut copied.marks).fill(None);
    assert_eq!(result.marks.as_ref(), [Some(1), Some(8), Some(1)]);
    drop(mesh);
    assert_ne!(result.source.node_bytes(), nodes);
}

#[test]
fn published_query_types_are_send_sync_and_concurrent_results_equal_serial_bytes() {
    assert_send_sync();
    let (nodes, elements) = blocks();
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &elements, LIMITS).unwrap();
    let ctx = context(1, 3, Some(-1), Some((0, 1)));
    let serial = checks::query_bytes(&mesh.project_body_nodes(ctx, None).unwrap());
    std::thread::scope(|scope| {
        let threads: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat_query(&mesh, ctx, &serial)))
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
    });
}

fn repeat_query(
    mesh: &SimulationBodyQueryMesh,
    context: SimulationBodyQueryContext,
    serial: &[u8],
) {
    for _ in 0..32 {
        let result = mesh.project_body_nodes(context, None).unwrap();
        assert_eq!(checks::query_bytes(&result), serial);
        assert_eq!(result.source.revision(), mesh.revision());
    }
}

fn assert_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationElementFamilyMask>();
    check::<SimulationElementSelectionView>();
    check::<SimulationBodySelectionId>();
    check::<SimulationBodyQueryContext>();
    check::<SimulationBodyMeshRevision>();
    check::<SimulationBodyQueryLimits>();
    check::<SimulationBodyQueryMesh>();
    check::<SimulationBodyQueryCompletion>();
    check::<SimulationBodyWriteSource>();
    check::<SimulationBodyNodeWrite>();
    check::<SimulationBodyQueryCheckMode>();
    check::<SimulationBodyQueryCoverage>();
    check::<SimulationBodyQueryDiagnostic>();
    check::<SimulationBodyQueryResult>();
}

fn assert_empty_range(mesh: &SimulationBodyQueryMesh, range: (i32, i32)) {
    let initial = [Some(0), None, Some(u32::MAX)];
    let result = mesh
        .project_body_nodes(context(1, 3, Some(-1), Some(range)), Some(&initial))
        .unwrap();
    assert_eq!(
        result.completion,
        SimulationBodyQueryCompletion::CompletedEmpty
    );
    assert_eq!(result.marks.as_ref(), initial);
    assert_eq!(result.coverage.visited_elements, 0);
    assert!(!result.type_flag_resolved);
}

fn assert_write_budget_failure(nodes: &[u8], elements: &[u8], maximum: usize) {
    let limits = SimulationBodyQueryLimits {
        maximum_write_visits: maximum,
        ..LIMITS
    };
    let mesh = SimulationBodyQueryMesh::from_blocks(nodes, elements, limits).unwrap();
    assert!(
        mesh.project_body_nodes(context(1, 3, Some(-1), Some((0, 1))), None)
            .is_err()
    );
    assert_eq!(mesh.element_bytes(), elements);
}
