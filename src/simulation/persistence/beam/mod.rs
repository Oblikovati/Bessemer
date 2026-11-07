//! Complete beam child Property wire grammar with explicit two-byte signed fields (#142).
mod archive;
mod contracts;
mod coverage;
mod decode;
mod edit;
mod fields;
mod result;
mod snapshot;

pub use super::mass_element::{
    SimulationMassElementCompletion as SimulationBeamCompletion,
    SimulationMassElementCoverage as SimulationBeamCoverage,
};
pub use super::thermal::SimulationThermalLimits as SimulationBeamLimits;
pub use super::thermal::{
    SimulationThermalCheckMode as SimulationBeamCheckMode,
    SimulationThermalDiagnostic as SimulationBeamDiagnostic,
    SimulationThermalFieldId as SimulationBeamFieldId,
    SimulationThermalHistory as SimulationBeamHistory,
    SimulationThermalHistoryParent as SimulationBeamHistoryParent,
    SimulationThermalNestedHistory as SimulationBeamNestedHistory,
    SimulationThermalNestedParent as SimulationBeamNestedParent,
    SimulationThermalRelation as SimulationBeamRelation,
};
pub use archive::SimulationBeamProperty;
pub use contracts::SimulationBeamEdit;
pub use fields::{SimulationBeamField, SimulationBeamValue};
