//! Complete binary LBC thermal framing with independent outer and embedded version contexts (#134).
mod archive;
use super::load_base as base;
mod contracts;
mod coverage;
mod edit;
mod history;
mod result;
mod tails;
mod values;

pub use archive::SimulationThermalCollection;
pub use contracts::*;
pub use values::{
    SimulationThermalField, SimulationThermalKind, SimulationThermalLimits, SimulationThermalValue,
};

pub(super) use archive::{construction_mismatch, encode_fields};
pub(super) use coverage::{diagnostics, nested_counts, read_completion};
