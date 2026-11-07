use super::super::{SimulationPersistVersion, SimulationThermalRelation};

/// Only the three object members with bound explicit reader zero stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendResetMember {
    /// Native object wordC0, assigned only before840008.
    WordC0,
    /// Native object wordC4, assigned only before840010.
    WordC4,
    /// Native object wordC8, assigned only before840010.
    WordC8,
}

impl SimulationLegendResetMember {
    /// Inspect the object-member offset, never a wire offset; e.g. `Self::WordC0.member_offset()`.
    pub fn member_offset(self) -> u16 {
        match self {
            Self::WordC0 => 0xc0,
            Self::WordC4 => 0xc4,
            Self::WordC8 => 0xc8,
        }
    }
}

/// One derived runtime assignment without a corresponding stored span.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendResetAssignment {
    /// Exact bound object destination.
    pub member: SimulationLegendResetMember,
    /// Zero for each assignment published by this reader policy.
    pub value_raw: i32,
}

/// Completion of the reset-event policy, not the full reader or resulting object state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendResetCompletion {
    /// This explicit-field version has no forced-zero assignments.
    CompletedNoAssignments,
    /// This explicit-field version has two or three forced-zero assignments.
    CompletedZeroAssignments,
}

/// The normalized caller version is the sole input to reset selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendResetHistory {
    /// Original explicit reader version, without file-name or constructor inference.
    pub version: SimulationPersistVersion,
    /// Reset events derive from this version context.
    pub relation: SimulationThermalRelation,
}

/// Exact local policy coverage, distinct from full reader activity and wire fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendResetCoverage {
    /// True; the normalized explicit-field version was inspected.
    pub version_inspected: bool,
    /// Zero; reset selection consumes no serialized scalar values.
    pub stored_words_read_for_reset_policy: usize,
    /// Zero, two or three represented runtime word assignments.
    pub runtime_words_assigned: usize,
    /// One32-bit native store per represented assignment.
    pub native_store_instructions: usize,
    /// Four runtime bytes per assignment, not additional consumed wire bytes.
    pub runtime_store_bytes: usize,
    /// Zero; absent stored fields remain absent.
    pub additional_wire_bytes: usize,
    /// True; all declared version comparisons and zero patterns are exact.
    pub exact_version_policy: bool,
    /// Zero local raw-pattern error.
    pub achieved_pattern_error_bits: u32,
    /// False; the operation changes no serialized bytes.
    pub wire_bytes_modified: bool,
    /// False; other wire reads, helpers and resulting object values remain separate.
    pub full_reader_emulated: bool,
    /// False; no constructor or prior-object defaults were supplied.
    pub constructor_state_inferred: bool,
    /// False; scalar policy results establish no native object pointer validity.
    pub native_object_pointer_validated: bool,
}

/// Qualifications accompanying every completed explicit reset-event result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendResetDiagnostic {
    /// Target code was inspected without execution.
    NativeNotExecuted,
    /// The operation selects only explicit reader resets, not all reader activity.
    ExplicitReaderPolicyOnly,
    /// These events add no stored field, wire offset or byte credit.
    DerivedAssignmentsHaveNoWireOffsets,
    /// Other skipped members do not acquire zeros from this operation.
    OtherSkippedFieldsRemainUnknown,
    /// Constructor and reused-object state are not inferred from absence.
    ConstructorAndPriorStateNotInferred,
}
