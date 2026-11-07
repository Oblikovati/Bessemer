//! Query-to-storage composition preserves EOF independently of delegate validity.
use super::{
    ElectricalUserdataKey, ElectricalUserdataQuery, ElectricalUserdataReadBranch,
    ElectricalUserdataReadEffects, ElectricalUserdataState, ElectricalUserdataStateRead,
};
use crate::{BessemerError, FailureClass, Result};

impl ElectricalUserdataQuery {
    /// Projects matching query storage; e.g. `let read = query.project_storage(&prior, effects.as_ref())?`.
    pub fn project_storage(
        &self,
        prior: &ElectricalUserdataState,
        effects: Option<&ElectricalUserdataReadEffects>,
    ) -> Result<ElectricalUserdataStateRead> {
        require_query_identity(self.key(), prior)?;
        if !self.exists() {
            let mut unchanged = prior.project_recordset(None);
            unchanged.branch = ElectricalUserdataReadBranch::QueryEof;
            return Ok(unchanged);
        }
        let effects = effects.ok_or_else(|| BessemerError::new(
            "Electrical userdata query projection", "provider destination effects",
            FailureClass::Missing,
            format!("{} matching rows with absent effects; expected explicit converted destination effects for the selected row", self.rows().rows().len()),
        ))?;
        Ok(prior.project_recordset(Some(effects)))
    }
}

fn require_query_identity(
    key: ElectricalUserdataKey,
    prior: &ElectricalUserdataState,
) -> Result<()> {
    if key.object_id == prior.object_id && key.object_no == prior.object_no {
        return Ok(());
    }
    Err(BessemerError::new(
        "Electrical userdata query projection",
        "prior storage identity",
        FailureClass::Invalid,
        format!(
            "prior ({}, {}), query ({}, {}); expected identical signed object ID and object number",
            prior.object_id, prior.object_no, key.object_id, key.object_no
        ),
    ))
}
