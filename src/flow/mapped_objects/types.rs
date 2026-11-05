use super::*;

/// Validated nonnegative signed-i32 runtime cardinality, without native matching credit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMatchedObjectCount(pub(super) u32);

impl TryFrom<u32> for FlowMatchedObjectCount {
    type Error = BessemerError;
    fn try_from(value: u32) -> Result<Self> {
        if value <= i32::MAX as u32 {
            return Ok(Self(value));
        }
        Err(mapped_failure(
            FailureClass::Invalid,
            format!("matched count {value}; expected 0 through {}", i32::MAX),
        ))
    }
}

impl FlowMatchedObjectCount {
    /// Returns the validated external count; e.g. `let count = matched.value()`.
    pub fn value(self) -> u32 {
        self.0
    }
}

/// External framing context, retained without inference from identifier pairs.
#[derive(Clone, Debug)]
pub enum FlowMappedContext {
    /// No matched counts supplied; positive factors require a typed missing-context failure.
    Unavailable,
    /// Exactly one optional count per physical entry; null entries require None.
    Aligned(Arc<[Option<FlowMatchedObjectCount>]>),
}

/// Positive total allocation bounds for the complete selected list.
#[derive(Clone, Copy, Debug)]
pub struct FlowMappedLimits {
    /// Maximum physical entries, including null entries.
    pub maximum_entries: usize,
    /// Maximum total stored identifier pairs across all bodies.
    pub maximum_pairs: usize,
    /// Maximum total numeric words across fixed, mapped and legacy blocks.
    pub maximum_numeric_words: usize,
}

/// One unchanged stored identifier/value pair; duplicates remain physical entries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMappedIdentifierPair {
    pub(super) source: i32,
    pub(super) associated: i32,
}

/// Complete numeric block, retaining exact binary64 patterns and payload position.
#[derive(Clone, Debug)]
pub struct FlowMappedNumericBlock {
    pub(super) offset: usize,
    pub(super) bits: Arc<[u64]>,
}

/// Body-version-5-only wire block, including words discarded by native runtime storage.
#[derive(Clone, Debug)]
pub struct FlowMappedLegacy {
    pub(super) prefix: FlowMappedNumericBlock,
    pub(super) arrays: [FlowMappedNumericBlock; 3],
    pub(super) tripled: FlowMappedNumericBlock,
}

/// Complete populated entry body with explicit supplied matched count.
#[derive(Clone, Debug)]
pub struct FlowMappedBody {
    pub(super) version: u32,
    pub(super) state: u8,
    pub(super) numeric: FlowMappedNumericBlock,
    pub(super) pairs: Arc<[FlowMappedIdentifierPair]>,
    pub(super) pair_offset: usize,
    pub(super) factor: u32,
    pub(super) matched: Option<FlowMatchedObjectCount>,
    pub(super) mapped: FlowMappedNumericBlock,
    pub(super) legacy: Option<FlowMappedLegacy>,
}

/// One physical entry, with a null entry distinct from a populated empty mapped array.
#[derive(Clone, Debug)]
pub struct FlowMappedEntry {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) presence: u8,
    pub(super) body: Option<FlowMappedBody>,
}

/// Complete immutable selected list and all external framing inputs.
#[derive(Clone, Debug)]
pub struct FlowMappedObjects {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) payload: Arc<[u8]>,
    pub(super) version: u32,
    pub(super) entries: Arc<[FlowMappedEntry]>,
    pub(super) context: FlowMappedContext,
    pub(super) limits: FlowMappedLimits,
    pub(super) coverage: FlowMappedCoverage,
}

/// Exact raw framing check scope, without native runtime-map validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMappedCheckMode {
    /// Complete raw entries, bounded products, numeric bits and payload EOF.
    ExactExplicitContextFrames,
}

/// Total stored checks and explicit external-context coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMappedCoverage {
    /// Exact framing scope.
    pub mode: FlowMappedCheckMode,
    /// Entire selected payload extent.
    pub checked_bytes: usize,
    /// All physical entries.
    pub checked_entries: usize,
    /// All retained identifier pairs.
    pub checked_pairs: usize,
    /// All fixed, mapped and legacy numeric words.
    pub checked_numeric_words: usize,
    /// Populated bodies with an explicit caller-supplied count.
    pub supplied_matched_counts: usize,
    /// Zero-factor bodies whose runtime count remains unknown.
    pub unresolved_zero_factor_counts: usize,
    /// Exact stored-byte disagreement, zero for completed views.
    pub achieved_byte_error: usize,
}

/// Explicit exclusions from exact supplied-context wire framing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMappedDiagnostic {
    /// Supplied cardinalities were not matched against actual native runtime objects.
    NativeObjectContextNotValidated,
    /// Numeric meanings, axes, units and concrete context-object classes remain unknown.
    EngineeringSemanticsNotEstablished,
    /// No native loader, solver or application accepted these results.
    NativeApplicationNotChecked,
}

/// One existing numeric block selected by its wire role.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMappedNumericField {
    /// Nine-word fixed body prefix.
    Prefix,
    /// Factor × explicit matched-count words.
    Mapped,
    /// Version-5 sixteen-word prefix.
    LegacyPrefix,
    /// Version-5 first counted array.
    LegacyFirst,
    /// Version-5 second counted array.
    LegacySecond,
    /// Version-5 third counted array.
    LegacyThird,
    /// Version-5 tripled product words.
    LegacyTripled,
}

/// Existing fixed-width field edit; structural counts, versions and presence are unchanged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMappedFieldEdit {
    /// Complete raw state byte.
    State(u8),
    /// Existing pair's original source identifier.
    PairSource {
        /// Existing pair position within the selected body.
        index: usize,
        /// Replacement signed interpretation, preserving all four wire bytes.
        value: i32,
    },
    /// Existing pair's associated value.
    PairValue {
        /// Existing pair position within the selected body.
        index: usize,
        /// Replacement associated signed interpretation.
        value: i32,
    },
    /// One exact binary64 wire pattern, without floating-point normalization.
    Numeric {
        /// Existing numeric block selected by its wire role.
        field: FlowMappedNumericField,
        /// Existing word position within that block.
        index: usize,
        /// Complete replacement binary64 pattern, including NaN payload bits.
        bits: u64,
    },
}

/// Completed isolated edit with source/result typed views and complete carrier history.
#[derive(Clone, Debug)]
pub struct FlowMappedEdit {
    pub(super) source: FlowMappedObjects,
    pub(super) result: FlowMappedObjects,
    pub(super) carrier_edit: FlowCarrierEdit,
}
