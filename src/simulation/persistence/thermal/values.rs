use crate::model_archives::{ModelArchiveLimits, SimulationEmbeddedArchive};
use crate::simulation::SimulationPersistText;
use std::sync::Arc;

/// Explicit binary LBC collection family, distinct from legacy text thermal records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationThermalKind {
    /// Temperature collection.
    Temperature,
    /// Convection collection.
    Convection,
    /// HeatFlux collection.
    HeatFlux,
    /// HeatPower collection.
    HeatPower,
    /// Radiation collection.
    Radiation,
}

/// Explicit operation budgets in addition to the fixed documented count caps.
#[derive(Debug, Clone, Copy)]
pub struct SimulationThermalLimits {
    /// Entire collection input and reconstructed output budget.
    pub maximum_bytes: usize,
    /// Outer physical fields, counting each complete embedded archive once.
    pub maximum_fields: usize,
    /// Aggregate records, references, pairs and terminal table entries.
    pub maximum_entries: usize,
    /// Maximum payload bytes for each counted text frame.
    pub maximum_text_bytes: usize,
    /// Independent budgets for each embedded object archive.
    pub embedded: ModelArchiveLimits,
}

/// Owned physical stored value; doubles retain bits without unit or geometry interpretation.
#[derive(Debug, Clone)]
pub enum SimulationThermalValue {
    /// Signed four-byte word.
    Signed(i32),
    /// Unsigned four-byte word.
    Unsigned(u32),
    /// Raw eight-byte double pattern.
    DoubleBits(u64),
    /// Counted Cosworks text, including null and allocated empty distinctions.
    Text(SimulationPersistText),
    /// Complete counted opaque reference blob.
    Bytes(Arc<[u8]>),
    /// Complete typed object archive with its own version and object map.
    Embedded(Box<SimulationEmbeddedArchive>),
}

/// Owned field parts in physical order; paths describe the selected stored grammar.
#[derive(Debug, Clone)]
pub struct SimulationThermalField {
    /// Parser-derived structural role.
    pub path: Arc<str>,
    /// Complete stored value.
    pub value: SimulationThermalValue,
}

impl PartialEq for SimulationThermalValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Signed(a), Self::Signed(b)) => a == b,
            (Self::Unsigned(a), Self::Unsigned(b)) => a == b,
            (Self::DoubleBits(a), Self::DoubleBits(b)) => a == b,
            (Self::Bytes(a), Self::Bytes(b)) => a == b,
            (Self::Text(a), Self::Text(b)) => {
                a.header() == b.header() && a.payload() == b.payload()
            }
            (Self::Embedded(a), Self::Embedded(b)) => a.fields() == b.fields(),
            _ => false,
        }
    }
}
impl Eq for SimulationThermalValue {}
impl PartialEq for SimulationThermalField {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.value == other.value
    }
}
impl Eq for SimulationThermalField {}
