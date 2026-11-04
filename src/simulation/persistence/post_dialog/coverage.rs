use super::super::thermal;
use super::{
    SimulationPostDialog, SimulationPostDialogCheckMode, SimulationPostDialogCompletion,
    SimulationPostDialogCoverage,
};

impl SimulationPostDialog {
    /// Return exact profile/preservation checks and explicit semantic exclusions; e.g. `dialog.coverage().completion`.
    pub fn coverage(&self) -> SimulationPostDialogCoverage {
        let (embedded_archives, embedded_fields) = thermal::nested_counts(&self.wire_fields);
        SimulationPostDialogCoverage {
            version: self.version,
            limits: self.limits,
            completion: SimulationPostDialogCompletion::CompletedBody,
            check_mode: SimulationPostDialogCheckMode::ExactSelectedProfiles,
            checked_bytes: self.parent.len(),
            checked_fields: self.fields.len(),
            body_count: 1,
            declared_entries: self.entries,
            embedded_archives,
            embedded_fields,
            diagnostics: thermal::diagnostics(),
            requested_pattern_error_bits: 0,
        }
    }
}
