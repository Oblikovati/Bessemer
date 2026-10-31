use super::super::load_identity;
use super::{
    SimulationSolidManagerField, SimulationSolidManagerFieldId, SimulationSolidManagerProperty,
};
use crate::Result;

impl SimulationSolidManagerProperty {
    /// Bind one physical field to this immutable revision; e.g. `body.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationSolidManagerFieldId> {
        load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "direct SolidManager Property",
        )
    }

    /// Resolve a field only on the owning revision; e.g. `body.field(&id)?`.
    pub fn field(
        &self,
        id: &SimulationSolidManagerFieldId,
    ) -> Result<&SimulationSolidManagerField> {
        load_identity::field(
            &self.fields,
            &self.revision,
            id,
            "direct SolidManager Property",
        )
    }
}
