use super::*;
use crate::archive_codec::ArchiveString;
use crate::model_archives::{
    ModelArchiveCoverage, ModelEditCompletion, ModelEditFidelity, ModelFieldHandle,
};

impl ModelSplineEquationStoredState {
    /// Borrow the complete immutable source; e.g. `state.source().to_bytes()?`.
    pub fn source(&self) -> &PartResolvedFeaturesArchive {
        &self.source
    }

    /// Read an exact saved CString; e.g. `state.text(ModelSplineEquationTextRole::XExpression).text()`.
    pub fn text(&self, role: ModelSplineEquationTextRole) -> &ArchiveString {
        &self.strings[role.slot()]
    }

    /// Copy the complete32C word; e.g. `let raw = state.flags_word();`.
    pub fn flags_word(&self) -> u32 {
        self.flags
    }

    /// Inspect only stored mask0x10; e.g. `let requested = state.lock_start();`.
    pub fn lock_start(&self) -> bool {
        self.flags & 0x10 != 0
    }

    /// Inspect only stored mask0x20; e.g. `let requested = state.lock_end();`.
    pub fn lock_end(&self) -> bool {
        self.flags & 0x20 != 0
    }

    /// Copy the full signed option associated with IsAngleRange; e.g. `let raw = state.saved_option();`.
    pub fn saved_option(&self) -> i32 {
        self.option
    }

    /// Keep the separate compatibility word raw; e.g. `let raw = state.compatibility_word();`.
    pub fn compatibility_word(&self) -> i32 {
        self.compatibility
    }

    /// Borrow flags, six strings, compatibility and option identities in that order; e.g. `state.source_fields()[0].index()`.
    pub fn source_fields(&self) -> &[ModelFieldHandle; 9] {
        &self.fields
    }

    /// Borrow copied relations with all parents; e.g. `state.history()[0].parents.len()`.
    pub fn history(&self) -> &[ModelSplineEquationFieldCopy; 9] {
        &self.copies
    }

    /// Return unchanged completion for this read; e.g. `let status = state.completion();`.
    pub fn completion(&self) -> ModelEditCompletion {
        ModelEditCompletion::Unchanged
    }

    /// Return complete source-schema coverage; e.g. `let coverage = state.coverage();`.
    pub fn coverage(&self) -> ModelArchiveCoverage {
        self.source.coverage()
    }

    /// Report stored-field fidelity rather than geometric accuracy; e.g. `let fidelity = state.fidelity();`.
    pub fn fidelity(&self) -> ModelEditFidelity {
        ModelEditFidelity::ExactStoredFields
    }

    /// Return zero disagreement in projected stored values; e.g. `state.achieved_stored_value_error()`.
    pub fn achieved_stored_value_error(&self) -> usize {
        0
    }

    /// Borrow the declared semantic exclusions; e.g. `state.diagnostics().len()`.
    pub fn diagnostics(&self) -> &[ModelSplineEquationDiagnostic; 3] {
        &EQUATION_DIAGNOSTICS
    }
}

const EQUATION_DIAGNOSTICS: [ModelSplineEquationDiagnostic; 3] = [
    ModelSplineEquationDiagnostic::ExpressionsAndApiConversionsNotEvaluated,
    ModelSplineEquationDiagnostic::NativeSetterAndProvidersNotExecuted,
    ModelSplineEquationDiagnostic::GeometryUnitsAndExternalDocumentsNotChecked,
];
