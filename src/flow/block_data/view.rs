use super::*;

impl FlowLegacyBlockData {
    /// Borrows the complete source checkpoint; e.g. `let wire = blocks.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the selected occurrence identity; e.g. `let ordinal = blocks.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Borrows every decoded byte; e.g. `let raw = blocks.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.decoded
    }
    /// Returns the exact count prefix and both total views; e.g. `let sum = blocks.prefix().mathematical_total`.
    pub fn prefix(&self) -> FlowBlockCountPrefix {
        self.prefix
    }
    /// Borrows the complete bitmap, including any trailing native extra byte; e.g. `let raw = blocks.bitmap_bytes()`.
    pub fn bitmap_bytes(&self) -> &[u8] {
        &self.decoded[12..self.bitmap_end]
    }
    /// Returns physical bitmap offsets; e.g. `let span = blocks.bitmap_interval()`.
    pub fn bitmap_interval(&self) -> (usize, usize) {
        (12, self.bitmap_end)
    }
    /// Returns exact options and supplied context; e.g. `let leaves = blocks.options().leaf_count`.
    pub fn options(&self) -> FlowBlockDataOptions {
        self.options
    }
    /// Returns complete serialized coverage; e.g. `let count = blocks.coverage().checked_entries`.
    pub fn coverage(&self) -> FlowBlockDataCoverage {
        self.coverage
    }
    /// Reports an observed equality only, never runtime context; e.g. `let observed = blocks.entries_equal_global_total()`.
    pub fn entries_equal_global_total(&self) -> bool {
        self.entries.len() as u64 == self.prefix.mathematical_total
    }
    /// Borrows acceptance exclusions; e.g. `let exclusions = blocks.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowBlockDataDiagnostic] {
        &[
            FlowBlockDataDiagnostic::NativeLeafContextNotExecuted,
            FlowBlockDataDiagnostic::BlockToTreeOrderNotAssigned,
            FlowBlockDataDiagnostic::NativeRuntimeStateNotReproduced,
            FlowBlockDataDiagnostic::Record433ContextNotProven,
            FlowBlockDataDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowBlockDataDiagnostic::NativeApplicationNotChecked,
        ]
    }
    /// Borrows one physical block without a tree mapping; e.g. `let first = blocks.block(0)`.
    pub fn block(&self, index: usize) -> Option<FlowLegacyBlock<'_>> {
        Some(FlowLegacyBlock {
            owner: self,
            ordinal: index,
            span: self.blocks.get(index)?,
        })
    }
    /// Enumerates blocks in physical wire order; e.g. `let count = blocks.blocks().len()`.
    pub fn blocks(&self) -> impl ExactSizeIterator<Item = FlowLegacyBlock<'_>> {
        self.blocks
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowLegacyBlock {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Borrows one flat entry ordinal; e.g. `let first = blocks.entry(0)`.
    pub fn entry(&self, index: usize) -> Option<FlowLegacyBlockEntry<'_>> {
        Some(FlowLegacyBlockEntry {
            owner: self,
            ordinal: index,
            span: self.entries.get(index)?,
        })
    }
    /// Enumerates every entry in complete wire order; e.g. `let count = blocks.entries().len()`.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = FlowLegacyBlockEntry<'_>> {
        self.entries
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowLegacyBlockEntry {
                owner: self,
                ordinal,
                span,
            })
    }
}

