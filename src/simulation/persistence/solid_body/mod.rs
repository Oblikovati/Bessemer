//! Complete single-body SolidManager solid-body Property framing with explicit embedded profiles (#150).
mod archive;
mod contracts;
mod coverage;
mod decode;
mod edit;
mod result;
mod types;

pub use super::thermal::{
    SimulationThermalCheckMode as SimulationSolidBodyCheckMode,
    SimulationThermalDiagnostic as SimulationSolidBodyDiagnostic,
    SimulationThermalField as SimulationSolidBodyField,
    SimulationThermalLimits as SimulationSolidBodyLimits,
    SimulationThermalValue as SimulationSolidBodyValue,
};
pub use archive::SimulationSolidBodyProperty;
pub use contracts::SimulationSolidBodyEdit;
pub use types::{SimulationSolidBodyCompletion, SimulationSolidBodyCoverage};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationSolidBodyFieldId,
    SimulationThermalHistory as SimulationSolidBodyHistory,
    SimulationThermalHistoryParent as SimulationSolidBodyHistoryParent,
    SimulationThermalNestedHistory as SimulationSolidBodyNestedHistory,
    SimulationThermalNestedParent as SimulationSolidBodyNestedParent,
    SimulationThermalRelation as SimulationSolidBodyRelation,
};
