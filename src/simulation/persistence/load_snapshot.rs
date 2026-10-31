use super::thermal::{SimulationThermalField, SimulationThermalFieldId};
use crate::Result;

pub(super) trait BinaryLoadSnapshot: Clone {
    fn fields(&self) -> &[SimulationThermalField];
    fn field_id(&self, ordinal: usize) -> Result<SimulationThermalFieldId>;
    fn field_offsets(&self) -> &[(usize, usize)];
    fn wire_bytes(&self) -> &[u8];
    fn subject(&self) -> &'static str;
}
