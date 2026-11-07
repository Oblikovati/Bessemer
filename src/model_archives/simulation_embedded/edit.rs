use super::SimulationEmbeddedArchive;
use crate::model_archives::{
    ModelArchiveCoverage, ModelArchiveDiagnostic, ModelEditCompletion, ModelEditFidelity,
    ModelField, ModelFieldChange, ModelFieldHandle,
};
use crate::{BessemerError, FailureClass, Result};

/// Completed isolated embedded edit with stored-field history and exact validation coverage.
#[derive(Debug, Clone)]
pub struct SimulationEmbeddedEdit {
    archive: SimulationEmbeddedArchive,
    completion: ModelEditCompletion,
    changes: Vec<ModelFieldChange>,
    unchanged_fields: usize,
}

impl SimulationEmbeddedArchive {
    /// Bind a physical occurrence to this revision; e.g. `archive.field_handle(0)?`.
    pub fn field_handle(&self, index: usize) -> Result<ModelFieldHandle> {
        if index >= self.fields.len() {
            return Err(BessemerError::new(
                "Simulation embedded archive",
                "field handle",
                FailureClass::Invalid,
                format!("field index {index}; expected < {}", self.fields.len()),
            ));
        }
        Ok(ModelFieldHandle {
            revision: self.revision.clone(),
            index,
        })
    }

    /// Validate an owned replacement without mutating the source; e.g. `archive.edit_fields(archive.fields().to_vec())?`.
    pub fn edit_fields(&self, fields: Vec<ModelField>) -> Result<SimulationEmbeddedEdit> {
        let mut archive = Self::from_fields(fields, self.profile, self.limits)?;
        let completion = completion(self, &mut archive);
        let (changes, unchanged_fields) = crate::model_archives::edit_history::field_history(
            self.fields(),
            &self.revision,
            archive.fields(),
            &archive.revision,
        );
        Ok(SimulationEmbeddedEdit {
            archive,
            completion,
            changes,
            unchanged_fields,
        })
    }
}

fn completion(
    source: &SimulationEmbeddedArchive,
    result: &mut SimulationEmbeddedArchive,
) -> ModelEditCompletion {
    if source.fields == result.fields {
        result.revision = source.revision.clone();
        return ModelEditCompletion::Unchanged;
    }
    ModelEditCompletion::Changed
}

impl SimulationEmbeddedEdit {
    /// Borrow the fully validated immutable result; e.g. `edit.archive().to_bytes()?`.
    pub fn archive(&self) -> &SimulationEmbeddedArchive {
        &self.archive
    }

    /// Distinguish completed changed and unchanged results; e.g. `edit.completion()`.
    pub fn completion(&self) -> ModelEditCompletion {
        self.completion
    }

    /// Return whole-archive validation coverage; e.g. `edit.coverage().typed_fields`.
    pub fn coverage(&self) -> ModelArchiveCoverage {
        self.archive.coverage()
    }

    /// Return achieved bit-pattern fidelity without a geometry claim; e.g. `edit.fidelity()`.
    pub fn fidelity(&self) -> ModelEditFidelity {
        ModelEditFidelity::ExactStoredFields
    }

    /// Borrow explicitly excluded checks; e.g. `edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[ModelArchiveDiagnostic] {
        self.archive.diagnostics()
    }

    /// Borrow changes in physical source/result occurrence order; e.g. `edit.changes()`.
    pub fn changes(&self) -> &[ModelFieldChange] {
        &self.changes
    }

    /// Return unchanged occurrence count, including framing; e.g. `edit.unchanged_fields()`.
    pub fn unchanged_fields(&self) -> usize {
        self.unchanged_fields
    }
}
