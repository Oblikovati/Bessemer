use super::super::load_identity;
use super::{SimulationMaterialField, SimulationMaterialFieldId, SimulationStudyMaterial};
use crate::Result;

impl SimulationStudyMaterial {
    /// Bind one physical field to this immutable revision; e.g. `body.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationMaterialFieldId> {
        load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "study-manager Material",
        )
    }

    /// Resolve a field only on the owning revision; e.g. `body.field(&id)?`.
    pub fn field(&self, id: &SimulationMaterialFieldId) -> Result<&SimulationMaterialField> {
        load_identity::field(&self.fields, &self.revision, id, "study-manager Material")
    }
}
