use super::super::thermal;
use super::*;

impl SimulationSurfaceBody {
    /// Return exact complete header checks and explicit exclusions; e.g. `header.coverage().completion`.
    pub fn coverage(&self) -> SimulationSurfaceBodyCoverage {
        SimulationSurfaceBodyCoverage {
            version: self.version,
            limits: self.limits,
            completion: SimulationSurfaceBodyCompletion::CompletedBody,
            check_mode: SimulationSurfaceBodyCheckMode::ExactStandalone,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            body_count: 1,
            list_entries: self.entries,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
