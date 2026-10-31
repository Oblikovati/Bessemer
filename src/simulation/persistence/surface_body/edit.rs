use super::super::{load_edit, thermal};
use super::*;
use crate::Result;
use std::sync::Arc;

/// Immutable isolated edit retaining complete source/replacement/result and every physical field relation.
#[derive(Debug, Clone)]
pub struct SimulationSurfaceBodyEdit {
    source: SimulationSurfaceBody,
    archive: SimulationSurfaceBody,
    parent: SimulationSurfaceBodyFieldId,
    result: SimulationSurfaceBodyFieldId,
    replacement: SimulationSurfaceBodyValue,
    coverage: SimulationSurfaceBodyCoverage,
    history: Arc<[SimulationSurfaceBodyHistory]>,
}

impl SimulationSurfaceBody {
    /// Replace one typed body value with exact unchanged surrounding bytes; e.g. `body.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationSurfaceBodyFieldId,
        replacement: SimulationSurfaceBodyValue,
    ) -> Result<SimulationSurfaceBodyEdit> {
        let original = self.field(id)?;
        load_edit::require_kind(&original.value, &replacement, &original.path)?;
        let archive = load_edit::replaced_field(self, id.ordinal(), &replacement, |fields| {
            Self::from_fields(fields, self.version, self.limits)
        })?;
        let changed = archive.revision != self.revision;
        let (history, _) = thermal::compose(self, &archive, id.ordinal(), &replacement, changed)?;
        let result = archive.field_id(id.ordinal())?;
        let coverage = edit_coverage(&archive, changed);
        Ok(SimulationSurfaceBodyEdit {
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

fn edit_coverage(archive: &SimulationSurfaceBody, changed: bool) -> SimulationSurfaceBodyCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationSurfaceBodyCheckMode::ExactBodyAndPreservation;
    coverage.completion = if changed {
        SimulationSurfaceBodyCompletion::Changed
    } else {
        SimulationSurfaceBodyCompletion::Unchanged
    };
    coverage
}

impl SimulationSurfaceBodyEdit {
    /// Borrow the complete immutable source body; e.g. `edit.source().to_bytes()`.
    pub fn source(&self) -> &SimulationSurfaceBody {
        &self.source
    }

    /// Borrow the complete validated result; e.g. `edit.archive().fields()`.
    pub fn archive(&self) -> &SimulationSurfaceBody {
        &self.archive
    }

    /// Retain the requested original occurrence, including no-ops; e.g. `edit.parent().ordinal()`.
    pub fn parent(&self) -> SimulationSurfaceBodyFieldId {
        self.parent.clone()
    }

    /// Retain the requested result occurrence independently; e.g. `edit.result().ordinal()`.
    pub fn result(&self) -> SimulationSurfaceBodyFieldId {
        self.result.clone()
    }

    /// Borrow the complete supplied replacement parent; e.g. `edit.replacement()`.
    pub fn replacement(&self) -> &SimulationSurfaceBodyValue {
        &self.replacement
    }

    /// Return completion, exact coverage, exclusions and achieved pattern error; e.g. `edit.coverage().completion`.
    pub fn coverage(&self) -> SimulationSurfaceBodyCoverage {
        self.coverage
    }

    /// Borrow every ordered source/replacement field relation; e.g. `edit.history()`.
    pub fn history(&self) -> &[SimulationSurfaceBodyHistory] {
        &self.history
    }
}
