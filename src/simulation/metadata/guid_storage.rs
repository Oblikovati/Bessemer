use super::{
    Result, SimulationGuidEdit, SimulationGuidFieldId, SimulationGuidStreamCompletion,
    SimulationMetadataArchive, require,
};
use crate::compound::CompoundEdit;
use crate::{BessemerError, FailureClass};

/// Complete existing four-string stream replacement, distinct from native stream creation.
#[derive(Debug, Clone)]
pub struct SimulationMetadataGuidEdit {
    /// Immutable resulting complete carrier and cached metadata.
    pub archive: SimulationMetadataArchive,
    /// Exact stored-field replacement, all field derivations and semantic exclusions.
    pub guid_edit: SimulationGuidEdit,
    /// All-entry carrier derivation and logical-preservation coverage.
    pub carrier_edit: CompoundEdit,
}

impl SimulationMetadataArchive {
    /// Edit one complete existing four-string stream field, e.g. `metadata.edit_guid_field(&id, Some(&[65]))?`; missing streams and unknown tails are explicit failures.
    pub fn edit_guid_field(
        &self,
        id: &SimulationGuidFieldId,
        units: Option<&[u16]>,
    ) -> Result<SimulationMetadataGuidEdit> {
        let guids = self.publishable_guids()?;
        let guid_edit = guids.edit_field(id, units)?;
        let entry = self.carrier.entry_at_path("TimeStampGUIDs")?;
        let carrier_edit = self
            .carrier
            .edit_stream(entry.id(), &guid_edit.archive.to_bytes())?;
        let archive = self.revalidated(
            carrier_edit.file.clone(),
            self.scalars.clone(),
            Some(guid_edit.archive.clone()),
        )?;
        Ok(SimulationMetadataGuidEdit {
            archive,
            guid_edit,
            carrier_edit,
        })
    }
    fn publishable_guids(&self) -> Result<&super::SimulationGuidArchive> {
        let guids = self.guids.as_ref().ok_or_else(|| BessemerError::new("simulation metadata", "TimeStampGUIDs", FailureClass::Missing,
            "missing root stream; expected an existing complete four-string stream; structural creation is not implied"))?;
        require(
            guids.coverage().completion == SimulationGuidStreamCompletion::CompleteFourFields,
            "TimeStampGUIDs publication",
            format!(
                "{} unassigned tail bytes; expected complete four-field framing before existing-stream publication",
                guids.tail().len()
            ),
        )?;
        Ok(guids)
    }
}
