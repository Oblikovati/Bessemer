use crate::database_hash::HashBufferHistory;
use std::{num::NonZeroU64, ops::Range};

/// Caller-selected physical storage grammar, separate from native callback requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverflowStorageProfile {
    /// Installed writer14600: first-page payload18, continuation8, explicit value checkpoint.
    InstalledWriter,
}

/// Explicit engine extent and independent allocation/traversal limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverflowStorageContext {
    /// Caller-declared provider grammar; not discovered from a database header.
    pub profile: OverflowStorageProfile,
    /// Every supplied page must have this extent in512..=65536.
    pub engine_extent: usize,
    /// Maximum aggregate supplied page bytes, including opaque tails.
    pub maximum_bytes: usize,
    /// Maximum supplied and visited pages.
    pub maximum_pages: usize,
    /// Maximum aggregate key and value bytes.
    pub maximum_payload_bytes: usize,
}

/// Explicit nonzero page identity and complete caller-supplied bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OverflowPageImage {
    /// Provider identity; no file-offset arithmetic is inferred.
    pub identifier: NonZeroU64,
    /// Complete page; published snapshots own a separate copy.
    pub bytes: Vec<u8>,
}

/// Exact local check scope for the complete supplied writer chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverflowCheckMode {
    /// All supplied pages visited once, complete payloads and consistent value checkpoint.
    ExactSuppliedWriterChain,
}

/// Measured interpretation and retained bytes of a complete supplied chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OverflowCoverage {
    /// Declared local validation mode.
    pub mode: OverflowCheckMode,
    /// Complete supplied bytes.
    pub buffer_bytes: usize,
    /// Page links, first-page checkpoint and complete payload bytes.
    pub interpreted_bytes: usize,
    /// Retained bytes without assigned semantics.
    pub uninterpreted_bytes: usize,
    /// Number of uniquely visited pages.
    pub pages: usize,
    /// Reparse disagreement; successful operations require zero.
    pub achieved_byte_error: usize,
}

/// Page-qualified copied/derived history in total page-ID and byte-offset order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OverflowPageHistory {
    /// Explicit page identity in both immutable snapshots.
    pub identifier: NonZeroU64,
    /// Complete source/result ranges and their relation.
    pub span: HashBufferHistory,
}

/// Qualifications returned with every isolated overflow edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverflowDiagnostic {
    /// Static provider and page identities are caller-declared.
    ProviderAssumed,
    /// Stored-page framing differs from native first-key callback requests.
    NativeKeyReaderDiffers,
    /// Values retain opaque bytes without a universal MsgPack assignment.
    OpaqueValues,
    /// No whole-file graph, allocator, engine, transaction or commit validation.
    FileAndAllocationNotValidated,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PayloadSpan {
    pub page: NonZeroU64,
    pub range: Range<usize>,
}
