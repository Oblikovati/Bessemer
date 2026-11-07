//! Exact record-433 arrays with external runtime-count context and isolated signed-cell edits.
mod decode;
mod edit;
mod types;
use super::*;
pub use types::*;

impl FlowCarrier {
    /// Reads one exact record-433 occurrence; e.g. `let arrays = carrier.record_433_arrays(&id, options)?`.
    pub fn record_433_arrays(
        &self,
        id: &FlowRecordId,
        options: FlowArrayOptions,
    ) -> Result<FlowMapArrays> {
        decode::read(self, id, options)
    }
}

impl FlowMapArrays {
    /// Borrows the complete source carrier; e.g. `let wire = arrays.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the exact source occurrence; e.g. `let id = arrays.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns the unchanged nonnegative selector; e.g. `let selector = arrays.selector()`.
    pub fn selector(&self) -> i32 {
        self.selector
    }
    /// Distinguishes omitted first arrays from present empty arrays; e.g. `let optional = arrays.first()`.
    pub fn first(&self) -> Option<&[i32]> {
        self.first.as_deref()
    }
    /// Borrows every signed second-array element in wire order; e.g. `let values = arrays.second()`.
    pub fn second(&self) -> &[i32] {
        &self.second
    }
    /// Borrows complete exact decoded bytes; e.g. `let raw = arrays.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.decoded
    }
    /// Reports complete serialized checks and external count coverage; e.g. `let checked = arrays.coverage()`.
    pub fn coverage(&self) -> FlowArrayCoverage {
        FlowArrayCoverage {
            mode: FlowArrayCheckMode::ExactZlibAndArrayFrames,
            compressed_bytes: self.compressed_bytes,
            decoded_bytes: self.decoded.len(),
            elements_per_array: self.second.len(),
            present_arrays: if self.first.is_some() { 2 } else { 1 },
            count_check: self.options.expected_count.map_or(
                FlowArrayCountCheck::NotSupplied,
                FlowArrayCountCheck::MatchedCallerCount,
            ),
            achieved_byte_error: 0,
        }
    }
    /// Borrows explicit semantic/native exclusions; e.g. `let exclusions = arrays.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowArrayDiagnostic] {
        &[
            FlowArrayDiagnostic::ArraySemanticsNotEstablished,
            FlowArrayDiagnostic::NativeOwnerAndApplicationNotChecked,
        ]
    }
}

fn array_failure(entity: &str, class: FailureClass, cause: impl Into<String>) -> BessemerError {
    BessemerError::new("Flow record 433", entity, class, cause)
}
