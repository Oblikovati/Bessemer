use bessemer::model_archives::*;

// Retain the names because saved operation traces include their Debug representation.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ModelVisualRelation {
    pub(crate) path: String,
    pub(crate) relation: ModelFieldRelation,
    pub(crate) parents: Vec<ModelField>,
    pub(crate) result: Option<ModelField>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ModelVisualOperation {
    pub(crate) bytes: Vec<u8>,
    pub(crate) fields: Vec<ModelField>,
    pub(crate) completion: ModelEditCompletion,
    pub(crate) coverage: ModelArchiveCoverage,
    pub(crate) fidelity: ModelEditFidelity,
    pub(crate) diagnostics: Vec<ModelArchiveDiagnostic>,
    pub(crate) history: Vec<ModelVisualRelation>,
}

fn model_visual_relation(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
    change: &ModelFieldChange,
) -> ModelVisualRelation {
    ModelVisualRelation {
        path: change.path().to_owned(),
        relation: change.relation(),
        parents: change
            .parents()
            .iter()
            .map(|parent| source.resolve_field(parent).unwrap().clone())
            .collect(),
        result: change
            .result()
            .map(|field| result.archive().resolve_field(field).unwrap().clone()),
    }
}

/// Captures every operation field and resolved history; e.g. `model_visual_operation(&source, &edit)`.
pub(crate) fn model_visual_operation(
    source: &PartResolvedFeaturesArchive,
    result: &PartResolvedFeaturesEdit,
) -> ModelVisualOperation {
    ModelVisualOperation {
        bytes: result.archive().to_bytes().unwrap(),
        fields: result.archive().fields().to_vec(),
        completion: result.completion(),
        coverage: result.coverage(),
        fidelity: result.fidelity(),
        diagnostics: result.diagnostics().to_vec(),
        history: result
            .changes()
            .iter()
            .map(|change| model_visual_relation(source, result, change))
            .collect(),
    }
}
