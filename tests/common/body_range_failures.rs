use super::checks::{self, LIMITS};
use bessemer::simulation::*;

fn auxiliary(low: i32, high: i32) -> SimulationAuxiliaryBodyRow {
    let mut raw = vec![0xabu8; 288];
    raw[..4].copy_from_slice(&1u32.to_le_bytes());
    raw[16..20].copy_from_slice(&low.to_le_bytes());
    raw[20..24].copy_from_slice(&high.to_le_bytes());
    SimulationAuxiliaryBodyRow::from_bytes(&raw, SimulationAuxiliaryBodyProfile::Stored288).unwrap()
}

fn mesh_blocks() -> (Vec<u8>, Vec<u8>) {
    let mut nodes = vec![0xabu8; 3 * 52];
    nodes[0] = 1;
    nodes[52] = 0;
    nodes[104] = 255;
    let mut element = vec![0x5au8; 140];
    element[0] = 1;
    element[20] = 4;
    element[23] = 1;
    element[28..30].copy_from_slice(&1u16.to_le_bytes());
    element[96..100].copy_from_slice(&1i32.to_le_bytes());
    (nodes, element)
}

fn inputs(start: i32, upper: i32) -> SimulationBodyTypeInputs {
    SimulationBodyTypeInputs {
        body_id: SimulationBodySelectionId::from_pattern(1),
        start,
        exclusive_upper: upper,
    }
}

#[test]
fn profile_shapes_reject_every_prefix_extra_byte_and_unassigned_stride() {
    for stride in [0, 283, 285, 287, 289, usize::MAX] {
        assert!(SimulationAuxiliaryBodyProfile::from_stride(stride).is_err());
    }
    for stride in [284, 288] {
        let profile = SimulationAuxiliaryBodyProfile::from_stride(stride).unwrap();
        for size in 0..stride {
            assert!(SimulationAuxiliaryBodyRow::from_bytes(&vec![0; size], profile).is_err());
        }
        assert!(SimulationAuxiliaryBodyRow::from_bytes(&vec![0; stride + 1], profile).is_err());
    }
}

#[test]
fn wide_field_math_is_retained_while_native_decrement_overflow_is_rejected() {
    for (low, high) in [(i32::MIN, 1), (1, i32::MIN), (i32::MIN, i32::MIN)] {
        let row = auxiliary(low, high);
        let fields = row.fields();
        assert_eq!(fields.runtime_start, i64::from(low) - 1);
        assert_eq!(fields.runtime_upper, i64::from(high) - 1);
        let original = row.to_bytes();
        let error = row.descriptor_proposal().unwrap_err();
        assert!(error.to_string().contains(&i32::MIN.to_string()));
        assert_eq!(row.to_bytes(), original);
    }
    let skipped = auxiliary(0, 0).descriptor_proposal().unwrap();
    assert_eq!(
        skipped.completion,
        SimulationAuxiliaryDescriptorCompletion::SkippedBothZero
    );
    assert!(skipped.inputs.is_none());
    assert!(!skipped.coverage.complete_descriptor_serialization);
}

#[test]
fn negative_bounds_and_callback_dispatch_precede_scan_only_safety_checks() {
    let (nodes, mut element) = mesh_blocks();
    element[23] = 255;
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &element, LIMITS).unwrap();
    for callback in [None, Some(0)] {
        let guarded = mesh
            .project_body_type(inputs(-1, i32::MAX), 3, callback)
            .unwrap();
        assert_eq!(guarded.type_flag, -1);
        assert_eq!(guarded.supplied_callback_result, None);
        assert_eq!(
            guarded.completion,
            SimulationBodyTypeCompletion::GuardedNegativeBounds
        );
    }
    let override_result = mesh
        .project_body_type(inputs(i32::MAX, i32::MAX), 3, Some(0))
        .unwrap();
    assert_eq!(override_result.type_flag, 16);
    assert!(override_result.visited_elements.is_empty());
    assert_fixed_modes(&mesh);
}

