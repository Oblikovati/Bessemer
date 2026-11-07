use super::{Result, SimulationEgrArchive, SimulationEgrRevision, invalid, require};

/// One physical stored record, bound to its complete source revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationEgrPhysicalId {
    pub(super) revision: SimulationEgrRevision,
    pub(super) index: usize,
}

/// One established live logical mapping, distinct from a hole or deleted record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationEgrRowId {
    pub(super) revision: SimulationEgrRevision,
    pub(super) logical_index: usize,
    pub(super) physical_index: usize,
}

impl SimulationEgrRowId {
    /// Original one-based logical ordinal, e.g. `id.logical_ordinal()`.
    pub fn logical_ordinal(&self) -> u32 {
        self.logical_index as u32 + 1
    }
    /// Resolved one-based physical record number, e.g. `id.record_number()`.
    pub fn record_number(&self) -> u32 {
        self.physical_index as u32 + 1
    }
}

impl SimulationEgrArchive {
    /// Normalize a one-based stored record number, e.g. `table.physical_id(1)?`; includes deleted records.
    pub fn physical_id(&self, number: u32) -> Result<SimulationEgrPhysicalId> {
        require(
            number > 0 && number as usize <= self.layout.header.allocated,
            "EGR physical record",
            format!("{number}; expected 1..={}", self.layout.header.allocated),
        )?;
        Ok(SimulationEgrPhysicalId {
            revision: self.revision.clone(),
            index: number as usize - 1,
        })
    }
    /// Exact complete stored bytes, e.g. `table.physical_record(&id)?`; no live-row semantics are assigned.
    pub fn physical_record(&self, id: &SimulationEgrPhysicalId) -> Result<&[u8; 128]> {
        require(
            id.revision == self.revision,
            "EGR physical identity",
            format!(
                "foreign revision at physical index {}; expected this complete EGR revision",
                id.index
            ),
        )?;
        self.allocated_record_bytes()
            .as_chunks::<128>()
            .0
            .get(id.index)
            .ok_or_else(|| {
                invalid(
                    "EGR physical identity",
                    format!(
                        "index {}; expected less than {}",
                        id.index, self.layout.header.allocated
                    ),
                )
            })
    }
    /// Resolve one-based logical position once, e.g. `table.live_row_id(1)?`; a checked hole returns None.
    pub fn live_row_id(&self, ordinal: u32) -> Result<Option<SimulationEgrRowId>> {
        require(
            ordinal > 0 && ordinal as usize <= self.layout.header.capacity,
            "EGR logical ordinal",
            format!("{ordinal}; expected 1..={}", self.layout.header.capacity),
        )?;
        let logical_index = ordinal as usize - 1;
        let number = self.layout.directory[logical_index];
        if number == 0 {
            return Ok(None);
        }
        Ok(Some(SimulationEgrRowId {
            revision: self.revision.clone(),
            logical_index,
            physical_index: number as usize - 1,
        }))
    }
    /// Complete existing live row, e.g. `table.live_row(&id)?`; foreign identities cannot alias equal bytes.
    pub fn live_row(&self, id: &SimulationEgrRowId) -> Result<&[u8; 128]> {
        self.require_live_id(id)?;
        self.physical_record(&SimulationEgrPhysicalId {
            revision: id.revision.clone(),
            index: id.physical_index,
        })
    }
    pub(super) fn require_live_id(&self, id: &SimulationEgrRowId) -> Result<()> {
        let mapped = self.layout.directory.get(id.logical_index).copied();
        require(
            id.revision == self.revision && mapped == Some(id.record_number() as i32),
            "EGR live identity",
            format!(
                "logical index {}, physical index {}, mapping {mapped:?}; expected this revision and its established live mapping",
                id.logical_index, id.physical_index
            ),
        )
    }
}
