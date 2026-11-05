//! Complete historical POSTPLOTDLG framing with a separate caller-version gate (#174).
mod archive;
mod decode;
mod edit;
mod limits;
mod outcome;
mod values;
mod wide;

pub use super::post_info::{
    SimulationPostInfoCheckMode as SimulationLegacyPlotCheckMode,
    SimulationPostInfoCompletion as SimulationLegacyPlotCompletion,
    SimulationPostInfoDiagnostic as SimulationLegacyPlotDiagnostic,
    SimulationPostInfoText as SimulationLegacyPlotText,
};
pub use super::thermal::{
    SimulationThermalFieldId as SimulationLegacyPlotFieldId,
    SimulationThermalHistory as SimulationLegacyPlotHistory,
    SimulationThermalHistoryParent as SimulationLegacyPlotHistoryParent,
    SimulationThermalRelation as SimulationLegacyPlotRelation,
};
pub use archive::SimulationLegacyPlot;
pub use edit::SimulationLegacyPlotEdit;
pub use limits::SimulationLegacyPlotLimits;
pub use outcome::SimulationLegacyPlotCoverage;
pub use values::{SimulationLegacyPlotField, SimulationLegacyPlotValue};
pub use wide::{SimulationLegacyPlotWideMember, SimulationLegacyPlotWideView};
