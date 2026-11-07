//! Complete single-body inherited SolidManager sheet Property framing with explicit embedded profiles (#151).
mod archive;
mod contracts;
mod coverage;
mod decode;
mod edit;
mod result;
mod types;

pub use super::thermal::{
    SimulationThermalCheckMode as SimulationSheetCheckMode,
    SimulationThermalDiagnostic as SimulationSheetDiagnostic,
    SimulationThermalField as SimulationSheetField,
    SimulationThermalLimits as SimulationSheetLimits,
    SimulationThermalValue as SimulationSheetValue,
};
pub use archive::SimulationSheetProperty;
pub use contracts::SimulationSheetEdit;
pub use types::{SimulationSheetCompletion, SimulationSheetCoverage};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationSheetFieldId,
    SimulationThermalHistory as SimulationSheetHistory,
    SimulationThermalHistoryParent as SimulationSheetHistoryParent,
    SimulationThermalNestedHistory as SimulationSheetNestedHistory,
    SimulationThermalNestedParent as SimulationSheetNestedParent,
    SimulationThermalRelation as SimulationSheetRelation,
};
