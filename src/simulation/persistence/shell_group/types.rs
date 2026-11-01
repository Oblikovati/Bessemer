use super::{
    SimulationShellGroupCheckMode, SimulationShellGroupCompletion, SimulationShellGroupDiagnostic,
};
use crate::simulation::{SimulationPersistVersion, SimulationShellWireProfile};

/// Complete group byte/primitive/record/text budgets, applied before publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationShellGroupLimits {
    /// Entire input and reconstructed output bytes, including every CString prefix.
    pub maximum_bytes: usize,
    /// Count plus every complete scalar/CString occurrence.
    pub maximum_fields: usize,
    /// Declared direct records; the recovered parser also caps these at 100,000.
    pub maximum_entries: usize,
    /// Aggregate CString payload bytes across all records, excluding length/encoding prefixes.
    pub maximum_text_bytes: usize,
}

/// Exact stored grammar, completion, exclusions and achieved byte-pattern error together.
#[derive(Debug, Clone, Copy)]
pub struct SimulationShellGroupCoverage {
    /// Once-normalized supplied context, without inferred runtime dispatch.
    pub version: SimulationPersistVersion,
    /// Explicit late-field reader/current-writer grammar; text representations remain retained.
    pub wire_profile: SimulationShellWireProfile,
    /// Caller budgets actually applied to the complete body.
    pub limits: SimulationShellGroupLimits,
    /// Complete empty/nonempty record framing or isolated edit outcome.
    pub completion: SimulationShellGroupCompletion,
    /// Exact selected grammar and optional isolated-preservation check.
    pub check_mode: SimulationShellGroupCheckMode,
    /// Every input byte, checked through mandatory EOF.
    pub checked_bytes: usize,
    /// Count and all complete physical field occurrences.
    pub checked_fields: usize,
    /// Validated direct record count, distinct from folders or storage enumeration.
    pub record_count: usize,
    /// Aggregate checked CString payload bytes, including embedded NULs.
    pub text_bytes: usize,
    /// Native execution, runtime dispatch, physical semantics and geometry exclusions.
    pub diagnostics: [SimulationShellGroupDiagnostic; 4],
    /// Exact stored-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

pub(super) fn coverage(group: &super::SimulationShellGroup) -> SimulationShellGroupCoverage {
    let completion = if group.entries == 0 {
        SimulationShellGroupCompletion::CompletedEmptyRecords
    } else {
        SimulationShellGroupCompletion::CompletedRecords
    };
    SimulationShellGroupCoverage {
        version: group.version,
        wire_profile: group.profile,
        limits: group.limits,
        completion,
        check_mode: SimulationShellGroupCheckMode::ExactSelectedProfiles,
        checked_bytes: group.parent.len(),
        checked_fields: group.fields.len(),
        record_count: group.entries,
        text_bytes: group.text_bytes,
        diagnostics: super::super::thermal::diagnostics(),
        requested_pattern_error_bits: 0,
    }
}
