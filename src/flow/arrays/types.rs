use super::*;

/// Serialized record-433 resource bounds and optional external runtime-count assertion.
#[derive(Clone, Copy, Debug)]
pub struct FlowArrayOptions {
    /// Maximum complete decoded bytes, at least eight.
    pub maximum_decoded_bytes: usize,
    /// Positive upper bound for each signed integer array's element count.
    pub maximum_elements: usize,
    /// Caller-supplied independent owner count, without guessing initialization state.
    pub expected_count: Option<u32>,
}

/// Exact serialized profile, without interpreting the integer-array roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowArrayCheckMode {
    /// One checksum-validated zlib stream and exact selector/count/array EOF.
    ExactZlibAndArrayFrames,
}

/// Runtime equality coverage, separate from structural payload validity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowArrayCountCheck {
    /// No external owner count was supplied; native count acceptance remains unchecked.
    NotSupplied,
    /// Every stored element count matched this explicit caller-supplied value.
    MatchedCallerCount(u32),
}

/// Complete serialized checks and explicitly qualified runtime-count coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowArrayCoverage {
    /// Required serialized check scope.
    pub mode: FlowArrayCheckMode,
    /// Complete compressed selected payload bytes, including zlib framing.
    pub compressed_bytes: usize,
    /// Complete decoded payload bytes, including selector and count.
    pub decoded_bytes: usize,
    /// Common array count; zero is a completed empty array, not a failed read.
    pub elements_per_array: usize,
    /// One or two physically present arrays; omitted differs from present empty.
    pub present_arrays: u8,
    /// External count-equality coverage; supplied values do not prove their provenance.
    pub count_check: FlowArrayCountCheck,
    /// Exact serialized byte disagreement; successful checks report zero.
    pub achieved_byte_error: usize,
}

/// Missing semantic and native acceptance checks accompanying serialized array results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowArrayDiagnostic {
    /// Integer-array roles, domains, units and owner population were not inferred.
    ArraySemanticsNotEstablished,
    /// Caller-supplied count provenance and actual native execution were not checked.
    NativeOwnerAndApplicationNotChecked,
}

/// One array destination; selector zero physically contains both arrays.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowArrayGroup {
    /// Conditional first array, present only for selector zero.
    First,
    /// Always-present second array for every admitted nonnegative selector.
    Second,
}

/// Immutable complete record-433 view with source occurrence and decoded bytes retained.
#[derive(Clone, Debug)]
pub struct FlowMapArrays {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) selector: i32,
    pub(super) first: Option<Arc<[i32]>>,
    pub(super) second: Arc<[i32]>,
    pub(super) decoded: Arc<[u8]>,
    pub(super) compressed_bytes: usize,
    pub(super) options: FlowArrayOptions,
}

/// Exact selected cell's source/result relation with signed values preserved.
#[derive(Clone, Debug)]
pub struct FlowArrayCellHistory {
    /// Source record occurrence.
    pub source_record: FlowRecordId,
    /// Corresponding result record occurrence.
    pub result_record: FlowRecordId,
    /// Physically selected array.
    pub group: FlowArrayGroup,
    /// Validated zero-based array element ordinal.
    pub index: usize,
    /// Complete signed original value.
    pub before: i32,
    /// Complete signed requested value.
    pub after: i32,
    /// Copied for a no-op, otherwise derived from this original cell.
    pub relation: FlowPayloadRelation,
}

/// Completed bounded cell edit; fresh opposite edits need not recover original compressed bytes.
#[derive(Clone, Debug)]
pub struct FlowArrayEdit {
    pub(super) source: FlowMapArrays,
    pub(super) result: FlowMapArrays,
    pub(super) carrier: FlowCarrierEdit,
    pub(super) history: FlowArrayCellHistory,
}

/// Compression provenance limitation accompanying a completed typed edit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowArrayEditDiagnostic {
    /// Original zlib choices need not be reproduced by a fresh opposite edit; use the retained source checkpoint for exact bytes.
    FreshReverseWireIdentityNotGuaranteed,
}

impl FlowArrayEdit {
    /// Borrows explicit compressed-wire reversal exclusions; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowArrayEditDiagnostic] {
        &[FlowArrayEditDiagnostic::FreshReverseWireIdentityNotGuaranteed]
    }
    /// Borrows original arrays and exact carrier checkpoint; e.g. `let original = edit.source()`.
    pub fn source(&self) -> &FlowMapArrays {
        &self.source
    }
    /// Borrows fully reparsed result arrays; e.g. `let checked = edit.result().coverage()`.
    pub fn result(&self) -> &FlowMapArrays {
        &self.result
    }
    /// Borrows complete carrier history and semantic exclusions; e.g. `let parent = edit.carrier_edit()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier
    }
    /// Borrows selected-cell provenance; e.g. `let old = edit.history().before`.
    pub fn history(&self) -> &FlowArrayCellHistory {
        &self.history
    }
}
