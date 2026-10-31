use super::super::load_identity;
use super::{
    SimulationShellManagerField, SimulationShellManagerFieldId, SimulationShellManagerProperty,
};
use crate::Result;

impl SimulationShellManagerProperty {
    /// Bind one physical field to this immutable revision; e.g. `body.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationShellManagerFieldId> {
        load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "direct ShellManager Property",
        )
    }

    /// Resolve a field only on the owning revision; e.g. `body.field(&id)?`.
    pub fn field(
        &self,
        id: &SimulationShellManagerFieldId,
    ) -> Result<&SimulationShellManagerField> {
        load_identity::field(
            &self.fields,
            &self.revision,
            id,
            "direct ShellManager Property",
        )
    }
}
