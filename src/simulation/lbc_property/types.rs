use super::SimulationLbcPropertyProfile;
use crate::archive_codec::ArchiveRevision;

/// Immutable identity of one exact LBC Property image and its explicit profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLbcPropertyRevision(pub(super) ArchiveRevision);

/// Normalized four-byte field position tied to its original image revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLbcPropertyFieldId {
    pub(super) revision: SimulationLbcPropertyRevision,
    pub(super) ordinal: usize,
}

/// Exact stored signed word and separate documented runtime member destination.
#[derive(Debug, Clone)]
pub struct SimulationLbcPropertyField {
    /// Revision-bound physical field identity.
    pub id: SimulationLbcPropertyFieldId,
    /// Original physical byte offset from the beginning of the bare stream.
    pub stream_offset: usize,
    /// Runtime owner destination, without assigned physical field meaning.
    pub member: super::SimulationLbcRuntimeMember,
    /// Signed view of the complete stored four-byte pattern.
    pub value: i32,
}

/// Exact checks establishing only the declared bare-stream profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcPropertyCheckMode {
    /// All selected fields and exact EOF.
    ExactSelectedProfile,
    /// Selected profile and every unchanged byte outside one supplied field.
    ExactSelectedProfileAndPreservation,
}

/// Explicit exclusions from LBC framing acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcPropertyDiagnostic {
    /// No installed writer, COM operation or application loader executed.
    NativeApplicationNotExecuted,
    /// Runtime member labels do not assign business meanings to words.
    FieldMeaningsUnassigned,
    /// The selected profile does not prove the native runtime version path.
    RuntimeVersionNotInferred,
}

/// Successful completion of the declared full bare-stream framing profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLbcReadCompletion {
    /// Every selected field belongs to the complete declared profile at exact EOF.
    CompletedFraming,
}

/// Quantified complete framing coverage with its explicit supplied context and exclusions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLbcPropertyCoverage {
    /// Original selected reader or fixed-writer profile.
    pub profile: SimulationLbcPropertyProfile,
    /// Exact declared check scope.
    pub check_mode: SimulationLbcPropertyCheckMode,
    /// All bytes consumed at exact EOF.
    pub checked_bytes: usize,
    /// All selected four-byte fields checked in wire order.
    pub checked_fields: usize,
    /// Complete framing, including no unassigned or discarded suffix bytes.
    pub completion: SimulationLbcReadCompletion,
    /// Native and semantic exclusions.
    pub diagnostics: [SimulationLbcPropertyDiagnostic; 3],
    /// Achieved requested byte-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}
