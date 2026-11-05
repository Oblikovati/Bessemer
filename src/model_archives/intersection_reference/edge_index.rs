use super::admission::{edge_index_scope, require_reference_owner};
use crate::model_archives::{
    ModelField, ModelFieldHandle, ModelFieldValue, PartResolvedFeaturesArchive,
    PartResolvedFeaturesEdit,
};
use crate::{BessemerError, FailureClass, Result};

#[cfg(test)]
#[path = "../../../tests/model_intersection_reference/edge_index_unit.rs"]
mod tests;

/// A signed saved handle-array index owned by one complete ordinary model revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelIntersectionEdgeStoredIndex {
    value: i32,
    field: ModelFieldHandle,
}

impl ModelIntersectionEdgeStoredIndex {
    /// Copies the saved index without mapping or normalization; e.g. `let index = edge.value();`.
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Borrows this physical field's revision identity; e.g. `source.resolve_field(edge.source_field())?`.
    pub fn source_field(&self) -> &ModelFieldHandle {
        &self.field
    }
}

fn index_error(path: &str, actual: impl std::fmt::Debug) -> BessemerError {
    BessemerError::new(
        "model intersection edge stored index",
        path,
        FailureClass::Invalid,
        format!(
            "{actual:?} at {path:?}; expected an ordinary sgSketch marker0 edge with canonical in-range indices and adjacent same-scope I32 raw_158/raw_15c/raw_160/raw_164/raw_168"
        ),
    )
}

fn project_edge_index(
    fields: &[ModelField],
    field: &ModelFieldHandle,
) -> Result<ModelIntersectionEdgeStoredIndex> {
    let selected = fields
        .get(field.index())
        .ok_or_else(|| index_error("intersection edge", field.index()))?;
    let scope =
        edge_index_scope(&selected.path).map_err(|error| index_error(&selected.path, error))?;
    require_reference_owner(&fields[..field.index()], &scope)
        .map_err(|error| index_error(&selected.path, error))?;
    let start = field
        .index()
        .checked_sub(4)
        .ok_or_else(|| index_error(&selected.path, field.index()))?;
    validate_edge_index_header(&fields[start..=field.index()], scope.group)?;
    let ModelFieldValue::I32(value) = selected.value else {
        return Err(index_error(&selected.path, &selected.value));
    };
    Ok(ModelIntersectionEdgeStoredIndex {
        value,
        field: field.clone(),
    })
}

fn validate_edge_index_header(fields: &[ModelField], prefix: &str) -> Result<()> {
    for (field, suffix) in fields
        .iter()
        .zip([".raw_158", ".raw_15c", ".raw_160", ".raw_164", ".raw_168"])
    {
        if field.path.strip_prefix(prefix) != Some(suffix)
            || !matches!(field.value, ModelFieldValue::I32(_))
        {
            return Err(index_error(prefix, field));
        }
    }
    Ok(())
}

fn index_candidate(path: &str) -> bool {
    path.contains(".styled_intersections.intersections[")
        && path.contains(".edges_48[")
        && path.ends_with(".raw_168")
}

impl PartResolvedFeaturesArchive {
    /// Enumerates physical saved edge indices in wire order; e.g. `let indices = source.intersection_edge_indices()?;`.
    pub fn intersection_edge_indices(&self) -> Result<Vec<ModelIntersectionEdgeStoredIndex>> {
        self.fields()
            .iter()
            .enumerate()
            .filter(|(_, field)| index_candidate(&field.path))
            .map(|(index, _)| self.intersection_edge_index(&self.field_handle(index)?))
            .collect()
    }

    /// Binds a physical raw168 field to its signed saved role; e.g. `let index = source.intersection_edge_index(&field)?;`.
    pub fn intersection_edge_index(
        &self,
        field: &ModelFieldHandle,
    ) -> Result<ModelIntersectionEdgeStoredIndex> {
        self.resolve_field(field)?;
        project_edge_index(self.fields(), field).map_err(|error| {
            crate::model_archives::resolved_failure::selected_feature_error(
                error,
                self.configuration(),
            )
        })
    }

    /// Replaces only saved raw168 without creating correspondence maps; e.g. `source.edit_intersection_edge_index(&index, -2)?;`.
    pub fn edit_intersection_edge_index(
        &self,
        index: &ModelIntersectionEdgeStoredIndex,
        value: i32,
    ) -> Result<PartResolvedFeaturesEdit> {
        self.resolve_field(index.source_field())?;
        let mut fields = self.fields().to_vec();
        let field = fields
            .get_mut(index.field.index())
            .ok_or_else(|| index_error("intersection edge", index.field.index()))?;
        field.value = ModelFieldValue::I32(value);
        self.edit_fields(fields)
    }
}
