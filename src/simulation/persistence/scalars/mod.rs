//! Explicit bounded scalar profiles with capacity retained solely as metadata.
mod codec;
mod construction;
mod directory;
mod edit;
mod types;
use super::{Result, invalid, require};
use crate::archive_codec::ArchiveRevision;
pub use edit::*;
use std::sync::Arc;
pub use types::*;

/// Complete immutable counted i32 array or UniqueIds i64 table with explicit resource context.
#[derive(Debug, Clone)]
pub struct SimulationPersistScalarArchive {
    pub(super) bytes: Arc<[u8]>,
    pub(super) fields: Arc<[SimulationPersistScalarField]>,
    pub(super) profile: SimulationPersistScalarProfile,
    layout: codec::ScalarLayout,
    pub(super) revision: SimulationPersistScalarRevision,
    pub(super) maximum_entries: usize,
}

impl SimulationPersistScalarArchive {
    /// Validate one explicit complete profile, e.g. `SimulationPersistScalarArchive::from_bytes(bytes, SimulationPersistScalarProfile::CountedI32, 10000)?`.
    pub fn from_bytes(
        bytes: &[u8],
        profile: SimulationPersistScalarProfile,
        maximum_entries: usize,
    ) -> Result<Self> {
        let layout = codec::decode(bytes, profile, maximum_entries)?;
        let revision = SimulationPersistScalarRevision(ArchiveRevision::default());
        let fields = directory::fields(bytes, layout, &revision)?;
        Ok(Self {
            bytes: bytes.into(),
            fields,
            profile,
            layout,
            revision,
            maximum_entries,
        })
    }
    /// Exact complete bare-stream image, e.g. `archive.to_bytes()`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Complete original four/eight-byte header, e.g. `archive.header()`.
    pub fn header(&self) -> &[u8] {
        &self.bytes[..self.layout.header]
    }
    /// Exactly used-count payload bytes, e.g. `archive.payload()`.
    pub fn payload(&self) -> &[u8] {
        &self.bytes[self.layout.header..]
    }
    /// Original explicitly selected grammar, e.g. `archive.profile()`.
    pub fn profile(&self) -> SimulationPersistScalarProfile {
        self.profile
    }
    /// Normalized declared number of entries, e.g. `archive.used_count()`.
    pub fn used_count(&self) -> usize {
        self.layout.used
    }
    /// Original allocation metadata, absent for CountedI32, e.g. `archive.allocation_count()`.
    pub fn allocation_count(&self) -> Option<i32> {
        self.layout.capacity
    }
    /// Immutable image/profile identity, e.g. `archive.revision()`.
    pub fn revision(&self) -> &SimulationPersistScalarRevision {
        &self.revision
    }
    /// All header and entry fields in physical order, e.g. `archive.fields()`.
    pub fn fields(&self) -> &[SimulationPersistScalarField] {
        &self.fields
    }
    /// Normalize an actual zero-based payload entry, e.g. `archive.entry_id(0)?`.
    pub fn entry_id(&self, index: usize) -> Result<SimulationPersistScalarId> {
        require(
            index < self.layout.used,
            "scalar entry ordinal",
            format!("{index}; expected below {}", self.layout.used),
        )?;
        Ok(self.fields[self.layout.header / 4 + index].id.clone())
    }
    /// Resolve a field on its original image/profile, e.g. `archive.field(&id)?`.
    pub fn field(&self, id: &SimulationPersistScalarId) -> Result<&SimulationPersistScalarField> {
        require(
            id.revision == self.revision,
            "scalar field revision",
            format!(
                "foreign revision at ordinal {}; expected this complete image/profile revision",
                id.ordinal
            ),
        )?;
        self.fields.get(id.ordinal).ok_or_else(|| {
            invalid(
                "scalar field ordinal",
                format!("{}; expected below {}", id.ordinal, self.fields.len()),
            )
        })
    }
    /// Complete exact scope and exclusions, e.g. `archive.coverage()`.
    pub fn coverage(&self) -> SimulationPersistScalarCoverage {
        directory::coverage(self)
    }
}
