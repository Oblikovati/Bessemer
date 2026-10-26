use super::{
    ConfigurationId, ModelField, ModelFieldChange, ModelFieldHandle, ModelFieldRelation,
    PartConfigurationArchive,
};
use crate::archive_codec::ArchiveRevision;
use crate::{BessemerError, FailureClass, Result};

pub(super) fn history(
    source: &PartConfigurationArchive,
    result: &PartConfigurationArchive,
) -> (Vec<ModelFieldChange>, usize) {
    field_history(
        source.fields(),
        &source.revision,
        result.fields(),
        &result.revision,
    )
}

pub(super) fn field_history(
    source: &[ModelField],
    source_revision: &ArchiveRevision,
    result: &[ModelField],
    result_revision: &ArchiveRevision,
) -> (Vec<ModelFieldChange>, usize) {
    let (relations, unchanged) =
        crate::archive_codec::field_correspondences(source, result, |field| field.path.as_ref());
    let changes = relations
        .into_iter()
        .map(|relation| field_change(source, source_revision, result, result_revision, relation))
        .collect();
    (changes, unchanged)
}

fn field_change(
    source: &[ModelField],
    source_revision: &ArchiveRevision,
    result: &[ModelField],
    result_revision: &ArchiveRevision,
    relation: crate::archive_codec::FieldCorrespondence,
) -> ModelFieldChange {
    use crate::archive_codec::FieldCorrespondence::{Created, Deleted, Derived};
    let (kind, before, after, path) = match relation {
        Derived { before, after } => (
            ModelFieldRelation::Derived,
            Some(before),
            Some(after),
            source[before].path.clone(),
        ),
        Deleted { before } => (
            ModelFieldRelation::Deleted,
            Some(before),
            None,
            source[before].path.clone(),
        ),
        Created { after } => (
            ModelFieldRelation::Created,
            None,
            Some(after),
            result[after].path.clone(),
        ),
    };
    ModelFieldChange {
        path,
        relation: kind,
        parents: before
            .into_iter()
            .map(|index| snapshot_handle(source_revision, index))
            .collect(),
        result: after.map(|index| snapshot_handle(result_revision, index)),
    }
}

fn snapshot_handle(revision: &ArchiveRevision, index: usize) -> ModelFieldHandle {
    ModelFieldHandle {
        revision: revision.clone(),
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
        checked_handle(
            self.fields(),
            &self.revision,
            self.context.configuration,
            index,
        )
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
        resolve_snapshot(
            self.fields(),
            &self.revision,
            self.context.configuration,
            handle,
        )
    }
}

pub(super) fn checked_handle(
    fields: &[ModelField],
    revision: &ArchiveRevision,
    configuration: ConfigurationId,
    index: usize,
) -> Result<ModelFieldHandle> {
    fields.get(index).ok_or_else(|| {
        field_error(
            configuration,
            fields.len(),
            index,
            "expected an existing physical field",
        )
    })?;
    Ok(snapshot_handle(revision, index))
}

pub(super) fn resolve_snapshot<'a>(
    fields: &'a [ModelField],
    revision: &ArchiveRevision,
    configuration: ConfigurationId,
    handle: &ModelFieldHandle,
) -> Result<&'a ModelField> {
    if &handle.revision != revision {
        return Err(field_error(
            configuration,
            fields.len(),
            handle.index,
            "handle belongs to another revision; expected this snapshot",
        ));
    }
    fields.get(handle.index).ok_or_else(|| {
        field_error(
            configuration,
            fields.len(),
            handle.index,
            "expected an existing physical field",
        )
    })
}

fn field_error(
    configuration: ConfigurationId,
    length: usize,
    index: usize,
    cause: &str,
) -> BessemerError {
    BessemerError::new(
        "model field lookup",
        &configuration.stream_name(),
        FailureClass::Invalid,
        format!("field index {index}, length {length}: {cause}"),
    )
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
