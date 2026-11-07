mod framing;
mod groups;
mod mesh;
mod outcome;
mod parts;
use super::{Result, SimulationMasterArchive, SimulationMasterRevision, invalid, require};
use crate::archive_codec::ArchiveRevision;
pub use groups::*;
pub use mesh::*;
pub use outcome::*;
pub use parts::*;
use std::sync::Arc;

/// Identity of one immutable complete GEN image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationGenRevision(ArchiveRevision);

/// Complete paired GEN framing; retained regions have no invented semantic roles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationGenLayout {
    /// All fourteen nonnegative prefix patterns.
    pub counts: [u32; 14],
    /// Selected auxiliary record width, following the native trial predicate.
    pub auxiliary_stride: usize,
    /// Start and length of each complete retained block in storage order.
    pub blocks: [(usize, usize); 10],
    /// Optional vector spans; absence differs from a present empty vector.
    pub vectors: Arc<[Option<(usize, usize)>]>,
}

/// Immutable complete GEN bytes validated with an actual complete MAS image.
#[derive(Debug, Clone)]
pub struct SimulationGenArchive {
    bytes: Arc<[u8]>,
    layout: SimulationGenLayout,
    master: SimulationMasterArchive,
    revision: SimulationGenRevision,
}

impl SimulationGenArchive {
    pub(super) fn auxiliary_trial_slice(&self, index: usize) -> Result<(&[u8], usize)> {
        let count = self.layout.counts[8] as usize;
        require(
            index < count,
            "GEN auxiliary trial",
            format!("logical row index {index}; expected less than {count}"),
        )?;
        let (start, end) = auxiliary_trial_bounds(self.layout.blocks[7].0, index)?;
        let bytes=self.bytes.get(start..end).ok_or_else(||invalid("GEN auxiliary trial",format!("288-byte slice at {start} in {} bytes; expected the complete original trial read",self.bytes.len())))?;
        Ok((bytes, start))
    }
    /// Validate complete paired framing, e.g. `SimulationGenArchive::from_bytes(gen, &master)?`.
    pub fn from_bytes(bytes: &[u8], master: &SimulationMasterArchive) -> Result<Self> {
        let layout = framing::decode(bytes, master.version())?;
        Ok(Self {
            bytes: bytes.into(),
            layout,
            master: master.clone(),
            revision: SimulationGenRevision(ArchiveRevision::default()),
        })
    }

    /// Exact complete stored image, e.g. `gen.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    /// Validated full-image boundaries, e.g. `gen.layout().auxiliary_stride`.
    pub fn layout(&self) -> &SimulationGenLayout {
        &self.layout
    }

    /// Actual paired MAS identity, e.g. `gen.master_revision()`.
    pub fn master_revision(&self) -> &SimulationMasterRevision {
        self.master.revision()
    }

    /// Immutable GEN identity, e.g. `gen.revision()`.
    pub fn revision(&self) -> &SimulationGenRevision {
        &self.revision
    }

    /// Read one complete retained block, e.g. `gen.block(5)?` for node records.
    pub fn block(&self, index: usize) -> Result<&[u8]> {
        let (start, length) = self
            .layout
            .blocks
            .get(index)
            .copied()
            .ok_or_else(|| invalid("GEN block", format!("index {index}; expected 0..10")))?;
        Ok(&self.bytes[start..start + length])
    }

    /// Read an optional auxiliary vector, e.g. `gen.vector(0)?`.
    pub fn vector(&self, index: usize) -> Result<Option<&[u8]>> {
        let span = self.layout.vectors.get(index).ok_or_else(|| {
            invalid(
                "GEN vector",
                format!(
                    "index {index}; expected less than {}",
                    self.layout.vectors.len()
                ),
            )
        })?;
        Ok(span.map(|(start, length)| &self.bytes[start..start + length]))
    }

    /// Replace one node's coordinate bit patterns on an isolated image, e.g. `gen.edit_node_coordinates(&id, bits)?`.
    pub fn edit_node_coordinates(
        &self,
        id: &SimulationNodeId,
        bits: [u32; 3],
    ) -> Result<SimulationGenEdit> {
        let archive = self.replaced_node_coordinates(id, bits)?;
        Ok(SimulationGenEdit {
            completion: if archive.revision == self.revision {
                crate::display_lists::DisplayEditCompletion::Unchanged
            } else {
                crate::display_lists::DisplayEditCompletion::Changed
            },
            parent_revision: self.revision.clone(),
            parent: id.clone(),
            result: archive.node_id(id.index)?,
            coverage: archive.coverage(),
            coordinate_pattern_error_bits: 0,
            diagnostics: super::study::diagnostics(),
            archive,
        })
    }

    fn replaced_node_coordinates(&self, id: &SimulationNodeId, bits: [u32; 3]) -> Result<Self> {
        require(
            id.revision == self.revision,
            "GEN node",
            "foreign revision; expected this complete GEN image",
        )?;
        let node = self.node(id)?;
        if node.coordinate_patterns == bits {
            return Ok(self.clone());
        }
        let mut bytes = self.to_bytes();
        let start = self.layout.blocks[5].0 + id.index * 52 + 32;
        for (word, pattern) in bytes[start..start + 12]
            .as_chunks_mut::<4>()
            .0
            .iter_mut()
            .zip(bits)
        {
            word.copy_from_slice(&pattern.to_le_bytes());
        }
        Self::from_bytes(&bytes, &self.master)
    }
}

fn auxiliary_trial_bounds(base: usize, index: usize) -> Result<(usize, usize)> {
    index.checked_mul(288).and_then(|delta|base.checked_add(delta))
        .and_then(|start|start.checked_add(288).map(|end|(start,end)))
        .ok_or_else(||invalid("GEN auxiliary trial offsets",format!("base {base}, row index {index}; expected addressable complete 288-byte trial offsets")))
}