#[test]
fn selected_unsafe_references_fail_without_mutating_source_or_publishing_partial_results() {
    let (nodes, element) = mesh_blocks();
    for id in [i32::MIN, -1, 4, i32::MAX] {
        let mut changed = element.clone();
        changed[96..100].copy_from_slice(&id.to_le_bytes());
        assert_bad_scan(&nodes, &changed);
    }
    for count in [12, 255] {
        let mut changed = element.clone();
        changed[23] = count;
        assert_bad_scan(&nodes, &changed);
    }
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &element, LIMITS).unwrap();
    for bounds in [inputs(0, 2), inputs(i32::MAX, 0), inputs(0, i32::MAX)] {
        assert!(mesh.project_body_type(bounds, 3, Some(-1)).is_err());
    }
}

fn assert_bad_scan(nodes: &[u8], element: &[u8]) {
    let mesh = SimulationBodyQueryMesh::from_blocks(nodes, element, LIMITS).unwrap();
    assert!(mesh.project_body_type(inputs(0, 1), 3, Some(-1)).is_err());
    assert_eq!(mesh.node_bytes(), nodes);
    assert_eq!(mesh.element_bytes(), element);
    let empty = mesh.project_body_type(inputs(0, 0), 3, Some(-1)).unwrap();
    assert_eq!(
        empty.completion,
        SimulationBodyTypeCompletion::CompletedNoAssociation
    );
    assert!(empty.first_association.is_none());
}

#[test]
fn inclusive_node_mark_and_exclusive_body_type_consumers_keep_identical_proposed_bounds_distinct() {
    let row = auxiliary(1, 1);
    let proposal = row.descriptor_proposal().unwrap();
    let proposed = proposal.inputs.unwrap();
    assert_eq!((proposed.start, proposed.exclusive_upper), (0, 0));
    let (nodes, element) = mesh_blocks();
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &element, LIMITS).unwrap();
    let exclusive = mesh.project_body_type(proposed, 3, Some(-1)).unwrap();
    assert_eq!(exclusive.type_flag, -1);
    assert!(exclusive.visited_elements.is_empty());
    assert_inclusive_marks(&mesh, proposed);
    let first = mesh.project_body_type(inputs(0, 1), 3, Some(-1)).unwrap();
    assert_eq!(first.type_flag, 1);
    assert!(!first.file_descriptor_binding);
    assert!(!proposal.coverage.complete_descriptor_serialization);
}

#[test]
fn unknown_first_association_stops_before_later_family_or_unvisited_invalid_reference() {
    let (nodes, mut first) = mesh_blocks();
    first[20] = 7;
    first[23] = 2;
    first[100..104].copy_from_slice(&(-1i32).to_le_bytes());
    let (_, second) = mesh_blocks();
    let elements = [first.clone(), second].concat();
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &elements, LIMITS).unwrap();
    let result = mesh.project_body_type(inputs(0, 2), 3, Some(-1)).unwrap();
    assert_eq!(result.type_flag, -1);
    assert_eq!(
        result.completion,
        SimulationBodyTypeCompletion::CompletedFirstAssociation
    );
    assert_eq!(result.visited_elements.as_ref(), [0]);
    assert_eq!(result.coverage.visited_references, 1);
    assert_eq!(result.first_association.unwrap().reference_slot, 0);
    let context =
        SimulationBodyQueryContext::new(inputs(0, 2).body_id, 3, Some(-1), Some((0, 1))).unwrap();
    assert!(mesh.project_body_nodes(context, None).is_err());
}

#[test]
fn body_type_is_read_only_and_does_not_consume_the_node_mark_write_budget() {
    let (nodes, element) = mesh_blocks();
    let limits = SimulationBodyQueryLimits {
        maximum_write_visits: 0,
        ..LIMITS
    };
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &element, limits).unwrap();
    let result = mesh.project_body_type(inputs(0, 1), 3, Some(-1)).unwrap();
    assert_eq!(result.type_flag, 1);
    assert_eq!(mesh.element_bytes(), element);
    assert_eq!(mesh.node_bytes(), nodes);
}

