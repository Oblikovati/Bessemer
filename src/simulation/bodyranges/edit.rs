use super::*;
use crate::display_lists::DisplayEditCompletion;

/// Exact standalone-row edit fidelity, without whole GEN/vector framing publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryBodyFidelity {
    /// Every byte outside identifier/L/H is retained exactly.
    ExactOutsideIdentifierAndBounds,
}

/// Isolated complete-row replacement with exact preservation outside three interpreted fields.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryBodyEdit {
    /// Complete preservation scope outside requested fields.
    pub fidelity: SimulationAuxiliaryBodyFidelity,
    /// Complete resulting immutable row; no GEN framing/publication is implied.
    pub row: SimulationAuxiliaryBodyRow,
    /// Authentic byte-change completion, retaining identity for a no-op.
    pub completion: DisplayEditCompletion,
    /// Original complete source parent.
    pub parent: SimulationAuxiliaryBodyRevision,
    /// Complete resulting row identity.
    pub result: SimulationAuxiliaryBodyRevision,
    /// Exact source/semantic scope; every other row byte is preserved.
    pub coverage: SimulationAuxiliaryBodyCoverage,
    /// Achieved requested field-pattern error, always zero.
    pub requested_pattern_error_bits: u32,
    /// Unassigned descriptor/runtime/ownership and geometry responsibilities.
    pub diagnostics: [SimulationAuxiliaryBodyDiagnostic; 4],
}

impl SimulationAuxiliaryBodyRow {
    /// Replace only identifier/L/H bytes on a copy, e.g. `row.edit_fields(body, 2, 5)?`; no automatic GEN/vector rewrite occurs.
    pub fn edit_fields(
        &self,
        body: SimulationBodySelectionId,
        low: i32,
        high: i32,
    ) -> Result<SimulationAuxiliaryBodyEdit> {
        let mut bytes = self.to_bytes();
        let offset = self.profile.bounds_offset();
        bytes[..4].copy_from_slice(&body.pattern().to_le_bytes());
        bytes[offset..offset + 4].copy_from_slice(&low.to_le_bytes());
        bytes[offset + 4..offset + 8].copy_from_slice(&high.to_le_bytes());
        let row = if bytes.as_slice() == self.bytes.as_ref() {
            self.clone()
        } else {
            derived_row(self, &bytes)?
        };
        Ok(edit_result(self, row))
    }
}

fn edit_result(
    parent: &SimulationAuxiliaryBodyRow,
    row: SimulationAuxiliaryBodyRow,
) -> SimulationAuxiliaryBodyEdit {
    SimulationAuxiliaryBodyEdit {
        fidelity: SimulationAuxiliaryBodyFidelity::ExactOutsideIdentifierAndBounds,
        completion: if row.revision == parent.revision {
            DisplayEditCompletion::Unchanged
        } else {
            DisplayEditCompletion::Changed
        },
        parent: parent.revision.clone(),
        result: row.revision.clone(),
        coverage: row.field_coverage(),
        row,
        requested_pattern_error_bits: 0,
        diagnostics: proposal::body_diagnostics(),
    }
}

pub(super) fn derived_row(
    parent: &SimulationAuxiliaryBodyRow,
    bytes: &[u8],
) -> Result<SimulationAuxiliaryBodyRow> {
    let mut row = SimulationAuxiliaryBodyRow::from_bytes(bytes, parent.profile)?;
    row.gen_parent = parent.gen_parent.clone();
    Ok(row)
}
