use super::super::{load_edit, load_snapshot::BinaryLoadSnapshot, thermal};
use super::*;
use crate::Result;
use std::sync::Arc;

/// Immutable isolated edit retaining complete source/replacement/result and every physical field relation.
#[derive(Debug, Clone)]
pub struct SimulationMaterialEdit {
    source: SimulationStudyMaterial,
    archive: SimulationStudyMaterial,
    parent: SimulationMaterialFieldId,
    result: SimulationMaterialFieldId,
    replacement: SimulationMaterialValue,
    coverage: SimulationMaterialCoverage,
    history: Arc<[SimulationMaterialHistory]>,
}

impl SimulationStudyMaterial {
    /// Replace one typed body value with exact unchanged surrounding bytes; e.g. `body.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationMaterialFieldId,
        replacement: SimulationMaterialValue,
    ) -> Result<SimulationMaterialEdit> {
        let original = self.field(id)?;
        load_edit::require_kind(&original.value, &replacement, &original.path)?;
        let archive = load_edit::replaced_field(self, id.ordinal(), &replacement, |fields| {
            Self::from_fields(fields, self.version, self.limits)
        })?;
        let changed = archive.revision != self.revision;
        let (history, _) = thermal::compose(self, &archive, id.ordinal(), &replacement, changed)?;
        let result = archive.field_id(id.ordinal())?;
        let coverage = edit_coverage(&archive, changed);
        Ok(SimulationMaterialEdit {
            source: self.clone(),
            archive,
            parent: id.clone(),
            result,
            replacement,
            coverage,
            history,
        })
    }
}

fn edit_coverage(archive: &SimulationStudyMaterial, changed: bool) -> SimulationMaterialCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationMaterialCheckMode::ExactBodyAndPreservation;
    coverage.completion = if changed {
        SimulationMaterialCompletion::Changed
    } else {
        SimulationMaterialCompletion::Unchanged
    };
    coverage
}

impl SimulationMaterialEdit {
    /// Borrow the complete immutable source body; e.g. `edit.source().to_bytes()`.
    pub fn source(&self) -> &SimulationStudyMaterial {
        &self.source
    }

    /// Borrow the complete validated result; e.g. `edit.archive().fields()`.
    pub fn archive(&self) -> &SimulationStudyMaterial {
        &self.archive
    }

    /// Retain the requested original occurrence, including no-ops; e.g. `edit.parent().ordinal()`.
    pub fn parent(&self) -> SimulationMaterialFieldId {
        self.parent.clone()
    }

    /// Retain the requested result occurrence independently; e.g. `edit.result().ordinal()`.
    pub fn result(&self) -> SimulationMaterialFieldId {
        self.result.clone()
    }

    /// Borrow the complete supplied replacement parent; e.g. `edit.replacement()`.
    pub fn replacement(&self) -> &SimulationMaterialValue {
        &self.replacement
    }

    /// Return completion, exact coverage, exclusions and achieved pattern error; e.g. `edit.coverage().completion`.
    pub fn coverage(&self) -> SimulationMaterialCoverage {
        self.coverage
    }

    /// Borrow every ordered source/replacement field relation; e.g. `edit.history()`.
    pub fn history(&self) -> &[SimulationMaterialHistory] {
        &self.history
    }
}

impl BinaryLoadSnapshot for SimulationStudyMaterial {
    fn fields(&self) -> &[SimulationMaterialField] {
        self.fields()
    }
    fn field_id(&self, ordinal: usize) -> Result<SimulationMaterialFieldId> {
        self.field_id(ordinal)
    }
    fn field_offsets(&self) -> &[(usize, usize)] {
        self.field_offsets()
    }
    fn wire_bytes(&self) -> &[u8] {
        &self.parent
    }
    fn subject(&self) -> &'static str {
        "study-manager Material"
    }
}
