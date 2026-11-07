//! Complete direct SolidManager Property words and counted text lists (#148).
mod archive;
mod coverage;
mod decode;
mod edit;
mod identity;
mod types;

pub use super::mass_element::SimulationMassElementCompletion as SimulationSolidManagerCompletion;
pub use super::thermal::{
    SimulationThermalDiagnostic as SimulationSolidManagerDiagnostic,
    SimulationThermalField as SimulationSolidManagerField,
    SimulationThermalFieldId as SimulationSolidManagerFieldId,
    SimulationThermalHistory as SimulationSolidManagerHistory,
    SimulationThermalHistoryParent as SimulationSolidManagerHistoryParent,
    SimulationThermalRelation as SimulationSolidManagerRelation,
    SimulationThermalValue as SimulationSolidManagerValue,
};
pub use archive::SimulationSolidManagerProperty;
pub use edit::SimulationSolidManagerEdit;
pub use types::{
    SimulationSolidManagerCheckMode, SimulationSolidManagerCoverage, SimulationSolidManagerLimits,
};
