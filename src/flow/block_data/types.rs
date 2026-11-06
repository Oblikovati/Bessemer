use super::*;

/// External leaf cardinality parsed once into the native signed-count domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLeafCount(u32);

impl TryFrom<u32> for FlowLeafCount {
    type Error = BessemerError;
    fn try_from(value: u32) -> Result<Self> {
        if value <= i32::MAX as u32 {
            return Ok(Self(value));
        }
        Err(invalid_block(format!(
            "leaf count {value}; expected 0 through {}",
            i32::MAX
        )))
    }
}

impl FlowLeafCount {
    /// Returns the validated count; e.g. `let count = leaf_count.value()`.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Shared explicit raw/zlib transport.
pub type FlowBlockDataCompression = FlowGroupCompression;

/// External leaf count and independent complete-payload allocation limits.
#[derive(Clone, Copy, Debug)]
pub struct FlowBlockDataOptions {
    /// Owner-selected transport, never guessed from payload signatures.
    pub compression: FlowBlockDataCompression,
    /// Explicit supplied context; never inferred from global prefix totals.
    pub leaf_count: FlowLeafCount,
    /// Complete decoded payload limit, at least twelve bytes.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum physical blocks.
    pub maximum_blocks: usize,
    /// Positive maximum entries across every block.
    pub maximum_entries: usize,
    /// Positive maximum words across both arrays in every entry.
    pub maximum_integer_words: usize,
}

/// Observed strict count prefix; mathematical and wrapped totals remain distinct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowBlockCountPrefix {
    /// Legacy body version, one through seventeen.
    pub version: i32,
    /// Nonnegative native G.Fluid label, without inferred engineering meaning.
    pub g_fluid: i32,
    /// Nonnegative native G.Solid label, without inferred engineering meaning.
    pub g_solid: i32,
    /// Exact mathematical sum in the nonnegative strict profile.
    pub mathematical_total: u64,
    /// Native signed 32-bit wrapped sum used for the bitmap branch.
    pub native_total: i32,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct BlockSpan {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) first_entry: usize,
    pub(super) entry_end: usize,
}

#[derive(Clone, Debug)]
pub(super) struct EntrySpan {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) scalars: [Option<usize>; 10],
    pub(super) descriptors: usize,
    pub(super) first: (usize, usize),
    pub(super) second: (usize, usize),
}

/// Immutable exact serialized legacy body with source checkpoint and flat metadata.
#[derive(Clone, Debug)]
pub struct FlowLegacyBlockData {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) decoded: Arc<[u8]>,
    pub(super) prefix: FlowBlockCountPrefix,
    pub(super) bitmap_end: usize,
    pub(super) blocks: Arc<[BlockSpan]>,
    pub(super) entries: Arc<[EntrySpan]>,
    pub(super) options: FlowBlockDataOptions,
    pub(super) coverage: FlowBlockDataCoverage,
}

/// Borrowed physical block in wire order, without a tree-node assignment.
#[derive(Clone, Copy, Debug)]
pub struct FlowLegacyBlock<'a> {
    pub(super) owner: &'a FlowLegacyBlockData,
    pub(super) ordinal: usize,
    pub(super) span: &'a BlockSpan,
}

/// Borrowed entry in flat wire order, retaining every stored field.
#[derive(Clone, Copy, Debug)]
pub struct FlowLegacyBlockEntry<'a> {
    pub(super) owner: &'a FlowLegacyBlockData,
    pub(super) ordinal: usize,
    pub(super) span: &'a EntrySpan,
}

/// Borrowed exact raw unsigned 32-bit array with decoded offsets.
#[derive(Clone, Copy, Debug)]
pub struct FlowBlockWordArray<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) start: usize,
}

/// Complete exact wire check scope, separate from native runtime acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockDataCheckMode {
    /// Every raw payload byte through exact EOF.
    ExactRawLegacyBody,
    /// Every checksum-validated RFC 1950 byte and decoded byte through exact EOF.
    ExactZlibLegacyBody,
}

/// Exact serialized validation counts without geometry acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowBlockDataCoverage {
    /// Complete check mode.
    pub mode: FlowBlockDataCheckMode,
    /// All stored payload bytes.
    pub stored_bytes: usize,
    /// All decoded payload bytes.
    pub decoded_bytes: usize,
    /// Physical blocks checked under the external count.
    pub checked_blocks: usize,
    /// Total physically present entries.
    pub checked_entries: usize,
    /// Total first-array words.
    pub first_integer_words: usize,
    /// Total second-array words.
    pub second_integer_words: usize,
    /// Exact byte check, without approximation.
    pub achieved_error: u64,
}

/// Acceptance exclusions for the complete legacy framing profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowBlockDataDiagnostic {
    /// Leaf count was supplied explicitly; native runtime cardinality was not executed.
    NativeLeafContextNotExecuted,
    /// Wire block ordinals are not assigned to tree nodes.
    BlockToTreeOrderNotAssigned,
    /// Native runtime flags and allocations are not reconstructed.
    NativeRuntimeStateNotReproduced,
    /// Prefix totals do not prove record 433's runtime count.
    Record433ContextNotProven,
    /// Compression context was supplied explicitly.
    OwnerCompressionContextNotReconstructed,
    /// No native application acceptance or geometry validation.
    NativeApplicationNotChecked,
}

/// Native block-byte operation observed in the versioned reader, not executed here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowNativeBlockByteOperation {
    /// Versions thirteen and later assign the stored byte.
    Assign,
    /// Earlier versions mask runtime state with 0xe7 rather than assigning the stored byte.
    AndE7OfRuntimeValue,
}

/// Completed edit with immutable source/result views and full physical history.
#[derive(Clone, Debug)]
pub struct FlowBlockDataEdit {
    pub(super) source: FlowLegacyBlockData,
    pub(super) result: FlowLegacyBlockData,
    pub(super) carrier_edit: FlowCarrierEdit,
}
