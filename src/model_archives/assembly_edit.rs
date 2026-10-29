use super::{AssemblyConfigurationArchive, ModelArchiveCoverage, ModelArchiveDiagnostic};
use super::{ModelEditCompletion, ModelEditFidelity, ModelField, ModelFieldChange};
use crate::Result;

/// Completed isolated assembly edit with stored-schema coverage and complete field history.
#[derive(Debug, Clone)]
pub struct AssemblyConfigurationEdit {
    archive: AssemblyConfigurationArchive,
    completion: ModelEditCompletion,
    changes: Vec<ModelFieldChange>,
    unchanged_fields: usize,
}

impl AssemblyConfigurationArchive {
    /// Validate replacement fields without mutating the source, e.g. `archive.edit_fields(fields)?`.
    pub fn edit_fields(&self, fields: Vec<ModelField>) -> Result<AssemblyConfigurationEdit> {
        let mut archive = Self::from_fields(fields, self.context.clone(), self.limits)?;
        let completion = completion(self, &mut archive);
        let (changes, unchanged_fields) = super::edit_history::field_history(
            self.fields(),
            &self.revision,
            archive.fields(),
            &archive.revision,
        );
        Ok(AssemblyConfigurationEdit {
            archive,
            completion,
            changes,
            unchanged_fields,
        })
    }
}

fn completion(
    source: &AssemblyConfigurationArchive,
    result: &mut AssemblyConfigurationArchive,
) -> ModelEditCompletion {
    if source.fields() == result.fields() {
        result.revision = source.revision.clone();
        return ModelEditCompletion::Unchanged;
    }
    ModelEditCompletion::Changed
}

impl AssemblyConfigurationEdit {
    /// Borrow the validated result, e.g. `edit.archive().to_bytes()?`.
    pub fn archive(&self) -> &AssemblyConfigurationArchive {
        &self.archive
    }

    /// Distinguish completed unchanged and changed results, e.g. `edit.completion()`.
    pub fn completion(&self) -> ModelEditCompletion {
        self.completion
    }

    /// Return exact whole-archive checks, e.g. `edit.coverage().wire_bytes`.
    pub fn coverage(&self) -> ModelArchiveCoverage {
        self.archive.coverage()
    }

    /// Report stored-field fidelity, e.g. `edit.fidelity()`.
    pub fn fidelity(&self) -> ModelEditFidelity {
        ModelEditFidelity::ExactStoredFields
    }

    /// Borrow checks outside the operation, e.g. `edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[ModelArchiveDiagnostic] {
        self.archive.diagnostics()
    }

    /// Borrow complete field correspondence, e.g. `edit.changes()`.
    pub fn changes(&self) -> &[ModelFieldChange] {
        &self.changes
    }

    /// Count unchanged source occurrences, e.g. `edit.unchanged_fields()`.
    pub fn unchanged_fields(&self) -> usize {
        self.unchanged_fields
    }
}
