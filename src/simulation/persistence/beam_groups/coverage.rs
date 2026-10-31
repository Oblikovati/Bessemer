use super::super::thermal;
use super::{SimulationBeamGroupBody, SimulationBeamGroupCheckMode, SimulationBeamGroupCoverage};

impl SimulationBeamGroupBody {
    /// Return complete exact checks, diagnostics and achieved error together; e.g. `property.coverage().completion`.
    pub fn coverage(&self) -> SimulationBeamGroupCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(self.fields());
        SimulationBeamGroupCoverage {
            kind: self.kind,
            version: self.version,
            limits: self.limits,
            completion: super::SimulationBeamGroupCompletion::CompletedBody,
            check_mode: SimulationBeamGroupCheckMode::ExactStoredProfile {
                expected_joint_count: self.kind.expected_count(),
            },
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
