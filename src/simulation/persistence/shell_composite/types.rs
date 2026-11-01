use super::{
    SimulationShellCompositeCheckMode, SimulationShellCompositeDiagnostic,
    SimulationShellCompositeLimits,
};
use crate::simulation::SimulationPersistVersion;

/// Complete mandatory body reads remain distinct from isolated edit outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationShellCompositeCompletion {
    /// A complete counted FlipTurn body declared zero records.
    CompletedEmptyRecords,
    /// Every declared FlipTurn record was checked.
    CompletedRecords,
    /// Every field of one mandatory complete body was checked.
    CompletedBody,
    /// The requested exact wire pattern already matched.
    Unchanged,
    /// One isolated complete value changed.
    Changed,
}

pub(super) fn read_completion(
    profile: super::SimulationShellCompositeProfile,
    entries: usize,
) -> SimulationShellCompositeCompletion {
    use super::SimulationShellCompositeProfile::{FlipTurnReader, FlipTurnWriter};
    if !matches!(profile, FlipTurnReader | FlipTurnWriter) {
        return SimulationShellCompositeCompletion::CompletedBody;
    }
    if entries == 0 {
        return SimulationShellCompositeCompletion::CompletedEmptyRecords;
    }
    SimulationShellCompositeCompletion::CompletedRecords
}

/// Exact stored-profile read coverage with semantic exclusions and achieved pattern error.
#[derive(Debug, Clone, Copy)]
pub struct SimulationShellCompositeCoverage {
    /// Once-normalized outer version.
    pub version: SimulationPersistVersion,
    /// Caller budgets actually applied.
    pub limits: SimulationShellCompositeLimits,
    /// Explicit stream and reader/writer grammar, without first-success dispatch.
    pub profile: super::SimulationShellCompositeProfile,
    /// Aggregate declared FlipTurn records; Extra has zero.
    pub declared_entries: usize,
    /// Complete mandatory-body or isolated-edit status, independent of errors.
    pub completion: SimulationShellCompositeCompletion,
    /// Exact full stored-profile check mode.
    pub check_mode: SimulationShellCompositeCheckMode,
    /// Every checked input byte.
    pub checked_bytes: usize,
    /// Every complete outer field.
    pub checked_fields: usize,
    /// One mandatory complete body.
    pub body_count: usize,
    /// Every checked embedded root.
    pub embedded_archives: usize,
    /// Every physical field inside embedded roots.
    pub embedded_fields: usize,
    /// Native, runtime and geometry exclusions.
    pub diagnostics: [SimulationShellCompositeDiagnostic; 4],
    /// Achieved requested byte-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
