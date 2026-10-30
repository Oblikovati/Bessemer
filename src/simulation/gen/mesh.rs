use super::{Result, SimulationGenArchive, SimulationGenRevision, invalid, require};
use crate::simulation::wire;

/// Normalized node index bound to one complete GEN revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationNodeId {
    pub(super) revision: SimulationGenRevision,
    pub(super) index: usize,
}

/// Documented stored node fields; units and coordinate frame remain unknown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationNodeView {
    /// Raw activity marker; every nonzero pattern denotes active.
    pub activity: u8,
    /// Signed body/group tag, without an assigned ownership meaning.
    pub group_tag: i16,
    /// Exact IEEE binary32 patterns, including NaNs and signed zero.
    pub coordinate_patterns: [u32; 3],
}

/// Documented element accessors, without geometry validation claims.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationElementView {
    /// Raw activity marker.
    pub activity: u8,
    /// Selected prefix length of the eleven retained node slots.
    pub node_count: u8,
    /// All eight signed ELIS slots, with no additional assigned role.
    pub elis_slots: [i32; 8],
    /// All eleven signed slots, including unused negative patterns.
    pub node_slots: [i32; 11],
}

impl SimulationGenArchive {
    /// Bind a zero-based node index once, e.g. `gen.node_id(0)?`.
    pub fn node_id(&self, index: usize) -> Result<SimulationNodeId> {
        require(
            index < self.layout.counts[3] as usize,
            "GEN node",
            format!(
                "index {index}; expected less than {}",
                self.layout.counts[3]
            ),
        )?;
        Ok(SimulationNodeId {
            revision: self.revision.clone(),
            index,
        })
    }

    /// Project known stored fields, e.g. `gen.node(&id)?`; this does not validate geometry.
    pub fn node(&self, id: &SimulationNodeId) -> Result<SimulationNodeView> {
        require(
            id.revision == self.revision,
            "GEN node",
            "foreign revision; expected this GEN image",
        )?;
        let row = self.mesh_record::<52>(5, id.index, "GEN node")?;
        Ok(SimulationNodeView {
            activity: row[0],
            group_tag: i16::from_le_bytes([row[22], row[23]]),
            coordinate_patterns: wire::patterns(&row[32..44], "GEN node coordinates")?,
        })
    }

    /// Project one element's stored accessor fields, e.g. `gen.element(0)?`.
    pub fn element(&self, index: usize) -> Result<SimulationElementView> {
        let row = self.mesh_record::<140>(6, index, "GEN element")?;
        require(
            row[23] <= 11,
            "GEN element accessor",
            format!("node count {}; expected 0..=11", row[23]),
        )?;
        Ok(SimulationElementView {
            activity: row[0],
            node_count: row[23],
            elis_slots: wire::patterns::<8>(&row[64..96], "GEN ELIS")?.map(|value| value as i32),
            node_slots: wire::patterns::<11>(&row[96..140], "GEN node slots")?
                .map(|value| value as i32),
        })
    }
    fn mesh_record<const N: usize>(
        &self,
        block: usize,
        index: usize,
        entity: &str,
    ) -> Result<&[u8; N]> {
        let records = self.block(block)?.as_chunks::<N>().0;
        records.get(index).ok_or_else(|| {
            invalid(
                entity,
                format!(
                    "index {index}; expected less than {} records",
                    records.len()
                ),
            )
        })
    }
}

impl SimulationElementView {
    /// Selected signed node IDs, e.g. `element.node_ids()`; unused slots stay available separately.
    pub fn node_ids(&self) -> &[i32] {
        &self.node_slots[..usize::from(self.node_count.min(11))]
    }

    /// Selected references outside the paired node table, e.g. `element.invalid_node_ids(gen.layout().counts[3])`.
    pub fn invalid_node_ids(&self, node_count: u32) -> Vec<i32> {
        self.node_ids()
            .iter()
            .copied()
            .filter(|id| *id < 1 || i64::from(*id) > i64::from(node_count))
            .collect()
    }
}
