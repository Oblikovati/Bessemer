use super::super::thermal;
use super::*;

impl SimulationMeshProperty {
    /// Return complete framing/coverage and explicit exclusions; e.g. `body.coverage().record_count`.
    pub fn coverage(&self) -> SimulationMeshPropertyCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(self.fields());
        SimulationMeshPropertyCoverage {
            version: self.version,
            limits: self.limits,
            completion: SimulationMeshPropertyCompletion::CompletedBody,
            check_mode: SimulationMeshPropertyCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            record_count: self.records.len(),
            nested_record_count: self.nested_records,
            body_count: 1,
            declared_entries: self.entries,
            embedded_archives,
            embedded_fields,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
