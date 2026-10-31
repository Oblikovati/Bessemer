//! Complete MeshControlManager headers with explicit reader/current-writer profiles (#161).
mod archive;
mod coverage;
mod decode;
mod edit;
mod identity;
mod profile;
mod types;
pub use profile::SimulationMeshControlHeaderProfile;

pub use super::mass_element::SimulationMassElementCompletion as SimulationMeshControlHeaderCompletion;
pub use super::shell_manager_header::{
    SimulationShellManagerCheckMode as SimulationMeshControlHeaderCheckMode,
    SimulationShellManagerLimits as SimulationMeshControlHeaderLimits,
};
pub use super::thermal::{
    SimulationThermalDiagnostic as SimulationMeshControlHeaderDiagnostic,
    SimulationThermalField as SimulationMeshControlHeaderField,
    SimulationThermalFieldId as SimulationMeshControlHeaderFieldId,
    SimulationThermalHistory as SimulationMeshControlHeaderHistory,
    SimulationThermalHistoryParent as SimulationMeshControlHeaderHistoryParent,
    SimulationThermalRelation as SimulationMeshControlHeaderRelation,
    SimulationThermalValue as SimulationMeshControlHeaderValue,
};
pub use archive::SimulationMeshControlHeader;
pub use edit::SimulationMeshControlHeaderEdit;
pub use types::SimulationMeshControlHeaderCoverage;
