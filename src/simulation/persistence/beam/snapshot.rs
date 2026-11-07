use super::super::{load_snapshot::BinaryLoadSnapshot, thermal::SimulationThermalField};
use super::{SimulationBeamFieldId, SimulationBeamProperty};
use crate::Result;

impl BinaryLoadSnapshot for SimulationBeamProperty {
    fn fields(&self) -> &[SimulationThermalField] {
        &self.wire_fields
    }
    fn field_id(&self, ordinal: usize) -> Result<SimulationBeamFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        &self.offsets
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "beam child Property"
    }
}
