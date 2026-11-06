use super::*;
impl FlowSelectedValues {
    /// Borrows the immutable value carrier; e.g. `let raw = values.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the revision-bound value occurrence; e.g. `let id = values.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows retained immutable input revisions and selection; e.g. `let selected = values.context().selected_ordinals()`.
    pub fn context(&self) -> &FlowSelectionContext {
        &self.context
    }
    /// Returns the original raw-transfer bounds; e.g. `let bounds = values.options()`.
    pub fn options(&self) -> FlowSelectedValueOptions {
        self.options
    }
    /// Borrows the complete headerless raw payload; e.g. `let raw = values.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Borrows every dense raw pattern; e.g. `let count = values.values().word_count()`.
    pub fn values(&self) -> FlowTableWideArray<'_> {
        FlowTableWideArray {
            bytes: &self.wire.decoded,
            start: 0,
        }
    }
    /// Creates a checked context/revision-bound identity; e.g. `let id = values.value_id(0)?`.
    pub fn value_id(&self, index: usize) -> Result<FlowSelectedValueId> {
        wire::element_offset(112, 0, self.context.selected_count(), index, 8)?;
        Ok(FlowSelectedValueId {
            record: self.wire.record.clone(),
            links: self.context.links.record().clone(),
            states: self.context.states.record().clone(),
            transport: self.context.links.options().compression,
            assumption: self.context.options.assumption,
            index,
        })
    }
    /// Returns a checked entry's value identity or absence; e.g. `let id = values.entry_value_id(0)?`.
    pub fn entry_value_id(&self, ordinal: usize) -> Result<Option<FlowSelectedValueId>> {
        self.context
            .dense_index(ordinal)?
            .map(|index| self.value_id(index))
            .transpose()
    }
    /// Reads a revision/context-bound value; e.g. `let bits = values.value(&id)?`.
    pub fn value(&self, id: &FlowSelectedValueId) -> Result<u64> {
        let index = self.checked_index(id)?;
        Ok(wire::wide_at(&self.wire.decoded, index * 8))
    }
    /// Projects a physical input entry to its raw value or absence; e.g. `let bits = values.entry_value(0)?`.
    pub fn entry_value(&self, ordinal: usize) -> Result<Option<u64>> {
        self.entry_value_id(ordinal)?
            .map(|id| self.value(&id))
            .transpose()
    }
    pub(super) fn checked_index(&self, id: &FlowSelectedValueId) -> Result<usize> {
        let expected = self.value_id(id.index)?;
        if *id == expected {
            return Ok(id.index);
        }
        Err(identity_failure(id, &expected))
    }
    /// Reports exact headerless raw framing under the retained selection; e.g. `let proof = values.coverage()`.
    pub fn coverage(&self) -> FlowSelectedValueCoverage {
        FlowSelectedValueCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            links: self.context.link_count(),
            selected_values: self.context.selected_count(),
            absent_entries: self.context.link_count() - self.context.selected_count(),
            stored_values: self.context.selected_count(),
            achieved_error: 0,
        }
    }
    /// Borrows native owner, fresh-map and scatter qualifications; e.g. `let exclusions = values.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSelectionDiagnostic] {
        diagnostics()
    }
}
fn identity_failure(actual: &FlowSelectedValueId, expected: &FlowSelectedValueId) -> BessemerError {
    selected_invalid(format!(
        "value index {}, value revision matches {}, link revision matches {}, state revision matches {}, transport {:?}, assumption {:?}; expected current value/input revisions, {:?} transport and {:?} map assumption",
        actual.index,
        actual.record == expected.record,
        actual.links == expected.links,
        actual.states == expected.states,
        actual.transport,
        actual.assumption,
        expected.transport,
        expected.assumption
    ))
}
impl FlowSelectedValueId {
    /// Borrows the bound value occurrence; e.g. `let record = id.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Borrows both bound input occurrences; e.g. `let (links, states) = id.context_records()`.
    pub fn context_records(&self) -> (&FlowRecordId, &FlowRecordId) {
        (&self.links, &self.states)
    }
    /// Returns the declared map assumption; e.g. `let assumption = id.assumption()`.
    pub fn assumption(&self) -> FlowSelectionMapAssumption {
        self.assumption
    }
    /// Returns the dense physical value index; e.g. `let index = id.index()`.
    pub fn index(&self) -> usize {
        self.index
    }
}
