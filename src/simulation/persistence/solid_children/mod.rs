//! Separate direct component and static-folder Property profiles under explicit context (#149).
mod archive;
mod coverage;
mod decode;
mod edit;
mod identity;
mod kinds;
mod types;

pub use super::mass_element::SimulationMassElementCompletion as SimulationSolidChildCompletion;
pub use super::thermal::{
    SimulationThermalDiagnostic as SimulationSolidChildDiagnostic,
    SimulationThermalField as SimulationSolidChildField,
    SimulationThermalFieldId as SimulationSolidChildFieldId,
    SimulationThermalHistory as SimulationSolidChildHistory,
    SimulationThermalHistoryParent as SimulationSolidChildHistoryParent,
    SimulationThermalRelation as SimulationSolidChildRelation,
    SimulationThermalValue as SimulationSolidChildValue,
};
pub use archive::SimulationSolidChildProperty;
pub use edit::SimulationSolidChildEdit;
pub use kinds::{SimulationSolidChildEvidence, SimulationSolidChildKind};
pub use types::{
    SimulationSolidChildCheckMode, SimulationSolidChildCoverage, SimulationSolidChildLimits,
};
