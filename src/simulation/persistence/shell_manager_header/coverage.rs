use super::super::thermal;
use super::*;

impl SimulationShellManagerProperty {
    /// Return exact complete header checks and explicit exclusions; e.g. `header.coverage().completion`.
    pub fn coverage(&self) -> SimulationShellManagerCoverage {
        SimulationShellManagerCoverage {
            version: self.version,
            limits: self.limits,
            completion: SimulationShellManagerCompletion::CompletedBody,
            check_mode: SimulationShellManagerCheckMode::ExactStandalone,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            body_count: 1,
            wire_profile: self.profile,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
