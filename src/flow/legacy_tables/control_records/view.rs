use super::*;
mod index;
impl FlowSavedSpans {
    /// Borrows the immutable physical checkpoint; e.g. `let raw = saved.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the revision-bound occurrence; e.g. `let ordinal = saved.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the original independent limits; e.g. `let limit = saved.options().maximum_entries`.
    pub fn options(&self) -> FlowControlOptions {
        self.options
    }
    /// Borrows the entire decoded marker/count/pair body; e.g. `let raw = saved.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the validated strict writer marker; e.g. `assert_eq!(saved.writer_marker(), 0)`.
    pub fn writer_marker(&self) -> u64 {
        0
    }
    /// Returns actual entries admitted from the full unsigned count; e.g. `let count = saved.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.count
    }
    /// Returns the stored checksum-valid member interval; e.g. `let span = saved.compressed_interval()`.
    pub fn compressed_interval(&self) -> (usize, usize) {
        (0, self.wire.stored_bytes)
    }
    /// Borrows a checked physical pair without deduplication; e.g. `let entry = saved.entry(0)?`.
    pub fn entry(&self, ordinal: usize) -> Result<FlowSavedSpan<'_>> {
        let start = wire::element_offset(SAVED, 16, self.count, ordinal, 16)?;
        Ok(FlowSavedSpan {
            owner: self,
            ordinal,
            start,
        })
    }
    /// Reports complete strict framing while excluding key/value semantics; e.g. `let proof = saved.coverage()`.
    pub fn coverage(&self) -> FlowControlCoverage {
        FlowControlCoverage {
            mode: FlowControlCheckMode::ExactSavedSpanWriter,
            length_width: None,
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            stored_entries: self.count,
            decoded_prefix_bytes: 16,
            entry_bytes: self.count * 16,
            compressed_bytes: self.wire.stored_bytes,
            raw_prefix_bytes: 0,
            footer_bytes: 0,
            achieved_error: 0,
        }
    }
    /// Qualifies strict writer, native, semantic and application limits; e.g. `let exclusions = saved.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowControlDiagnostic] {
        use FlowControlDiagnostic::*;
        &[
            WriterProfileAssumed,
            InstalledNativePayloadNotVerified,
            NativeRuntimeNotReproduced,
            SavedSpanMeaningsNotAssigned,
            OtherControlRecordsNotRewritten,
            NativeApplicationNotChecked,
        ]
    }
}
impl FlowSavedSpan<'_> {
    /// Returns physical order rather than a key-derived identity; e.g. `let ordinal = entry.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the exact decoded pair interval; e.g. `let span = entry.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.start, self.start + 16)
    }
    /// Reads the entire raw unsigned key; e.g. `let bits = entry.key_bits()`.
    pub fn key_bits(self) -> u64 {
        wire::wide_at(&self.owner.wire.decoded, self.start)
    }
    /// Reads the entire raw unsigned value without semantic mapping; e.g. `let bits = entry.value_bits()`.
    pub fn value_bits(self) -> u64 {
        wire::wide_at(&self.owner.wire.decoded, self.start + 8)
    }
}
