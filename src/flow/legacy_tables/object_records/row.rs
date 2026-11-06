use super::*;
impl FlowObjectRow<'_> {
    /// Returns the physical stored ordinal; e.g. `let index = row.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
    /// Returns the complete payload interval; e.g. `let span = row.interval()`.
    pub fn interval(&self) -> (usize, usize) {
        (self.start, self.start + self.owner.stride)
    }
    /// Reads the first signed member; e.g. `let member = row.member_10()`.
    pub fn member_10(&self) -> i32 {
        wire::signed_at(self.raw(), 0)
    }
    /// Retains the four discarded legacy words, when stored; e.g. `let legacy = row.discarded_words()`.
    pub fn discarded_words(&self) -> Option<[i32; 4]> {
        (self.owner.version() < 5)
            .then(|| std::array::from_fn(|index| wire::signed_at(self.raw(), 4 + index * 4)))
    }
    /// Borrows the complete opaque 16-byte block; e.g. `let block = row.block()`.
    pub fn block(&self) -> &[u8] {
        &self.raw()[self.block_offset()..self.block_offset() + 16]
    }
    /// Reads the additional signed member when stored; e.g. `let member = row.member_7c()`.
    pub fn member_7c(&self) -> Option<i32> {
        (self.owner.version() >= 4).then(|| wire::signed_at(self.raw(), self.block_offset() + 16))
    }
    /// Reads the physically stored state byte; e.g. `let state = row.state_byte()`.
    pub fn state_byte(&self) -> Option<u8> {
        (self.owner.version() >= 6).then(|| self.raw()[24])
    }
    /// Returns the documented stored-or-zero native projection; e.g. `let state = row.native_state_byte()`.
    pub fn native_state_byte(&self) -> u8 {
        self.state_byte().unwrap_or(0)
    }
    /// Borrows the unresolved 56-byte version-7 extension; e.g. `let bytes = row.extension()`.
    pub fn extension(&self) -> Option<&[u8]> {
        (self.owner.version() == 7).then(|| &self.raw()[25..81])
    }
    fn raw(&self) -> &[u8] {
        &self.owner.wire.decoded[self.start..self.start + self.owner.stride]
    }
    pub(super) fn block_offset(&self) -> usize {
        if self.owner.version() < 5 { 20 } else { 4 }
    }
}
impl FlowObjectTrailer<'_> {
    /// Returns the uncounted trailer's physical interval; e.g. `let span = trailer.interval()`.
    pub fn interval(&self) -> (usize, usize) {
        (self.start, self.start + 20)
    }
    /// Reads the trailer's signed member; e.g. `let member = trailer.member_10()`.
    pub fn member_10(&self) -> i32 {
        wire::signed_at(&self.owner.wire.decoded, self.start)
    }
    /// Borrows its complete opaque block; e.g. `let block = trailer.block()`.
    pub fn block(&self) -> &[u8] {
        &self.owner.wire.decoded[self.start + 4..self.start + 20]
    }
    /// Reports the reference append predicate without populating runtime objects; e.g. `let appended = trailer.native_appended_object()`.
    pub fn native_appended_object(&self) -> bool {
        self.owner.count == 1
    }
}
