use super::*;
impl FlowCountedBytes {
    /// Borrows the immutable carrier checkpoint; e.g. `let raw = values.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let id = values.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns admitted independent bounds; e.g. `let bounds = values.options()`.
    pub fn options(&self) -> FlowCountedByteOptions {
        self.options
    }
    /// Borrows the complete payload including its count; e.g. `let raw = values.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the validated stored count; e.g. `let count = values.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.raw_bytes().len()
    }
    /// Borrows every uninterpreted stored byte; e.g. `let raw = values.raw_bytes()`.
    pub fn raw_bytes(&self) -> &[u8] {
        &self.wire.decoded[4..]
    }
    /// Returns the raw-value interval, excluding the count; e.g. `let interval = values.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (4, self.wire.decoded.len())
    }
    /// Reads an existing raw byte; e.g. `let first = values.byte(0)`.
    pub fn byte(&self, index: usize) -> Option<u8> {
        self.raw_bytes().get(index).copied()
    }
    /// Reports exact complete framing and transferred-byte coverage; e.g. `let proof = values.coverage()`.
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
    /// Qualifies runtime indexing, engineering and application acceptance; e.g. `let exclusions = values.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        use FlowSimpleTableDiagnostic::*;
        &[
            EngineeringRolesNotResolved,
            NativeRuntimeNotReproduced,
            NativeApplicationNotChecked,
        ]
    }
}
