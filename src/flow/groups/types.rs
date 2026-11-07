use super::*;

/// Explicit native owner-selected compression profile; never inferred from bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGroupCompression {
    /// Ordinary uncompressed payload.
    Raw,
    /// Exactly one complete checksum-validated RFC 1950 stream.
    Zlib,
}

/// Admitted payload dialect, independent of the carrier's two-byte version word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGroupVersion {
    /// Separate word/wide arrays without wire flags.
    V3,
    /// Auxiliary i32 and separate arrays without wire flags.
    V4,
    /// Separate word/wide/flag arrays.
    V5,
    /// Thirteen-byte interleaved word/wide/flag members.
    V6,
}

impl FlowGroupVersion {
    /// Returns the stored signed version; e.g. `assert_eq!(FlowGroupVersion::V6.wire_value(), 6)`.
    pub fn wire_value(self) -> i32 {
        match self {
            Self::V3 => 3,
            Self::V4 => 4,
            Self::V5 => 5,
            Self::V6 => 6,
        }
    }
}

/// Positive operation-local resource bounds and explicit compression context.
#[derive(Clone, Copy, Debug)]
pub struct FlowGroupOptions {
    /// Explicit owner-selected profile.
    pub compression: FlowGroupCompression,
    /// Maximum complete decoded bytes, at least eight.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum group count, including empty groups.
    pub maximum_groups: usize,
    /// Positive maximum sum of members across all groups.
    pub maximum_members: usize,
}

/// Complete serialized check scope, distinct from native context or semantic validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGroupCheckMode {
    /// Raw selected payload and exact versioned group EOF.
    ExactRawGroupedFrames,
    /// One complete RFC 1950 stream and exact versioned group EOF.
    ExactZlibGroupedFrames,
}

/// Complete selected record checks and exact output disagreement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowGroupCoverage {
    /// Required structural check scope.
    pub mode: FlowGroupCheckMode,
    /// Complete stored payload extent.
    pub stored_bytes: usize,
    /// Complete decoded payload extent.
    pub decoded_bytes: usize,
    /// Every group, including empty groups.
    pub checked_groups: usize,
    /// Sum of member counts across all groups.
    pub checked_members: usize,
    /// Successful exact structural/edit checks require zero disagreement.
    pub achieved_byte_error: usize,
}

/// Checks not implied by successful grouped-word decoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGroupDiagnostic {
    /// Word/value/auxiliary/flag domains, units and generated native flags remain unknown.
    MemberSemanticsNotEstablished,
    /// Compression is explicitly supplied, without reconstructing native owner state.
    OwnerCompressionContextNotReconstructed,
    /// Native application and geometry acceptance were not checked.
    NativeApplicationNotChecked,
}

#[derive(Clone, Debug)]
pub(super) struct GroupSpan {
    pub start: usize,
    pub end: usize,
    pub values: usize,
    pub members: usize,
}

#[derive(Debug)]
pub(super) struct GroupRevision {
    pub source: FlowCarrier,
    pub record: FlowRecordId,
    pub decoded: Arc<[u8]>,
    pub spans: Vec<GroupSpan>,
    pub version: FlowGroupVersion,
    pub options: FlowGroupOptions,
    pub stored_bytes: usize,
    pub members: usize,
}

/// Immutable complete record-404 payload with retained carrier and validated group spans.
#[derive(Clone, Debug)]
pub struct FlowGroupedRecord(pub(super) Arc<GroupRevision>);

/// Borrowed exact group view; no native defaults or engineering types are synthesized.
#[derive(Clone, Debug)]
pub struct FlowGroupView<'a> {
    pub(super) revision: &'a GroupRevision,
    pub(super) span: &'a GroupSpan,
    pub(super) ordinal: usize,
}

/// Exactly one existing wire field and its explicitly typed replacement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGroupFieldEdit {
    /// Four-byte unsigned bit pattern in one member.
    Word {
        /// Zero-based existing member ordinal.
        member: usize,
        /// Complete replacement bit pattern.
        value: u32,
    },
    /// Eight-byte raw bit pattern, without binary64 interpretation.
    WideWord {
        /// Zero-based existing member ordinal.
        member: usize,
        /// Complete replacement bit pattern.
        value: u64,
    },
    /// One stored flag byte, available only for versions 5/6.
    Flag {
        /// Zero-based existing member ordinal.
        member: usize,
        /// Complete stored byte, without a Boolean-domain restriction.
        value: u8,
    },
    /// Version-4 auxiliary i32, including in empty groups.
    Auxiliary(i32),
}

/// Exact wire scalar with its kind retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGroupValue {
    /// Four-byte raw word.
    Word(u32),
    /// Eight-byte raw word.
    WideWord(u64),
    /// Stored byte, without a Boolean-domain assumption.
    Flag(u8),
    /// Raw signed auxiliary word.
    Auxiliary(i32),
}

/// Selected field's complete source/result provenance.
#[derive(Clone, Debug)]
pub struct FlowGroupFieldHistory {
    /// Exact source occurrence.
    pub source_record: FlowRecordId,
    /// Exact corresponding result occurrence.
    pub result_record: FlowRecordId,
    /// Validated group ordinal.
    pub group: usize,
    /// Member ordinal, absent for the auxiliary field.
    pub member: Option<usize>,
    /// Complete original scalar.
    pub before: FlowGroupValue,
    /// Complete requested scalar.
    pub after: FlowGroupValue,
    /// Copied for a no-op, otherwise derived from this source field.
    pub relation: FlowPayloadRelation,
}

/// Completed isolated field edit with exact source checkpoint and complete carrier history.
#[derive(Clone, Debug)]
pub struct FlowGroupEdit {
    pub(super) source: FlowGroupedRecord,
    pub(super) result: FlowGroupedRecord,
    pub(super) carrier: FlowCarrierEdit,
    pub(super) history: FlowGroupFieldHistory,
}

/// Compression-specific limitation of fresh opposite edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGroupEditDiagnostic {
    /// Logical reversal can use a different zlib encoding; exact original bytes remain in the source checkpoint.
    FreshReverseWireIdentityNotGuaranteed,
}
