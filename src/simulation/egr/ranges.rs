use super::{Result, SimulationEgrArchive, SimulationEgrCheckMode, SimulationEgrRowId, require};
use crate::{BessemerError, FailureClass};
use std::sync::Arc;

/// Normalized existing-live-row inclusive field range; raw ordinal inputs are not reused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationEgrRangeId {
    pub(super) row: SimulationEgrRowId,
    pub(super) first: u32,
    pub(super) last: u32,
    pub(super) start: usize,
    pub(super) end: usize,
}

/// Complete existing live-field projection with exact loaded-width and source context retained.
#[derive(Debug, Clone)]
pub struct SimulationEgrFieldRange {
    /// Immutable complete source image.
    pub source: SimulationEgrArchive,
    /// Normalized source identity and loaded field bounds.
    pub id: SimulationEgrRangeId,
    /// One-based logical ordinal.
    pub logical_ordinal: u32,
    /// Established one-based physical record number.
    pub record_number: u32,
    /// Inclusive one-based first field.
    pub first_field: u32,
    /// Inclusive one-based last field.
    pub last_field: u32,
    /// Start relative to the complete physical row.
    pub row_offset: usize,
    /// Exclusive end relative to the row.
    pub row_end: usize,
    /// Exact selected raw bytes; no value-type interpretation is added.
    pub bytes: Arc<[u8]>,
    /// Exact existing-live-field check mode.
    pub check_mode: SimulationEgrCheckMode,
    /// Always false: missing-row/cache returns are not invented.
    pub native_cache_state_projected: bool,
}

impl SimulationEgrArchive {
    /// Bind a live row and inclusive loaded field range, e.g. `table.field_range_id(1, 1, 2)?`; holes are Missing errors.
    pub fn field_range_id(
        &self,
        ordinal: u32,
        first: u32,
        last: u32,
    ) -> Result<SimulationEgrRangeId> {
        let row = self.live_row_id(ordinal)?;
        require(
            first > 0 && first <= last && last as usize <= self.layout.fields.len(),
            "EGR inclusive field range",
            format!(
                "{first}..={last}; expected 1 <= first <= last <= {}",
                self.layout.fields.len()
            ),
        )?;
        let row = row.ok_or_else(|| BessemerError::new("EGR field range", "logical row", FailureClass::Missing,
            format!("ordinal {ordinal} is a directory hole; expected an established live row, without a fabricated native cache result")))?;
        Ok(SimulationEgrRangeId {
            row,
            first,
            last,
            start: self.layout.fields[first as usize - 1].row_offset,
            end: self.layout.fields[last as usize - 1].row_end,
        })
    }
    /// Read exact loaded-width bytes, e.g. `table.field_range(&id)?`; no native missing-row result is projected.
    pub fn field_range(&self, id: &SimulationEgrRangeId) -> Result<SimulationEgrFieldRange> {
        let row = self.live_row(&id.row)?;
        Ok(SimulationEgrFieldRange {
            source: self.clone(),
            id: id.clone(),
            logical_ordinal: id.row.logical_ordinal(),
            record_number: id.row.record_number(),
            first_field: id.first,
            last_field: id.last,
            row_offset: id.start,
            row_end: id.end,
            bytes: row[id.start..id.end].into(),
            check_mode: SimulationEgrCheckMode::ExactExistingLiveFieldRange,
            native_cache_state_projected: false,
        })
    }
}
