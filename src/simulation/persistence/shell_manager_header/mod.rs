//! Complete ShellManager headers with explicit reader/current-writer profiles (#156).
mod archive;
mod coverage;
mod decode;
mod edit;
mod identity;
mod types;

pub use super::mass_element::SimulationMassElementCompletion as SimulationShellManagerCompletion;
pub use super::thermal::{
    SimulationThermalDiagnostic as SimulationShellManagerDiagnostic,
    SimulationThermalField as SimulationShellManagerField,
    SimulationThermalFieldId as SimulationShellManagerFieldId,
    SimulationThermalHistory as SimulationShellManagerHistory,
    SimulationThermalHistoryParent as SimulationShellManagerHistoryParent,
    SimulationThermalRelation as SimulationShellManagerRelation,
    SimulationThermalValue as SimulationShellManagerValue,
};
pub use archive::SimulationShellManagerProperty;
pub use edit::SimulationShellManagerEdit;
pub use types::{
    SimulationShellManagerCheckMode, SimulationShellManagerCoverage, SimulationShellManagerLimits,
};
