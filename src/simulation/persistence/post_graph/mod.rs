//! Complete bounded cwPostCommon graph parameters with atomic character text (#165).
mod archive;
mod contracts;
mod coverage;
mod decode;
mod edit;
mod result;
mod types;

/// Graph budgets; `maximum_entries` sums declared characters and `maximum_text_bytes` bounds each payload.
pub use super::thermal::SimulationThermalLimits as SimulationPostGraphLimits;
pub use super::thermal::{
    SimulationThermalCheckMode as SimulationPostGraphCheckMode,
    SimulationThermalDiagnostic as SimulationPostGraphDiagnostic,
};
pub use archive::SimulationPostGraphParameters;
pub use contracts::SimulationPostGraphEdit;
pub use types::{SimulationPostGraphCompletion, SimulationPostGraphCoverage};

pub use super::thermal::{
    SimulationThermalFieldId as SimulationPostGraphFieldId,
    SimulationThermalHistory as SimulationPostGraphHistory,
    SimulationThermalHistoryParent as SimulationPostGraphHistoryParent,
    SimulationThermalNestedHistory as SimulationPostGraphNestedHistory,
    SimulationThermalNestedParent as SimulationPostGraphNestedParent,
    SimulationThermalRelation as SimulationPostGraphRelation,
};

mod character_text;
mod values;
pub use character_text::SimulationPostGraphText;
pub use values::{SimulationPostGraphField, SimulationPostGraphValue};
