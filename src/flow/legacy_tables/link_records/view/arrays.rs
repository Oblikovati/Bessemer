use super::*;

impl<'a> FlowLinkIntegers<'a> {
    /// Borrows every serialized byte; e.g. `let raw = array.as_bytes()`.
    pub fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }
    /// Returns exact decoded offsets, including empty arrays; e.g. `let interval = array.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.start, self.start + self.bytes.len())
    }
    /// Returns the complete word count; e.g. `let count = array.word_count()`.
    pub fn word_count(self) -> usize {
        self.bytes.len() / 4
    }
    /// Returns a checked signed word; e.g. `let first = array.value(0)`.
    pub fn value(self, index: usize) -> Option<i32> {
        let start = index.checked_mul(4)?;
        let raw = self.bytes.get(start..start.checked_add(4)?)?;
        Some(wire::signed_at(raw, 0))
    }
    /// Enumerates all words without allocating or removing duplicates; e.g. `let count = array.values().len()`.
    pub fn values(self) -> impl ExactSizeIterator<Item = i32> + 'a {
        self.bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|raw| i32::from_le_bytes(*raw))
    }
}
