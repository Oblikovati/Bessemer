use super::*;

/// Explicit raw/zlib transport shared with other Flow schemas.
pub type FlowLegacyTableTransport = FlowGroupCompression;

/// Caller-supplied native nonnegative signed pointer-array count; no pointer population is implied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowGlobalPointerCount(u32);

impl TryFrom<u32> for FlowGlobalPointerCount {
    type Error = BessemerError;
    fn try_from(value: u32) -> Result<Self> {
        if value <= i32::MAX as u32 {
            return Ok(Self(value));
        }
        Err(table_failure(
            401,
            FailureClass::Invalid,
            format!(
                "global pointer count {value}; expected zero through {}",
                i32::MAX
            ),
        ))
    }
}

impl FlowGlobalPointerCount {
    /// Returns the validated bound; e.g. `let bound = count.value()`.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Explicit G.Fluid/G.Solid count context admitted once into the native signed allocation profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLegacyWordCounts {
    pair: [u32; 2],
    total: u32,
}

impl TryFrom<[u32; 2]> for FlowLegacyWordCounts {
    type Error = BessemerError;
    fn try_from(pair: [u32; 2]) -> Result<Self> {
        let total: u64 = pair.into_iter().map(u64::from).sum();
        if total <= i32::MAX as u64 {
            return Ok(Self {
                pair,
                total: total as u32,
            });
        }
        Err(table_failure(
            405,
            FailureClass::Invalid,
            format!(
                "legacy G counts {pair:?}, sum {total}; expected two nonnegative native signed counts with sum <= {}",
                i32::MAX
            ),
        ))
    }
}

impl FlowLegacyWordCounts {
    /// Returns original fluid/solid counts; e.g. `let pair = counts.pair()`.
    pub fn pair(self) -> [u32; 2] {
        self.pair
    }
    /// Returns the validated mathematical sum; e.g. `let count = counts.total()`.
    pub fn total(self) -> u32 {
        self.total
    }
}

/// Independent transport, optional pointer-index context and total allocation bounds.
#[derive(Clone, Copy, Debug)]
pub struct FlowPointerTableOptions {
    /// Caller-selected compression.
    pub compression: FlowLegacyTableTransport,
    /// Optional validated range; absence preserves unbounded signed pair values.
    pub global_pointer_count: Option<FlowGlobalPointerCount>,
    /// Maximum complete decoded bytes, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum pair count across both groups.
    pub maximum_pairs: usize,
    /// Positive maximum raw wide words across all six arrays.
    pub maximum_wide_words: usize,
}

/// Explicit legacy context and independent raw-word table bounds.
#[derive(Clone, Copy, Debug)]
pub struct FlowLegacyWordOptions {
    /// Caller-selected compression.
    pub compression: FlowLegacyTableTransport,
    /// Required only for version zero; version one rejects unused context.
    pub legacy_counts: Option<FlowLegacyWordCounts>,
    /// Maximum complete decoded bytes, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum complete raw-word count.
    pub maximum_words: usize,
}

#[derive(Clone, Debug)]
pub(super) struct TableWire {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) decoded: Arc<[u8]>,
    pub(super) version: i32,
    pub(super) stored_bytes: usize,
    pub(super) compression: FlowLegacyTableTransport,
    pub(super) identifier: u32,
}

#[derive(Clone, Debug)]
pub(super) struct GroupSpan {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) count: usize,
    pub(super) discarded_count: Option<usize>,
    pub(super) body: Option<GroupBodySpan>,
}

#[derive(Clone, Debug)]
pub(super) struct GroupBodySpan {
    pub(super) members: usize,
    pub(super) lengths: [usize; 3],
    pub(super) discarded: (usize, usize),
    pub(super) arrays: [(usize, usize); 3],
    pub(super) pre_pairs: Option<usize>,
    pub(super) pairs: (usize, usize),
}

/// Immutable complete legacy record-401 table, retaining exactly two groups.
#[derive(Clone, Debug)]
pub struct FlowPointerTables {
    pub(super) wire: TableWire,
    pub(super) groups: [GroupSpan; 2],
    pub(super) options: FlowPointerTableOptions,
    pub(super) pair_count: usize,
    pub(super) wide_count: usize,
}

/// Immutable complete legacy record-405 raw words and original context.
#[derive(Clone, Debug)]
pub struct FlowLegacyWords {
    pub(super) wire: TableWire,
    pub(super) options: FlowLegacyWordOptions,
    pub(super) words_start: usize,
    pub(super) count: usize,
}

/// Borrowed group header, preserving empty groups without inventing bodies.
#[derive(Clone, Copy, Debug)]
pub struct FlowPointerTableGroup<'a> {
    pub(super) owner: &'a FlowPointerTables,
    pub(super) ordinal: usize,
    pub(super) span: &'a GroupSpan,
}

/// Borrowed populated-group fields, arrays and ordered pair multiplicity.
#[derive(Clone, Copy, Debug)]
pub struct FlowPointerGroupBody<'a> {
    pub(super) owner: &'a FlowPointerTables,
    pub(super) span: &'a GroupBodySpan,
}

/// Borrowed exact raw wide-word array and decoded offset.
#[derive(Clone, Copy, Debug)]
pub struct FlowTableWideArray<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) start: usize,
}

/// Complete exact serialized table scope, without native pointer or engineering acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLegacyTableCheckMode {
    /// Every raw payload byte through exact table EOF.
    ExactRawTable,
    /// Complete checksum-valid RFC 1950 stream and exact decoded table EOF.
    ExactZlibTable,
}

/// Exact serialized counts and independently supplied context coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLegacyTableCoverage {
    /// Complete exact check mode.
    pub mode: FlowLegacyTableCheckMode,
    /// Every stored payload byte.
    pub stored_bytes: usize,
    /// Every decoded payload byte.
    pub decoded_bytes: usize,
    /// Complete pairs across record 401, otherwise zero.
    pub pointer_pairs: usize,
    /// Complete wide words across record 401, otherwise zero.
    pub wide_words: usize,
    /// Complete record-405 raw words, otherwise zero.
    pub raw_words: usize,
    /// Supplied pair-index range, including a present zero bound.
    pub pointer_range: Option<FlowGlobalPointerCount>,
    /// Explicit version-zero G-count context, without inferred provenance.
    pub external_word_counts: Option<FlowLegacyWordCounts>,
    /// Exact serialized byte error, without approximation.
    pub achieved_error: usize,
}

/// Native/semantic exclusions accompanying table reads and edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLegacyTableDiagnostic {
    /// Index range checking does not populate or resolve native pointers.
    NativePointerPopulationNotReproduced,
    /// Raw record-405 word roles remain unresolved.
    WordRolesNotResolved,
    /// Compression was supplied without reconstructing its native owner.
    OwnerCompressionContextNotReconstructed,
    /// Native application and geometry acceptance were not executed.
    NativeApplicationNotChecked,
}

/// Completed isolated pointer-table edit with full carrier history.
#[derive(Clone, Debug)]
pub struct FlowPointerTableEdit {
    pub(super) source: FlowPointerTables,
    pub(super) result: FlowPointerTables,
    pub(super) carrier_edit: FlowCarrierEdit,
}

/// Completed isolated raw-word edit with full carrier history.
#[derive(Clone, Debug)]
pub struct FlowLegacyWordEdit {
    pub(super) source: FlowLegacyWords,
    pub(super) result: FlowLegacyWords,
    pub(super) carrier_edit: FlowCarrierEdit,
}
