//! Complete binary LBC thermal framing with independent outer and embedded version contexts (#134).
mod archive;
mod base;
mod contracts;
mod coverage;
mod edit;
mod history;
mod reader;
mod result;
mod tails;
mod values;

pub use archive::SimulationThermalCollection;
pub use contracts::*;
pub use values::{
    SimulationThermalField, SimulationThermalKind, SimulationThermalLimits, SimulationThermalValue,
};
