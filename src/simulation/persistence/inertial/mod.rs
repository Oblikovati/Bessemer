//! Complete binary inertial/excitation framing with explicit outer and embedded profiles (#136).
mod archive;
mod contracts;
mod coverage;
mod edit;
mod result;
mod tails;
mod types;

pub use super::thermal::{
    SimulationThermalCheckMode as SimulationInertialCheckMode,
    SimulationThermalCompletion as SimulationInertialCompletion,
    SimulationThermalDiagnostic as SimulationInertialDiagnostic,
    SimulationThermalField as SimulationInertialField,
    SimulationThermalLimits as SimulationInertialLimits,
    SimulationThermalValue as SimulationInertialValue,
};
pub use archive::SimulationInertialCollection;
pub use contracts::SimulationInertialEdit;
pub use types::{SimulationInertialCoverage, SimulationInertialKind};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationInertialFieldId,
    SimulationThermalHistory as SimulationInertialHistory,
    SimulationThermalHistoryParent as SimulationInertialHistoryParent,
    SimulationThermalNestedHistory as SimulationInertialNestedHistory,
    SimulationThermalNestedParent as SimulationInertialNestedParent,
    SimulationThermalRelation as SimulationInertialRelation,
};
