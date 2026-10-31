use super::*;
use crate::display_lists::DisplayEditCompletion;

/// Exact bounded stored-name view, without MFC trimming or CAD provenance claims.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryNameView {
    /// Signed stored vector count, without allocation or geometry validation.
    pub vector_count: i32,
    /// Unassigned prefix between the first two words and the bounds.
    pub opaque_prefix: Arc<[u8]>,
    /// Full 256-byte image, including all historical residual bytes.
    pub name_buffer: [u8; 256],
    /// Every raw UTF-16 unit, without replacement or normalization.
    pub stored_units: [u16; 128],
    /// Prefix length under the native 127-unit scan bound.
    pub prefix_units: usize,
    /// NUL encountered inside the scan; unit 128 is not scanned.
    pub source_nul_index: Option<usize>,
    /// Two separately retained, unassigned trailing words.
    pub trailing_words: [u32; 2],
    /// Always false: native TrimRight policy is unassigned.
    pub native_trim_right_applied: bool,
}

impl SimulationAuxiliaryNameView {
    /// Surrogate-preserving bounded prefix units, e.g. `view.bounded_units()`; not a trimmed native returned name.
    pub fn bounded_units(&self) -> &[u16] {
        &self.stored_units[..self.prefix_units.min(127)]
    }
    /// All unconsumed name bytes including any encountered NUL, e.g. `view.unconsumed_name_bytes()`.
    pub fn unconsumed_name_bytes(&self) -> &[u8] {
        &self.name_buffer[self.prefix_units.min(127) * 2..]
    }
}

/// Exact copy/terminator scope against a supplied complete destination image.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxiliaryNameCoverage {
    /// Exact bounded copy and residual-byte scope.
    pub mode: SimulationAuxiliaryNameCheckMode,
    /// Complete retained stored row extent.
    pub row_bytes: usize,
    /// Name-buffer start in this stored profile.
    pub name_offset: usize,
    /// Actual input units copied before one terminator.
    pub copied_units: usize,
    /// Written prefix plus terminator bytes; all other bytes survive.
    pub written_bytes: usize,
    /// Always false: temporary-buffer provenance and in-place file production are unproven.
    pub native_in_place_row_update_proven: bool,
}

/// Proposed stored image, distinct from native CString/storage execution.
#[derive(Debug, Clone)]
pub struct SimulationAuxiliaryNameStore {
    /// Complete preservation outside copied units and the terminator.
    pub fidelity: SimulationAuxiliaryNameFidelity,
    /// Complete resulting row with destination residual bytes retained.
    pub row: SimulationAuxiliaryBodyRow,
    /// Authentic byte-change completion and no-op identity reuse.
    pub completion: DisplayEditCompletion,
    /// Complete supplied destination parent.
    pub parent: SimulationAuxiliaryBodyRevision,
    /// Complete result identity.
    pub result: SimulationAuxiliaryBodyRevision,
    /// Exact written-region and production scope.
    pub coverage: SimulationAuxiliaryNameCoverage,
    /// Always false: the bounded store never clears its residual tail.
    pub tail_cleared: bool,
    /// Always false: no MFC trim was executed or substituted.
    pub native_trim_right_applied: bool,
    /// Exact requested code-unit/terminator pattern error, zero.
    pub requested_pattern_error_bits: u32,
    /// String/provenance responsibilities left unassigned.
    pub diagnostics: [SimulationAuxiliaryNameDiagnostic; 3],
}

impl SimulationAuxiliaryBodyRow {
    /// Read complete raw name fields, e.g. `row.name_view()`; unit 128 and residual bytes stay available.
    pub fn name_view(&self) -> SimulationAuxiliaryNameView {
        let start = self.profile.bounds_offset() + 8;
        let buffer = std::array::from_fn(|i| self.bytes[start + i]);
        let units = std::array::from_fn(|i| u16::from_le_bytes([buffer[i * 2], buffer[i * 2 + 1]]));
        let prefix_units = units
            .iter()
            .take(127)
            .position(|unit| *unit == 0)
            .unwrap_or(127);
        SimulationAuxiliaryNameView {
            vector_count: read_pattern(&self.bytes, 4) as i32,
            opaque_prefix: self.bytes[8..start - 8].into(),
            name_buffer: buffer,
            stored_units: units,
            prefix_units,
            source_nul_index: (prefix_units < 127).then_some(prefix_units),
            trailing_words: trailing_words(self, start),
            native_trim_right_applied: false,
        }
    }

