use crate::archive_codec::ArchiveRevision;
use std::sync::Arc;

/// All stored four-field content and any explicitly unassigned tail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationGuidParts {
    /// Raw UTF-16 units; null differs from an allocated empty string.
    pub fields: [Option<Arc<[u16]>>; 4],
    /// Bytes beyond the fourth field; retained without assigning a grammar.
    pub tail: Arc<[u8]>,
}

/// Immutable identity of one complete preserved optional string-stream snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationGuidRevision(pub(super) ArchiveRevision);

/// One normalized zero-based stored-field identity, without assigning a GUID role.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationGuidFieldId {
    pub(super) revision: SimulationGuidRevision,
    pub(super) index: usize,
}

/// One exact stored string frame and its separate caller-visible prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationGuidField {
    /// Immutable revision-bound field identity.
    pub id: SimulationGuidFieldId,
    /// Zero-based field ordinal, without an assigned semantic role.
    pub index: usize,
    /// Original bare stream offset of its four-byte count.
    pub stream_offset: usize,
    /// Exact byte count including the two-byte terminator, or zero for null.
    pub byte_length: u32,
    pub(super) units: Option<Arc<[u16]>>,
}

impl SimulationGuidField {
    /// Exact content excluding the final terminator, e.g. `field.stored_units()`; unpaired surrogates and embedded NULs survive.
    pub fn stored_units(&self) -> Option<&[u16]> {
        self.units.as_deref()
    }

    /// Caller-visible prefix before the first stored NUL, e.g. `field.caller_visible_units()`; null remains distinct from empty.
    pub fn caller_visible_units(&self) -> Option<&[u16]> {
        self.units.as_deref().map(|units| {
            &units[..units
                .iter()
                .position(|unit| *unit == 0)
                .unwrap_or(units.len())]
        })
    }
}

/// Exact four-field decoding remains distinct from an unassigned trailing region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationGuidStreamCompletion {
    /// All source bytes belong to the four validated frames.
    CompleteFourFields,
    /// Four frames decoded; remaining bytes have no assigned grammar.
    IncompleteUnassignedTail,
}

/// Quantified field framing and explicit stream completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationGuidCheckMode {
    /// Every known field is checked and any unassigned tail is preserved without claiming its validity.
    ExactFourFramesAndTailPreservation,
}

/// Quantified known-frame checks and explicit unknown-tail state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationGuidCoverage {
    /// Explicit exact check scope.
    pub mode: SimulationGuidCheckMode,
    /// Complete preserved source bytes, including an unknown tail.
    pub source_bytes: usize,
    /// Exact known frame bytes through the fourth field.
    pub decoded_bytes: usize,
    /// Unassigned trailing bytes preserved unchanged.
    pub unassigned_tail_bytes: usize,
    /// Explicit complete versus incomplete-stream status.
    pub completion: SimulationGuidStreamCompletion,
    /// Always false: these are bare field frames, without a transfer envelope.
    pub transfer_prefix: bool,
}
