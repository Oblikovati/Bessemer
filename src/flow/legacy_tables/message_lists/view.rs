use super::*;
impl FlowMessageLists {
    /// Borrows the immutable carrier; e.g. `let bytes = messages.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected occurrence; e.g. `let id = messages.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the stored version; e.g. `let version = messages.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns admitted resource bounds and context; e.g. `let bounds = messages.options()`.
    pub fn options(&self) -> FlowMessageListOptions {
        self.options
    }
    /// Borrows the complete payload; e.g. `let raw = messages.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Borrows an existing item; e.g. `let first = messages.item(0)`.
    pub fn item(&self, ordinal: usize) -> Option<FlowMessageItem<'_>> {
        Some(FlowMessageItem {
            owner: self,
            ordinal,
            span: self.items.get(ordinal)?,
        })
    }
    /// Enumerates physical items; e.g. `let count = messages.items().len()`.
    pub fn items(&self) -> impl ExactSizeIterator<Item = FlowMessageItem<'_>> {
        self.items
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowMessageItem {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Borrows a string by physical ordinal; e.g. `let primary = messages.text_at(0)`.
    pub fn text_at(&self, ordinal: usize) -> Option<FlowMessageText<'_>> {
        Some(FlowMessageText {
            owner: self,
            ordinal,
            span: self.texts.get(ordinal)?,
        })
    }
    /// Enumerates all primary and list strings; e.g. `let count = messages.texts().len()`.
    pub fn texts(&self) -> impl ExactSizeIterator<Item = FlowMessageText<'_>> {
        self.texts
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowMessageText {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Resolves a revision-bound identity; e.g. `let text = messages.text(&id)?`.
    pub fn text(&self, id: &FlowMessageTextId) -> Result<FlowMessageText<'_>> {
        if id.record != self.wire.record {
            return Err(message_invalid(format!(
                "text record {:?}; expected selected revision {:?}",
                id.record, self.wire.record
            )));
        }
        self.text_at(id.ordinal)
            .ok_or_else(|| table_index_error(301, "text", id.ordinal, self.texts.len()))
    }
    /// Reports exact framing and actual transferred values; e.g. `let proof = messages.coverage()`.
    pub fn coverage(&self) -> FlowMessageCoverage {
        FlowMessageCoverage {
            mode: wire::mode(&self.wire),
            encoding: self.encoding,
            applied_string_mode: (self.encoding == FlowMessageTextEncoding::Utf16Units)
                .then_some(FlowMetadataStringMode::NativeUtf16),
            payload_bytes: self.wire.decoded.len(),
            items: self.items.len(),
            strings: self.texts.len(),
            text_bytes: self.totals[0],
            stored_values: self.totals[1],
            scopes: self.totals[2],
            achieved_error: 0,
        }
    }
    /// Returns native interpretation exclusions; e.g. `let exclusions = messages.diagnostics()`.
    pub fn diagnostics(&self) -> [FlowMessageDiagnostic; 4] {
        use FlowMessageDiagnostic::*;
        let text = match self.encoding {
            FlowMessageTextEncoding::LegacyBytes => LegacyStringInterpretationNotReproduced,
            FlowMessageTextEncoding::Utf16Units => NativeStringModeAssumed,
        };
        [
            text,
            NativeRuntimeObjectsNotReconstructed,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
}
impl<'a> FlowMessageItem<'a> {
    /// Returns the item ordinal; e.g. `let ordinal = item.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns its complete payload interval; e.g. `let interval = item.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.span.interval
    }
    /// Returns the uninterpreted member bits; e.g. `let bits = item.member_pattern()`.
    pub fn member_pattern(self) -> u32 {
        wire::unsigned_at(&self.owner.wire.decoded, self.span.interval.0)
    }
    /// Borrows the primary string; e.g. `let raw = item.primary().raw_bytes()`.
    pub fn primary(self) -> FlowMessageText<'a> {
        let ordinal = self.span.primary;
        FlowMessageText {
            owner: self.owner,
            ordinal,
            span: &self.owner.texts[ordinal],
        }
    }
    /// Borrows the independently counted first list; e.g. `let count = item.first_list().string_count()`.
    pub fn first_list(self) -> FlowMessageStringList<'a> {
        FlowMessageStringList {
            owner: self.owner,
            span: &self.span.first,
        }
    }
    /// Distinguishes absent and stored-empty second lists; e.g. `let second = item.second_list()`.
    pub fn second_list(self) -> Option<FlowMessageStringList<'a>> {
        Some(FlowMessageStringList {
            owner: self.owner,
            span: self.span.second.as_ref()?,
        })
    }
}
impl<'a> FlowMessageStringList<'a> {
    /// Returns the interval including its count; e.g. `let interval = list.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.span.interval
    }
    /// Returns the independent count; e.g. `let count = list.string_count()`.
    pub fn string_count(self) -> usize {
        self.span.texts.1 - self.span.texts.0
    }
    /// Borrows a local string; e.g. `let first = list.text(0)`.
    pub fn text(self, index: usize) -> Option<FlowMessageText<'a>> {
        if index >= self.string_count() {
            return None;
        }
        self.owner.text_at(self.span.texts.0 + index)
    }
    /// Enumerates every local string; e.g. `let count = list.texts().len()`.
    pub fn texts(self) -> impl ExactSizeIterator<Item = FlowMessageText<'a>> {
        (self.span.texts.0..self.span.texts.1).map(move |ordinal| FlowMessageText {
            owner: self.owner,
            ordinal,
            span: &self.owner.texts[ordinal],
        })
    }
}
impl<'a> FlowMessageText<'a> {
    /// Returns its revision-bound identity; e.g. `let id = text.id()`.
    pub fn id(self) -> FlowMessageTextId {
        FlowMessageTextId {
            record: self.owner.wire.record.clone(),
            ordinal: self.ordinal,
        }
    }
    /// Returns physical ancestry; e.g. `let role = text.role()`.
    pub fn role(self) -> FlowMessageTextRole {
        self.span.role
    }
    /// Returns the interval including its count; e.g. `let interval = text.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.span.interval
    }
    /// Returns the uninterpreted stored count; e.g. `let count = text.stored_count()`.
    pub fn stored_count(self) -> usize {
        self.span.count
    }
    /// Returns the normalized transfer representation; e.g. `let width = text.encoding().element_width()`.
    pub fn encoding(self) -> FlowMessageTextEncoding {
        self.owner.encoding
    }
    /// Borrows every transferred byte, including hidden suffixes; e.g. `let raw = text.raw_bytes()`.
    pub fn raw_bytes(self) -> &'a [u8] {
        &self.owner.wire.decoded[self.span.interval.0 + 4..self.span.interval.1]
    }
    /// Reads a legacy byte only; e.g. `let byte = text.byte(0)`.
    pub fn byte(self, index: usize) -> Option<u8> {
        if self.encoding() != FlowMessageTextEncoding::LegacyBytes {
            return None;
        }
        self.raw_bytes().get(index).copied()
    }
    /// Reads a modern UTF16 unit only; e.g. `let unit = text.unit(0)`.
    pub fn unit(self, index: usize) -> Option<u16> {
        if self.encoding() != FlowMessageTextEncoding::Utf16Units || index >= self.stored_count() {
            return None;
        }
        Some(wire::unit_at(self.raw_bytes(), index * 2))
    }
    /// Returns the first-NUL prefix length only for modern text; e.g. `let length = text.prefix_units()`.
    pub fn prefix_units(self) -> Option<usize> {
        self.span.prefix_units
    }
    /// Borrows the modern prefix without decoding surrogates; e.g. `let prefix = text.prefix_bytes()`.
    pub fn prefix_bytes(self) -> Option<&'a [u8]> {
        Some(&self.raw_bytes()[..self.span.prefix_units? * 2])
    }
}
impl FlowMessageTextId {
    /// Borrows the bound occurrence; e.g. `let record = id.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns the physical text ordinal; e.g. `let ordinal = id.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}
impl FlowMessageListEdit {
    /// Borrows the original checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowMessageLists {
        &self.source
    }
    /// Borrows the reopened result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowMessageLists {
        &self.result
    }
    /// Borrows complete physical history; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
