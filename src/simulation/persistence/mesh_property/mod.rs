//! Complete documented Mesh Property fields and isolated edits (#163).
mod archive;
mod contracts;
mod coverage;
mod decode;
mod direct;
mod edit;
mod initial;
mod nested;
mod result;
mod tail;
mod types;

pub use super::sheet_property::SimulationSheetCompletion as SimulationMeshPropertyCompletion;
pub use super::thermal::{
    SimulationThermalCheckMode as SimulationMeshPropertyCheckMode,
    SimulationThermalDiagnostic as SimulationMeshPropertyDiagnostic,
    SimulationThermalField as SimulationMeshPropertyField,
    SimulationThermalFieldId as SimulationMeshPropertyFieldId,
    SimulationThermalHistory as SimulationMeshPropertyHistory,
    SimulationThermalHistoryParent as SimulationMeshPropertyHistoryParent,
    SimulationThermalLimits as SimulationMeshPropertyLimits,
    SimulationThermalNestedHistory as SimulationMeshPropertyNestedHistory,
    SimulationThermalNestedParent as SimulationMeshPropertyNestedParent,
    SimulationThermalRelation as SimulationMeshPropertyRelation,
    SimulationThermalValue as SimulationMeshPropertyValue,
};
pub use archive::SimulationMeshProperty;
pub use contracts::SimulationMeshPropertyEdit;
pub use types::{SimulationMeshPropertyCoverage, SimulationMeshPropertyRecordSpan};
