use super::super::thermal;
use super::{
    SimulationMassElementCheckMode, SimulationMassElementCoverage, SimulationMassElementProperty,
};

impl SimulationMassElementProperty {
    /// Return complete exact checks, diagnostics and achieved error together; e.g. `property.coverage().completion`.
    pub fn coverage(&self) -> SimulationMassElementCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(self.fields());
        SimulationMassElementCoverage {
            version: self.version,
            limits: self.limits,
            completion: super::SimulationMassElementCompletion::CompletedBody,
            check_mode: SimulationMassElementCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            body_count: 1,
            embedded_archives,
            embedded_fields,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
