use super::*;

impl FlowConditionalScopeId {
    /// Borrows the original physical record identity; e.g. `let record = id.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns its scope-descriptor ordinal; e.g. `let ordinal = id.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}
impl FlowConditionalFieldId {
    /// Borrows the original physical record identity; e.g. `let record = id.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns its field-descriptor ordinal; e.g. `let ordinal = id.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}
impl<'a> FlowConditionalField<'a> {
    /// Returns a revision-bound existing-field identity; e.g. `let id = field.id()`.
    pub fn id(self) -> FlowConditionalFieldId {
        FlowConditionalFieldId {
            record: self.owner.record.clone(),
            ordinal: self.ordinal,
        }
    }
    /// Returns its complete physical scope identity; e.g. `let scope = items.scope(&field.scope())?`.
    pub fn scope(self) -> FlowConditionalScopeId {
        FlowConditionalScopeId {
            record: self.owner.record.clone(),
            ordinal: self.span.scope,
        }
    }
    /// Returns exact stored role; e.g. `let kind = field.kind()`.
    pub fn kind(self) -> FlowConditionalFieldKind {
        self.span.kind
    }
    /// Returns exact raw scalar encoding; e.g. `let width = field.encoding().width()`.
    pub fn encoding(self) -> FlowConditionalEncoding {
        self.span.encoding
    }
    /// Returns complete offset/count/stride; e.g. `let count = field.layout().element_count`.
    pub fn layout(self) -> FlowConditionalFieldLayout {
        self.span.layout
    }
    /// Reports existing-field edit eligibility, subject to complete reparse; e.g. `let editable = field.is_editable()`.
    pub fn is_editable(self) -> bool {
        !self.span.kind.is_structural()
    }
    /// Returns a checked element offset; e.g. `let offset = field.element_offset(0)`.
    pub fn element_offset(self, index: usize) -> Option<usize> {
        if index >= self.span.layout.element_count {
            return None;
        }
        self.span
            .layout
            .first_byte
            .checked_add(index.checked_mul(self.span.layout.stride_bytes)?)
    }
    /// Borrows exactly one raw element; e.g. `let bytes = field.element_bytes(0)`.
    pub fn element_bytes(self, index: usize) -> Option<&'a [u8]> {
        let start = self.element_offset(index)?;
        self.owner
            .decoded
            .get(start..start + self.span.encoding.width())
    }
    /// Reads a checked exact typed value, preserving all bits; e.g. `let bits = field.value(0)`.
    pub fn value(self, index: usize) -> Option<FlowConditionalValue> {
        Some(wire::scalar_value(
            self.span.encoding,
            self.element_bytes(index)?,
        ))
    }
    /// Projects only a stored item mode using nonzero truth; e.g. `let mode = field.native_mode_boolean()`.
    pub fn native_mode_boolean(self) -> Option<bool> {
        if self.kind() != FlowConditionalFieldKind::ItemMode {
            return None;
        }
        Some(self.value(0) != Some(FlowConditionalValue::Byte(0)))
    }
}
