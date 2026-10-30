use super::{Result, SimulationElementDiagnostic, SimulationElementRecord, require};

/// One ordered midside association from the named solid averaging consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationSolidEdgeAssociation {
    /// Zero-based midside slot within the retained node list.
    pub slot: usize,
    /// Stored signed one-based ID; invalid and duplicate IDs are preserved.
    pub node_id: i32,
    /// Zero-based corner slots, in native consumer order.
    pub endpoint_slots: [usize; 2],
    /// Corresponding stored signed IDs.
    pub endpoint_node_ids: [i32; 2],
}

/// Exact node association projection, distinct from validated geometry or averaging.
#[derive(Debug, Clone)]
pub struct SimulationSolidTenNodeProjection {
    /// Complete original immutable record and its identity.
    pub record: SimulationElementRecord,
    /// Stored first four corner IDs in order.
    pub corner_node_ids: [i32; 4],
    /// Six ordered pair associations, without midpoint coordinate constraints.
    pub edge_nodes: [SimulationSolidEdgeAssociation; 6],
    /// Invalid selected IDs in original order, including duplicates.
    pub invalid_node_ids: Vec<i32>,
    /// Exact projection check scope and supplied optional bound.
    pub coverage: SimulationSolidTopologyCoverage,
    /// Explicit runtime and engineering exclusions.
    pub diagnostics: [SimulationElementDiagnostic; 3],
}

/// Declared exact association scope, without a geometry-validity claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSolidTopologyCheckMode {
    /// Every selected stored ID and association is read, with the supplied optional bound.
    ExactStoredAssociationsWithOptionalNodeBound,
}

/// Explicit record gate and reference-check scope; no full topology validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationSolidTopologyCoverage {
    /// Exact association extraction scope.
    pub mode: SimulationSolidTopologyCheckMode,
    /// Optional caller-supplied table bound; absence checks only positivity.
    pub node_count: Option<u32>,
    /// Always false: association extraction does not execute averaging.
    pub averaging_executed: bool,
    /// Always false: IDs and associations do not establish consistent geometry.
    pub geometry_complete: bool,
}

impl SimulationElementRecord {
    /// Extract active family-4/count-10 associations, e.g. `record.solid_ten_node(Some(node_count))?`.
    pub fn solid_ten_node(
        &self,
        node_count: Option<u32>,
    ) -> Result<SimulationSolidTenNodeProjection> {
        validate_solid_gate(&self.bytes)?;
        Ok(topology_result(self, node_count))
    }

    /// Test a caller-supplied first record after native count/pointer guards, e.g. `first.beam_first_record_predicate()`; never scans other records.
    pub fn beam_first_record_predicate(&self) -> bool {
        self.bytes[20] == 2 && self.classification().stored_code != 37
    }
}

fn validate_solid_gate(bytes: &[u8; 140]) -> Result<()> {
    require(
        bytes[0] != 0 && bytes[20] == 4 && bytes[23] == 10,
        "Simulation solid topology",
        format!(
            "activity {}, family {}, count {}; expected nonzero activity, family 4, count 10",
            bytes[0], bytes[20], bytes[23]
        ),
    )
}

fn topology_result(
    record: &SimulationElementRecord,
    node_count: Option<u32>,
) -> SimulationSolidTenNodeProjection {
    let ids = std::array::from_fn::<_, 10, _>(|slot| {
        let start = 96 + slot * 4;
        i32::from_le_bytes([
            record.bytes[start],
            record.bytes[start + 1],
            record.bytes[start + 2],
            record.bytes[start + 3],
        ])
    });
    SimulationSolidTenNodeProjection {
        record: record.clone(),
        corner_node_ids: [ids[0], ids[1], ids[2], ids[3]],
        edge_nodes: edge_associations(&ids),
        invalid_node_ids: ids
            .into_iter()
            .filter(|id| *id < 1 || node_count.is_some_and(|n| i64::from(*id) > i64::from(n)))
            .collect(),
        coverage: topology_coverage(node_count),
        diagnostics: super::projection_diagnostics(),
    }
}

fn topology_coverage(node_count: Option<u32>) -> SimulationSolidTopologyCoverage {
    SimulationSolidTopologyCoverage {
        mode: SimulationSolidTopologyCheckMode::ExactStoredAssociationsWithOptionalNodeBound,
        node_count,
        averaging_executed: false,
        geometry_complete: false,
    }
}

fn edge_associations(ids: &[i32; 10]) -> [SimulationSolidEdgeAssociation; 6] {
    [
        (4, 0, 1),
        (5, 1, 2),
        (6, 0, 2),
        (7, 0, 3),
        (8, 1, 3),
        (9, 2, 3),
    ]
    .map(|(slot, a, b)| SimulationSolidEdgeAssociation {
        slot,
        node_id: ids[slot],
        endpoint_slots: [a, b],
        endpoint_node_ids: [ids[a], ids[b]],
    })
}
