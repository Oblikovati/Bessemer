use crate::archive_codec::ArchiveRevision;

/// Complete changed/unchanged operation state.
pub use crate::display_lists::DisplayEditCompletion as ComposerArrayCompletion;

/// Caller-supplied vector shape; dimensions/count are not stored in a packed block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerVectorLayout {
    /// Number of vector rows.
    pub count: u32,
    /// Two or three component lanes.
    pub dimensions: ComposerVectorDimensions,
}

/// The documented vector dimensions, with distinct wire headers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerVectorDimensions {
    /// No axis flags or constants are serialized.
    Two,
    /// Three canonical constant-axis flags are serialized.
    Three,
}

impl ComposerVectorDimensions {
    pub(super) fn lanes(self) -> usize {
        match self {
            Self::Two => 2,
            Self::Three => 3,
        }
    }
}

/// Stored scalar width and meaning; quantized codes are not reconstructed floats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerVectorPacking {
    /// Mode `a`: four-byte patterns.
    Direct32,
    /// Mode `b`: two-byte unsigned codes.
    Quantized16,
    /// Mode `c`: one-byte unsigned codes.
    Quantized8,
}

impl ComposerVectorPacking {
    pub(super) fn width(self) -> usize {
        match self {
            Self::Direct32 => 4,
            Self::Quantized16 => 2,
            Self::Quantized8 => 1,
        }
    }

    pub(super) fn tag(self) -> u8 {
        match self {
            Self::Direct32 => b'a',
            Self::Quantized16 => b'b',
            Self::Quantized8 => b'c',
        }
    }
}

/// One lane's stored header, with arbitrary binary32 bit patterns preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerVectorAxis {
    /// Constant lane (V3 only), omitted from row codes.
    Constant(u32),
    /// Direct four-byte patterns stored for each row.
    Direct,
    /// Endpoint patterns for the lane's stored unsigned codes.
    Quantized {
        /// First endpoint, without numeric normalization.
        first: u32,
        /// Second endpoint, without numeric normalization.
        second: u32,
    },
}

/// Immutable identity of an array revision; clones retain identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerArrayRevision(pub(super) ArchiveRevision);

/// Explicit scope of successful array checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerArrayCheckMode {
    /// Entire wire span, header/context consistency and forced reconstruction.
    ExactPackedArray,
}

/// Validation coverage for one complete stored array.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerArrayCoverage {
    /// Explicit check mode, with no implied geometry/numeric checks.
    pub mode: ComposerArrayCheckMode,
    /// Entire encoded block size.
    pub wire_bytes: usize,
    /// Stored scalar count; constant vector lanes are excluded.
    pub stored_values: usize,
}

/// Achieved error/fidelity domain for a successful edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerArrayFidelity {
    /// Zero stored-bit error; no floating-point error bound is claimed.
    ExactStoredValues,
}

/// Required checks outside an exact wire operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerArrayDiagnostic {
    /// Quantized floats have not been reconstructed under an arithmetic profile.
    FloatingReconstructionNotChecked,
    /// Topology, geometry, self-intersection and tolerance checks are outside this operation.
    GeometryNotChecked,
    /// Native application acceptance has not been checked.
    NativeApplicationNotChecked,
}

/// History of a non-destructive array replacement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposerArrayHistory {
    /// Completed no-op retained this revision.
    Unchanged(ComposerArrayRevision),
    /// Complete array derived from one parent.
    Derived {
        /// Original array revision.
        parent: ComposerArrayRevision,
        /// Published result revision.
        result: ComposerArrayRevision,
    },
}

/// Completed isolated edit with its checks, fidelity, history and explicit exclusions.
#[derive(Debug, Clone)]
pub struct ComposerArrayEdit<T> {
    /// Immutable completed array.
    pub block: T,
    /// Completed changed/unchanged state.
    pub completion: ComposerArrayCompletion,
    /// Complete wire validation coverage.
    pub coverage: ComposerArrayCoverage,
    /// Achieved stored-bit fidelity.
    pub fidelity: ComposerArrayFidelity,
    /// Parent/result relation.
    pub history: ComposerArrayHistory,
    /// Checks not established by this operation.
    pub diagnostics: [ComposerArrayDiagnostic; 3],
}
