use super::super::{load_edit, thermal};
use super::*;
use crate::Result;
use std::sync::Arc;

/// Immutable isolated edit retaining complete source/replacement/result and every physical field relation.
#[derive(Debug, Clone)]
pub struct SimulationSolidManagerEdit {
    source: SimulationSolidManagerProperty,
    archive: SimulationSolidManagerProperty,
    parent: SimulationSolidManagerFieldId,
    result: SimulationSolidManagerFieldId,
    replacement: SimulationSolidManagerValue,
    coverage: SimulationSolidManagerCoverage,
    history: Arc<[SimulationSolidManagerHistory]>,
}

impl SimulationSolidManagerProperty {
    /// Replace one typed body value with exact unchanged surrounding bytes; e.g. `body.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationSolidManagerFieldId,
        replacement: SimulationSolidManagerValue,
    ) -> Result<SimulationSolidManagerEdit> {
        let original = self.field(id)?;
        load_edit::require_kind(&original.value, &replacement, &original.path)?;
        let archive = load_edit::replaced_field(self, id.ordinal(), &replacement, |fields| {
            Self::from_fields(fields, self.version, self.limits)
        })?;
        let changed = archive.revision != self.revision;
        let (history, _) = thermal::compose(self, &archive, id.ordinal(), &replacement, changed)?;
        let result = archive.field_id(id.ordinal())?;
        let coverage = edit_coverage(&archive, changed);
        Ok(SimulationSolidManagerEdit {
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
    archive: &SimulationSolidManagerProperty,
    changed: bool,
) -> SimulationSolidManagerCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationSolidManagerCheckMode::ExactBodyAndPreservation;
    coverage.completion = if changed {
        SimulationSolidManagerCompletion::Changed
    } else {
        SimulationSolidManagerCompletion::Unchanged
    };
    coverage
}

impl SimulationSolidManagerEdit {
    /// Borrow the complete immutable source body; e.g. `edit.source().to_bytes()`.
    pub fn source(&self) -> &SimulationSolidManagerProperty {
        &self.source
    }

    /// Borrow the complete validated result; e.g. `edit.archive().fields()`.
    pub fn archive(&self) -> &SimulationSolidManagerProperty {
        &self.archive
    }

    /// Retain the requested original occurrence, including no-ops; e.g. `edit.parent().ordinal()`.
    pub fn parent(&self) -> SimulationSolidManagerFieldId {
        self.parent.clone()
    }

    /// Retain the requested result occurrence independently; e.g. `edit.result().ordinal()`.
    pub fn result(&self) -> SimulationSolidManagerFieldId {
        self.result.clone()
    }

    /// Borrow the complete supplied replacement parent; e.g. `edit.replacement()`.
    pub fn replacement(&self) -> &SimulationSolidManagerValue {
        &self.replacement
    }

    /// Return completion, exact coverage, exclusions and achieved pattern error; e.g. `edit.coverage().completion`.
    pub fn coverage(&self) -> SimulationSolidManagerCoverage {
        self.coverage
    }

    /// Borrow every ordered source/replacement field relation; e.g. `edit.history()`.
    pub fn history(&self) -> &[SimulationSolidManagerHistory] {
        &self.history
    }
}
