use crate::archive_codec::ArchiveRevision;

/// Explicit bare-stream grammar; header length or payload size never selects the profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistScalarProfile {
    /// Signed i32 count and ordered signed i32 StudyVersionNumbers values.
    CountedI32,
    /// Signed i32 used/capacity header and ordered signed i64 UniqueIds keys.
    UniqueIdsI64,
}

/// Exact signed word width without implicit narrowing, widening or release/identifier mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistScalarValue {
    /// Complete four-byte signed pattern.
    I32(i32),
    /// Complete eight-byte signed pattern.
    I64(i64),
}

/// Stored role, without assigning runtime study or CAD-reference meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistScalarRole {
    /// Nonnegative stored count tied to the complete payload length.
    UsedCount,
    /// Retained allocation metadata, never a payload or allocation extent.
    AllocationCapacity,
    /// One ordered payload entry, including duplicate values.
    Entry(usize),
}

/// Immutable identity of one complete scalar archive and its declared profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPersistScalarRevision(pub(super) ArchiveRevision);

/// Once-normalized stored field position tied to its original revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPersistScalarId {
    pub(super) revision: SimulationPersistScalarRevision,
    pub(super) ordinal: usize,
}

/// One exact stored header or payload value, retaining physical position and typed role.
#[derive(Debug, Clone)]
pub struct SimulationPersistScalarField {
    /// Revision-bound identity.
    pub id: SimulationPersistScalarId,
    /// Exact offset in the complete original bare stream.
    pub stream_offset: usize,
    /// Typed header or ordered-entry role.
    pub role: SimulationPersistScalarRole,
    /// Original full-width signed pattern.
    pub value: SimulationPersistScalarValue,
}

/// Successful complete empty and populated arrays remain distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistScalarCompletion {
    /// Complete header and no payload entries, including metadata-only capacity.
    CompletedEmpty,
    /// Every declared entry consumed at exact EOF.
    CompletedValues,
}

/// Exact scope establishing only the declared bare-stream grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistScalarCheckMode {
    /// Header constraints, explicit resource limit, all values and exact EOF.
    ExactCompleteFrame,
    /// Complete frame plus literal preservation outside one exact stored field.
    ExactCompleteFrameAndPreservation,
}

/// Explicit exclusions from structural scalar framing acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistScalarDiagnostic {
    /// No installed loader, allocator, mapper or COM operation executed.
    NativeApplicationNotExecuted,
    /// No version selection/fallback, last-key cache or reference resolution is inferred.
    SemanticMappingNotInferred,
    /// Allocation capacity never drives memory allocation or payload consumption.
    CapacityNotAllocated,
}

/// Complete exact framing scope, explicit context, diagnostics and achieved byte error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPersistScalarCoverage {
    /// Original explicitly selected grammar.
    pub profile: SimulationPersistScalarProfile,
    /// Complete empty versus complete populated result.
    pub completion: SimulationPersistScalarCompletion,
    /// Exact frame or frame/preservation checks.
    pub check_mode: SimulationPersistScalarCheckMode,
    /// Complete checked original image extent.
    pub checked_bytes: usize,
    /// Exact four/eight-byte original header extent.
    pub checked_header_bytes: usize,
    /// All payload entries checked, independent of allocation capacity.
    pub checked_entries: usize,
    /// Explicit resource limit used for this archive.
    pub maximum_entries: usize,
    /// Native and semantic exclusions.
    pub diagnostics: [SimulationPersistScalarDiagnostic; 3],
    /// Achieved requested byte-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
