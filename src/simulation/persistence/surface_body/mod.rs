//! Complete SurfaceBody words and counted text lists (#153).
mod archive;
mod coverage;
mod decode;
mod edit;
mod identity;
mod types;

pub use super::mass_element::SimulationMassElementCompletion as SimulationSurfaceBodyCompletion;
pub use super::thermal::{
    SimulationThermalDiagnostic as SimulationSurfaceBodyDiagnostic,
    SimulationThermalField as SimulationSurfaceBodyField,
    SimulationThermalFieldId as SimulationSurfaceBodyFieldId,
    SimulationThermalHistory as SimulationSurfaceBodyHistory,
    SimulationThermalHistoryParent as SimulationSurfaceBodyHistoryParent,
    SimulationThermalRelation as SimulationSurfaceBodyRelation,
    SimulationThermalValue as SimulationSurfaceBodyValue,
};
pub use archive::SimulationSurfaceBody;
pub use edit::SimulationSurfaceBodyEdit;
pub use types::{
    SimulationSurfaceBodyCheckMode, SimulationSurfaceBodyCoverage, SimulationSurfaceBodyLimits,
};
