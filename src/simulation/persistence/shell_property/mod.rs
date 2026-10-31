//! Complete single-body shell Property framing with explicit embedded profiles (#159).
mod archive;
mod context;
mod contracts;
mod coverage;
mod decode;
mod edit;
mod layers;
mod presence;
mod result;
pub use context::{
    SimulationShellLayerPresence, SimulationShellPropertyContext, SimulationShellPropertyKind,
};
mod types;

pub use super::thermal::{
    SimulationThermalCheckMode as SimulationShellPropertyCheckMode,
    SimulationThermalDiagnostic as SimulationShellPropertyDiagnostic,
    SimulationThermalField as SimulationShellPropertyField,
    SimulationThermalLimits as SimulationShellPropertyLimits,
    SimulationThermalValue as SimulationShellPropertyValue,
};
pub use archive::SimulationShellProperty;
pub use contracts::SimulationShellPropertyEdit;
pub use types::{SimulationShellPropertyCompletion, SimulationShellPropertyCoverage};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationShellPropertyFieldId,
    SimulationThermalHistory as SimulationShellPropertyHistory,
    SimulationThermalHistoryParent as SimulationShellPropertyHistoryParent,
    SimulationThermalNestedHistory as SimulationShellPropertyNestedHistory,
    SimulationThermalNestedParent as SimulationShellPropertyNestedParent,
    SimulationThermalRelation as SimulationShellPropertyRelation,
};
