use super::*;
use crate::model_archives::{
    ModelArchiveCoverage, ModelEditCompletion, ModelEditFidelity, ModelFieldHandle,
};

impl ModelResolvedRootGraph {
    /// Borrow the complete immutable source; e.g. `let source = graph.source();`.
    pub fn source(&self) -> &PartResolvedFeaturesArchive {
        &self.source
    }

    /// Borrow every stored root in wire order; e.g. `let roots = graph.roots();`.
    pub fn roots(&self) -> &[ModelResolvedRootOccurrence] {
        &self.roots
    }

    /// Resolve a same-revision root occurrence; e.g. `let root = graph.root(&handle)?;`.
    pub fn root(&self, handle: &ModelFieldHandle) -> Result<&ModelResolvedRootOccurrence> {
        let field = self.source.resolve_field(handle)?;
        self.roots
            .iter()
            .find(|root| root.field == *handle)
            .ok_or_else(|| {
                projection::invalid_root((&field.path, "selection is not a physical root"))
            })
    }

    /// Borrow copied physical provenance; e.g. `let copies = graph.history();`.
    pub fn history(&self) -> &[ModelResolvedRootCopy] {
        &self.copies
    }

    /// Report unchanged completion, including an empty list; e.g. `let completion = graph.completion();`.
    pub fn completion(&self) -> ModelEditCompletion {
        ModelEditCompletion::Unchanged
    }

    /// Return complete source schema coverage; e.g. `let coverage = graph.coverage();`.
    pub fn coverage(&self) -> ModelArchiveCoverage {
        self.source.coverage()
    }

    /// Declare exact stored-field fidelity; e.g. `let fidelity = graph.fidelity();`.
    pub fn fidelity(&self) -> ModelEditFidelity {
        ModelEditFidelity::ExactStoredFields
    }

    /// Return zero disagreement in copied values; e.g. `let error = graph.achieved_stored_value_error();`.
    pub fn achieved_stored_value_error(&self) -> usize {
        0
    }

    /// Borrow the semantic exclusions; e.g. `let diagnostics = graph.diagnostics();`.
    pub fn diagnostics(&self) -> &[ModelResolvedRootDiagnostic] {
        &[
            ModelResolvedRootDiagnostic::FeatureDependenciesNotInferred,
            ModelResolvedRootDiagnostic::GeometryAndApplicationNotChecked,
            ModelResolvedRootDiagnostic::ExternalDocumentsNotResolved,
        ]
    }
}
