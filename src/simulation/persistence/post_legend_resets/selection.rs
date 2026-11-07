use super::super::SimulationPersistVersion;
use super::SimulationLegendResetPlan;
use super::plan::LegendResetAssignmentSet;
use crate::{BessemerError, FailureClass, Result};

impl SimulationPersistVersion {
    /// Select explicit legend zero events without loading object state; e.g. `version.legend_read_zero_assignments()?.assignments()`.
    pub fn legend_read_zero_assignments(self) -> Result<SimulationLegendResetPlan> {
        let version = self.value();
        if version < 700006 {
            return Err(BessemerError::new(
                "legend read-time zero assignments",
                "explicit-field reader version",
                FailureClass::Unsupported,
                format!(
                    "{version}; expected explicit-field version700006..2147483647; raw-class conversion is separate"
                ),
            ));
        }
        let assignments = match version {
            700006..840008 => LegendResetAssignmentSet::all(),
            840008..840010 => LegendResetAssignmentSet::formats(),
            _ => LegendResetAssignmentSet::None,
        };
        Ok(SimulationLegendResetPlan::complete(self, assignments))
    }
}
