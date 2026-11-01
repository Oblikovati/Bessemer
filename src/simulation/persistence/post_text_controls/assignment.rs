use super::super::SimulationPostStyleText;
use super::{
    post_text_budget, post_text_completion, post_text_coverage, post_text_diagnostics,
    required_post_runtime_text, types::*,
};
use crate::Result;

/// Immutable known primary result with both assignment parents and exact local coverage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostTextAssignment {
    wire: SimulationPostStyleText,
    previous: Option<SimulationPostStyleText>,
    primary: SimulationPostStyleText,
    kind: SimulationPostTextAssignmentKind,
    coverage: SimulationPostTextCoverage,
}

impl SimulationPostTextControls {
    /// Resolve primary after an explicit load assignment; e.g. `controls.assign_primary(previous.as_ref(), 512)?`.
    pub fn assign_primary(
        &self,
        previous: Option<&SimulationPostStyleText>,
        maximum_code_units: usize,
    ) -> Result<SimulationPostTextAssignment> {
        let count = self.stored_text.character_count();
        post_text_budget("post text assignment", count, maximum_code_units)?;
        let (kind, primary) = assigned_post_primary(&self.stored_text, previous)?;
        let coverage = assigned_post_coverage(count, &primary, kind, maximum_code_units)?;
        Ok(SimulationPostTextAssignment {
            wire: self.stored_text.clone(),
            previous: previous.cloned(),
            primary,
            kind,
            coverage,
        })
    }
}

fn assigned_post_coverage(
    count: usize,
    primary: &SimulationPostStyleText,
    kind: SimulationPostTextAssignmentKind,
    maximum: usize,
) -> Result<SimulationPostTextCoverage> {
    post_text_budget("post text assignment", primary.character_count(), maximum)?;
    let required = usize::from(kind == SimulationPostTextAssignmentKind::PreservedPrevious);
    Ok(post_text_coverage(
        count,
        primary.character_count(),
        required,
        maximum,
    ))
}

fn assigned_post_primary(
    wire: &SimulationPostStyleText,
    previous: Option<&SimulationPostStyleText>,
) -> Result<(SimulationPostTextAssignmentKind, SimulationPostStyleText)> {
    if wire.character_count() == 0 {
        let known = required_post_runtime_text(
            previous,
            "post text assignment",
            "previous primary CString",
        )?;
        return Ok((
            SimulationPostTextAssignmentKind::PreservedPrevious,
            known.clone(),
        ));
    }
    let units = wire.payload().as_chunks::<2>().0;
    let stop = units
        .iter()
        .position(|unit| *unit == [0, 0])
        .unwrap_or(units.len());
    let primary =
        SimulationPostStyleText::from_payload(&wire.payload()[..2 * stop], wire.payload().len())?;
    Ok((SimulationPostTextAssignmentKind::AssignedPrefix, primary))
}

impl SimulationPostTextAssignment {
    /// Inspect complete counted bytes, including the suffix after NUL; e.g. `assignment.source_wire_text()`.
    pub fn source_wire_text(&self) -> &SimulationPostStyleText {
        &self.wire
    }
    /// Inspect the optional prior parent without inventing initialization; e.g. `assignment.previous_primary()`.
    pub fn previous_primary(&self) -> Option<&SimulationPostStyleText> {
        self.previous.as_ref()
    }
    /// Inspect the known resulting primary CString; e.g. `assignment.primary_after()`.
    pub fn primary_after(&self) -> &SimulationPostStyleText {
        &self.primary
    }
    /// Distinguish positive assignment from count0 preservation; e.g. `assignment.kind()`.
    pub fn kind(&self) -> SimulationPostTextAssignmentKind {
        self.kind
    }
    /// Inspect exact units and the supplied budget; e.g. `assignment.coverage()`.
    pub fn coverage(&self) -> SimulationPostTextCoverage {
        self.coverage
    }
    /// Distinguish completed empty text from completed nonempty text; e.g. `assignment.completion()`.
    pub fn completion(&self) -> SimulationPostTextCompletion {
        post_text_completion(self.primary.character_count())
    }
    /// Carry execution, Unicode and layout exclusions; e.g. `assignment.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationPostTextDiagnostic; 4] {
        post_text_diagnostics()
    }
}
