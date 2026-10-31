//! Complete stored study-manager Material bodies with explicit prefix/EOF boundaries (#140).
mod archive;
mod coverage;
mod decode;
mod edit;
mod groups;
mod identity;
mod tails;
mod types;

pub use super::mass_element::SimulationMassElementCompletion as SimulationMaterialCompletion;
pub use super::thermal::{
    SimulationThermalDiagnostic as SimulationMaterialDiagnostic,
    SimulationThermalField as SimulationMaterialField,
    SimulationThermalFieldId as SimulationMaterialFieldId,
    SimulationThermalHistory as SimulationMaterialHistory,
    SimulationThermalHistoryParent as SimulationMaterialHistoryParent,
    SimulationThermalRelation as SimulationMaterialRelation,
    SimulationThermalValue as SimulationMaterialValue,
};
pub use archive::SimulationStudyMaterial;
pub use edit::SimulationMaterialEdit;
pub use types::{
    SimulationMaterialBoundary, SimulationMaterialCheckMode, SimulationMaterialCoverage,
    SimulationMaterialLimits,
};
