use super::*;
impl FlowTextSlots {
    /// Projects nonempty prefixes onto a copy of existing state under an output bound; e.g. `let projection = slots.project(&previous, 4096)?`.
    pub fn project(
        &self,
        previous: &FlowTextSlotState,
        maximum_units: usize,
    ) -> Result<FlowTextSlotProjection> {
        let units = projected_units(self, previous, maximum_units)?;
        let slots = std::array::from_fn(|ordinal| projected_slot(self, previous, ordinal));
        let history = std::array::from_fn(|ordinal| {
            if self.spans[ordinal].prefix == 0 {
                FlowTextSlotRelation::RetainedPrevious
            } else {
                FlowTextSlotRelation::AppliedNativePrefix
            }
        });
        Ok(FlowTextSlotProjection {
            source: self.clone(),
            previous: previous.clone(),
            result: FlowTextSlotState { slots, units },
            history,
            maximum_units,
        })
    }
}
fn projected_units(
    source: &FlowTextSlots,
    previous: &FlowTextSlotState,
    maximum: usize,
) -> Result<usize> {
    source
        .spans
        .iter()
        .enumerate()
        .try_fold(0, |total, (ordinal, span)| {
            let units = if span.prefix == 0 {
                previous.slots[ordinal].len()
            } else {
                span.prefix
            };
            admit_total(total, units, maximum, "projected text state")
        })
}
fn projected_slot(
    source: &FlowTextSlots,
    previous: &FlowTextSlotState,
    ordinal: usize,
) -> Arc<[u16]> {
    let span = source.spans[ordinal];
    if span.prefix == 0 {
        return Arc::clone(&previous.slots[ordinal]);
    }
    let raw = &source.wire.decoded[span.start + 4..span.start + 4 + span.prefix * 2];
    raw.as_chunks::<2>()
        .0
        .iter()
        .map(|unit| u16::from_le_bytes(*unit))
        .collect()
}
impl FlowTextSlotProjection {
    /// Borrows the validated source record checkpoint; e.g. `let id = projection.source().record()`.
    pub fn source(&self) -> &FlowTextSlots {
        &self.source
    }
    /// Borrows the complete previous state checkpoint; e.g. `let before = projection.previous().units(0)?`.
    pub fn previous(&self) -> &FlowTextSlotState {
        &self.previous
    }
    /// Borrows the immutable projected result; e.g. `let after = projection.result().units(0)?`.
    pub fn result(&self) -> &FlowTextSlotState {
        &self.result
    }
    /// Borrows physical-ordinal relations to the retained source and previous parents; e.g. `let relation = projection.history()[0]`.
    pub fn history(&self) -> &[FlowTextSlotRelation; SLOT_COUNT] {
        &self.history
    }
    /// Reports exact source framing and resulting state coverage; e.g. `let proof = projection.coverage()`.
    pub fn coverage(&self) -> FlowTextSlotProjectionCoverage {
        let retained = self
            .history
            .iter()
            .filter(|relation| **relation == FlowTextSlotRelation::RetainedPrevious)
            .count();
        FlowTextSlotProjectionCoverage {
            source: self.source.coverage(),
            previous_units: self.previous.units,
            result_units: self.result.units,
            replaced_slots: SLOT_COUNT - retained,
            retained_slots: retained,
            maximum_result_units: self.maximum_units,
        }
    }
    /// Borrows source mode/runtime and application exclusions; e.g. `let exclusions = projection.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowTextSlotDiagnostic] {
        self.source.diagnostics()
    }
}