#[test]
fn raw_source_returned_bytes_and_source_drop_do_not_change_published_snapshots() {
    let (mut nodes, mut element) = mesh_blocks();
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &element, LIMITS).unwrap();
    let result = mesh.project_body_type(inputs(0, 1), 3, Some(-1)).unwrap();
    nodes.fill(0);
    element.fill(0);
    drop(mesh);
    assert_ne!(result.source.node_bytes(), nodes);
    assert_ne!(result.source.element_bytes(), element);
    let row = auxiliary(2, 5);
    let original = row.to_bytes();
    let edit = row
        .edit_fields(SimulationBodySelectionId::from_pattern(7), 3, 8)
        .unwrap();
    let mut returned = edit.row.to_bytes();
    returned.fill(0);
    assert_ne!(edit.row.to_bytes(), returned);
    assert_eq!(row.to_bytes(), original);
    let proposal = row.descriptor_proposal().unwrap();
    drop(row);
    assert_eq!(proposal.source.to_bytes(), original);
}

#[test]
fn body_query_and_auxiliary_public_types_are_send_sync_and_parallel_bytes_match_serial() {
    assert_send_sync();
    let (nodes, element) = mesh_blocks();
    let mesh = SimulationBodyQueryMesh::from_blocks(&nodes, &element, LIMITS).unwrap();
    let row = auxiliary(2, 5);
    let serial = checks::type_bytes(&mesh.project_body_type(inputs(0, 1), 3, Some(-1)).unwrap());
    std::thread::scope(|scope| {
        let threads: Vec<_> = (0..8)
            .map(|_| scope.spawn(|| repeat_projection(&mesh, &row, &serial)))
            .collect();
        for task in threads {
            task.join().unwrap();
        }
    });
}

fn repeat_projection(
    mesh: &SimulationBodyQueryMesh,
    row: &SimulationAuxiliaryBodyRow,
    serial: &[u8],
) {
    let original = row.to_bytes();
    for _ in 0..32 {
        let result = mesh.project_body_type(inputs(0, 1), 3, Some(-1)).unwrap();
        assert_eq!(checks::type_bytes(&result), serial);
        assert_eq!(result.source.revision(), mesh.revision());
        let edit = row
            .edit_fields(SimulationBodySelectionId::from_pattern(7), 3, 8)
            .unwrap();
        let reverted = edit.row.edit_fields(row.fields().body_id, 2, 5).unwrap();
        assert_eq!(reverted.row.to_bytes(), original);
        assert_eq!(
            row.descriptor_proposal().unwrap().source.revision(),
            row.revision()
        );
    }
}

fn assert_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationBodyTypeInputs>();
    check::<SimulationBodyTypeResult>();
    check::<SimulationBodyTypeCompletion>();
    check::<SimulationBodyTypeCheckMode>();
    check::<SimulationBodyTypeAssociation>();
    check::<SimulationBodyTypeCoverage>();
    check::<SimulationAuxiliaryBodyProfile>();
    check::<SimulationAuxiliaryBodyRevision>();
    check::<SimulationAuxiliaryBodyFields>();
    check::<SimulationAuxiliaryBodyRow>();
    check::<SimulationAuxiliaryBodyCheckMode>();
    check::<SimulationAuxiliaryBodyCoverage>();
    check::<SimulationAuxiliaryDescriptorCompletion>();
    check::<SimulationAuxiliaryBodyDiagnostic>();
    check::<SimulationAuxiliaryDescriptorProposal>();
    check::<SimulationAuxiliaryBodyFidelity>();
    check::<SimulationAuxiliaryBodyEdit>();
}

fn assert_fixed_modes(mesh: &SimulationBodyQueryMesh) {
    for (mode, flag) in [(0, 1), (1, 2), (2, 2), (4, 4), (5, -1), (6, 8), (7, -1)] {
        assert_eq!(
            mesh.project_body_type(inputs(i32::MAX, i32::MAX), mode, Some(-1))
                .unwrap()
                .type_flag,
            flag
        );
        assert!(mesh.project_body_type(inputs(0, 0), mode, None).is_err());
    }
}

fn assert_inclusive_marks(mesh: &SimulationBodyQueryMesh, proposed: SimulationBodyTypeInputs) {
    let context = SimulationBodyQueryContext::new(
        proposed.body_id,
        3,
        Some(-1),
        Some((proposed.start, proposed.exclusive_upper)),
    )
    .unwrap();
    let inclusive = mesh.project_body_nodes(context, None).unwrap();
    assert_eq!(inclusive.writes.len(), 1);
    assert_eq!(inclusive.type_flag, 1);
}
