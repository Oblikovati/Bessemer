use super::packet::Cursor;
use bessemer::simulation::*;

pub fn classification(record: &SimulationElementRecord, expected: &[u8]) -> [u32; 6] {
    let mut cursor = Cursor(expected);
    let result = record.classification();
    let values = [
        u32::from(result.selector_family),
        u32::from(result.accessor_node_count),
        u32::from(result.selector_variant),
        u32::from(result.stored_code),
        result.selected_code,
        result.selected_signed() as u32,
    ];
    for value in values {
        assert_eq!(value, cursor.word());
    }
    assert!(cursor.0.is_empty());
    values
}

pub fn topology(record: &SimulationElementRecord, bound: Option<u32>, expected: &[u8]) -> Vec<u8> {
    let result = record.solid_ten_node(bound).unwrap();
    assert_eq!(result.record.id(), record.id());
    assert_eq!(result.record.to_bytes(), record.to_bytes());
    assert_topology_scope(&result, bound);
    let actual: Vec<_> = topology_patterns(&result)
        .into_iter()
        .flat_map(u32::to_le_bytes)
        .collect();
    assert_eq!(actual, expected);
    actual
}

#[test]
fn record_and_legacy_profile_guards_reject_unassigned_sizes_and_versions() {
    for size in 0..140 {
        assert!(SimulationElementRecord::from_bytes(&vec![0; size]).is_err());
    }
    for size in [141, 144, 280] {
        assert!(SimulationElementRecord::from_bytes(&vec![0; size]).is_err());
    }
    for version in [i32::MIN, -1, 0, 800049, 820025, 830038, i32::MAX] {
        let error = SimulationElementLegacyContext::new(version, 4).unwrap_err();
        assert!(error.to_string().contains(&version.to_string()));
    }
}

#[test]
fn solid_topology_gate_rejects_inactive_wrong_family_and_wrong_count() {
    for (offset, value) in [
        (0, 0),
        (20, 0),
        (20, 2),
        (23, 0),
        (23, 9),
        (23, 11),
        (23, 255),
    ] {
        let mut raw = solid();
        raw[offset] = value;
        let record = SimulationElementRecord::from_bytes(&raw).unwrap();
        assert!(record.solid_ten_node(None).is_err());
        assert_eq!(record.to_bytes(), raw);
    }
}

fn solid() -> [u8; 140] {
    let mut raw = [0x5a; 140];
    raw[0] = 255;
    raw[20] = 4;
    raw[23] = 10;
    raw
}

#[test]
fn source_mutation_failure_and_projection_do_not_change_the_published_snapshot() {
    let mut raw = solid();
    let record = SimulationElementRecord::from_bytes(&raw).unwrap();
    let copied = record.to_bytes();
    raw.fill(0);
    assert!(record.solid_ten_node(Some(0)).is_ok());
    assert_eq!(record.to_bytes(), copied);
    let context = SimulationElementLegacyContext::new(820024, i32::MIN).unwrap();
    let edit = record.project_legacy(context);
    assert_ne!(edit.record.id(), record.id());
    assert_eq!(record.to_bytes(), copied);
    let mut returned = edit.record.to_bytes();
    returned.fill(0);
    assert_ne!(edit.record.to_bytes(), returned);
    assert_eq!(record.to_bytes(), copied);
    assert!(SimulationElementRecord::from_bytes(&raw[..139]).is_err());
    assert_eq!(record.to_bytes(), copied);
}

#[test]
fn immutable_element_types_are_send_sync_and_parallel_projections_equal_serial_bytes() {
    assert_element_send_sync();
    let record = SimulationElementRecord::from_bytes(&solid()).unwrap();
    let context = SimulationElementLegacyContext::new(800050, 4).unwrap();
    let serial = record.project_legacy(context).record.to_bytes();
    std::thread::scope(|scope| {
        let threads: Vec<_> = (0..8)
            .map(|_| {
                scope.spawn(|| {
                    for _ in 0..32 {
                        assert_eq!(record.project_legacy(context).record.to_bytes(), serial);
                        assert_eq!(record.solid_ten_node(None).unwrap().edge_nodes.len(), 6);
                    }
                })
            })
            .collect();
        for task in threads {
            task.join().unwrap();
        }
    });
}

fn assert_element_send_sync() {
    fn check<T: Send + Sync>() {}
    check::<SimulationElementRecordId>();
    check::<SimulationElementRecord>();
    check::<SimulationElementClassification>();
    check::<SimulationElementLegacyContext>();
    check::<SimulationElementLegacyProjection>();
    check::<SimulationElementLegacyCoverage>();
    check::<SimulationElementDiagnostic>();
    check::<SimulationSolidEdgeAssociation>();
    check::<SimulationSolidTenNodeProjection>();
    check::<SimulationSolidTopologyCoverage>();
    check::<SimulationSolidTopologyCheckMode>();
}

fn assert_topology_scope(result: &SimulationSolidTenNodeProjection, bound: Option<u32>) {
    assert_eq!(result.coverage.node_count, bound);
    assert_eq!(
        result.coverage.mode,
        SimulationSolidTopologyCheckMode::ExactStoredAssociationsWithOptionalNodeBound
    );
    assert!(
        result
            .diagnostics
            .contains(&SimulationElementDiagnostic::GeometryNotValidated)
    );
    assert!(!result.coverage.averaging_executed);
    assert!(!result.coverage.geometry_complete);
}

fn topology_patterns(result: &SimulationSolidTenNodeProjection) -> Vec<u32> {
    let mut values = result.corner_node_ids.map(|id| id as u32).to_vec();
    for edge in result.edge_nodes {
        values.extend([
            edge.slot as u32,
            edge.node_id as u32,
            edge.endpoint_slots[0] as u32,
            edge.endpoint_slots[1] as u32,
            edge.endpoint_node_ids[0] as u32,
            edge.endpoint_node_ids[1] as u32,
        ]);
    }
    values.push(result.invalid_node_ids.len() as u32);
    values.extend(result.invalid_node_ids.iter().map(|id| *id as u32));
    values.push(u32::from(result.coverage.node_count.is_some()));
    values
}
