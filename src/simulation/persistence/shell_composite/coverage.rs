use super::super::thermal;
use super::{
    SimulationShellComposite, SimulationShellCompositeCheckMode, SimulationShellCompositeCoverage,
};

impl SimulationShellComposite {
    /// Return complete exact checks, diagnostics and achieved error together; e.g. `property.coverage().completion`.
    pub fn coverage(&self) -> SimulationShellCompositeCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(self.fields());
        SimulationShellCompositeCoverage {
            version: self.version,
            limits: self.limits,
            profile: self.profile,
            declared_entries: self.entries,
            completion: super::types::read_completion(self.profile, self.entries),
            check_mode: SimulationShellCompositeCheckMode::ExactSelectedProfiles,
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
