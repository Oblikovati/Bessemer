use super::{SimulationOutArchive, SimulationOutFieldId, SimulationOutRevision};
use std::sync::Arc;

/// Exact report check scope, separate from any solver or geometry acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationOutCheckMode {
    /// Every wrapped byte, line boundary and recognized occurrence is checked through exact EOF.
    ExactTransferAndReportSyntax,
}

/// Quantified coverage of complete stored syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationOutCoverage {
    /// Declared exact check mode.
    pub mode: SimulationOutCheckMode,
    /// Complete stream bytes, including transfer word.
    pub bytes: usize,
    /// Every retained line, including empty and unknown lines.
    pub lines: usize,
    /// Ordered numeric occurrences; symbols are not unique keys.
    pub fields: usize,
    /// Associated printed legend occurrences.
    pub legends: usize,
    /// Every embedded body NUL retained without repair.
    pub embedded_nuls: usize,
}

/// Required checks beyond exact stored-report syntax.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationOutDiagnostic {
    /// Unknown lines and printed controls do not establish complete solver semantics.
    ReportAndSolverSemanticsNotChecked,
    /// A report-local unit declaration does not establish GEN coordinates or geometry acceptance.
    GeometryAndCoordinateUnitsNotChecked,
    /// The native application did not load the edited report.
    NativeApplicationNotChecked,
}

/// Exact byte preservation achieved by a numeric-token replacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationOutEditFidelity {
    /// Only the selected token and derived transfer word may change; all other body bytes are exact.
    ExactBodyExceptSelectedTokenAndDerivedTransferWord,
}

/// Derivation of one ordered field occurrence with its original parent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationOutFieldHistory {
    /// Original occurrence identity.
    pub parent: SimulationOutFieldId,
    /// Derived occurrence identity, whether or not its token changed.
    pub result: SimulationOutFieldId,
    /// Whether the stored numeric token changed for this occurrence.
    pub replaced_token: bool,
}

/// Complete isolated numeric-token replacement and its exact acceptance scope.
#[derive(Debug, Clone)]
pub struct SimulationOutEdit {
    /// Immutable complete report result.
    pub archive: SimulationOutArchive,
    /// Whether stored token bytes changed.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Exact stored-byte preservation scope.
    pub fidelity: SimulationOutEditFidelity,
    /// Original report identity.
    pub parent_revision: SimulationOutRevision,
    /// All ordered field derivations, with every parent retained.
    pub history: Arc<[SimulationOutFieldHistory]>,
    /// Exact complete stored syntax coverage.
    pub coverage: SimulationOutCoverage,
    /// Achieved requested token error in bytes, zero; no physical error was established.
    pub requested_token_error_bytes: usize,
    /// Required unchecked semantic responsibilities.
    pub diagnostics: [SimulationOutDiagnostic; 3],
}

impl SimulationOutArchive {
    /// Quantify exact stored syntax, e.g. `report.coverage().legends`.
    pub fn coverage(&self) -> SimulationOutCoverage {
        SimulationOutCoverage {
            mode: SimulationOutCheckMode::ExactTransferAndReportSyntax,
            bytes: self.bytes.len(),
            lines: self.lines.len(),
            fields: self.fields.len(),
            legends: self.fields.iter().map(|field| field.legends.len()).sum(),
            embedded_nuls: self.lines.iter().map(|line| line.nul_positions.len()).sum(),
        }
    }

    /// Explicit semantic exclusions, e.g. `report.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationOutDiagnostic; 3] {
        [
            SimulationOutDiagnostic::ReportAndSolverSemanticsNotChecked,
            SimulationOutDiagnostic::GeometryAndCoordinateUnitsNotChecked,
            SimulationOutDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

pub(super) fn edit_result(
    parent: &SimulationOutArchive,
    archive: SimulationOutArchive,
) -> SimulationOutEdit {
    SimulationOutEdit {
        completion: completion(parent.revision != archive.revision),
        fidelity: SimulationOutEditFidelity::ExactBodyExceptSelectedTokenAndDerivedTransferWord,
        history: field_history(parent, &archive),
        parent_revision: parent.revision.clone(),
        coverage: archive.coverage(),
        diagnostics: archive.diagnostics(),
        requested_token_error_bytes: 0,
        archive,
    }
}

fn completion(changed: bool) -> crate::display_lists::DisplayEditCompletion {
    if changed {
        return crate::display_lists::DisplayEditCompletion::Changed;
    }
    crate::display_lists::DisplayEditCompletion::Unchanged
}

fn field_history(
    parent: &SimulationOutArchive,
    result: &SimulationOutArchive,
) -> Arc<[SimulationOutFieldHistory]> {
    parent
        .fields
        .iter()
        .zip(result.fields.iter())
        .map(|(parent, result)| SimulationOutFieldHistory {
            parent: parent.id.clone(),
            result: result.id.clone(),
            replaced_token: parent.numeric_lexeme != result.numeric_lexeme,
        })
        .collect()
}
