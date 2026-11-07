use super::{
    SimulationMaterialBoundary, SimulationMaterialCheckMode, SimulationMaterialCompletion,
    SimulationMaterialCoverage, SimulationStudyMaterial,
};

impl SimulationStudyMaterial {
    /// Return complete status, exact scope, diagnostics and achieved error together; e.g. `body.coverage().boundary`.
    pub fn coverage(&self) -> SimulationMaterialCoverage {
        SimulationMaterialCoverage {
            version: self.version,
            limits: self.limits,
            completion: SimulationMaterialCompletion::CompletedBody,
            boundary: self.boundary,
            check_mode: match self.boundary {
                SimulationMaterialBoundary::Standalone => {
                    SimulationMaterialCheckMode::ExactStandalone
                }
                SimulationMaterialBoundary::Prefix => SimulationMaterialCheckMode::ExactPrefix,
            },
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            property_count: self.statistics.properties,
            delegated_count: self.statistics.delegates,
            point_pairs: self.statistics.pairs,
            declared_entries: self.statistics.entries,
            diagnostics: super::super::thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
