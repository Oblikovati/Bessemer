use super::*;
impl FlowTaggedState {
    /// Borrows the immutable original carrier; e.g. `let raw = state.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = state.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns supported version nine; e.g. `let version = state.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows all complete raw bytes; e.g. `let raw = state.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns admitted raw-byte, row and wide-value bounds; e.g. `let maximum = state.options().maximum_rows`.
    pub fn options(&self) -> FlowTaggedStateOptions {
        self.options
    }
    /// Returns the number of physical tag/row pairs; e.g. `let count = state.row_count()`.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
    /// Borrows the complete tag block before every row body; e.g. `let tags = state.tags_bytes()`.
    pub fn tags_bytes(&self) -> &[u8] {
        &self.wire.decoded[8..8 + self.rows.len()]
    }
    /// Borrows an existing physical row; e.g. `let first = state.row(0)`.
    pub fn row(&self, ordinal: usize) -> Option<FlowTaggedStateRow<'_>> {
        Some(FlowTaggedStateRow {
            owner: self,
            ordinal,
            span: self.rows.get(ordinal)?,
        })
    }
    /// Enumerates every row, including zero-body rows; e.g. `let count = state.rows().len()`.
    pub fn rows(&self) -> impl ExactSizeIterator<Item = FlowTaggedStateRow<'_>> {
        self.rows
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowTaggedStateRow {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Returns the signed tail or absence for an empty record; e.g. `let tail = state.tail()`.
    pub fn tail(&self) -> Option<i32> {
        (!self.rows.is_empty())
            .then(|| wire::signed_at(&self.wire.decoded, self.wire.decoded.len() - 4))
    }
    /// Reports exact complete raw framing and all scalar counts; e.g. `let coverage = state.coverage()`.
    pub fn coverage(&self) -> FlowTaggedStateCoverage {
        let rows = self.rows.len();
        let present = self.rows.iter().filter(|row| row.values.is_some()).count();
        let tail = usize::from(rows > 0);
        FlowTaggedStateCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            rows,
            tag_bytes: rows,
            present_rows: present,
            wide_words: self.wide_words,
            tail_present: tail != 0,
            stored_values: 2 + rows + present + self.wide_words + tail,
            achieved_error: 0,
        }
    }
    /// Borrows native context, runtime, engineering and application exclusions; e.g. `let exclusions = state.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowTaggedStateDiagnostic] {
        use FlowTaggedStateDiagnostic::*;
        &[
            NativeContextValidationNotReproduced,
            NativeRuntimeStateNotReproduced,
            EngineeringColumnsNotAssigned,
            NativeApplicationNotChecked,
        ]
    }
}
impl<'a> FlowTaggedStateRow<'a> {
    /// Returns physical tag/body ordinal; e.g. `let ordinal = row.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the tag's separate decoded position; e.g. `let offset = row.tag_offset()`.
    pub fn tag_offset(self) -> usize {
        8 + self.ordinal
    }
    /// Returns the unnormalized stored tag; e.g. `let tag = row.tag()`.
    pub fn tag(self) -> u8 {
        self.owner.wire.decoded[self.tag_offset()]
    }
    /// Returns the body interval, excluding the leading tag block; e.g. `let span = row.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.span.interval
    }
    /// Retains absent widths separately from present zero; e.g. `let width = row.declared_width()`.
    pub fn declared_width(self) -> Option<usize> {
        self.values().map(|array| array.word_count())
    }
    /// Borrows present wide values, including present-empty arrays; e.g. `let values = row.values()`.
    pub fn values(self) -> Option<FlowTableWideArray<'a>> {
        let (start, end) = self.span.values?;
        Some(FlowTableWideArray {
            bytes: &self.owner.wire.decoded[start..end],
            start,
        })
    }
}
impl FlowTaggedStateEdit {
    /// Borrows the validated original checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowTaggedState {
        &self.source
    }
    /// Borrows the completely reopened result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowTaggedState {
        &self.result
    }
    /// Borrows physical history and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
