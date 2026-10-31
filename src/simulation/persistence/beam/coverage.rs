use super::super::thermal;
use super::{SimulationBeamCheckMode, SimulationBeamCoverage, SimulationBeamProperty};

impl SimulationBeamProperty {
    /// Return complete exact checks, diagnostics and achieved error together; e.g. `property.coverage().completion`.
    pub fn coverage(&self) -> SimulationBeamCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(&self.wire_fields);
        SimulationBeamCoverage {
            version: self.version,
            limits: self.limits,
            completion: super::SimulationBeamCompletion::CompletedBody,
            check_mode: SimulationBeamCheckMode::ExactSelectedProfiles,
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
