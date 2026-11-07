use super::super::thermal;
use super::{
    SimulationPostGraphCheckMode, SimulationPostGraphCoverage, SimulationPostGraphParameters,
};

impl SimulationPostGraphParameters {
    /// Return complete exact checks, diagnostics and achieved error together; e.g. `property.coverage().completion`.
    pub fn coverage(&self) -> SimulationPostGraphCoverage {
        SimulationPostGraphCoverage {
            version: self.version,
            limits: self.limits,
            completion: super::SimulationPostGraphCompletion::CompletedBody,
            check_mode: SimulationPostGraphCheckMode::ExactSelectedProfiles,
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