    /// Copy into these supplied destination bytes, e.g. `row.project_name_store(&[65, 0xd800])?`; no tail clearing or native file producer is implied.
    pub fn project_name_store(&self, units: &[u16]) -> Result<SimulationAuxiliaryNameStore> {
        let copied = units
            .iter()
            .take(127)
            .position(|unit| *unit == 0)
            .unwrap_or(units.len().min(127));
        let start = self.profile.bounds_offset() + 8;
        let mut bytes = self.to_bytes();
        for (index, unit) in units[..copied].iter().enumerate() {
            bytes[start + index * 2..start + index * 2 + 2].copy_from_slice(&unit.to_le_bytes());
        }
        bytes[start + copied * 2..start + copied * 2 + 2].fill(0);
        let row = if bytes.as_slice() == self.bytes.as_ref() {
            self.clone()
        } else {
            edit::derived_row(self, &bytes)?
        };
        Ok(name_store_result(self, row, copied, start))
    }
}

fn name_store_result(
    parent: &SimulationAuxiliaryBodyRow,
    row: SimulationAuxiliaryBodyRow,
    copied: usize,
    start: usize,
) -> SimulationAuxiliaryNameStore {
    SimulationAuxiliaryNameStore {
        fidelity: SimulationAuxiliaryNameFidelity::ExactOutsideCopiedPrefixAndTerminator,
        completion: if parent.revision == row.revision {
            DisplayEditCompletion::Unchanged
        } else {
            DisplayEditCompletion::Changed
        },
        parent: parent.revision.clone(),
        result: row.revision.clone(),
        coverage: name_coverage(row.bytes.len(), start, copied),
        row,
        tail_cleared: false,
        native_trim_right_applied: false,
        requested_pattern_error_bits: 0,
        diagnostics: name_diagnostics(),
    }
}

/// Exact stored-name scope, not a native returned-string claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryNameCheckMode {
    /// Every copied unit/terminator and retained destination byte.
    ExactBoundedCopyAndResidualPreservation,
}

/// Exact stored-image preservation fidelity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryNameFidelity {
    /// Every byte beyond copied units and terminator survives.
    ExactOutsideCopiedPrefixAndTerminator,
}

/// Responsibilities not inferred from raw text or supplied destination bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxiliaryNameDiagnostic {
    /// Native trimming is not executed or replaced with Unicode whitespace rules.
    MfcTrimPolicyUnassigned,
    /// Native row selection and CAD name-array provenance remain outside this view.
    CadNameProvenanceUnassigned,
    /// Supplied bytes model a destination, not observed historical stack contents.
    NativeTemporaryBufferNotObserved,
}

fn name_diagnostics() -> [SimulationAuxiliaryNameDiagnostic; 3] {
    [
        SimulationAuxiliaryNameDiagnostic::MfcTrimPolicyUnassigned,
        SimulationAuxiliaryNameDiagnostic::CadNameProvenanceUnassigned,
        SimulationAuxiliaryNameDiagnostic::NativeTemporaryBufferNotObserved,
    ]
}

fn trailing_words(row: &SimulationAuxiliaryBodyRow, start: usize) -> [u32; 2] {
    [
        read_pattern(&row.bytes, start + 256),
        read_pattern(&row.bytes, start + 260),
    ]
}

fn name_coverage(
    row_bytes: usize,
    name_offset: usize,
    copied_units: usize,
) -> SimulationAuxiliaryNameCoverage {
    SimulationAuxiliaryNameCoverage {
        mode: SimulationAuxiliaryNameCheckMode::ExactBoundedCopyAndResidualPreservation,
        row_bytes,
        name_offset,
        copied_units,
        written_bytes: (copied_units + 1) * 2,
        native_in_place_row_update_proven: false,
    }
}
