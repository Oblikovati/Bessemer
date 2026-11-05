use std::ops::Range;

/// Explicitly selected buffer grammar; runtime engine identity is caller-declared.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashBufferProfile {
    /// Installed hash-buffer layout with complete inline cell payloads.
    InstalledInline,
}

/// Current engine extent and caller budgets, independent of the database header P.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashBufferContext {
    /// Explicit selected grammar.
    pub profile: HashBufferProfile,
    /// Current engine buffer extent; must equal the supplied byte length.
    pub engine_extent: usize,
    /// Maximum owned buffer bytes.
    pub maximum_bytes: usize,
    /// Aggregate maximum cell and free-record visits.
    pub maximum_entries: usize,
}

/// Exact twelve-byte page metadata; outgoing pages are not traversed by this buffer operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashBufferHeader {
    /// BE16 cell-chain head.
    pub cell_head: u16,
    /// BE16 free-record head.
    pub free_head: u16,
    /// Raw BE64 outgoing page identifier.
    pub next_page: u64,
}

/// Four-byte free-chain record; size is preserved without an allocation-fit claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashFreeRecord {
    /// Unsigned16 location in the selected buffer.
    pub offset: u16,
    /// Next free record, or zero.
    pub next: u16,
    /// Raw declared size, including values below the update writer's minimum.
    pub size: u16,
}

/// Complete owned inline cell and its raw twenty-six-byte metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashInlineCell {
    pub(super) offset: u16,
    pub(super) hash: u32,
    pub(super) next: u16,
    pub(super) key: Box<[u8]>,
    pub(super) value: Box<[u8]>,
}

impl HashInlineCell {
    /// Returns the stored buffer offset; e.g. `let offset = cell.offset()`.
    pub fn offset(&self) -> u16 {
        self.offset
    }
    /// Returns the unverified key-function result; e.g. `let hash = cell.key_hash()`.
    pub fn key_hash(&self) -> u32 {
        self.hash
    }
    /// Returns the raw next-cell offset; e.g. `let next = cell.next_offset()`.
    pub fn next_offset(&self) -> u16 {
        self.next
    }
    /// Borrows exact key bytes, including NUL; e.g. `let key = cell.key()`.
    pub fn key(&self) -> &[u8] {
        &self.key
    }
    /// Borrows opaque value bytes; e.g. `let value = cell.value()`.
    pub fn value(&self) -> &[u8] {
        &self.value
    }
    pub(super) fn value_range(&self) -> Range<usize> {
        let start = usize::from(self.offset) + 26 + self.key.len();
        start..start + self.value.len()
    }
}

/// Declared check scope for an isolated page-buffer operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashBufferCheckMode {
    /// Bounded disjoint metadata/cells, complete inline payloads and exact retained bytes.
    ExactInlineBuffer,
}

/// Measured local coverage; retained opaque/free bytes have no semantic validation credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashBufferCoverage {
    /// Explicit local check scope.
    pub mode: HashBufferCheckMode,
    /// Complete supplied buffer length.
    pub buffer_bytes: usize,
    /// Header, free-record metadata and complete inline cells.
    pub interpreted_bytes: usize,
    /// Bytes retained without assigning meaning.
    pub uninterpreted_bytes: usize,
    /// Reparse disagreement; successful edits require zero.
    pub achieved_byte_error: usize,
}

/// Completion distinguishes exact byte no-ops from changed buffers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashBufferCompletion {
    /// Encoded bytes equal the source checkpoint.
    Unchanged,
    /// Encoded bytes differ from the source checkpoint.
    Changed,
}

/// Full-span relation to the operation's immutable source buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashBufferRelation {
    /// Byte-for-byte retained span.
    Copied,
    /// Span derived from its original value.
    Derived,
}

/// History is ordered by physical byte range and retains empty value spans.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashBufferHistory {
    /// Complete source span.
    pub parent: Range<usize>,
    /// Complete result span.
    pub result: Range<usize>,
    /// Whether the span was copied or derived.
    pub relation: HashBufferRelation,
}

/// Exclusions carried with every local edit result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashBufferDiagnostic {
    /// Raw values are not assigned a universal MsgPack format.
    OpaqueValues,
    /// Sizes and their modulo16 total are not validated distinct free-byte counts.
    FreeSizesNotAllocatorValidation,
    /// Outgoing page IDs are preserved without a whole-file reachability check.
    PageGraphNotValidated,
    /// Static descriptor/provider identity is declared by the caller.
    ProviderAssumed,
}
