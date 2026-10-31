use super::super::thermal;
use super::*;

impl SimulationMeshControlRecords {
    /// Return complete framing/coverage and explicit exclusions; e.g. `body.coverage().record_count`.
    pub fn coverage(&self) -> SimulationMeshControlCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(self.fields());
        SimulationMeshControlCoverage {
            version: self.version,
            limits: self.limits,
            completion: thermal::read_completion(self.records.len()),
            check_mode: SimulationMeshControlCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            record_count: self.records.len(),
            declared_entries: self.entries,
            embedded_archives,
            embedded_fields,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
