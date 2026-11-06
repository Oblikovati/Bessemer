use super::*;

impl<'a> FlowReferenceEntry<'a> {
    /// Returns physical group and entry ordinals; e.g. `let path = entry.path()`.
    pub fn path(self) -> (usize, usize) {
        (self.group, self.ordinal)
    }
    /// Returns the complete interval including the reference-count preamble; e.g. `let interval = entry.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.span.full
    }
    /// Returns the stored nonnegative reference count; e.g. `let count = entry.reference_count()`.
    pub fn reference_count(self) -> usize {
        (self.span.references.1 - self.span.references.0) / 4
    }
    /// Borrows signed reference words as their exact raw patterns; e.g. `let bytes = entry.reference_words().as_bytes()`.
    pub fn reference_words(self) -> FlowBlockWordArray<'a> {
        word_array(&self.owner.wire, self.span.references)
    }
    /// Returns one unmodified signed reference; e.g. `let reference = entry.reference_word(0)`.
    pub fn reference_word(self, index: usize) -> Option<i32> {
        self.reference_words()
            .raw_word(index)
            .map(|word| word as i32)
    }
    /// Borrows the complete first array; e.g. `let array = entry.first_words()`.
    pub fn first_words(self) -> FlowBlockWordArray<'a> {
        word_array(&self.owner.wire, self.span.continuation.first)
    }
    /// Borrows the complete second array, empty for negative counts; e.g. `let array = entry.second_words()`.
    pub fn second_words(self) -> FlowBlockWordArray<'a> {
        word_array(&self.owner.wire, self.span.continuation.second)
    }
    /// Reads an existing scalar/array element with its stored width; e.g. `let state = entry.field(FlowReferenceEntryField::State)?`.
    pub fn field(self, field: FlowReferenceEntryField) -> Result<FlowBlockFieldValue> {
        Ok(selection::entry_field(self, field)?.read(&self.owner.wire.decoded))
    }
    /// Returns an existing field's decoded interval; e.g. `let interval = entry.field_interval(FlowReferenceEntryField::State)?`.
    pub fn field_interval(self, field: FlowReferenceEntryField) -> Result<(usize, usize)> {
        let selected = selection::entry_field(self, field)?;
        Ok((
            selected.offset,
            selected.offset + selection::width(selected.kind),
        ))
    }
    /// Returns the original small count word before sign/zero extension; e.g. `let raw = entry.raw_small_second_count()`.
    pub fn raw_small_second_count(self) -> Option<u16> {
        let offset = self.span.continuation.scalars[4]?;
        let raw = &self.owner.wire.decoded[offset..];
        match self.owner.version() {
            1 => Some(u16::from(raw[0])),
            0 | 2..=9 => Some(u16::from_le_bytes([raw[0], raw[1]])),
            _ => None,
        }
    }
}
