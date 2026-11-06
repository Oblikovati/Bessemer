use super::*;

/// Three independent native signed counts; their mathematical sum need not fit an i32.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowPointerBodyCounts([u32; 3]);

impl TryFrom<[u32; 3]> for FlowPointerBodyCounts {
    type Error = BessemerError;
    fn try_from(counts: [u32; 3]) -> Result<Self> {
        if counts.iter().all(|count| *count <= i32::MAX as u32) {
            return Ok(Self(counts));
        }
        Err(body_invalid(format!(
            "collection counts {counts:?}; expected three values zero through {}",
            i32::MAX
        )))
    }
}

impl FlowPointerBodyCounts {
    /// Returns all three admitted external counts; e.g. `let counts = context.values()`.
    pub fn values(self) -> [u32; 3] {
        self.0
    }
    /// Returns the unwrapped mathematical total; e.g. `let count = context.total()`.
    pub fn total(self) -> u64 {
        self.0.into_iter().map(u64::from).sum()
    }
}

/// Explicit immutable runtime context and independent allocation/payload limits.
#[derive(Clone, Debug)]
pub struct FlowPointerBodyOptions {
    /// Caller-selected transport, without inferring its owner.
    pub compression: FlowLegacyTableTransport,
    /// Explicit leaf pool count, without tree-order assignment.
    pub leaf_count: FlowLeafCount,
    /// Explicit counts for the three physical body collections.
    pub group_counts: FlowPointerBodyCounts,
    /// Current runtime codes, required for nonempty legacy third collections, forbidden in tagged versions.
    pub entry_codes: Option<std::sync::Arc<[u8]>>,
    /// Complete decoded byte bound, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum physical leaf pools.
    pub maximum_pools: usize,
    /// Positive maximum total collection entries.
    pub maximum_entries: usize,
    /// Positive maximum byte-vector descriptors, including empty vectors.
    pub maximum_vectors: usize,
    /// Positive maximum total raw wide words in pools and bodies.
    pub maximum_wide_words: usize,
    /// Positive maximum opaque byte/vector payload bytes, excluding framing.
    pub maximum_raw_bytes: usize,
}
