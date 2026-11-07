use super::super::thermal;
use super::*;

impl SimulationShellProperty {
    /// Return complete status, explicit context, validation and exclusions together; e.g. `property.coverage().materials`.
    pub fn coverage(&self) -> SimulationShellPropertyCoverage {
        self.coverage_with_nested(thermal::nested_counts(self.fields()))
    }

    fn coverage_with_nested(&self, nested: (usize, usize)) -> SimulationShellPropertyCoverage {
        SimulationShellPropertyCoverage {
            version: self.version,
            limits: self.limits,
            context: self.context.clone(),
            declared_entries: self.entries,
            completion: SimulationShellPropertyCompletion::CompletedBody,
            check_mode: SimulationShellPropertyCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            body_count: 1,
            base_end: self.statistics.base_end,
            subtype: self.statistics.subtype,
            layers: self.statistics.layers,
            materials: self.statistics.materials,
            descriptors: self.statistics.descriptors,
            embedded_archives: nested.0,
            embedded_fields: nested.1,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
