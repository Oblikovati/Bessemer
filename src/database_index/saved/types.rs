/// Raw saved index words, without partition or free-page validity admission.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashIndexHeader {
    /// Saved free-page head, including out-of-file identities.
    pub free_head: u64,
    /// Saved split word, without normalization or a range repair.
    pub split: u64,
    /// Saved maximum word, including zero and non-power-of-two values.
    pub maximum: u64,
}

/// Explicit check scope of a lossless saved-index operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashIndexCheckMode {
    /// Complete selected header and map chain; other page roles and lookup are unchecked.
    ExactSavedIndexChain,
}

/// Accounting for the interpreted index and all retained file bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HashIndexCoverage {
    /// Declared saved-index-only validation scope.
    pub mode: HashIndexCheckMode,
    /// Complete supplied file extent.
    pub buffer_bytes: usize,
    /// Interpreted file header, map headers and complete pairs.
    pub interpreted_bytes: usize,
    /// Retained bytes without index interpretation.
    pub uninterpreted_bytes: usize,
    /// Complete root and continuation map pages visited exactly once.
    pub index_pages: usize,
    /// Ordered saved mapping pairs, including duplicates.
    pub mappings: usize,
    /// Reparse disagreement; successful edits require zero.
    pub achieved_byte_error: usize,
}

/// Qualifications returned with every saved-index edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashIndexDiagnostic {
    /// The provider, descriptor and current extents are caller-selected.
    ProviderAssumed,
    /// Non-index bytes are retained without assigning meanings.
    OpaqueRemainder,
    /// Ordered duplicates are preserved without guessing native cache precedence.
    DuplicatePrecedenceNotInferred,
    /// No complete graph, allocator, transaction, application or geometry validity claim.
    FileGraphAndApplicationNotValidated,
}
