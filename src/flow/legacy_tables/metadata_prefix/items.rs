use super::*;

impl FlowMetadataVector<'_> {
    /// Returns the wire ordinal; e.g. `let ordinal = vector.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete header/value interval; e.g. `let span = vector.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        let span = &self.owner.vectors[self.ordinal];
        (span.start, span.end)
    }
    /// Enumerates every stored signed value; e.g. `let values: Vec<_> = vector.values().collect()`.
    pub fn values(self) -> impl ExactSizeIterator<Item = i32> {
        let span = &self.owner.vectors[self.ordinal];
        (0..span.count)
            .map(move |index| wire::signed_at(&self.owner.raw, span.start + 4 + index * 4))
    }
}
impl<'a> FlowMetadataObject<'a> {
    /// Returns the wire ordinal; e.g. `let ordinal = object.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the entire stored object interval; e.g. `let span = object.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        let span = &self.owner.objects[self.ordinal];
        (span.start, span.end)
    }
    /// Borrows a stored string, distinguishing absent from stored empty; e.g. `let fourth = object.string(FlowMetadataStringRole::Fourth)`.
    pub fn string(self, role: FlowMetadataStringRole) -> Option<FlowMetadataText<'a>> {
        self.owner.objects[self.ordinal].strings[role_index(role)]
            .as_ref()
            .map(|span| FlowMetadataText {
                owner: self.owner,
                span,
            })
    }
    /// Returns all stored state bits; e.g. `let state = object.state_byte()`.
    pub fn state_byte(self) -> u8 {
        self.owner.raw[self.owner.objects[self.ordinal].state]
    }
    /// Projects the native Boolean without changing raw bits; e.g. `let enabled = object.native_state()`.
    pub fn native_state(self) -> bool {
        self.state_byte() != 0
    }
    /// Returns the signed member-C4 word; e.g. `let value = object.member_c4()`.
    pub fn member_c4(self) -> i32 {
        wire::signed_at(&self.owner.raw, self.owner.objects[self.ordinal].state + 1)
    }
    /// Distinguishes version-two fallback from a stored fourth string; e.g. `let derived = object.fourth_text_is_derived()`.
    pub fn fourth_text_is_derived(self) -> bool {
        self.owner.version_byte() == 2
    }
    /// Borrows the native fourth text as raw UTF-16LE through its first NUL; e.g. `let text = object.native_fourth_text_bytes()`.
    pub fn native_fourth_text_bytes(self) -> &'a [u8] {
        let role = if self.fourth_text_is_derived() {
            FlowMetadataStringRole::First
        } else {
            FlowMetadataStringRole::Fourth
        };
        self.string(role)
            .map_or(&[], |text| text.native_text_bytes())
    }
}
impl<'a> FlowMetadataText<'a> {
    /// Returns the count-prefix/text interval; e.g. `let span = text.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Returns every stored UTF-16 unit, including a terminator; e.g. `let count = text.stored_units()`.
    pub fn stored_units(self) -> usize {
        (self.span.end - self.span.start - 4) / 2
    }
    /// Borrows unnormalized raw text bytes; e.g. `let raw = text.raw_bytes()`.
    pub fn raw_bytes(self) -> &'a [u8] {
        &self.owner.raw[self.span.start + 4..self.span.end]
    }
    /// Borrows the lossless native text projection before the first NUL; e.g. `let native = text.native_text_bytes()`.
    pub fn native_text_bytes(self) -> &'a [u8] {
        &self.owner.raw[self.span.start + 4..self.span.native_end]
    }
}
pub(super) fn role_index(role: FlowMetadataStringRole) -> usize {
    match role {
        FlowMetadataStringRole::First => 0,
        FlowMetadataStringRole::Second => 1,
        FlowMetadataStringRole::Third => 2,
        FlowMetadataStringRole::Fourth => 3,
    }
}
