use super::super::{load_edit, thermal};
use super::*;
use crate::Result;
use std::sync::Arc;

/// Immutable isolated edit retaining complete source/replacement/result and every physical field relation.
#[derive(Debug, Clone)]
pub struct SimulationShellManagerEdit {
    source: SimulationShellManagerProperty,
    archive: SimulationShellManagerProperty,
    parent: SimulationShellManagerFieldId,
    result: SimulationShellManagerFieldId,
    replacement: SimulationShellManagerValue,
    coverage: SimulationShellManagerCoverage,
    history: Arc<[SimulationShellManagerHistory]>,
}

impl SimulationShellManagerProperty {
    /// Replace one typed body value with exact unchanged surrounding bytes; e.g. `body.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationShellManagerFieldId,
        replacement: SimulationShellManagerValue,
    ) -> Result<SimulationShellManagerEdit> {
        let original = self.field(id)?;
        load_edit::require_kind(&original.value, &replacement, &original.path)?;
        let archive = load_edit::replaced_field(self, id.ordinal(), &replacement, |fields| {
            Self::from_fields(fields, self.version, self.profile, self.limits)
        })?;
        let changed = archive.revision != self.revision;
        let (history, _) = thermal::compose(self, &archive, id.ordinal(), &replacement, changed)?;
        let result = archive.field_id(id.ordinal())?;
        let coverage = edit_coverage(&archive, changed);
        Ok(SimulationShellManagerEdit {
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

fn edit_coverage(
    archive: &SimulationShellManagerProperty,
    changed: bool,
) -> SimulationShellManagerCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationShellManagerCheckMode::ExactBodyAndPreservation;
    coverage.completion = if changed {
        SimulationShellManagerCompletion::Changed
    } else {
        SimulationShellManagerCompletion::Unchanged
    };
    coverage
}

impl SimulationShellManagerEdit {
    /// Borrow the complete immutable source body; e.g. `edit.source().to_bytes()`.
    pub fn source(&self) -> &SimulationShellManagerProperty {
        &self.source
    }

    /// Borrow the complete validated result; e.g. `edit.archive().fields()`.
    pub fn archive(&self) -> &SimulationShellManagerProperty {
        &self.archive
    }

    /// Retain the requested original occurrence, including no-ops; e.g. `edit.parent().ordinal()`.
    pub fn parent(&self) -> SimulationShellManagerFieldId {
        self.parent.clone()
    }

    /// Retain the requested result occurrence independently; e.g. `edit.result().ordinal()`.
    pub fn result(&self) -> SimulationShellManagerFieldId {
        self.result.clone()
    }

    /// Borrow the complete supplied replacement parent; e.g. `edit.replacement()`.
    pub fn replacement(&self) -> &SimulationShellManagerValue {
        &self.replacement
    }

    /// Return completion, exact coverage, exclusions and achieved pattern error; e.g. `edit.coverage().completion`.
    pub fn coverage(&self) -> SimulationShellManagerCoverage {
        self.coverage
    }

    /// Borrow every ordered source/replacement field relation; e.g. `edit.history()`.
    pub fn history(&self) -> &[SimulationShellManagerHistory] {
        &self.history
    }
}
