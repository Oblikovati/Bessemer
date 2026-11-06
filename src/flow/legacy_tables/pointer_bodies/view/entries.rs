use super::*;

impl<'a> FlowPointerBodyEntry<'a> {
    /// Returns physical collection and entry ordinals; e.g. `let path = entry.path()`.
    pub fn path(self) -> (usize, usize) {
        (self.group, self.ordinal)
    }
    /// Returns the complete entry interval including framing; e.g. `let interval = entry.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.full.start, self.span.full.end)
    }
    /// Returns the selected serialized branch; e.g. `let kind = entry.kind()`.
    pub fn kind(self) -> FlowPointerBodyKind {
        self.span.kind
    }
    /// Returns supplied current runtime code for legacy third-collection entries; e.g. `let code = entry.runtime_code()`.
    pub fn runtime_code(self) -> Option<u8> {
        if self.group != 2 || self.owner.version() >= 3 {
            return None;
        }
        self.owner
            .options
            .entry_codes
            .as_deref()?
            .get(self.ordinal)
            .copied()
    }
    /// Returns the raw tag for tagged entries only; e.g. `let tag = entry.tag_byte()`.
    pub fn tag_byte(self) -> Option<u8> {
        (self.owner.version() >= 3).then(|| self.owner.wire.decoded[self.span.full.start])
    }
    /// Returns a stored entry count where present, excluding individual vector lengths; e.g. `let count = entry.stored_count()`.
    pub fn stored_count(self) -> Option<u32> {
        self.span.stored_count
    }
    /// Distinguishes recognized opaque payloads from unknown tags and legacy entries; e.g. `let decoded = entry.packed_payload_decoded()`.
    pub fn packed_payload_decoded(self) -> Option<bool> {
        matches!(
            self.kind(),
            FlowPointerBodyKind::TaggedWide | FlowPointerBodyKind::TaggedRaw
        )
        .then_some(false)
    }
    /// Borrows existing wide patterns, including present empty arrays; e.g. `let words = entry.wide_words()`.
    pub fn wide_words(self) -> Option<FlowTableWideArray<'a>> {
        matches!(
            self.kind(),
            FlowPointerBodyKind::WideTriplets | FlowPointerBodyKind::TaggedWide
        )
        .then(|| body_wide(&self.owner.wire, self.span.payload))
    }
    /// Borrows recognized tagged raw bytes without attempting decompression; e.g. `let raw = entry.raw_payload()`.
    pub fn raw_payload(self) -> Option<&'a [u8]> {
        (self.kind() == FlowPointerBodyKind::TaggedRaw)
            .then(|| &self.owner.wire.decoded[self.span.payload.start..self.span.payload.end])
    }
    /// Returns the tagged raw-byte interval, including an empty interval; e.g. `let interval = entry.raw_interval()`.
    pub fn raw_interval(self) -> Option<(usize, usize)> {
        self.raw_payload()
            .map(|_| (self.span.payload.start, self.span.payload.end))
    }
    /// Returns one for single vectors or the stored multiple-vector count; e.g. `let count = entry.vector_count()`.
    pub fn vector_count(self) -> usize {
        self.span.vectors.end - self.span.vectors.start
    }
    /// Borrows an individual byte vector, retaining present empty vectors; e.g. `let bytes = entry.vector(0)`.
    pub fn vector(self, index: usize) -> Option<&'a [u8]> {
        let (start, end) = self.vector_interval(index)?;
        Some(&self.owner.wire.decoded[start..end])
    }
    /// Returns a vector interval excluding its length byte; e.g. `let interval = entry.vector_interval(0)`.
    pub fn vector_interval(self, index: usize) -> Option<(usize, usize)> {
        if index >= self.vector_count() {
            return None;
        }
        let span = self.owner.vectors[self.span.vectors.start + index];
        Some((span.start, span.end))
    }
}
