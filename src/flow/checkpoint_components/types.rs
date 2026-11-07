use super::*;
/// Caller-supplied logical parent context; no physical record context is inferred.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCheckpointContext {
    /// Logical buffers supplied by the documented version-six checkpoint graph.
    LogicalVersionSix,
}
/// Explicit standalone logical layout, independent of physical record identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCheckpointKind {
    /// Exactly seven raw 64-bit patterns.
    Graph1,
    /// Cyclic grid with indices and tagged numeric-container slots.
    Graph2,
    /// Cyclic grid with indices and no slot stream.
    Auxiliary,
    /// Exactly three numeric containers.
    TrailingContainers,
    /// Version-zero encoded chunks; codec and raw-size agreement remain unchecked.
    ChunkBuffer,
}
/// Independent complete-input and aggregate entity bounds; zero admits only absent entities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowCheckpointOptions {
    /// Complete standalone input bytes, checked before copying.
    pub maximum_component_bytes: usize,
    /// Aggregate dimension-derived row positions, including absent rows.
    pub maximum_row_positions: usize,
    /// Aggregate raw 32-bit indices across rows and containers.
    pub maximum_indices: usize,
    /// Aggregate stored slots, including bodyless slots.
    pub maximum_slots: usize,
    /// Aggregate numeric containers, including empty containers.
    pub maximum_containers: usize,
    /// Aggregate numeric-array raw 64-bit patterns, including graph one's seven values.
    pub maximum_wide_patterns: usize,
    /// Aggregate encoded chunks, including empty chunks.
    pub maximum_chunks: usize,
    /// Aggregate encoded bytes; does not bound or validate decoded codec output.
    pub maximum_encoded_bytes: usize,
}
/// Exact stored totals; scalar headers and signed members are separate from numeric-array totals.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FlowCheckpointCounts {
    /// All dimension-derived row positions.
    pub row_positions: usize,
    /// Positions with a nonzero presence byte.
    pub present_rows: usize,
    /// All stored row and numeric-container indices.
    pub indices: usize,
    /// All graph-two slots.
    pub slots: usize,
    /// All numeric containers, including empty ones.
    pub containers: usize,
    /// All numeric-array 64-bit patterns.
    pub wide_patterns: usize,
    /// All encoded chunks.
    pub chunks: usize,
    /// Complete encoded chunk bytes.
    pub encoded_bytes: usize,
    /// Slots with tags greater than two, accepted by the documented reader.
    pub noncanonical_tags: usize,
}
/// Required serialized check scope, excluding the complete physical parent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCheckpointCheckMode {
    /// Complete logical component framing through exact EOF.
    ExactLogicalComponent,
}
/// Complete component coverage with explicit parent, semantic and codec exclusions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowCheckpointCoverage {
    /// Exact logical framing checks.
    pub mode: FlowCheckpointCheckMode,
    /// Explicit caller declaration.
    pub context: FlowCheckpointContext,
    /// Selected logical profile.
    pub kind: FlowCheckpointKind,
    /// Complete component bytes.
    pub component_bytes: usize,
    /// Aggregate stored totals.
    pub counts: FlowCheckpointCounts,
    /// Always true for a successful standalone component.
    pub complete_component_framing: bool,
    /// Always false: no complete physical parent was decoded.
    pub complete_parent_record: bool,
    /// Always false: raw words have not been assigned engineering meanings.
    pub engineering_fields_assigned: bool,
    /// Always false: encoded chunks remain opaque.
    pub chunk_codec_decoded: bool,
    /// Always false: declared raw size has not been validated.
    pub decoded_size_verified: bool,
    /// Zero for exact serialized checks, not a geometric error claim.
    pub achieved_byte_error: usize,
}
/// Native and semantic limitations retained after every read and edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCheckpointDiagnostic {
    /// Logical version-six context comes from the caller.
    LogicalContextAssumed,
    /// No installed version-six native component fixture has been verified.
    InstalledNativeComponentNotVerified,
    /// The physical parent record has not been decoded.
    ParentRecordNotDecoded,
    /// Scalar engineering meanings remain unassigned.
    EngineeringMeaningsNotAssigned,
    /// Native runtime allocations and pointer state were not reproduced.
    NativeRuntimeNotReproduced,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
    /// Reader-permitted tags above two do not establish writer canonicality.
    NoncanonicalWriterTagsRetained,
    /// Chunk codec output remains unavailable.
    ChunkCodecNotDecoded,
    /// Declared raw size has not been checked against decoded bytes.
    DeclaredRawSizeNotVerified,
}
/// Exact serialized field class; signed integers and raw patterns remain distinct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowCheckpointValue {
    /// Arbitrary signed member.
    Signed32(i32),
    /// Raw unsigned 32-bit index pattern.
    Pattern32(u32),
    /// Raw unsigned 64-bit pattern or declared raw-size word.
    Pattern64(u64),
    /// Raw presence, slot tag or encoded byte.
    Byte(u8),
}
/// Immutable complete standalone logical component with operation-local parsing state.
#[derive(Clone, Debug)]
pub struct FlowCheckpointComponent {
    pub(super) revision: Arc<CheckpointRevision>,
}
/// Revision-bound editable field; debug output excludes retained buffers.
#[derive(Clone)]
pub struct FlowCheckpointField {
    pub(super) revision: Arc<CheckpointRevision>,
    pub(super) start: usize,
    pub(super) shape: ValueShape,
    pub(super) rule: ValueRule,
}
/// Complete source/result byte relation; enclosing edits retain both exact revisions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowCheckpointByteHistory {
    /// Half-open interval in the retained source component.
    pub source_interval: (usize, usize),
    /// Half-open interval in the retained result component.
    pub result_interval: (usize, usize),
    /// Copied or derived bytes with one complete source parent.
    pub relation: FlowPayloadRelation,
}
/// Isolated existing-field edit with complete byte history and exact field correspondence.
#[derive(Clone, Debug)]
pub struct FlowCheckpointComponentEdit {
    pub(super) source: FlowCheckpointComponent,
    pub(super) result: FlowCheckpointComponent,
    pub(super) source_field: FlowCheckpointField,
    pub(super) result_field: FlowCheckpointField,
    pub(super) history: Vec<FlowCheckpointByteHistory>,
}
