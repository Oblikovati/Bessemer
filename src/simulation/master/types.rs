use super::SimulationMasterArchive;
use crate::archive_codec::ArchiveRevision;
use std::sync::Arc;

/// All stored bytes of one physical 384-byte record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationMasterRow {
    /// Eighty-one uninterpreted four-byte field patterns.
    pub fields: [u32; 81],
    /// Sixty bytes without an assigned field role.
    pub padding: [u8; 60],
}

/// Complete observed 81-field/70-row MAS profile, with no dropped padding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationMasterParts {
    /// Unassigned bytes after the nine header words.
    pub header_padding: Arc<[u8]>,
    /// First word of each field-table pair; widths are the validated constant four.
    pub field_tags: [i32; 81],
    /// Unassigned field-table record tail.
    pub field_table_padding: Arc<[u8]>,
    /// One-based physical record numbers selected by logical row order.
    pub directory: [u8; 70],
    /// Unassigned row-directory record tail.
    pub directory_padding: Arc<[u8]>,
    /// Exactly seventy physical records in storage order.
    pub physical_rows: Arc<[SimulationMasterRow]>,
}

/// Immutable identity of one complete MAS snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationMasterRevision(pub(super) ArchiveRevision);

/// One logical cell, normalized once and bound to its MAS revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationMasterCellId {
    pub(super) revision: SimulationMasterRevision,
    pub(super) row: usize,
    pub(super) field: usize,
}

impl SimulationMasterCellId {
    /// One-based logical row, e.g. `id.logical_row()` returns 21 for the version row.
    pub fn logical_row(&self) -> usize {
        self.row + 1
    }

    /// One-based field, e.g. `id.field_number()` returns 3 for the version field.
    pub fn field_number(&self) -> usize {
        self.field + 1
    }

    /// Snapshot identity required by the cell, e.g. `id.revision()`.
    pub fn revision(&self) -> &SimulationMasterRevision {
        &self.revision
    }
}

/// Exact fixed-image framing; field semantics remain separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationMasterCoverage {
    /// Logical and physical rows validated in full.
    pub rows: usize,
    /// All four-byte fields retained.
    pub fields: usize,
    /// Fixed unassigned padding bytes retained.
    pub opaque_padding_bytes: usize,
    /// Complete typed reconstruction matched all 28,420 input bytes.
    pub reconstruction_checked: bool,
}

/// Complete immutable cell replacement with context identities.
#[derive(Debug, Clone)]
pub struct SimulationMasterEdit {
    /// Resulting complete MAS image.
    pub archive: SimulationMasterArchive,
    /// Changed versus byte-identical cell replacement.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Original logical cell identity.
    pub parent: SimulationMasterCellId,
    /// Corresponding result identity.
    pub result: SimulationMasterCellId,
    /// Complete fixed-image coverage.
    pub coverage: SimulationMasterCoverage,
    /// Exact field-pattern error, zero; no semantic quantity error was established.
    pub field_pattern_error_bits: u32,
    /// Required unchecked semantic responsibilities.
    pub diagnostics: [crate::simulation::SimulationDiagnostic; 3],
}
