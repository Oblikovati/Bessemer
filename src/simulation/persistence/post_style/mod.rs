//! Complete versioned PostStyle and embedded legacy/modern legend profiles (#166).
mod archive;
mod contracts;
mod coverage;
mod decode;
mod edit;
mod legacy;
mod result;
mod types;
mod values;

/// Raw word/float/text parts share the complete PostInfo representation; selected helper limits remain explicit.
pub use super::post_info::{
    SimulationPostInfoField as SimulationPostStyleField,
    SimulationPostInfoLimits as SimulationPostStyleLimits,
    SimulationPostInfoText as SimulationPostStyleText,
    SimulationPostInfoValue as SimulationPostStyleValue,
};
pub use super::thermal::{
    SimulationThermalCheckMode as SimulationPostStyleCheckMode,
    SimulationThermalDiagnostic as SimulationPostStyleDiagnostic,
    SimulationThermalFieldId as SimulationPostStyleFieldId,
    SimulationThermalHistory as SimulationPostStyleHistory,
    SimulationThermalHistoryParent as SimulationPostStyleHistoryParent,
    SimulationThermalNestedHistory as SimulationPostStyleNestedHistory,
    SimulationThermalNestedParent as SimulationPostStyleNestedParent,
    SimulationThermalRelation as SimulationPostStyleRelation,
};
pub use archive::SimulationPostStyle;
pub use contracts::SimulationPostStyleEdit;
pub use types::{SimulationPostStyleCompletion, SimulationPostStyleCoverage};
