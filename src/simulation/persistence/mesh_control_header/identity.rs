use super::super::load_identity;
use super::{
    SimulationMeshControlHeader, SimulationMeshControlHeaderField,
    SimulationMeshControlHeaderFieldId,
};
use crate::Result;

impl SimulationMeshControlHeader {
    /// Bind one physical field to this immutable revision; e.g. `body.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationMeshControlHeaderFieldId> {
        load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "direct MeshControlManager Property",
        )
    }

    /// Resolve a field only on the owning revision; e.g. `body.field(&id)?`.
    pub fn field(
        &self,
        id: &SimulationMeshControlHeaderFieldId,
    ) -> Result<&SimulationMeshControlHeaderField> {
        load_identity::field(
            &self.fields,
            &self.revision,
            id,
            "direct MeshControlManager Property",
        )
    }
}
