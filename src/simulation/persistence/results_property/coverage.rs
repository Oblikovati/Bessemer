use super::super::thermal;
use super::{SimulationResultsCheckMode, SimulationResultsCoverage, SimulationResultsProperty};

impl SimulationResultsProperty {
    /// Return complete exact checks, diagnostics and achieved error together; e.g. `property.coverage().completion`.
    pub fn coverage(&self) -> SimulationResultsCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(self.fields());
        SimulationResultsCoverage {
            context: self.context,
            limits: self.limits,
            completion: super::SimulationResultsCompletion::CompletedBody,
            check_mode: SimulationResultsCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            body_count: 1,
            declared_entries: self.entries,
            embedded_archives,
            embedded_fields,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
