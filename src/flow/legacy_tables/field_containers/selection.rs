use super::*;
/// Completed preferred-container selection, preserving the actual selected layout.
#[derive(Clone, Debug)]
pub enum FlowLegacyFieldSelection {
    /// Latest record113, including a complete empty vector.
    Words113(FlowLegacyWordFields),
    /// Latest record155 only when record113 is absent.
    Group155(FlowLegacyGroup155),
}
impl FlowLegacyFieldSelection {
    /// Returns the selected ordinary identifier; e.g. `let identifier = selection.identifier()`.
    pub fn identifier(&self) -> u32 {
        match self {
            Self::Words113(_) => 113,
            Self::Group155(_) => 155,
        }
    }
    /// Borrows the exact selected occurrence; e.g. `let ordinal = selection.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        match self {
            Self::Words113(value) => value.record(),
            Self::Group155(value) => value.record(),
        }
    }
}
impl FlowCarrier {
    /// Prefers latest113 and falls back to latest155 only on absence; e.g. `let selected = carrier.legacy_field_container_latest(words, groups)?`.
    pub fn legacy_field_container_latest(
        &self,
        words: FlowLegacyFieldWordOptions,
        groups: FlowConditionalItemOptions,
    ) -> Result<Option<FlowLegacyFieldSelection>> {
        if let Some(record) = self.latest(113)? {
            return self
                .record_113_fields(&record.id, words)
                .map(FlowLegacyFieldSelection::Words113)
                .map(Some);
        }
        if let Some(record) = self.latest(155)? {
            return self
                .record_155_fields(&record.id, groups)
                .map(FlowLegacyFieldSelection::Group155)
                .map(Some);
        }
        Ok(None)
    }
}
