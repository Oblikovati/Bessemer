use super::{ModelNodeStoredFlag, ModelNodeStoredState, projection::project_node};
use crate::Result;
use crate::model_archives::{
    ModelField, ModelFieldHandle, PartConfigurationArchive, PartConfigurationEdit,
    PartResolvedFeaturesArchive, PartResolvedFeaturesEdit,
};

fn replacement_fields(
    fields: &[ModelField],
    state: &ModelNodeStoredState,
    flag: ModelNodeStoredFlag,
    enabled: bool,
) -> Result<Vec<ModelField>> {
    let (handle, value) = state.replacement(flag, enabled)?;
    let mut replacement = fields.to_vec();
    let field = replacement.get_mut(handle.index).ok_or_else(|| {
        crate::BessemerError::new(
            "model node stored flag",
            "selected field",
            crate::FailureClass::Invalid,
            format!(
                "field index {}, length {}; expected an existing source word",
                handle.index,
                fields.len()
            ),
        )
    })?;
    field.value = value;
    Ok(replacement)
}

impl PartConfigurationArchive {
    /// Projects stored words from a revision-bound node state field in this complete archive.
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartConfigurationArchive, field: &bessemer::model_archives::ModelFieldHandle) -> bessemer::Result<()> {
    /// let node = source.node_state(field)?;
    /// assert_eq!(node.update_state() as u32 & 0xfffd3ef8, node.update_state_unresolved_bits());
    /// # Ok(())
    /// # }
    /// ```
    pub fn node_state(&self, field: &ModelFieldHandle) -> Result<ModelNodeStoredState> {
        self.resolve_field(field)?;
        project_node(self.fields(), field).map_err(|error| {
            crate::model_archives::context::selected_error(error, self.context.configuration)
        })
    }

    /// Changes only one stored bit through complete archive validation and existing structural history.
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartConfigurationArchive, node: &bessemer::model_archives::ModelNodeStoredState) -> bessemer::Result<()> {
    /// use bessemer::model_archives::ModelNodeStoredFlag;
    /// let edit = source.edit_node_flag(node, ModelNodeStoredFlag::NeedsUpdate, true)?;
    /// let complete_bytes = edit.archive().to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit_node_flag(
        &self,
        node: &ModelNodeStoredState,
        flag: ModelNodeStoredFlag,
        enabled: bool,
    ) -> Result<PartConfigurationEdit> {
        self.resolve_field(&node.source_fields[0])?;
        let fields = replacement_fields(self.fields(), node, flag, enabled).map_err(|error| {
            crate::model_archives::context::selected_error(error, self.context.configuration)
        })?;
        self.edit_fields(fields)
    }
}

impl PartResolvedFeaturesArchive {
    /// Projects a physically defined node, keeping inherited and alias-only occurrences separate.
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartResolvedFeaturesArchive, field: &bessemer::model_archives::ModelFieldHandle) -> bessemer::Result<()> {
    /// let node = source.node_state(field)?;
    /// let raw_flags = node.flags_28();
    /// # Ok(())
    /// # }
    /// ```
    pub fn node_state(&self, field: &ModelFieldHandle) -> Result<ModelNodeStoredState> {
        self.resolve_field(field)?;
        project_node(self.fields(), field).map_err(|error| {
            crate::model_archives::resolved_failure::selected_feature_error(
                error,
                self.configuration(),
            )
        })
    }

    /// Preserves every other field and bit, including native states that setters would normalize.
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartResolvedFeaturesArchive, node: &bessemer::model_archives::ModelNodeStoredState) -> bessemer::Result<()> {
    /// use bessemer::model_archives::ModelNodeStoredFlag;
    /// let edit = source.edit_node_flag(node, ModelNodeStoredFlag::UpdateWarning, false)?;
    /// let history = edit.changes();
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit_node_flag(
        &self,
        node: &ModelNodeStoredState,
        flag: ModelNodeStoredFlag,
        enabled: bool,
    ) -> Result<PartResolvedFeaturesEdit> {
        self.resolve_field(&node.source_fields[0])?;
        let fields = replacement_fields(self.fields(), node, flag, enabled).map_err(|error| {
            crate::model_archives::resolved_failure::selected_feature_error(
                error,
                self.configuration(),
            )
        })?;
        self.edit_fields(fields)
    }
}
