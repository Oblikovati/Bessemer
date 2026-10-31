use super::super::load_identity;
use super::{SimulationSolidChildField, SimulationSolidChildFieldId, SimulationSolidChildProperty};
use crate::Result;

impl SimulationSolidChildProperty {
    /// Bind one physical field to this immutable revision; e.g. `body.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationSolidChildFieldId> {
        load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "SolidManager child Property",
        )
    }

    /// Resolve a field only on the owning revision; e.g. `body.field(&id)?`.
    pub fn field(&self, id: &SimulationSolidChildFieldId) -> Result<&SimulationSolidChildField> {
        load_identity::field(
            &self.fields,
            &self.revision,
            id,
            "SolidManager child Property",
        )
    }
}
