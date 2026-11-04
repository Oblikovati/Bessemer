use super::admission::{intersection_candidate, reference_error};
use super::projection::project_intersection_reference;
use super::{ModelIntersectionReferenceStoredEdit, ModelIntersectionReferenceStoredState};
use crate::Result;
use crate::model_archives::{
    ModelField, ModelFieldHandle, ModelFieldValue, PartResolvedFeaturesArchive,
    PartResolvedFeaturesEdit,
};

fn edited_reference_fields(
    source: &[ModelField],
    reference: &ModelIntersectionReferenceStoredState,
    edit: ModelIntersectionReferenceStoredEdit,
) -> Result<Vec<ModelField>> {
    let mut fields = source.to_vec();
    for (handle, value) in reference.source_fields[2..]
        .iter()
        .zip(reference.replacements(edit))
    {
        let Some(field) = fields.get_mut(handle.index) else {
            return Err(reference_error(&reference.path, handle.index));
        };
        field.value = ModelFieldValue::I32(value);
    }
    Ok(fields)
}

impl PartResolvedFeaturesArchive {
    /// Replaces the stored selector while preserving signed siblings and references; e.g. `source.edit_intersection_reference_classification(&reference, 3)?;`.
    pub fn edit_intersection_reference_classification(
        &self,
        reference: &ModelIntersectionReferenceStoredState,
        selector: u16,
    ) -> Result<PartResolvedFeaturesEdit> {
        let leading = &reference.source_fields[0];
        self.resolve_field(leading)?;
        let mut fields = self.fields().to_vec();
        let field = fields
            .get_mut(leading.index())
            .ok_or_else(|| reference_error(&reference.path, leading.index()))?;
        field.value = ModelFieldValue::U16(selector);
        self.edit_fields(fields)
    }

    /// Enumerates physical ordinary sketch references in wire order; e.g. `let references = source.intersection_reference_states()?;`.
    /// A complete archive without records returns an empty vector.
    pub fn intersection_reference_states(
        &self,
    ) -> Result<Vec<ModelIntersectionReferenceStoredState>> {
        self.fields()
            .iter()
            .enumerate()
            .filter(|(_, field)| intersection_candidate(&field.path))
            .map(|(index, _)| self.intersection_reference_state(&self.field_handle(index)?))
            .collect()
    }

    /// Binds a leading raw_00 field to its complete stored reference; e.g. `let reference = source.intersection_reference_state(&field)?;`.
    pub fn intersection_reference_state(
        &self,
        field: &ModelFieldHandle,
    ) -> Result<ModelIntersectionReferenceStoredState> {
        self.resolve_field(field)?;
        project_intersection_reference(self.fields(), field).map_err(|error| {
            crate::model_archives::resolved_failure::selected_feature_error(
                error,
                self.configuration(),
            )
        })
    }

    /// Edits stored span/offset words with complete archive validation and history; e.g. `source.edit_intersection_reference_state(&reference, bessemer::model_archives::ModelIntersectionReferenceStoredEdit::SpanCountState(-2))?`.
    pub fn edit_intersection_reference_state(
        &self,
        reference: &ModelIntersectionReferenceStoredState,
        edit: ModelIntersectionReferenceStoredEdit,
    ) -> Result<PartResolvedFeaturesEdit> {
        self.resolve_field(&reference.source_fields[0])?;
        let fields = edited_reference_fields(self.fields(), reference, edit)?;
        self.edit_fields(fields)
    }
}
