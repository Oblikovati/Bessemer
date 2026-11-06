use super::*;
impl FlowMediumBackendPrefix {
    /// Borrows the complete outer-prefix checkpoint; e.g. `let source = backend.metadata_prefix()`.
    pub fn metadata_prefix(&self) -> &FlowMetadataPrefix {
        &self.prefix
    }
    /// Borrows the complete backend input; e.g. `let raw = backend.backend_bytes()`.
    pub fn backend_bytes(&self) -> &[u8] {
        self.prefix.opaque_tail()
    }
    /// Borrows all undecoded body or outer-record bytes; e.g. `let tail = backend.opaque_tail()`.
    pub fn opaque_tail(&self) -> &[u8] {
        &self.backend_bytes()[self.end..]
    }
    /// Enumerates the reached table headers; e.g. `let count = backend.tables().len()`.
    pub fn tables(&self) -> impl ExactSizeIterator<Item = FlowMediumBackendTable<'_>> {
        (0..self.tables.len()).map(|ordinal| FlowMediumBackendTable {
            owner: self,
            ordinal,
        })
    }
    /// Borrows a reached header; e.g. `let first = backend.table(0)`.
    pub fn table(&self, ordinal: usize) -> Option<FlowMediumBackendTable<'_>> {
        (ordinal < self.tables.len()).then_some(FlowMediumBackendTable {
            owner: self,
            ordinal,
        })
    }
    /// Identifies the exact framing stop; e.g. `let stop = backend.stop()`.
    pub fn stop(&self) -> FlowMediumBackendStop {
        if self.tables.iter().all(|table| table.count == 0) {
            return FlowMediumBackendStop::EmptyTablesEnd;
        }
        if self.tables.len() == 1 {
            FlowMediumBackendStop::FirstTableObjectBody
        } else {
            FlowMediumBackendStop::SecondTableObjectBody
        }
    }
    /// Reports exact prefix coverage and incomplete full-record validation; e.g. `let coverage = backend.coverage()`.
    pub fn coverage(&self) -> FlowMediumBackendCoverage {
        FlowMediumBackendCoverage {
            backend_bytes: self.backend_bytes().len(),
            known_backend_bytes: self.end,
            unknown_bytes: self.backend_bytes().len() - self.end,
            backend_framing_complete: self.stop() == FlowMediumBackendStop::EmptyTablesEnd,
            full_record_schema: false,
            achieved_error: 0,
        }
    }
    /// Borrows backend/runtime/application exclusions; e.g. `let exclusions = backend.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowMetadataPrefixDiagnostic] {
        self.prefix.diagnostics()
    }
}
impl<'a> FlowMediumBackendTable<'a> {
    /// Returns the native table ordinal; e.g. `let ordinal = table.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the full known table interval relative to the backend; e.g. `let span = table.backend_interval()`.
    pub fn backend_interval(self) -> (usize, usize) {
        let span = &self.owner.tables[self.ordinal];
        (span.start, span.end)
    }
    /// Returns the full declared nonnegative signed count; e.g. `let count = table.declared_count()`.
    pub fn declared_count(self) -> i32 {
        self.owner.tables[self.ordinal].count
    }
    /// Returns a populated table's first uninterpreted signed kind; e.g. `let kind = table.first_kind()`.
    pub fn first_kind(self) -> Option<i32> {
        self.owner.tables[self.ordinal].kind
    }
    /// Borrows exactly sixteen first-table kind-one key bytes; e.g. `let key = table.first_key_bytes()`.
    pub fn first_key_bytes(self) -> Option<&'a [u8]> {
        let span = &self.owner.tables[self.ordinal];
        span.key
            .then(|| &self.owner.backend_bytes()[span.start + 9..span.end])
    }
}
impl FlowMediumBackendEdit {
    /// Borrows the unchanged typed backend; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowMediumBackendPrefix {
        &self.source
    }
    /// Borrows the fully reparsed backend prefix; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowMediumBackendPrefix {
        &self.result
    }
    /// Borrows the outer-prefix edit and carrier history; e.g. `let history = edit.prefix_edit().carrier_edit().history()`.
    pub fn prefix_edit(&self) -> &FlowMetadataPrefixEdit {
        &self.prefix_edit
    }
}
