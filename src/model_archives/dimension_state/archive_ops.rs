use super::{ModelDimensionStoredEdit, ModelDimensionStoredState, projection::project_dimension};
use crate::Result;
use crate::model_archives::{
    ModelField, ModelFieldHandle, PartConfigurationArchive, PartConfigurationEdit,
    PartResolvedFeaturesArchive, PartResolvedFeaturesEdit,
};

fn edited_dimension_fields(
    fields: &[ModelField],
    dimension: &ModelDimensionStoredState,
    edit: ModelDimensionStoredEdit,
) -> Result<Vec<ModelField>> {
    let (handle, value) = dimension.replacement(edit)?;
    Ok(fields
        .iter()
        .enumerate()
        .map(|(index, field)| ModelField {
            path: field.path.clone(),
            value: if index == handle.index {
                value.clone()
            } else {
                field.value.clone()
            },
        })
        .collect())
}

impl PartConfigurationArchive {
    /// Projects stored dimension roles from a revision-bound mask field in this complete archive.
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartConfigurationArchive, field: &bessemer::model_archives::ModelFieldHandle) -> bessemer::Result<()> {
    /// let dimension = source.dimension_state(field)?;
    /// let raw = dimension.mask();
    /// # Ok(())
    /// # }
    /// ```
    pub fn dimension_state(&self, field: &ModelFieldHandle) -> Result<ModelDimensionStoredState> {
        self.resolve_field(field)?;
        project_dimension(self.fields(), field).map_err(|error| {
            crate::model_archives::context::selected_error(error, self.context.configuration)
        })
    }

    /// Changes one stored role through complete archive validation and ordered field history.
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartConfigurationArchive, dimension: &bessemer::model_archives::ModelDimensionStoredState) -> bessemer::Result<()> {
    /// use bessemer::model_archives::ModelDimensionStoredEdit;
    /// let edit = source.edit_dimension_state(dimension, ModelDimensionStoredEdit::RedundantId(-1))?;
    /// let bytes = edit.archive().to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit_dimension_state(
        &self,
        dimension: &ModelDimensionStoredState,
        edit: ModelDimensionStoredEdit,
    ) -> Result<PartConfigurationEdit> {
        self.resolve_field(&dimension.source_fields[0])?;
        let fields = edited_dimension_fields(self.fields(), dimension, edit).map_err(|error| {
            crate::model_archives::context::selected_error(error, self.context.configuration)
        })?;
        self.edit_fields(fields)
    }
}

impl PartResolvedFeaturesArchive {
    /// Projects direct or nested dimension-base words without resolving native dimension handles.
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartResolvedFeaturesArchive, field: &bessemer::model_archives::ModelFieldHandle) -> bessemer::Result<()> {
    /// let dimension = source.dimension_state(field)?;
    /// let identifier = dimension.redundant_id();
    /// # Ok(())
    /// # }
    /// ```
    pub fn dimension_state(&self, field: &ModelFieldHandle) -> Result<ModelDimensionStoredState> {
        self.resolve_field(field)?;
        project_dimension(self.fields(), field).map_err(|error| {
            crate::model_archives::resolved_failure::selected_feature_error(
                error,
                self.configuration(),
            )
        })
    }

    /// Preserves every unselected field and bit, including counters that native setters would change.
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartResolvedFeaturesArchive, dimension: &bessemer::model_archives::ModelDimensionStoredState) -> bessemer::Result<()> {
    /// use bessemer::model_archives::ModelDimensionStoredEdit;
    /// let edit = source.edit_dimension_state(dimension, ModelDimensionStoredEdit::FilletRefPointChangeNumber(0))?;
    /// let history = edit.changes();
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit_dimension_state(
        &self,
        dimension: &ModelDimensionStoredState,
        edit: ModelDimensionStoredEdit,
    ) -> Result<PartResolvedFeaturesEdit> {
        self.resolve_field(&dimension.source_fields[0])?;
        let fields = edited_dimension_fields(self.fields(), dimension, edit).map_err(|error| {
            crate::model_archives::resolved_failure::selected_feature_error(
                error,
                self.configuration(),
            )
        })?;
        self.edit_fields(fields)
    }
}
