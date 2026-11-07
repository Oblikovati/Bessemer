use super::*;
impl FlowContextState {
    /// Borrows the immutable original carrier; e.g. `let raw = state.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = state.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the observed supported version; e.g. `let version = state.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows all complete raw bytes; e.g. `let raw = state.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the admitted explicit context and bounds; e.g. `let supplied = state.options().index_count`.
    pub fn options(&self) -> FlowContextStateOptions {
        self.options
    }
    /// Returns the stored object count independently of index width; e.g. `let objects = state.object_count()`.
    pub fn object_count(&self) -> u32 {
        self.object_count
    }
    /// Distinguishes an applied count, including zero, from ignored context; e.g. `let applied = state.context_is_applied()`.
    pub fn context_is_applied(&self) -> bool {
        self.object_count > 0 && self.version() < 5
    }
    /// Borrows the exact raw index array, retaining its complete interval; e.g. `let first = state.indices().raw_word(0)`.
    pub fn indices(&self) -> FlowBlockWordArray<'_> {
        FlowBlockWordArray {
            bytes: &self.wire.decoded[8..self.index_end],
            start: 8,
        }
    }
    /// Reads the signed tail or its documented absence; e.g. `let tail = state.tail()`.
    pub fn tail(&self) -> Option<i32> {
        (self.version() < 4).then(|| wire::signed_at(&self.wire.decoded, self.index_end))
    }
    /// Reports exact physical framing and the applied assumption; e.g. `let coverage = state.coverage()`.
    pub fn coverage(&self) -> FlowContextStateCoverage {
        FlowContextStateCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            object_count: self.object_count,
            applied_index_count: self
                .options
                .index_count
                .filter(|_| self.context_is_applied()),
            index_words: (self.index_end - 8) / 4,
            tail_present: self.version() < 4,
            achieved_error: 0,
        }
    }
    /// Borrows context, runtime and engineering exclusions; e.g. `let exclusions = state.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowContextStateDiagnostic] {
        use FlowContextStateDiagnostic::*;
        if self.context_is_applied() {
            return &[
                NativeIndexCountAssumed,
                NativeRuntimeNotReproduced,
                EngineeringRolesNotResolved,
                NativeApplicationNotChecked,
            ];
        }
        &[
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
}
impl FlowContextStateEdit {
    /// Borrows the validated original checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowContextState {
        &self.source
    }
    /// Borrows the completely reopened result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowContextState {
        &self.result
    }
    /// Borrows physical history and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
