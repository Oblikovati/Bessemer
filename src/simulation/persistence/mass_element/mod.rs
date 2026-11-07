//! Complete single-body MassElement Property framing with explicit embedded profiles (#138).
mod archive;
mod contracts;
mod coverage;
mod edit;
mod result;
mod tails;
mod types;

pub use super::thermal::{
    SimulationThermalCheckMode as SimulationMassElementCheckMode,
    SimulationThermalDiagnostic as SimulationMassElementDiagnostic,
    SimulationThermalField as SimulationMassElementField,
    SimulationThermalLimits as SimulationMassElementLimits,
    SimulationThermalValue as SimulationMassElementValue,
};
pub use archive::SimulationMassElementProperty;
pub use contracts::SimulationMassElementEdit;
pub use types::{SimulationMassElementCompletion, SimulationMassElementCoverage};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationMassElementFieldId,
    SimulationThermalHistory as SimulationMassElementHistory,
    SimulationThermalHistoryParent as SimulationMassElementHistoryParent,
    SimulationThermalNestedHistory as SimulationMassElementNestedHistory,
    SimulationThermalNestedParent as SimulationMassElementNestedParent,
    SimulationThermalRelation as SimulationMassElementRelation,
};
