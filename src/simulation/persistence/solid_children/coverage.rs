use super::super::thermal;
use super::*;

impl SimulationSolidChildProperty {
    /// Return complete scoped checks, evidence, history policy and diagnostics; e.g. `property.coverage().evidence`.
    pub fn coverage(&self) -> SimulationSolidChildCoverage {
        SimulationSolidChildCoverage {
            kind: self.kind,
            evidence: self.kind.evidence(),
            behavior_profile: 1,
            version: self.version,
            limits: self.limits,
            completion: SimulationSolidChildCompletion::CompletedBody,
            check_mode: SimulationSolidChildCheckMode::ExactStandalone,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            body_count: 1,
            list_entries: self.entries,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
