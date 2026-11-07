use super::super::thermal;
use super::*;

impl SimulationSolidManagerProperty {
    /// Return exact complete header checks and explicit exclusions; e.g. `header.coverage().completion`.
    pub fn coverage(&self) -> SimulationSolidManagerCoverage {
        SimulationSolidManagerCoverage {
            version: self.version,
            limits: self.limits,
            completion: SimulationSolidManagerCompletion::CompletedBody,
            check_mode: SimulationSolidManagerCheckMode::ExactStandalone,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            body_count: 1,
            list_entries: self.entries,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