impl<'a> FlowLegacyBlock<'a> {
    /// Returns the physical ordinal; e.g. `let ordinal = block.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns exact decoded block offsets; e.g. `let span = block.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Reads a typed raw header field; e.g. `let byte = block.field(FlowBlockHeaderField::BlockByte)?`.
    pub fn field(self, field: FlowBlockHeaderField) -> Result<FlowBlockFieldValue> {
        Ok(selection::block_field(self, field).read(&self.owner.decoded))
    }
    /// Returns the signed interpretation of the raw local count; e.g. `let count = block.signed_entry_count()`.
    pub fn signed_entry_count(self) -> i16 {
        let start = self.span.start + 3;
        i16::from_le_bytes([self.owner.decoded[start], self.owner.decoded[start + 1]])
    }
    /// Reports the observed native operation without executing it; e.g. `let operation = block.native_byte_operation()`.
    pub fn native_byte_operation(self) -> FlowNativeBlockByteOperation {
        if self.owner.prefix.version >= 13 {
            FlowNativeBlockByteOperation::Assign
        } else {
            FlowNativeBlockByteOperation::AndE7OfRuntimeValue
        }
    }
    /// Borrows this block's entries in physical order; e.g. `let count = block.entries().len()`.
    pub fn entries(self) -> impl ExactSizeIterator<Item = FlowLegacyBlockEntry<'a>> {
        self.owner.entries[self.span.first_entry..self.span.entry_end]
            .iter()
            .enumerate()
            .map(move |(index, span)| FlowLegacyBlockEntry {
                owner: self.owner,
                ordinal: self.span.first_entry + index,
                span,
            })
    }
}

impl<'a> FlowLegacyBlockEntry<'a> {
    /// Returns the flat physical ordinal; e.g. `let ordinal = entry.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns exact decoded entry offsets; e.g. `let span = entry.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Reads a present typed field or reports its absence; e.g. `let state = entry.field(FlowBlockEntryField::State)?`.
    pub fn field(self, field: FlowBlockEntryField) -> Result<FlowBlockFieldValue> {
        Ok(selection::entry_field(self, field)?.read(&self.owner.decoded))
    }
    /// Returns the physically small second-count word, never a widened reconstruction; e.g. `let raw = entry.second_count_raw_small()`.
    pub fn second_count_raw_small(self) -> Option<u16> {
        let offset = self.span.scalars[8]?;
        let raw = &self.owner.decoded[offset..];
        match self.owner.prefix.version {
            1 => Some(u16::from(raw[0])),
            2..=9 => Some(u16::from_le_bytes([raw[0], raw[1]])),
            _ => None,
        }
    }
    /// Borrows the complete first array; e.g. `let first = entry.first_words()`.
    pub fn first_words(self) -> FlowBlockWordArray<'a> {
        word_view(&self.owner.decoded, self.span.first)
    }
    /// Borrows the complete second array; e.g. `let second = entry.second_words()`.
    pub fn second_words(self) -> FlowBlockWordArray<'a> {
        word_view(&self.owner.decoded, self.span.second)
    }
}

fn word_view(raw: &[u8], span: (usize, usize)) -> FlowBlockWordArray<'_> {
    FlowBlockWordArray {
        bytes: &raw[span.0..span.1],
        start: span.0,
    }
}

impl<'a> FlowBlockWordArray<'a> {
    /// Borrows every raw array byte; e.g. `let bytes = words.as_bytes()`.
    pub fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }
    /// Returns exact decoded offsets; e.g. `let span = words.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.start, self.start + self.bytes.len())
    }
    /// Returns the physical word count; e.g. `let count = words.word_count()`.
    pub fn word_count(self) -> usize {
        self.bytes.len() / 4
    }
    /// Reads one raw unsigned word with checked indexing; e.g. `let first = words.raw_word(0)`.
    pub fn raw_word(self, index: usize) -> Option<u32> {
        let start = index.checked_mul(4)?;
        let end = start.checked_add(4)?;
        Some(fields::u32_word(self.bytes.get(start..end)?))
    }
}

impl FlowBlockDataEdit {
    /// Borrows the original immutable body; e.g. `let original = edit.source()`.
    pub fn source(&self) -> &FlowLegacyBlockData {
        &self.source
    }
    /// Borrows the complete reopened body; e.g. `let updated = edit.result()`.
    pub fn result(&self) -> &FlowLegacyBlockData {
        &self.result
    }
    /// Borrows complete physical history and checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
