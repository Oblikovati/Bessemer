use super::*;

impl FlowPrimaryFieldId {
    /// Borrows the original record checkpoint; e.g. `let record = id.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns the physical descriptor ordinal; e.g. `let ordinal = id.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}

impl<'a> FlowPrimaryField<'a> {
    /// Returns a revision-bound edit identity; e.g. `let id = field.id()`.
    pub fn id(self) -> FlowPrimaryFieldId {
        FlowPrimaryFieldId {
            record: self.owner.wire.record.clone(),
            ordinal: self.ordinal,
        }
    }
    /// Returns physical ancestry; e.g. `let scope = field.scope()`.
    pub fn scope(self) -> FlowPrimaryScope {
        self.span.scope
    }
    /// Returns exact stored role; e.g. `let kind = field.kind()`.
    pub fn kind(self) -> FlowPrimaryFieldKind {
        self.span.kind
    }
    /// Returns storage interpretation without coercion; e.g. `let encoding = field.encoding()`.
    pub fn encoding(self) -> FlowPrimaryEncoding {
        self.span.encoding
    }
    /// Returns count, first offset and serialized stride; e.g. `let stride = field.layout().stride_bytes`.
    pub fn layout(self) -> FlowPrimaryFieldLayout {
        self.span.layout
    }
    /// Reports whether a proposed value may preserve layout; e.g. `let editable = field.is_editable()`.
    pub fn is_editable(self) -> bool {
        !self.span.kind.is_structural()
    }
    /// Returns a checked existing element offset; e.g. `let offset = field.element_offset(0)`.
    pub fn element_offset(self, index: usize) -> Option<usize> {
        let layout = self.span.layout;
        if index >= layout.element_count {
            return None;
        }
        layout
            .first_byte
            .checked_add(index.checked_mul(layout.stride_bytes)?)
    }
    /// Borrows exactly one element's stored bytes; e.g. `let bytes = field.element_bytes(0)`.
    pub fn element_bytes(self, index: usize) -> Option<&'a [u8]> {
        let offset = self.element_offset(index)?;
        self.owner
            .wire
            .decoded
            .get(offset..offset + self.span.encoding.width())
    }
    /// Reads a checked exact typed value; e.g. `let value = field.value(0)`.
    pub fn value(self, index: usize) -> Option<FlowPrimaryValue> {
        let raw = self.element_bytes(index)?;
        Some(match self.span.encoding {
            FlowPrimaryEncoding::Byte => FlowPrimaryValue::Byte(raw[0]),
            FlowPrimaryEncoding::Signed32 => FlowPrimaryValue::Signed32(wire::signed_at(raw, 0)),
            FlowPrimaryEncoding::WordBits => FlowPrimaryValue::WordBits(wire::unsigned_at(raw, 0)),
            FlowPrimaryEncoding::WideBits => FlowPrimaryValue::WideBits(wire::wide_at(raw, 0)),
        })
    }
    /// Projects only a stored body flag, using exact equality to one; e.g. `let flag = field.native_flag()`.
    pub fn native_flag(self) -> Option<bool> {
        if self.kind() != FlowPrimaryFieldKind::StoredFlag {
            return None;
        }
        Some(self.value(0) == Some(FlowPrimaryValue::Byte(1)))
    }
}
