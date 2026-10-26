use super::{
    ModelArchiveCoverage, ModelArchiveDiagnostic, ModelEditCompletion, ModelEditFidelity,
    ModelField, ModelFieldChange, ModelFieldHandle, PartResolvedFeaturesArchive, edit_history,
};
use crate::Result;

/// Completed isolated feature edit with exact coverage, fidelity and revision-bound field history.
#[derive(Debug, Clone)]
pub struct PartResolvedFeaturesEdit {
    archive: PartResolvedFeaturesArchive,
    completion: ModelEditCompletion,
    changes: Vec<ModelFieldChange>,
    unchanged_fields: usize,
}

impl PartResolvedFeaturesArchive {
    /// Validates complete replacement fields against the retained model/CMgr context.
    ///
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartResolvedFeaturesArchive) -> bessemer::Result<()> {
    /// let edit = source.edit_fields(source.fields().to_vec())?;
    /// assert!(edit.changes().is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit_fields(&self, fields: Vec<ModelField>) -> Result<PartResolvedFeaturesEdit> {
        let archive = self.with_fields(fields)?;
        let completion = if self.revision == archive.revision {
            ModelEditCompletion::Unchanged
        } else {
            ModelEditCompletion::Changed
        };
        let (changes, unchanged_fields) = edit_history::field_history(
            self.fields(),
            &self.revision,
            archive.fields(),
            &archive.revision,
        );
        Ok(PartResolvedFeaturesEdit {
            archive,
            completion,
            changes,
            unchanged_fields,
        })
    }

    /// Binds a physical field occurrence to this immutable revision.
    ///
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartResolvedFeaturesArchive) -> bessemer::Result<()> {
    /// let handle = source.field_handle(0)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn field_handle(&self, index: usize) -> Result<ModelFieldHandle> {
        edit_history::checked_handle(self.fields(), &self.revision, self.configuration(), index)
    }

    /// Resolves handles only in their owning revision; clones and unchanged edits share ownership.
    ///
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartResolvedFeaturesArchive) -> bessemer::Result<()> {
    /// let field = source.resolve_field(&source.field_handle(0)?)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn resolve_field(&self, handle: &ModelFieldHandle) -> Result<&ModelField> {
        edit_history::resolve_snapshot(self.fields(), &self.revision, self.configuration(), handle)
    }
}

impl PartResolvedFeaturesEdit {
    /// Borrows the validated immutable result.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartResolvedFeaturesEdit) -> bessemer::Result<()> {
    /// let bytes = edit.archive().to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn archive(&self) -> &PartResolvedFeaturesArchive {
        &self.archive
    }

    /// Distinguishes completed changed/unchanged edits from typed failures.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartResolvedFeaturesEdit) {
    /// let completion = edit.completion();
    /// # }
    /// ```
    pub fn completion(&self) -> ModelEditCompletion {
        self.completion
    }

    /// Returns exact whole-stream stored-schema coverage.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartResolvedFeaturesEdit) {
    /// let coverage = edit.coverage();
    /// # }
    /// ```
    pub fn coverage(&self) -> ModelArchiveCoverage {
        self.archive.coverage()
    }

    /// Reports exact stored fields without inventing a geometric error bound.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartResolvedFeaturesEdit) {
    /// let fidelity = edit.fidelity();
    /// # }
    /// ```
    pub fn fidelity(&self) -> ModelEditFidelity {
        ModelEditFidelity::ExactStoredFields
    }

    /// Borrows geometry and external-document exclusions.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartResolvedFeaturesEdit) {
    /// let diagnostics = edit.diagnostics();
    /// # }
    /// ```
    pub fn diagnostics(&self) -> &[ModelArchiveDiagnostic] {
        self.archive.diagnostics()
    }

    /// Borrows structural relations in deterministic source/result occurrence order.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartResolvedFeaturesEdit) {
    /// let changes = edit.changes();
    /// # }
    /// ```
    pub fn changes(&self) -> &[ModelFieldChange] {
        &self.changes
    }

    /// Counts source occurrences whose complete stored values stayed equal.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartResolvedFeaturesEdit) {
    /// let unchanged = edit.unchanged_fields();
    /// # }
    /// ```
    pub fn unchanged_fields(&self) -> usize {
        self.unchanged_fields
    }
}
