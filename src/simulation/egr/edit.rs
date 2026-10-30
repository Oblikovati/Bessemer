use super::{
    Result, SimulationEgrArchive, SimulationEgrCheckMode, SimulationEgrCoverage,
    SimulationEgrFieldRange, SimulationEgrPhysicalId, SimulationEgrRangeId, require,
};
use crate::display_lists::DisplayEditCompletion;
use std::sync::Arc;

/// Complete physical-record relationship in one isolated existing-field edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationEgrRecordRelation {
    /// Complete stored record survived byte for byte, including deleted rows.
    Copied,
    /// This stored row derives from the source row and explicit replacement bytes.
    Derived,
}

/// Revision-bound parent/result relation for one allocated physical record.
#[derive(Debug, Clone)]
pub struct SimulationEgrRecordHistory {
    /// Original complete stored record.
    pub parent: SimulationEgrPhysicalId,
    /// Resulting complete stored record.
    pub result: SimulationEgrPhysicalId,
    /// Copied or explicitly derived from both input parents.
    pub relation: SimulationEgrRecordRelation,
}

/// Complete non-destructive field edit, without a native save or allocation claim.
#[derive(Debug, Clone)]
pub struct SimulationEgrFieldEdit {
    /// Original immutable table.
    pub source: SimulationEgrArchive,
    /// Validated complete result, retaining all unrelated bytes.
    pub archive: SimulationEgrArchive,
    /// Original exact range and its complete source context.
    pub parent: SimulationEgrFieldRange,
    /// Normalized resulting range.
    pub result: SimulationEgrFieldRange,
    /// Complete supplied replacement parent.
    pub replacement: Arc<[u8]>,
    /// Changed versus authentic byte no-op.
    pub completion: DisplayEditCompletion,
    /// Complete framing and unchanged-region coverage.
    pub coverage: SimulationEgrCoverage,
    /// All allocated records in total stored order, including copied deleted rows.
    pub history: Arc<[SimulationEgrRecordHistory]>,
    /// Achieved requested byte-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

impl SimulationEgrArchive {
    /// Replace exactly one existing loaded-width field span, e.g. `table.edit_field_range(&id, replacement)?`.
    pub fn edit_field_range(
        &self,
        id: &SimulationEgrRangeId,
        replacement: &[u8],
    ) -> Result<SimulationEgrFieldEdit> {
        let parent = self.field_range(id)?;
        validate_replacement(id, replacement)?;
        let archive = self.replaced_range(id, replacement)?;
        let result_id = archive.field_range_id(
            parent.logical_ordinal,
            parent.first_field,
            parent.last_field,
        )?;
        let result = archive.field_range(&result_id)?;
        let history = edit_history(self, &archive, id.row.physical_index)?;
        Ok(edit_result(
            self,
            archive,
            parent,
            result,
            replacement,
            history,
        ))
    }
    fn replaced_range(&self, id: &SimulationEgrRangeId, replacement: &[u8]) -> Result<Self> {
        let start = self.layout.header.rows_start + id.row.physical_index * 128 + id.start;
        let end = start + replacement.len();
        if self.bytes[start..end] == *replacement {
            return Ok(self.clone());
        }
        let mut bytes = self.to_bytes();
        bytes[start..end].copy_from_slice(replacement);
        let archive = Self::from_bytes(&bytes, self.profile, self.limits)?;
        require(
            self.bytes[..start] == archive.bytes[..start]
                && self.bytes[end..] == archive.bytes[end..],
            "EGR edit preservation",
            format!(
                "replacement {start}..{end}; expected every original byte outside that span unchanged"
            ),
        )?;
        Ok(archive)
    }
}

fn edit_history(
    source: &SimulationEgrArchive,
    result: &SimulationEgrArchive,
    changed_index: usize,
) -> Result<Vec<SimulationEgrRecordHistory>> {
    let changed = source.revision != result.revision;
    (0..source.layout.header.allocated)
        .map(|index| {
            Ok(SimulationEgrRecordHistory {
                parent: source.physical_id(index as u32 + 1)?,
                result: result.physical_id(index as u32 + 1)?,
                relation: if changed && index == changed_index {
                    SimulationEgrRecordRelation::Derived
                } else {
                    SimulationEgrRecordRelation::Copied
                },
            })
        })
        .collect()
}

fn edit_result(
    source: &SimulationEgrArchive,
    archive: SimulationEgrArchive,
    parent: SimulationEgrFieldRange,
    result: SimulationEgrFieldRange,
    replacement: &[u8],
    history: Vec<SimulationEgrRecordHistory>,
) -> SimulationEgrFieldEdit {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationEgrCheckMode::ExactLiveFieldEditAndPreservedOtherBytes;
    SimulationEgrFieldEdit {
        completion: if source.revision == archive.revision {
            DisplayEditCompletion::Unchanged
        } else {
            DisplayEditCompletion::Changed
        },
        source: source.clone(),
        archive,
        parent,
        result,
        replacement: replacement.into(),
        coverage,
        history: history.into(),
        requested_pattern_error_bits: 0,
    }
}

fn validate_replacement(id: &SimulationEgrRangeId, replacement: &[u8]) -> Result<()> {
    require(
        replacement.len() == id.end - id.start,
        "EGR replacement width",
        format!(
            "{} bytes; expected exactly {} for fields {}..={}",
            replacement.len(),
            id.end - id.start,
            id.first,
            id.last
        ),
    )
}
