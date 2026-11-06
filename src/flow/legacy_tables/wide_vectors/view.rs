use super::*;
impl FlowWideVectors {
    /// Borrows the immutable original carrier; e.g. `let raw = vectors.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = vectors.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the admitted version zero; e.g. `let version = vectors.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns the original resource bounds; e.g. `let bound = vectors.options().maximum_vectors`.
    pub fn options(&self) -> FlowWideVectorOptions {
        self.options
    }
    /// Borrows the complete raw payload; e.g. `let raw = vectors.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns actual length-table entries, including empty vectors; e.g. `let count = vectors.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.spans.len()
    }
    /// Returns the complete stored length-table interval; e.g. `let span = vectors.length_table_interval()`.
    pub fn length_table_interval(&self) -> (usize, usize) {
        (8, 8 + self.spans.len() * 4)
    }
    /// Borrows a checked vector, preserving stored-empty presence; e.g. `let array = vectors.vector(0)?`.
    pub fn vector(&self, index: usize) -> Result<FlowTableWideArray<'_>> {
        let span = self
            .spans
            .get(index)
            .ok_or_else(|| table_index_error(148, "vector", index, self.spans.len()))?;
        Ok(FlowTableWideArray {
            bytes: &self.wire.decoded[span.start..span.end],
            start: span.start,
        })
    }
    /// Reports exact framing, vector/value counts and empty presence; e.g. `let proof = vectors.coverage()`.
    pub fn coverage(&self) -> FlowWideVectorCoverage {
        FlowWideVectorCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            stored_vectors: self.spans.len(),
            stored_values: self.values,
            empty_vectors: self
                .spans
                .iter()
                .filter(|span| span.start == span.end)
                .count(),
            achieved_error: 0,
        }
    }
    /// Qualifies runtime, engineering and application checks; e.g. `let exclusions = vectors.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        use FlowSimpleTableDiagnostic::*;
        &[
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
}
