//! Complete single-body standalone Component PersistID framing with explicit embedded profiles (#152).
mod archive;
mod contracts;
mod coverage;
mod edit;
mod result;
mod types;

pub use super::thermal::{
    SimulationThermalCheckMode as SimulationComponentPersistIdCheckMode,
    SimulationThermalDiagnostic as SimulationComponentPersistIdDiagnostic,
    SimulationThermalField as SimulationComponentPersistIdField,
    SimulationThermalLimits as SimulationComponentPersistIdLimits,
    SimulationThermalValue as SimulationComponentPersistIdValue,
};
pub use archive::SimulationComponentPersistId;
pub use contracts::SimulationComponentPersistIdEdit;
pub use types::{SimulationComponentPersistIdCompletion, SimulationComponentPersistIdCoverage};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationComponentPersistIdFieldId,
    SimulationThermalHistory as SimulationComponentPersistIdHistory,
    SimulationThermalHistoryParent as SimulationComponentPersistIdHistoryParent,
    SimulationThermalNestedHistory as SimulationComponentPersistIdNestedHistory,
    SimulationThermalNestedParent as SimulationComponentPersistIdNestedParent,
    SimulationThermalRelation as SimulationComponentPersistIdRelation,
};
