use super::*;
impl FlowByteStates {
    /// Borrows the immutable complete carrier; e.g. `let raw = states.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let id = states.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the original resource bounds; e.g. `let bounds = states.options()`.
    pub fn options(&self) -> FlowByteStateOptions {
        self.options
    }
    /// Borrows the complete decoded body; e.g. `let raw = states.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the signed header without dispatch; e.g. `let header = states.header()`.
    pub fn header(&self) -> i32 {
        self.wire.version
    }
    /// Returns the validated stored count; e.g. `let count = states.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.raw_bytes().len()
    }
    /// Borrows every uninterpreted raw value; e.g. `let raw = states.raw_bytes()`.
    pub fn raw_bytes(&self) -> &[u8] {
        &self.wire.decoded[8..]
    }
    /// Returns the raw value interval, excluding header/count; e.g. `let interval = states.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (8, self.wire.decoded.len())
    }
    /// Reads an existing raw byte; e.g. `let first = states.byte(0)`.
    pub fn byte(&self, index: usize) -> Option<u8> {
        self.raw_bytes().get(index).copied()
    }
    /// Reports exact zlib framing and actual transfer coverage; e.g. `let proof = states.coverage()`.
    pub fn coverage(&self) -> FlowSimpleTableCoverage {
        FlowSimpleTableCoverage {
            mode: wire::mode(&self.wire),
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            wire_rows: 0,
            wide_words: 0,
            raw_words: 0,
            raw_bytes: self.stored_count(),
            achieved_error: 0,
        }
    }
    /// Qualifies header dispatch, runtime binding and engineering/application checks; e.g. `let exclusions = states.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        use FlowSimpleTableDiagnostic::*;
        &[
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
}
