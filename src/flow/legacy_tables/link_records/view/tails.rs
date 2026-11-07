use super::*;

impl<'a> FlowLinkTail<'a> {
    /// Returns the complete tail interval; e.g. `let interval = tail.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.owner.tail.start, self.owner.wire.decoded.len())
    }
    /// Returns eleven raw members in documented physical order; e.g. `let members = tail.members()`.
    pub fn members(self) -> [i32; 11] {
        std::array::from_fn(|i| {
            wire::signed_at(&self.owner.wire.decoded, self.owner.tail.start + i * 4)
        })
    }
    /// Borrows version-two/three discarded words; e.g. `let values = tail.discarded_words()`.
    pub fn discarded_words(self) -> FlowLinkIntegers<'a> {
        let (start, end) = self.owner.tail.discarded;
        integers(self.owner, start, end)
    }
    /// Returns physically stored member 0x2f0 or absence; e.g. `let control = tail.control()`.
    pub fn control(self) -> Option<i32> {
        self.owner
            .tail
            .control
            .map(|offset| wire::signed_at(&self.owner.wire.decoded, offset))
    }
    /// Borrows the exact conditional array, including its empty interval; e.g. `let values = tail.conditional_integers()`.
    pub fn conditional_integers(self) -> FlowLinkIntegers<'a> {
        let (start, end) = self.owner.tail.conditional;
        integers(self.owner, start, end)
    }
    /// Borrows the numeric tail when present; e.g. `let numeric = tail.numeric()`.
    pub fn numeric(self) -> Option<FlowLinkNumericTail<'a>> {
        Some(FlowLinkNumericTail {
            owner: self.owner,
            span: self.owner.tail.numeric.as_ref()?,
        })
    }
    /// Borrows the version-selected final tail; e.g. `let final_tail = tail.final_array()`.
    pub fn final_array(self) -> Option<FlowLinkFinalTail<'a>> {
        Some(FlowLinkFinalTail {
            owner: self.owner,
            span: self.owner.tail.final_tail.as_ref()?,
        })
    }
}

impl<'a> FlowLinkNumericTail<'a> {
    /// Returns exact interval including the stored count; e.g. `let interval = numeric.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Borrows all unnormalized numeric patterns; e.g. `let words = numeric.wide_words()`.
    pub fn wide_words(self) -> FlowTableWideArray<'a> {
        wide(self.owner, self.span.start + 4, self.span.count)
    }
    /// Borrows stored indices, preserving present empty arrays; e.g. `let indices = numeric.stored_indices()`.
    pub fn stored_indices(self) -> Option<FlowLinkIntegers<'a>> {
        let start = self.span.indices?;
        Some(integers(self.owner, start, start + self.span.count * 4))
    }
    /// Reports -1 only when indices were runtime defaults; e.g. `let default = numeric.runtime_default_index()`.
    pub fn runtime_default_index(self) -> Option<i32> {
        self.span.indices.is_none().then_some(-1)
    }
}

impl<'a> FlowLinkFinalTail<'a> {
    /// Returns exact interval including count; e.g. `let interval = final_tail.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Returns the stored element width, four or eight; e.g. `let width = final_tail.element_bytes()`.
    pub fn element_bytes(self) -> usize {
        self.span.width
    }
    /// Borrows signed words only for the four-byte layout; e.g. `let values = final_tail.signed_words()`.
    pub fn signed_words(self) -> Option<FlowLinkIntegers<'a>> {
        (self.span.width == 4).then(|| integers(self.owner, self.span.start + 4, self.span.end))
    }
    /// Borrows raw wide words only for the eight-byte layout; e.g. `let values = final_tail.wide_words()`.
    pub fn wide_words(self) -> Option<FlowTableWideArray<'a>> {
        (self.span.width == 8).then(|| wide(self.owner, self.span.start + 4, self.span.count))
    }
}
