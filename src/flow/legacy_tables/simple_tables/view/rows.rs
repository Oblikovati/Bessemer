use super::*;

impl<'a> FlowLeafPairRow<'a> {
    /// Returns physical row order only; e.g. `let ordinal = row.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the exact sixteen-byte interval; e.g. `let span = row.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        let start = 4 + self.ordinal * 16;
        (start, start + 16)
    }
    /// Borrows both unnormalized wide patterns; e.g. `let bits = row.wide_words().raw_bits(0)`.
    pub fn wide_words(self) -> FlowTableWideArray<'a> {
        let (start, end) = self.payload_interval();
        FlowTableWideArray {
            bytes: &self.owner.wire.decoded[start..end],
            start,
        }
    }
}

impl<'a> FlowFiveWordRow<'a> {
    /// Returns its physical wire ordinal; e.g. `let ordinal = row.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns exact twenty-byte row bounds; e.g. `let span = row.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        let start = self.owner.start + self.ordinal * 20;
        (start, start + 20)
    }
    /// Borrows all five raw words as bytes; e.g. `let raw = row.as_bytes()`.
    pub fn as_bytes(self) -> &'a [u8] {
        let (start, end) = self.payload_interval();
        &self.owner.wire.decoded[start..end]
    }
    /// Returns a checked raw word without role conversion; e.g. `let first = row.word(0)`.
    pub fn word(self, component: usize) -> Option<u32> {
        if component >= 5 {
            return None;
        }
        Some(wire::unsigned_at(self.as_bytes(), component * 4))
    }
    /// Returns all raw patterns without allocating; e.g. `let words = row.words()`.
    pub fn words(self) -> [u32; 5] {
        std::array::from_fn(|component| wire::unsigned_at(self.as_bytes(), component * 4))
    }
}
