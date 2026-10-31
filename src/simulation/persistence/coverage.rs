/// Completion of a declared persistence framing operation, separate from application semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistCompletion {
    /// All bytes belong to the selected complete framing profile.
    CompletedFraming,
    /// A partial reader stopped with an explicitly retained unconsumed suffix.
    IncompleteUnconsumedTail,
}

/// Exact scope of checks performed on a persistence result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistCheckMode {
    /// One exact counted frame within its retained complete parent.
    ExactTextFrame,
    /// All study texts, count and exact EOF.
    ExactStudyOrder,
    /// Complete study framing and literal preservation outside the requested text.
    ExactStudyOrderAndPreservation,
    /// Caller-selected primary word sequence with full or partial consumption.
    ExactPrimaryVersionMode,
    /// Exact 128-byte six-digit profile with opaque suffix preservation.
    ExactLegacyVersionProfile,
}

/// Explicit exclusions from persistence byte framing acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPersistDiagnostic {
    /// No installed loader, COM stream or application operation executed.
    NativeApplicationNotExecuted,
    /// Byte framing does not assign business meanings to stored fields.
    FieldSemanticsUnassigned,
    /// Supplied primary numeric context does not establish native text conversion.
    NumericConversionNotInferred,
}

/// Quantified exact framing checks and preserved bytes without a decoded grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPersistCoverage {
    /// Successful complete framing versus explicit partial consumption.
    pub completion: SimulationPersistCompletion,
    /// Exact declared check scope.
    pub check_mode: SimulationPersistCheckMode,
    /// Complete original source parent extent.
    pub source_bytes: usize,
    /// Bytes consumed by the selected reader profile.
    pub consumed_bytes: usize,
    /// Raw suffix retained without field interpretation.
    pub opaque_bytes: usize,
    /// Native and semantic exclusions.
    pub diagnostics: [SimulationPersistDiagnostic; 3],
    /// Achieved requested byte-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

pub(super) fn coverage(
    check_mode: SimulationPersistCheckMode,
    source_bytes: usize,
    consumed_bytes: usize,
    opaque_bytes: usize,
    incomplete: bool,
) -> SimulationPersistCoverage {
    SimulationPersistCoverage {
        completion: if incomplete {
            SimulationPersistCompletion::IncompleteUnconsumedTail
        } else {
            SimulationPersistCompletion::CompletedFraming
        },
        check_mode,
        source_bytes,
        consumed_bytes,
        opaque_bytes,
        diagnostics: [
            SimulationPersistDiagnostic::NativeApplicationNotExecuted,
            SimulationPersistDiagnostic::FieldSemanticsUnassigned,
            SimulationPersistDiagnostic::NumericConversionNotInferred,
        ],
        requested_pattern_error_bits: 0,
    }
}
