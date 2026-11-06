use super::*;
use std::sync::Arc;

/// Explicit runtime mapping branch and independent physical allocation bounds.
#[derive(Clone, Copy, Debug)]
pub struct FlowReferenceGroupOptions {
    /// Caller-selected transport, without owner reconstruction.
    pub compression: FlowLegacyTableTransport,
    /// Explicit native +0x6F8 != -1 branch; no reset-state assumption is inferred.
    pub first_group_mapping_enabled: bool,
    /// Complete decoded byte bound, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum total entries across the two groups.
    pub maximum_entries: usize,
    /// Positive maximum raw signed reference words.
    pub maximum_reference_words: usize,
    /// Positive maximum total first/second array words.
    pub maximum_integer_words: usize,
    /// Positive maximum mapping words, separately from entry arrays.
    pub maximum_mapping_words: usize,
    /// Positive maximum appended wide patterns.
    pub maximum_wide_words: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ReferenceEntrySpan {
    pub full: (usize, usize),
    pub references: (usize, usize),
    pub member20: Option<usize>,
    pub continuation: EntryContinuationSpan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ReferenceGroupSpan {
    pub full: (usize, usize),
    pub entries: (usize, usize),
    pub level: Option<usize>,
    pub mapping: (usize, usize),
    pub wide: (usize, usize),
}

/// Immutable complete legacy reference lists with physical-order metadata only.
#[derive(Clone, Debug)]
pub struct FlowReferenceGroups {
    pub(super) wire: TableWire,
    pub(super) options: FlowReferenceGroupOptions,
    pub(super) headers: [Option<usize>; 2],
    pub(super) groups: [ReferenceGroupSpan; 2],
    pub(super) entries: Arc<[ReferenceEntrySpan]>,
    pub(super) word_counts: [usize; 2],
    pub(super) reference_words: usize,
    pub(super) mapping_words: usize,
    pub(super) wide_words: usize,
}

/// Borrowed physical group, including an empty group and its stored count.
#[derive(Clone, Copy, Debug)]
pub struct FlowReferenceGroup<'a> {
    pub(super) owner: &'a FlowReferenceGroups,
    pub(super) ordinal: usize,
    pub(super) span: &'a ReferenceGroupSpan,
}

/// Borrowed reference-list entry; stored reference words are not resolved pointers.
#[derive(Clone, Copy, Debug)]
pub struct FlowReferenceEntry<'a> {
    pub(super) owner: &'a FlowReferenceGroups,
    pub(super) group: usize,
    pub(super) ordinal: usize,
    pub(super) span: &'a ReferenceEntrySpan,
}

/// Existing header fields; neither field controls the fixed two-group loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowReferenceHeaderField {
    /// Discarded word in versions zero through five.
    Discarded,
    /// Stored native local in versions three through seventeen.
    Stored,
}

/// Existing raw entry fields, sharing the block-data scalar value codec.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowReferenceEntryField {
    /// Signed member 20 before version twelve; no record-406 alias is introduced.
    Member20,
    /// Signed raw reference at an existing ordinal.
    Reference(usize),
    /// Discarded byte in versions eleven through thirteen.
    DiscardedPrefix,
    /// Complete raw state byte.
    State,
    /// Complete selector byte from version five.
    Flag2,
    /// Complete selector byte in version seventeen.
    Flag4,
    /// Descriptor zero through six, returned as Word16 even for version-one bytes.
    Descriptor(usize),
    /// Raw first-array pattern at an existing ordinal.
    FirstWord(usize),
    /// Sign/zero-extended stored secondary count, returned as Signed32.
    SecondCount,
    /// Raw second-array pattern at an existing ordinal.
    SecondWord(usize),
    /// Discarded word from version sixteen.
    DiscardedSuffix,
}

/// Complete exact serialized coverage; context assumptions remain in the options.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowReferenceGroupCoverage {
    /// Complete raw or checksum-valid zlib check mode.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete stored payload bytes.
    pub stored_bytes: usize,
    /// Complete decoded payload bytes through exact EOF.
    pub decoded_bytes: usize,
    /// Physical entries across exactly two groups.
    pub entries: usize,
    /// Actual raw reference words.
    pub reference_words: usize,
    /// Actual first-array words.
    pub first_words: usize,
    /// Actual second-array words; negative counts contribute zero.
    pub second_words: usize,
    /// Actual mapping words.
    pub mapping_words: usize,
    /// Actual appended wide patterns.
    pub wide_words: usize,
    /// Exact serialized error, without approximation.
    pub achieved_error: usize,
}

/// Runtime and semantic exclusions from complete serialized coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowReferenceGroupDiagnostic {
    /// Caller mapping context does not prove native state provenance.
    MappingContextNotReconstructed,
    /// Native runtime allocation and flags are not reproduced.
    NativeRuntimeNotReproduced,
    /// Stored signed references are not repaired into native pointers.
    ReferenceFixupsNotReproduced,
    /// Stored levels and mappings are not shifted into runtime refinement state.
    RefinementPostprocessingNotReproduced,
    /// Raw patterns have no inferred engineering roles.
    EngineeringRolesNotResolved,
    /// Caller transport does not reconstruct its owner.
    OwnerCompressionContextNotReconstructed,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}

/// Completed isolated field edit with both typed checkpoints and physical history.
#[derive(Clone, Debug)]
pub struct FlowReferenceGroupEdit {
    pub(super) source: FlowReferenceGroups,
    pub(super) result: FlowReferenceGroups,
    pub(super) carrier_edit: FlowCarrierEdit,
}
