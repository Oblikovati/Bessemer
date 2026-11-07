use super::super::thermal;
use super::{SimulationSheetCheckMode, SimulationSheetCoverage, SimulationSheetProperty};

impl SimulationSheetProperty {
    /// Return complete exact checks, diagnostics and achieved error together; e.g. `property.coverage().completion`.
    pub fn coverage(&self) -> SimulationSheetCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(self.fields());
        SimulationSheetCoverage {
            version: self.version,
            limits: self.limits,
            completion: super::SimulationSheetCompletion::CompletedBody,
            check_mode: SimulationSheetCheckMode::ExactSelectedProfiles,
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
