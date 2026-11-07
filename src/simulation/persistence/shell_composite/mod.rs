//! Complete single-body shell composite framing with explicit embedded profiles (#157).
mod archive;
mod contracts;
mod coverage;
mod decode;
mod edit;
mod profile;
mod result;
pub use profile::SimulationShellCompositeProfile;
mod types;

pub use super::thermal::{
    SimulationThermalCheckMode as SimulationShellCompositeCheckMode,
    SimulationThermalDiagnostic as SimulationShellCompositeDiagnostic,
    SimulationThermalField as SimulationShellCompositeField,
    SimulationThermalLimits as SimulationShellCompositeLimits,
    SimulationThermalValue as SimulationShellCompositeValue,
};
pub use archive::SimulationShellComposite;
pub use contracts::SimulationShellCompositeEdit;
pub use types::{SimulationShellCompositeCompletion, SimulationShellCompositeCoverage};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationShellCompositeFieldId,
    SimulationThermalHistory as SimulationShellCompositeHistory,
    SimulationThermalHistoryParent as SimulationShellCompositeHistoryParent,
    SimulationThermalNestedHistory as SimulationShellCompositeNestedHistory,
    SimulationThermalNestedParent as SimulationShellCompositeNestedParent,
    SimulationThermalRelation as SimulationShellCompositeRelation,
};
