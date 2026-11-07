use super::*;

pub(super) fn unit_at(raw: &[u8], start: usize) -> u16 {
    u16::from_le_bytes([raw[start], raw[start + 1]])
}

impl FlowLoadFieldId {
    /// Borrows the original physical occurrence; e.g. `let record = id.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns the external mode interpretation; e.g. `let mode = id.mode()`.
    pub fn mode(&self) -> FlowLoadMode {
        self.mode
    }
    /// Returns the physical field ordinal; e.g. `let ordinal = id.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}

impl<'a> FlowLoadField<'a> {
    /// Returns a revision/mode-bound edit identity; e.g. `let id = field.id()`.
    pub fn id(self) -> FlowLoadFieldId {
        FlowLoadFieldId {
            record: self.owner.wire.record.clone(),
            mode: self.owner.options.mode,
            ordinal: self.ordinal,
        }
    }
    /// Returns complete physical ancestry; e.g. `let scope = field.scope()`.
    pub fn scope(self) -> FlowLoadScope {
        self.span.scope
    }
    /// Returns the exact stored role; e.g. `let kind = field.kind()`.
    pub fn kind(self) -> FlowLoadFieldKind {
        self.span.kind
    }
    /// Returns exact raw encoding; e.g. `let width = field.encoding().width()`.
    pub fn encoding(self) -> FlowLoadEncoding {
        self.span.encoding
    }
    /// Returns the complete scalar/vector layout; e.g. `let count = field.layout().element_count`.
    pub fn layout(self) -> FlowLoadFieldLayout {
        self.span.layout
    }
    /// Reports fixed-field edit eligibility; e.g. `let editable = field.is_editable()`.
    pub fn is_editable(self) -> bool {
        !self.span.kind.is_structural()
    }
    /// Returns a checked existing element's offset; e.g. `let offset = field.element_offset(0)`.
    pub fn element_offset(self, index: usize) -> Option<usize> {
        if index >= self.span.layout.element_count {
            return None;
        }
        self.span
            .layout
            .first_byte
            .checked_add(index.checked_mul(self.span.layout.stride_bytes)?)
    }
    /// Borrows one checked raw element; e.g. `let bytes = field.element_bytes(0)`.
    pub fn element_bytes(self, index: usize) -> Option<&'a [u8]> {
        let start = self.element_offset(index)?;
        self.owner
            .wire
            .decoded
            .get(start..start + self.span.encoding.width())
    }
    /// Reads one lossless typed value; e.g. `let value = field.value(0)`.
    pub fn value(self, index: usize) -> Option<FlowLoadValue> {
        let raw = self.element_bytes(index)?;
        Some(match self.span.encoding {
            FlowLoadEncoding::Scalar(encoding) => {
                FlowLoadValue::Scalar(wire::scalar_value(encoding, raw))
            }
            FlowLoadEncoding::Utf16Unit => FlowLoadValue::Utf16Unit(unit_at(raw, 0)),
        })
    }
    /// Resolves a typed native offset only on the nine-word member field; e.g. `let value = field.native_member(offset)`.
    pub fn native_member(self, offset: FlowLoadMemberOffset) -> Option<i32> {
        if self.kind() != FlowLoadFieldKind::MemberWords {
            return None;
        }
        match self.value(offset.slot())? {
            FlowLoadValue::Scalar(FlowPrimaryValue::Signed32(value)) => Some(value),
            _ => None,
        }
    }
}

impl<'a> FlowLoadText<'a> {
    /// Returns its item/name or description scope; e.g. `let scope = text.scope()`.
    pub fn scope(self) -> FlowLoadScope {
        self.span.scope
    }
    /// Returns count-prefix plus raw-unit interval; e.g. `let interval = text.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.span.interval
    }
    /// Borrows the complete raw-unit field; e.g. `let id = text.units().id()`.
    pub fn units(self) -> FlowLoadField<'a> {
        FlowLoadField {
            owner: self.owner,
            ordinal: self.span.units_field,
            span: &self.owner.fields[self.span.units_field],
        }
    }
    /// Returns the separately projected first-NUL prefix length in units; e.g. `let prefix = text.native_prefix_units()`.
    pub fn native_prefix_units(self) -> usize {
        self.span.prefix_units
    }
    /// Borrows every stored UTF16LE byte, including hidden suffixes; e.g. `let raw = text.raw_units_bytes()`.
    pub fn raw_units_bytes(self) -> &'a [u8] {
        &self.owner.wire.decoded[self.span.interval.0 + 4..self.span.interval.1]
    }
    /// Borrows the lossless first-NUL prefix as UTF16LE bytes; e.g. `let prefix = text.native_prefix_bytes()`.
    pub fn native_prefix_bytes(self) -> &'a [u8] {
        &self.raw_units_bytes()[..self.span.prefix_units * 2]
    }
}
