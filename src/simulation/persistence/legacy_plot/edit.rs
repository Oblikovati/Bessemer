use super::super::{load_edit, load_identity, thermal};
use super::*;
use crate::Result;
use std::sync::Arc;

/// Isolated complete edit retaining both immutable states, exact checks and ordered parents.
#[derive(Debug, Clone)]
pub struct SimulationLegacyPlotEdit {
    source: SimulationLegacyPlot,
    archive: SimulationLegacyPlot,
    parent: SimulationLegacyPlotFieldId,
    result: SimulationLegacyPlotFieldId,
    replacement: SimulationLegacyPlotValue,
    coverage: SimulationLegacyPlotCoverage,
    history: Arc<[SimulationLegacyPlotHistory]>,
}

impl SimulationLegacyPlot {
    /// Bind one occurrence to this immutable revision; e.g. `plot.field_id(0)?`.
    pub fn field_id(&self, ordinal: usize) -> Result<SimulationLegacyPlotFieldId> {
        load_identity::field_id(
            &self.revision,
            ordinal,
            self.fields.len(),
            "historical plot",
        )
    }

    /// Resolve only this revision's occurrence; e.g. `plot.field(&id)?`.
    pub fn field(&self, id: &SimulationLegacyPlotFieldId) -> Result<&SimulationLegacyPlotField> {
        load_identity::field(&self.fields, &self.revision, id, "historical plot")
    }

    /// Replace one whole value and prove unchanged surrounding bytes; e.g. `plot.edit_field(&id, value)?`.
    pub fn edit_field(
        &self,
        id: &SimulationLegacyPlotFieldId,
        replacement: SimulationLegacyPlotValue,
    ) -> Result<SimulationLegacyPlotEdit> {
        let field = self.field(id)?;
        values::require_kind(&field.value, &replacement, &field.path)?;
        let wire = values::wire_value(&replacement);
        let archive = load_edit::replaced_field(self, id.ordinal(), &wire, |fields| {
            Self::from_fields(
                values::typed_fields(&fields, self.limits)?,
                self.version,
                self.limits,
            )
        })?;
        SimulationLegacyPlotEdit::from_isolated(self, archive, id, replacement, &wire)
    }
}

fn edit_coverage(archive: &SimulationLegacyPlot, changed: bool) -> SimulationLegacyPlotCoverage {
    let mut coverage = archive.coverage();
    coverage.check_mode = SimulationLegacyPlotCheckMode::ExactSelectedProfilesAndPreservation;
    coverage.completion = if changed {
        SimulationLegacyPlotCompletion::Changed
    } else {
        SimulationLegacyPlotCompletion::Unchanged
    };
    coverage
}

impl SimulationLegacyPlotEdit {
    fn from_isolated(
        source: &SimulationLegacyPlot,
        archive: SimulationLegacyPlot,
        id: &SimulationLegacyPlotFieldId,
        replacement: SimulationLegacyPlotValue,
        wire: &thermal::SimulationThermalValue,
    ) -> Result<Self> {
        let changed = archive.revision != source.revision;
        let (history, _) = thermal::compose(source, &archive, id.ordinal(), wire, changed)?;
        let coverage = edit_coverage(&archive, changed);
        let result = archive.field_id(id.ordinal())?;
        Ok(Self {
            source: source.clone(),
            archive,
            parent: id.clone(),
            result,
            replacement,
            coverage,
            history,
        })
    }
}

impl SimulationLegacyPlotEdit {
    /// Borrow the complete immutable source; e.g. `edit.source().fields()`.
    pub fn source(&self) -> &SimulationLegacyPlot {
        &self.source
    }
    /// Borrow the complete validated result; e.g. `edit.archive().to_bytes()`.
    pub fn archive(&self) -> &SimulationLegacyPlot {
        &self.archive
    }
    /// Return the exact requested source occurrence; e.g. `edit.parent().ordinal()`.
    pub fn parent(&self) -> SimulationLegacyPlotFieldId {
        self.parent.clone()
    }
    /// Return the exact result occurrence; e.g. `edit.result().ordinal()`.
    pub fn result(&self) -> SimulationLegacyPlotFieldId {
        self.result.clone()
    }
    /// Retain the complete owned replacement; e.g. `edit.replacement()`.
    pub fn replacement(&self) -> &SimulationLegacyPlotValue {
        &self.replacement
    }
    /// Return completion, diagnostics, exact checks and achieved error; e.g. `edit.coverage()`.
    pub fn coverage(&self) -> SimulationLegacyPlotCoverage {
        self.coverage
    }
    /// Borrow all ordered source and replacement relations; e.g. `edit.history()`.
    pub fn history(&self) -> &[SimulationLegacyPlotHistory] {
        &self.history
    }
}
