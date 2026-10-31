use super::super::load_identity;
use super::{SimulationSurfaceBody, SimulationSurfaceBodyField, SimulationSurfaceBodyFieldId};
use crate::Result;

impl SimulationSurfaceBody {
    /// Bind one physical field to this immutable revision; e.g. `body.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationSurfaceBodyFieldId> {
        load_identity::field_id(&self.revision, ordinal, self.fields.len(), "SurfaceBody")
    }

    /// Resolve a field only on the owning revision; e.g. `body.field(&id)?`.
    pub fn field(&self, id: &SimulationSurfaceBodyFieldId) -> Result<&SimulationSurfaceBodyField> {
        load_identity::field(&self.fields, &self.revision, id, "SurfaceBody")
    }
}
