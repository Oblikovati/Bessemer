use super::super::thermal;
use super::{SimulationPostStyle, SimulationPostStyleCheckMode, SimulationPostStyleCoverage};

impl SimulationPostStyle {
    /// Return complete exact checks, diagnostics and achieved error together; e.g. `property.coverage().completion`.
    pub fn coverage(&self) -> SimulationPostStyleCoverage {
        SimulationPostStyleCoverage {
            version: self.version,
            limits: self.limits,
            completion: super::SimulationPostStyleCompletion::CompletedBody,
            check_mode: SimulationPostStyleCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            body_count: 1,
            declared_entries: self.entries,
            embedded_archives: 0,
            embedded_fields: 0,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
