use super::{
    ModelField, ModelFieldChange, ModelFieldHandle, ModelFieldRelation, PartConfigurationArchive,
};
use crate::{BessemerError, FailureClass, Result};

pub(super) fn history(
    source: &PartConfigurationArchive,
    result: &PartConfigurationArchive,
) -> (Vec<ModelFieldChange>, usize) {
    let (relations, unchanged) =
        crate::archive_codec::field_correspondences(source.fields(), result.fields(), |field| {
            field.path.as_ref()
        });
    let changes = relations
        .into_iter()
        .map(|relation| field_change(source, result, relation))
        .collect();
    (changes, unchanged)
}

fn field_change(
    source: &PartConfigurationArchive,
    result: &PartConfigurationArchive,
    relation: crate::archive_codec::FieldCorrespondence,
) -> ModelFieldChange {
    use crate::archive_codec::FieldCorrespondence::{Created, Deleted, Derived};
    let (kind, before, after, path) = match relation {
        Derived { before, after } => (
            ModelFieldRelation::Derived,
            Some(before),
            Some(after),
            source.fields()[before].path.clone(),
        ),
        Deleted { before } => (
            ModelFieldRelation::Deleted,
            Some(before),
            None,
            source.fields()[before].path.clone(),
        ),
        Created { after } => (
            ModelFieldRelation::Created,
            None,
            Some(after),
            result.fields()[after].path.clone(),
        ),
    };
    ModelFieldChange {
        path,
        relation: kind,
        parents: before
            .into_iter()
            .map(|index| field_handle(source, index))
            .collect(),
        result: after.map(|index| field_handle(result, index)),
    }
}

fn field_handle(archive: &PartConfigurationArchive, index: usize) -> ModelFieldHandle {
    ModelFieldHandle {
        revision: archive.revision.clone(),
        index,
    }
}

impl PartConfigurationArchive {
    /// Binds a physical field position to this snapshot's revision.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartConfigurationArchive) -> bessemer::Result<()> {
    /// let handle = archive.field_handle(0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn field_handle(&self, index: usize) -> Result<ModelFieldHandle> {
        self.fields()
            .get(index)
            .ok_or_else(|| self.field_error(index, "expected an existing physical field"))?;
        Ok(field_handle(self, index))
    }

    /// Resolves a field only in its owning revision; clones and equivalent edits share handles.
    ///
    /// ```no_run
    /// # fn example(archive: &bessemer::model_archives::PartConfigurationArchive) -> bessemer::Result<()> {
    /// let field = archive.resolve_field(&archive.field_handle(0)?)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn resolve_field(&self, handle: &ModelFieldHandle) -> Result<&ModelField> {
        if handle.revision != self.revision {
            return Err(self.field_error(
                handle.index,
                "handle belongs to another revision; expected this snapshot",
            ));
        }
        self.fields()
            .get(handle.index)
            .ok_or_else(|| self.field_error(handle.index, "expected an existing physical field"))
    }

    fn field_error(&self, index: usize, cause: &str) -> BessemerError {
        BessemerError::new(
            "model field lookup",
            &self.context.configuration.stream_name(),
            FailureClass::Invalid,
            format!(
                "field index {index}, length {}: {cause}",
                self.fields().len()
            ),
        )
    }
}

impl ModelFieldHandle {
    /// Returns the physical index without discarding the handle's revision binding.
    ///
    /// ```no_run
    /// # fn example(handle: &bessemer::model_archives::ModelFieldHandle) {
    /// let position = handle.index();
    /// # }
    /// ```
    pub fn index(&self) -> usize {
        self.index
    }
}
