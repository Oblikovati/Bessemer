//! Complete fixed-slot PostInfo with the exact version700002 text suffix (#166).
mod archive;
mod contracts;
mod coverage;
mod decode;
mod edit;
mod result;
mod types;

mod limits;
pub use super::thermal::{
    SimulationThermalCheckMode as SimulationPostInfoCheckMode,
    SimulationThermalDiagnostic as SimulationPostInfoDiagnostic,
};
pub use archive::SimulationPostInfo;
pub use contracts::SimulationPostInfoEdit;
pub use limits::SimulationPostInfoLimits;
pub use types::{SimulationPostInfoCompletion, SimulationPostInfoCoverage};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationPostInfoFieldId,
    SimulationThermalHistory as SimulationPostInfoHistory,
    SimulationThermalHistoryParent as SimulationPostInfoHistoryParent,
    SimulationThermalNestedHistory as SimulationPostInfoNestedHistory,
    SimulationThermalNestedParent as SimulationPostInfoNestedParent,
    SimulationThermalRelation as SimulationPostInfoRelation,
};

mod character_text;
mod values;
pub use character_text::SimulationPostInfoText;
pub use values::{SimulationPostInfoField, SimulationPostInfoValue};

pub(super) use values::{
    require_kind as info_require_kind, to_wire_fields as info_to_wire_fields,
    wire_value as info_wire_value,
};

pub(super) use decode::read as read_postinfo;
