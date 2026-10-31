use super::super::thermal;
use super::{
    SimulationInertialCheckMode, SimulationInertialCollection, SimulationInertialCoverage,
};

impl SimulationInertialCollection {
    /// Return complete exact checks, diagnostics and achieved error together; e.g. `collection.coverage().completion`.
    pub fn coverage(&self) -> SimulationInertialCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(self.fields());
        SimulationInertialCoverage {
            kind: self.kind,
            version: self.version,
            limits: self.limits,
            completion: thermal::read_completion(self.record_count),
            check_mode: SimulationInertialCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            record_count: self.record_count,
            embedded_archives,
            embedded_fields,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
