//! Complete parent dialog physical profiles with isolated typed operations (#166).
mod collections;
mod decode;
mod equation;
mod helpers;
mod late;
mod mesh_quality;
mod middle;
mod planes;
mod references;

mod archive;
mod contracts;
mod counted_bytes;
mod coverage;
mod edit;
mod result;
mod types;
mod values;

pub use super::post_info::SimulationPostInfoText as SimulationPostDialogText;
pub use super::thermal::{
    SimulationThermalCheckMode as SimulationPostDialogCheckMode,
    SimulationThermalDiagnostic as SimulationPostDialogDiagnostic,
    SimulationThermalFieldId as SimulationPostDialogFieldId,
    SimulationThermalHistory as SimulationPostDialogHistory,
    SimulationThermalHistoryParent as SimulationPostDialogHistoryParent,
    SimulationThermalLimits as SimulationPostDialogLimits,
    SimulationThermalNestedHistory as SimulationPostDialogNestedHistory,
    SimulationThermalNestedParent as SimulationPostDialogNestedParent,
    SimulationThermalRelation as SimulationPostDialogRelation,
};
pub use archive::SimulationPostDialog;
pub use contracts::SimulationPostDialogEdit;
pub use counted_bytes::SimulationPostDialogBytes;
pub use types::{SimulationPostDialogCompletion, SimulationPostDialogCoverage};
pub use values::{SimulationPostDialogField, SimulationPostDialogValue};
