use super::{Result, require};
use crate::archive_codec::ArchiveRevision;
use crate::simulation::{SimulationGenArchive, SimulationGenRevision};
use std::sync::Arc;

/// Identity of immutable supplied whole node/element blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationBodyMeshRevision(ArchiveRevision);

/// Explicit bounds for source extents and ordered duplicate write history.
#[derive(Debug, Clone, Copy)]
pub struct SimulationBodyQueryLimits {
    /// Maximum complete 52-byte node records.
    pub maximum_nodes: usize,
    /// Maximum complete 140-byte element records.
    pub maximum_elements: usize,
    /// Maximum actual write visits, including repeated destinations.
    pub maximum_write_visits: usize,
}

/// Immutable complete raw blocks; selected access validation occurs in each query.
#[derive(Debug, Clone)]
pub struct SimulationBodyQueryMesh {
    pub(super) nodes: Arc<[u8]>,
    pub(super) elements: Arc<[u8]>,
    pub(super) revision: SimulationBodyMeshRevision,
    pub(super) gen_revision: Option<SimulationGenRevision>,
    pub(super) limits: SimulationBodyQueryLimits,
}

impl SimulationBodyQueryMesh {
    /// Own whole blocks without validating unvisited references, e.g. `SimulationBodyQueryMesh::from_blocks(nodes, elements, limits)?`.
    pub fn from_blocks(
        nodes: &[u8],
        elements: &[u8],
        limits: SimulationBodyQueryLimits,
    ) -> Result<Self> {
        validate_blocks(nodes, elements, limits)?;
        Ok(Self {
            nodes: nodes.into(),
            elements: elements.into(),
            revision: SimulationBodyMeshRevision(ArchiveRevision::default()),
            gen_revision: None,
            limits,
        })
    }

    /// Source identity for every history index, e.g. `mesh.revision()`.
    pub fn revision(&self) -> &SimulationBodyMeshRevision {
        &self.revision
    }

    /// Optional paired GEN copy parent, e.g. `mesh.gen_revision()`; loose caller blocks have no inferred parent.
    pub fn gen_revision(&self) -> Option<&SimulationGenRevision> {
        self.gen_revision.as_ref()
    }

    /// Complete immutable node bytes, e.g. `mesh.node_bytes()`.
    pub fn node_bytes(&self) -> &[u8] {
        &self.nodes
    }

    /// Complete immutable element bytes, e.g. `mesh.element_bytes()`.
    pub fn element_bytes(&self) -> &[u8] {
        &self.elements
    }
}

impl SimulationGenArchive {
    /// Capture actual complete paired blocks for queries, e.g. `image.body_query_mesh(limits)?`.
    pub fn body_query_mesh(
        &self,
        limits: SimulationBodyQueryLimits,
    ) -> Result<SimulationBodyQueryMesh> {
        let mut mesh =
            SimulationBodyQueryMesh::from_blocks(self.block(5)?, self.block(6)?, limits)?;
        mesh.gen_revision = Some(self.revision().clone());
        Ok(mesh)
    }
}

fn validate_blocks(nodes: &[u8], elements: &[u8], limits: SimulationBodyQueryLimits) -> Result<()> {
    require(
        nodes.len().is_multiple_of(52) && elements.len().is_multiple_of(140),
        "body query blocks",
        format!(
            "{} node bytes and {} element bytes; expected whole 52-byte and 140-byte records",
            nodes.len(),
            elements.len()
        ),
    )?;
    require(
        nodes.len() / 52 <= limits.maximum_nodes && elements.len() / 140 <= limits.maximum_elements,
        "body query source budget",
        format!(
            "{} nodes and {} elements; expected at most {} nodes and {} elements",
            nodes.len() / 52,
            elements.len() / 140,
            limits.maximum_nodes,
            limits.maximum_elements
        ),
    )
}
