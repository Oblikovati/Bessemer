use super::*;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BodySpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BodyPoolSpan {
    pub full: BodySpan,
    pub wide: BodySpan,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BodyEntrySpan {
    pub full: BodySpan,
    pub payload: BodySpan,
    pub vectors: BodySpan,
    pub kind: FlowPointerBodyKind,
    pub stored_count: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct BodyGroupSpan {
    pub full: BodySpan,
    pub entries: BodySpan,
}

/// Immutable exact legacy record-421 view; ordinal paths refer to physical wire order.
#[derive(Clone, Debug)]
pub struct FlowPointerBodies {
    pub(super) wire: TableWire,
    pub(super) options: FlowPointerBodyOptions,
    pub(super) pools: Arc<[BodyPoolSpan]>,
    pub(super) entries: Arc<[BodyEntrySpan]>,
    pub(super) vectors: Arc<[BodySpan]>,
    pub(super) groups: [BodyGroupSpan; 3],
    pub(super) wide_words: usize,
    pub(super) raw_bytes: usize,
}

/// Branch selected by explicit legacy runtime codes or the stored tagged layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPointerBodyKind {
    /// One byte-counted byte vector in legacy collections zero and one.
    SingleVector,
    /// Byte-counted raw wide triplets in the legacy third collection.
    WideTriplets,
    /// Byte-counted collection of byte-counted vectors in the legacy third collection.
    MultipleVectors,
    /// Version-three tag two, with one more wide word than three times its count.
    TaggedWide,
    /// Recognized tag one/two with an uninterpreted byte payload.
    TaggedRaw,
    /// Unknown tag with no count or payload.
    UnknownTag,
}

/// Borrowed complete physical leaf pool, including an empty pool.
#[derive(Clone, Copy, Debug)]
pub struct FlowPointerBodyPool<'a> {
    pub(super) owner: &'a FlowPointerBodies,
    pub(super) span: &'a BodyPoolSpan,
}

/// Borrowed body entry with decoded intervals and context-selected payload shape.
#[derive(Clone, Copy, Debug)]
pub struct FlowPointerBodyEntry<'a> {
    pub(super) owner: &'a FlowPointerBodies,
    pub(super) group: usize,
    pub(super) ordinal: usize,
    pub(super) span: &'a BodyEntrySpan,
}

/// Exact serialized coverage; declared context is retained in the view's options.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowPointerBodyCoverage {
    /// Complete raw or checksum-valid zlib mode.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete original payload bytes.
    pub stored_bytes: usize,
    /// Complete decoded bytes through exact EOF.
    pub decoded_bytes: usize,
    /// Physical leaf pools.
    pub pools: usize,
    /// Physical collection entries.
    pub entries: usize,
    /// Byte vectors, including empty ones.
    pub vectors: usize,
    /// Actual raw wide words.
    pub wide_words: usize,
    /// Actual opaque payload bytes.
    pub raw_bytes: usize,
    /// Exact serialized error, without approximation.
    pub achieved_error: usize,
}

/// Explicit exclusions accompanying complete serialized coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowPointerBodyDiagnostic {
    /// Caller runtime codes do not reconstruct mutable native state.
    NativeRuntimeNotReproduced,
    /// Native orientation postprocessing was not executed.
    OrientationPostprocessingNotReproduced,
    /// Physical leaf order is not assigned to tree traversal.
    TreeOrderNotAssigned,
    /// Raw fields have no inferred engineering roles.
    EngineeringRolesNotResolved,
    /// Tagged raw bytes remain opaque, even when they resemble compressed bytes.
    PackedPayloadNotDecoded,
    /// Caller transport does not reconstruct the compression owner.
    OwnerCompressionContextNotReconstructed,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}

/// Completed isolated edit with typed checkpoints and complete physical history.
#[derive(Clone, Debug)]
pub struct FlowPointerBodyEdit {
    pub(super) source: FlowPointerBodies,
    pub(super) result: FlowPointerBodies,
    pub(super) carrier_edit: FlowCarrierEdit,
}
