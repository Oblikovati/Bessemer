use super::{
    ModelArchiveCoverage, ModelArchiveDiagnostic, ModelEditCompletion, ModelEditFidelity,
    ModelField, ModelFieldChange, PartConfigurationArchive, PartConfigurationEdit,
    context::selected_error,
};

impl PartConfigurationArchive {
    /// Validates an owned replacement under the source budgets and returns complete structural history.
    /// Derived section lengths are recalculated; failure leaves the source isolated.
    ///
    /// ```no_run
    /// # fn example(source: &bessemer::model_archives::PartConfigurationArchive) -> bessemer::Result<()> {
    /// let unchanged = source.edit_fields(source.fields().to_vec())?;
    /// assert!(unchanged.changes().is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub fn edit_fields(&self, fields: Vec<ModelField>) -> crate::Result<PartConfigurationEdit> {
        let parsed = self
            .parsed
            .with_fields(fields)
            .map_err(|error| selected_error(error, self.context.configuration))?;
        let mut archive = Self {
            context: self.context.clone(),
            parsed: std::sync::Arc::new(parsed),
            revision: crate::archive_codec::ArchiveRevision::default(),
        };
        let completion = completion(self, &mut archive);
        let coverage = archive.coverage();
        let (changes, unchanged_fields) = super::edit_history::history(self, &archive);
        Ok(PartConfigurationEdit {
            archive,
            completion,
            coverage,
            changes,
            unchanged_fields,
        })
    }
}

fn completion(
    source: &PartConfigurationArchive,
    result: &mut PartConfigurationArchive,
) -> ModelEditCompletion {
    if source.fields() == result.fields() {
        result.revision = source.revision.clone();
        return ModelEditCompletion::Unchanged;
    }
    ModelEditCompletion::Changed
}

impl PartConfigurationEdit {
    /// Borrows the fully validated immutable result.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartConfigurationEdit) -> bessemer::Result<()> {
    /// let bytes = edit.archive().to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn archive(&self) -> &PartConfigurationArchive {
        &self.archive
    }

    /// Distinguishes completed changed and unchanged edits from typed failures.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartConfigurationEdit) {
    /// let completion = edit.completion();
    /// # }
    /// ```
    pub fn completion(&self) -> ModelEditCompletion {
        self.completion
    }

    /// Returns exact whole-archive schema coverage.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartConfigurationEdit) {
    /// let bytes = edit.coverage().wire_bytes;
    /// # }
    /// ```
    pub fn coverage(&self) -> ModelArchiveCoverage {
        self.coverage
    }

    /// Reports achieved stored-field equality without assigning a geometric error bound.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartConfigurationEdit) {
    /// let fidelity = edit.fidelity();
    /// # }
    /// ```
    pub fn fidelity(&self) -> ModelEditFidelity {
        ModelEditFidelity::ExactStoredFields
    }

    /// Borrows the explicitly excluded geometry and external-document checks.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartConfigurationEdit) {
    /// let diagnostics = edit.diagnostics();
    /// # }
    /// ```
    pub fn diagnostics(&self) -> &[ModelArchiveDiagnostic] {
        self.archive.diagnostics()
    }

    /// Borrows changes in source path/occurrence order followed by created result occurrences.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartConfigurationEdit) {
    /// let history = edit.changes();
    /// # }
    /// ```
    pub fn changes(&self) -> &[ModelFieldChange] {
        &self.changes
    }

    /// Counts source field occurrences whose complete stored values stayed equal.
    ///
    /// ```no_run
    /// # fn example(edit: &bessemer::model_archives::PartConfigurationEdit) {
    /// let count = edit.unchanged_fields();
    /// # }
    /// ```
    pub fn unchanged_fields(&self) -> usize {
        self.unchanged_fields
    }
}
