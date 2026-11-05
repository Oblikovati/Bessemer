use super::{
    SimulationLegacyPlot, SimulationLegacyPlotCheckMode, SimulationLegacyPlotCompletion,
    SimulationLegacyPlotDiagnostic, SimulationLegacyPlotLimits,
};
use crate::simulation::SimulationPersistVersion;

/// Exact full helper coverage with explicit caller and native-runtime qualifications.
#[derive(Debug, Clone, Copy)]
pub struct SimulationLegacyPlotCoverage {
    /// Supplied context, never selected to fit the endpoint.
    pub version: SimulationPersistVersion,
    /// Explicit requested safety budgets.
    pub limits: SimulationLegacyPlotLimits,
    /// Complete mandatory-body, changed or unchanged status.
    pub completion: SimulationLegacyPlotCompletion,
    /// Exact selected grammar or grammar plus byte-preservation checks.
    pub check_mode: SimulationLegacyPlotCheckMode,
    /// All checked input bytes.
    pub checked_bytes: usize,
    /// All complete physical values.
    pub checked_fields: usize,
    /// Total declared code units across counted texts.
    pub declared_entries: usize,
    /// Independent maximum per counted text in the documented reader profile.
    pub profile_character_cap: usize,
    /// Only the recovered supplied-version gate, separate from helper completion.
    pub caller_version_gate_enabled: bool,
    /// Native loader/writer, runtime, physical and geometry exclusions.
    pub diagnostics: [SimulationLegacyPlotDiagnostic; 4],
    /// Achieved requested raw-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

impl SimulationLegacyPlot {
    /// Return completion, exact validation and exclusions together; e.g. `plot.coverage().checked_bytes`.
    pub fn coverage(&self) -> SimulationLegacyPlotCoverage {
        SimulationLegacyPlotCoverage {
            version: self.version,
            limits: self.limits,
            completion: SimulationLegacyPlotCompletion::CompletedBody,
            check_mode: SimulationLegacyPlotCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            declared_entries: self.entries,
            profile_character_cap: 512,
            caller_version_gate_enabled: self.named_stream_dispatch_enabled(),
            diagnostics: super::super::thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
