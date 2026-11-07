/// Explicit physical provider and default hash interpretation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashFileProfile {
    /// Caller-selected default Windows page offsets and installed writer grammar.
    DefaultWindowsWriter,
}

/// Independent physical extents and aggregate traversal/allocation limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashFileContext {
    /// Provider selection is an assumption, not runtime discovery.
    pub profile: HashFileProfile,
    /// Current engine extent, explicitly matched to the pager for this operation.
    pub engine_extent: usize,
    /// Current physical pager extent, explicitly matched to stored P.
    pub pager_extent: usize,
    /// Maximum complete supplied bytes, including uninterpreted tails.
    pub maximum_bytes: usize,
    /// Maximum complete supplied pages.
    pub maximum_pages: usize,
    /// Aggregate map pairs, bucket cells and bucket free-record visits.
    pub maximum_records: usize,
    /// Aggregate complete key and value bytes across all buckets.
    pub maximum_payload_bytes: usize,
}

/// A complete ordered persisted logical-bucket mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashFileMapEntry {
    /// Saved logical bucket identity.
    pub logical: u64,
    /// Saved physical root page identity.
    pub physical: u64,
}

/// Explicit uniquely owned physical page roles in page-ID order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashFilePageRole {
    /// File header, including its retained remainder.
    Header,
    /// Root or continuation index map page.
    Index,
    /// Root or continuation bucket belonging to one logical partition.
    Bucket(u64),
    /// Overflow storage belonging to exactly one complete cell.
    Overflow,
    /// Page linked from the saved free-page head.
    Free,
}

/// Explicit validation scope of a complete successful graph operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashFileCheckMode {
    /// Complete supplied page ownership, bounded framing, default hashes and routing.
    ExactSuppliedFileGraph,
}

/// Complete file accounting with no semantic credit for retained opaque bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashFileCoverage {
    /// Declared exact supplied graph scope.
    pub mode: HashFileCheckMode,
    /// Complete input extent.
    pub buffer_bytes: usize,
    /// Interpreted header/index/free-link and bucket/payload bytes.
    pub interpreted_bytes: usize,
    /// Retained bytes without assigned meaning.
    pub uninterpreted_bytes: usize,
    /// All uniquely classified physical pages.
    pub pages: usize,
    /// Complete cells across all root and continuation buckets.
    pub cells: usize,
    /// Reparse disagreement; successful edits require zero.
    pub achieved_byte_error: usize,
}

/// Qualifications carried with every complete graph edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashFileDiagnostic {
    /// Selected provider, descriptor and matching current extents are assumptions.
    ProviderAssumed,
    /// Raw values have no universal MsgPack or application interpretation.
    OpaqueValues,
    /// Writer storage differs from the native first-key callback request.
    NativeKeyReaderDiffers,
    /// Graph checks do not prove allocator operations, transactions or application validity.
    AllocationTransactionAndApplicationNotValidated,
}
