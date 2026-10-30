mod allocation;
mod decode;
mod descriptor;
mod edit;
mod groups;
mod header;
mod ids;
mod names;
mod outcome;
mod parts;
mod ranges;
mod row;
mod types;
use super::{Result, invalid, require, wire};
use crate::archive_codec::ArchiveRevision;
pub use descriptor::*;
pub use edit::*;
pub use groups::*;
pub use ids::*;
pub use names::*;
pub use outcome::*;
pub use parts::*;
pub use ranges::*;
pub use row::*;
use std::sync::Arc;
pub use types::*;
use types::{EgrHeader, EgrLayout};

/// Immutable complete EGR image under an explicitly established conditional flat profile.
#[derive(Debug, Clone)]
pub struct SimulationEgrArchive {
    bytes: Arc<[u8]>,
    layout: Arc<EgrLayout>,
    revision: SimulationEgrRevision,
    profile: SimulationEgrProfile,
    limits: SimulationEgrLimits,
}

impl SimulationEgrArchive {
    /// Validate the complete selected flat profile, e.g. `SimulationEgrArchive::from_bytes(bytes, profile, limits)?`.
    pub fn from_bytes(
        bytes: &[u8],
        profile: SimulationEgrProfile,
        limits: SimulationEgrLimits,
    ) -> Result<Self> {
        let layout = decode::decode(bytes, limits)?;
        Ok(Self {
            bytes: bytes.into(),
            layout: Arc::new(layout),
            revision: SimulationEgrRevision(ArchiveRevision::default()),
            profile,
            limits,
        })
    }
    /// Complete original bytes, e.g. `table.to_bytes()`; tags, padding and deleted records survive.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Exact immutable image identity, e.g. `table.revision()`.
    pub fn revision(&self) -> &SimulationEgrRevision {
        &self.revision
    }
    /// Explicit caller-established transport, e.g. `table.profile()`.
    pub fn profile(&self) -> SimulationEgrProfile {
        self.profile
    }
    /// Validated limits retained for reconstruction and edits, e.g. `table.limits()`.
    pub fn limits(&self) -> SimulationEgrLimits {
        self.limits
    }
    /// Original nine signed words, including unassigned values, e.g. `table.header_words()`.
    pub fn header_words(&self) -> &[i32; 9] {
        &self.layout.header.words
    }
    /// Ordered loaded descriptors, without interpreting changed widths, e.g. `table.fields()`.
    pub fn fields(&self) -> &[SimulationEgrLoadedField] {
        &self.layout.fields
    }
    /// Complete logical-to-physical map, including zero holes, e.g. `table.directory()`.
    pub fn directory(&self) -> &[i32] {
        &self.layout.directory
    }
    /// Exact checked deleted-record order, e.g. `table.free_records()`; these rows have no live solver view.
    pub fn free_records(&self) -> &[SimulationEgrFreeRecord] {
        &self.layout.free_records
    }
    /// All physical record bytes in stored order, e.g. `table.allocated_record_bytes()`.
    pub fn allocated_record_bytes(&self) -> &[u8] {
        &self.bytes[self.layout.header.rows_start..]
    }
    /// Unknown header remainder, e.g. `table.header_padding()`.
    pub fn header_padding(&self) -> &[u8] {
        &self.bytes[36..128]
    }
    /// Unknown descriptor alignment bytes, e.g. `table.field_alignment()`.
    pub fn field_alignment(&self) -> &[u8] {
        &self.bytes[128 + self.layout.fields.len() * 8..self.layout.header.directory_start]
    }
    /// Unknown logical-directory alignment bytes, e.g. `table.directory_padding()`.
    pub fn directory_padding(&self) -> &[u8] {
        let header = &self.layout.header;
        &self.bytes[header.directory_start + header.capacity * 4..header.rows_start]
    }
}
