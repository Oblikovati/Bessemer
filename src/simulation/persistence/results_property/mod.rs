//! Complete Results hierarchy Property framing with explicit reader/current-writer contexts (#164).
mod archive;
mod contracts;
mod coverage;
mod decode;
mod edit;
mod result;
mod types;

pub use super::thermal::{
    SimulationThermalCheckMode as SimulationResultsCheckMode,
    SimulationThermalDiagnostic as SimulationResultsDiagnostic,
    SimulationThermalField as SimulationResultsField,
    SimulationThermalLimits as SimulationResultsLimits,
    SimulationThermalValue as SimulationResultsValue,
};
pub use archive::SimulationResultsProperty;
pub use contracts::SimulationResultsEdit;
pub use types::{SimulationResultsCompletion, SimulationResultsCoverage};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationResultsFieldId,
    SimulationThermalHistory as SimulationResultsHistory,
    SimulationThermalHistoryParent as SimulationResultsHistoryParent,
    SimulationThermalNestedHistory as SimulationResultsNestedHistory,
    SimulationThermalNestedParent as SimulationResultsNestedParent,
    SimulationThermalRelation as SimulationResultsRelation,
};

mod context;
pub use context::SimulationResultsContext;
