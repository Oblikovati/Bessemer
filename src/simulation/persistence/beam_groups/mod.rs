//! Complete separate beam joint/group/list/folder bodies under explicit contexts (#144).
mod archive;
mod contracts;
mod coverage;
mod coverage_types;
mod decode;
mod edit;
mod kinds;
mod result;

pub use super::mass_element::SimulationMassElementCompletion as SimulationBeamGroupCompletion;
pub use super::thermal::{
    SimulationThermalDiagnostic as SimulationBeamGroupDiagnostic,
    SimulationThermalField as SimulationBeamGroupField,
    SimulationThermalFieldId as SimulationBeamGroupFieldId,
    SimulationThermalHistory as SimulationBeamGroupHistory,
    SimulationThermalHistoryParent as SimulationBeamGroupHistoryParent,
    SimulationThermalLimits as SimulationBeamGroupLimits,
    SimulationThermalNestedHistory as SimulationBeamGroupNestedHistory,
    SimulationThermalNestedParent as SimulationBeamGroupNestedParent,
    SimulationThermalRelation as SimulationBeamGroupRelation,
    SimulationThermalValue as SimulationBeamGroupValue,
};
pub use archive::SimulationBeamGroupBody;
pub use contracts::SimulationBeamGroupEdit;
pub use coverage_types::SimulationBeamGroupCoverage;
pub use kinds::{SimulationBeamGroupCheckMode, SimulationBeamGroupKind};
