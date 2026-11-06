use super::*;

impl FlowBlockExtraFieldId {
    /// Borrows the original record identity; e.g. `let record = field_id.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns the admitted leaf interpretation; e.g. `let count = field_id.leaf_count()`.
    pub fn leaf_count(&self) -> FlowLeafCount {
        self.leaf_count
    }
    /// Returns its physical descriptor ordinal; e.g. `let ordinal = field_id.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}

impl<'a> FlowBlockExtraField<'a> {
    /// Returns the revision/count-bound edit identity; e.g. `let id = field.id()`.
    pub fn id(self) -> FlowBlockExtraFieldId {
        FlowBlockExtraFieldId {
            record: self.owner.wire.record.clone(),
            leaf_count: self.owner.options.leaf_count,
            ordinal: self.ordinal,
        }
    }
    /// Returns its physical descriptor ordinal; e.g. `let ordinal = field.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete physical scope key; e.g. `let entity = field.entity()`.
    pub fn entity(self) -> FlowBlockExtraEntity {
        self.span.entity
    }
    /// Returns the exact stored role; e.g. `let kind = field.kind()`.
    pub fn kind(self) -> FlowBlockExtraFieldKind {
        self.span.kind
    }
    /// Returns the exact encoding without coercion; e.g. `let encoding = field.encoding()`.
    pub fn encoding(self) -> FlowBlockExtraEncoding {
        self.span.encoding
    }
    /// Returns width, count and stride explicitly; e.g. `let layout = field.layout()`.
    pub fn layout(self) -> FlowBlockExtraFieldLayout {
        FlowBlockExtraFieldLayout {
            first_byte: self.span.start,
            element_count: self.span.count,
            element_bytes: self.span.encoding.element_bytes(),
            stride_bytes: self.span.stride,
        }
    }
    /// Reports eligibility for existing-element edits; proposed values must preserve structure, e.g. `let editable = field.is_editable()`.
    pub fn is_editable(self) -> bool {
        !self.span.kind.is_structural()
    }
    /// Returns a checked element's decoded offset; e.g. `let offset = field.element_offset(0)`.
    pub fn element_offset(self, index: usize) -> Option<usize> {
        if index >= self.span.count {
            return None;
        }
        self.span
            .start
            .checked_add(index.checked_mul(self.span.stride)?)
    }
    /// Borrows one checked element, retaining all raw bits; e.g. `let raw = field.element_bytes(0)`.
    pub fn element_bytes(self, index: usize) -> Option<&'a [u8]> {
        let start = self.element_offset(index)?;
        let end = start.checked_add(self.span.encoding.element_bytes())?;
        self.owner.wire.decoded.get(start..end)
    }
    /// Reads one exact typed value; e.g. `let value = field.value(0)`.
    pub fn value(self, index: usize) -> Option<FlowBlockExtraValue> {
        let raw = self.element_bytes(index)?;
        Some(match self.span.encoding {
            FlowBlockExtraEncoding::Byte => FlowBlockExtraValue::Byte(raw[0]),
            FlowBlockExtraEncoding::Signed32 => {
                FlowBlockExtraValue::Signed32(wire::signed_at(raw, 0))
            }
            FlowBlockExtraEncoding::WideBits => {
                FlowBlockExtraValue::WideBits(wire::wide_at(raw, 0))
            }
        })
    }
    /// Borrows a packed field, including present empty arrays; strided fields return None, e.g. `let packed = field.contiguous_bytes()`.
    pub fn contiguous_bytes(self) -> Option<&'a [u8]> {
        let width = self.span.encoding.element_bytes();
        if self.span.stride != width {
            return None;
        }
        self.owner
            .wire
            .decoded
            .get(self.span.start..self.span.start + self.span.count * width)
    }
}
