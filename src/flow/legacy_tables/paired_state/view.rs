use super::*;
impl FlowPairedState {
    /// Borrows the immutable original carrier; e.g. `let raw = state.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = state.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the supported version one; e.g. `let version = state.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows the complete raw payload; e.g. `let raw = state.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns admitted aggregate bounds; e.g. `let maximum = state.options().maximum_index_words`.
    pub fn options(&self) -> FlowPairedStateOptions {
        self.options
    }
    /// Borrows a checked physical group; e.g. `let first = state.group(0)`.
    pub fn group(&self, ordinal: usize) -> Option<FlowPairedStateGroup<'_>> {
        Some(FlowPairedStateGroup {
            owner: self,
            ordinal,
            span: self.groups.get(ordinal)?,
        })
    }
    /// Enumerates exactly two groups in wire order; e.g. `let count = state.groups().len()`.
    pub fn groups(&self) -> impl ExactSizeIterator<Item = FlowPairedStateGroup<'_>> {
        self.groups
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowPairedStateGroup {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Reports complete physical framing and actual array totals; e.g. `let coverage = state.coverage()`.
    pub fn coverage(&self) -> FlowPairedStateCoverage {
        FlowPairedStateCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            groups: 2,
            index_words: self.totals[0],
            wide_words: self.totals[1],
            present_wide_arrays: self
                .groups
                .iter()
                .filter(|span| span.wides.is_some())
                .count(),
            achieved_error: 0,
        }
    }
    /// Borrows runtime and engineering exclusions; e.g. `let exclusions = state.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowPairedStateDiagnostic] {
        use FlowPairedStateDiagnostic::*;
        &[
            EngineeringRolesNotResolved,
            NativeRuntimeNotReproduced,
            NativeApplicationNotChecked,
        ]
    }
}
impl<'a> FlowPairedStateGroup<'a> {
    /// Returns physical group zero or one; e.g. `let ordinal = group.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete group interval, including count prefixes; e.g. `let span = group.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.span.interval
    }
    /// Borrows raw index patterns without range or engineering interpretation; e.g. `let first = group.indices().raw_word(0)`.
    pub fn indices(self) -> FlowBlockWordArray<'a> {
        let (start, end) = self.span.indices;
        FlowBlockWordArray {
            bytes: &self.owner.wire.decoded[start..end],
            start,
        }
    }
    /// Distinguishes absent from present-empty wide arrays; e.g. `let present = group.wides().is_some()`.
    pub fn wides(self) -> Option<FlowTableWideArray<'a>> {
        let (start, end) = self.span.wides?;
        Some(FlowTableWideArray {
            bytes: &self.owner.wire.decoded[start..end],
            start,
        })
    }
    /// Projects native integer division, retaining nondivisible counts; e.g. `let quotient = group.native_quotient()`.
    pub fn native_quotient(self) -> Option<usize> {
        self.wides()?
            .word_count()
            .checked_div(self.indices().word_count())
    }
}
impl FlowPairedStateEdit {
    /// Borrows the validated original checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowPairedState {
        &self.source
    }
    /// Borrows the completely reopened result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowPairedState {
        &self.result
    }
    /// Borrows physical history and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
