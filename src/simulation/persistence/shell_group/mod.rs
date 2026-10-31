//! Complete direct ShellManagerGroup records with retained MFC CString frames (#156).
mod archive;
mod decode;
mod edit;
mod fields;
mod reader;
mod types;

pub use super::thermal::{
    SimulationThermalCheckMode as SimulationShellGroupCheckMode,
    SimulationThermalCompletion as SimulationShellGroupCompletion,
    SimulationThermalDiagnostic as SimulationShellGroupDiagnostic,
    SimulationThermalFieldId as SimulationShellGroupFieldId,
    SimulationThermalHistory as SimulationShellGroupHistory,
    SimulationThermalHistoryParent as SimulationShellGroupHistoryParent,
    SimulationThermalRelation as SimulationShellGroupRelation,
};
pub use archive::SimulationShellGroup;
pub use edit::SimulationShellGroupEdit;
pub use fields::{SimulationShellGroupField, SimulationShellGroupValue};
pub use types::{SimulationShellGroupCoverage, SimulationShellGroupLimits};
