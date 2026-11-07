use super::super::page_reader::{InlinePayloads, StoredCellHeader};
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum WriterPayload {
    Inline(InlinePayloads),
    Overflow {
        first: NonZeroU64,
        payload: OverflowPayload,
    },
}

/// One complete stored cell, preserving metadata and its physical storage kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashWriterCell {
    pub(super) header: StoredCellHeader,
    pub(super) payload: WriterPayload,
}

impl HashWriterCell {
    /// Returns its bucket offset; e.g. `let offset = cell.offset()`.
    pub fn offset(&self) -> u16 {
        self.header.offset
    }
    /// Returns the unverified stored key hash; e.g. `let hash = cell.key_hash()`.
    pub fn key_hash(&self) -> u32 {
        self.header.hash
    }
    /// Returns its raw next-cell offset; e.g. `let next = cell.next_offset()`.
    pub fn next_offset(&self) -> u16 {
        self.header.next
    }
    /// Returns the explicit first overflow identity, if present; e.g. `let first = cell.overflow_identifier()`.
    pub fn overflow_identifier(&self) -> Option<NonZeroU64> {
        match &self.payload {
            WriterPayload::Inline(_) => None,
            WriterPayload::Overflow { first, .. } => Some(*first),
        }
    }
    /// Borrows the complete key; e.g. `let key = cell.key()`.
    pub fn key(&self) -> &[u8] {
        match &self.payload {
            WriterPayload::Inline(p) => &p.key,
            WriterPayload::Overflow { payload, .. } => payload.key(),
        }
    }
    /// Borrows the complete opaque value; e.g. `let value = cell.value()`.
    pub fn value(&self) -> &[u8] {
        match &self.payload {
            WriterPayload::Inline(p) => &p.value,
            WriterPayload::Overflow { payload, .. } => payload.value(),
        }
    }
}

/// Exact local validation scope, excluding the outgoing bucket/file graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashWriterCheckMode {
    /// Disjoint bucket records and complete uniquely owned supplied overflow chains.
    ExactSuppliedWriterBucket,
}

/// Complete bucket/overflow coverage, retaining all unassigned bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashWriterCoverage {
    /// Declared validation mode.
    pub mode: HashWriterCheckMode,
    /// Aggregate bucket and overflow bytes.
    pub buffer_bytes: usize,
    /// Bucket metadata/inline payloads and overflow metadata/payloads.
    pub interpreted_bytes: usize,
    /// Retained bytes without semantic interpretation.
    pub uninterpreted_bytes: usize,
    /// Aggregate bucket and overflow page count.
    pub pages: usize,
    /// Complete cells in native chain order.
    pub cells: usize,
    /// Successful edit reparse error, always zero.
    pub achieved_byte_error: usize,
}

/// Total-order history: bucket spans first, then overflow page IDs and byte ranges.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HashWriterHistory {
    /// A complete source/result bucket span.
    Bucket(HashBufferHistory),
    /// A complete source/result overflow span with explicit identity.
    Overflow(OverflowPageHistory),
}
