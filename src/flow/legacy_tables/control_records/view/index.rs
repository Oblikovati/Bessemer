use super::*;
impl FlowTerminalIndex {
    /// Borrows the immutable physical checkpoint; e.g. `let raw = index.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the revision-bound occurrence; e.g. `let ordinal = index.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the original independent limits; e.g. `let limit = index.options().maximum_entries`.
    pub fn options(&self) -> FlowControlOptions {
        self.options
    }
    /// Returns the explicit carrier length/footer declaration; e.g. `let width = index.length_width().byte_width()`.
    pub fn length_width(&self) -> FlowControlLengthWidth {
        self.width
    }
    /// Borrows all decoded count/entry bytes; e.g. `let raw = index.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the validated zero raw-prefix writer marker; e.g. `assert_eq!(index.writer_marker(), 0)`.
    pub fn writer_marker(&self) -> u64 {
        0
    }
    /// Returns actual ordered entries from the signed nonnegative count; e.g. `let count = index.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.count
    }
    /// Returns only the exact checksum-valid member interval; e.g. `let span = index.compressed_interval()`.
    pub fn compressed_interval(&self) -> (usize, usize) {
        (8, self.footer.start)
    }
    /// Returns the checked raw footer interval; e.g. `let span = index.footer_interval()`.
    pub fn footer_interval(&self) -> (usize, usize) {
        (self.footer.start, self.wire.stored_bytes)
    }
    /// Returns the validated unsigned backward distance; e.g. `let distance = index.backward_distance()`.
    pub fn backward_distance(&self) -> u64 {
        self.footer.distance
    }
    /// Returns the validated footer identifier; e.g. `assert_eq!(index.footer_identifier(), 0xffffff9b)`.
    pub fn footer_identifier(&self) -> u32 {
        INDEX
    }
    /// Borrows one physical entry without adopting or reconciling it; e.g. `let entry = index.entry(0)?`.
    pub fn entry(&self, ordinal: usize) -> Result<FlowTerminalIndexEntry<'_>> {
        let start = wire::element_offset(INDEX, 4, self.count, ordinal, 20)?;
        Ok(FlowTerminalIndexEntry {
            owner: self,
            ordinal,
            start,
        })
    }
    /// Reports complete strict framing and explicit footer context; e.g. `let proof = index.coverage()`.
    pub fn coverage(&self) -> FlowControlCoverage {
        FlowControlCoverage {
            mode: FlowControlCheckMode::ExactTerminalIndexWriter,
            length_width: Some(self.width),
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            stored_entries: self.count,
            decoded_prefix_bytes: 4,
            entry_bytes: self.count * 20,
            compressed_bytes: self.footer.start - 8,
            raw_prefix_bytes: 8,
            footer_bytes: self.width.byte_width() + 4,
            achieved_error: 0,
        }
    }
    /// Qualifies writer, target, adoption, native and application limits; e.g. `let exclusions = index.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowControlDiagnostic] {
        if self.width == FlowControlLengthWidth::FourBytes {
            return &INDEX_DIAGNOSTICS;
        }
        &INDEX_DIAGNOSTICS[..7]
    }
}
impl FlowTerminalIndexEntry<'_> {
    /// Returns physical order, retaining duplicate identifiers; e.g. `let ordinal = entry.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete decoded 20-byte entry interval; e.g. `let span = entry.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.start, self.start + 20)
    }
    /// Reads the raw unsigned target identifier, including reserved values; e.g. `let identifier = entry.identifier()`.
    pub fn identifier(self) -> u32 {
        wire::unsigned_at(&self.owner.wire.decoded, self.start)
    }
    /// Reads the raw unsigned record start without target validation; e.g. `let start = entry.record_start()`.
    pub fn record_start(self) -> u64 {
        wire::wide_at(&self.owner.wire.decoded, self.start + 4)
    }
    /// Reads the raw unsigned record span without target validation; e.g. `let span = entry.record_span()`.
    pub fn record_span(self) -> u64 {
        wire::wide_at(&self.owner.wire.decoded, self.start + 12)
    }
}

const INDEX_DIAGNOSTICS: [FlowControlDiagnostic; 8] = {
    use FlowControlDiagnostic::*;
    [
        WriterProfileAssumed,
        InstalledNativePayloadNotVerified,
        NativeRuntimeNotReproduced,
        IndexTargetsNotValidated,
        IndexAdoptionNotPerformed,
        OtherControlRecordsNotRewritten,
        NativeApplicationNotChecked,
        FourByteCarrierFormatNotEstablished,
    ]
};
