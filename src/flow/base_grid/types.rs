use super::*;

/// Explicit raw or RFC 1950 owner context, shared with other Flow records.
pub type FlowGridCompression = FlowGroupCompression;

/// Independent byte and total boundary-word allocation bounds.
#[derive(Clone, Copy, Debug)]
pub struct FlowGridOptions {
    /// Supplied transport, never inferred from a payload signature.
    pub compression: FlowGridCompression,
    /// Complete decoded payload limit, at least 36 bytes.
    pub maximum_decoded_bytes: usize,
    /// Total words across all three arrays, at least three.
    pub maximum_boundary_words: usize,
}

/// Exact serialized validation mode, without native geometry acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGridCheckMode {
    /// Complete raw payload dimensions and boundary arrays through exact EOF.
    ExactRawBoundaryArrays,
    /// Complete checksum-validated zlib stream and decoded arrays through exact EOF.
    ExactZlibBoundaryArrays,
}

/// Complete wire coverage, separate from geometry checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowGridCoverage {
    /// Explicit exact serialized check scope.
    pub mode: FlowGridCheckMode,
    /// Every stored payload byte.
    pub stored_bytes: usize,
    /// Every decoded payload byte.
    pub decoded_bytes: usize,
    /// Total physically present binary64 words.
    pub boundary_words: usize,
    /// No approximation in the serialized check.
    pub achieved_error: u64,
}

/// Explicit exclusions from serialized grid acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowGridDiagnostic {
    /// Units, monotonicity and model geometry are not established by raw words.
    GeometryNotValidated,
    /// Native refined boundary arrays are not reconstructed.
    NativeRefinementNotReproduced,
    /// Runtime object and block-vector indexing are not proved.
    NativeObjectIndexingNotValidated,
    /// Compression was supplied by the caller, without reconstructing its owner.
    OwnerCompressionContextNotReconstructed,
    /// Native application loading has not run.
    NativeApplicationNotChecked,
}

/// Immutable complete decoded grid and source checkpoint.
#[derive(Clone, Debug)]
pub struct FlowBaseGrid {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) decoded: Arc<[u8]>,
    pub(super) dimensions: [i32; 3],
    pub(super) spans: [(usize, usize); 3],
    pub(super) options: FlowGridOptions,
    pub(super) coverage: FlowGridCoverage,
}

/// Borrowed exact boundary words in their stored array order.
#[derive(Clone, Copy, Debug)]
pub struct FlowGridBoundary<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) start: usize,
}

/// Completed fixed-width edit with immutable checkpoints and complete physical history.
#[derive(Clone, Debug)]
pub struct FlowGridEdit {
    pub(super) source: FlowBaseGrid,
    pub(super) result: FlowBaseGrid,
    pub(super) carrier_edit: FlowCarrierEdit,
}
