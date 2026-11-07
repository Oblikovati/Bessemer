use super::*;
impl FlowLogicalIndex {
    /// Borrows the complete immutable physical provenance; e.g. `let raw = index.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Returns the requested policy, including scan requests; e.g. `let policy = index.policy()`.
    pub fn policy(&self) -> FlowIndexPolicy {
        self.policy
    }
    /// Returns independent map and control bounds; e.g. `let limit = index.options().maximum_saved_entries`.
    pub fn options(&self) -> FlowIndexOptions {
        self.options
    }
    /// Returns the actual replay path; e.g. `let method = index.method()`.
    pub fn method(&self) -> FlowIndexMethod {
        if self.terminal.is_some() {
            FlowIndexMethod::TerminalIndex
        } else {
            FlowIndexMethod::PhysicalScan
        }
    }
    /// Borrows active spans in unsigned identifier order; e.g. `let target = index.active_entries().get(&14)`.
    pub fn active_entries(&self) -> &BTreeMap<u32, FlowIndexedSpan> {
        &self.active
    }
    /// Borrows saved pairs in unsigned key order; e.g. `let span = index.saved_entries().get(&2)`.
    pub fn saved_entries(&self) -> &BTreeMap<u64, u64> {
        &self.saved
    }
    /// Borrows ordered terminal entries and their exact source occurrence; e.g. `let selected = index.terminal_control()`.
    pub fn terminal_control(&self) -> Option<&FlowTerminalIndex> {
        self.terminal.as_ref()
    }
    /// Borrows the exact selected saved control, retaining duplicate provenance; e.g. `let selected = index.saved_control()`.
    pub fn saved_control(&self) -> Option<&FlowSavedSpans> {
        self.saved_control.as_ref()
    }
    /// Reports exact replay checks and explicit semantic exclusions; e.g. `let checked = index.coverage()`.
    pub fn coverage(&self) -> FlowIndexCoverage {
        let (terminal_decoded_bytes, saved_decoded_bytes) = self.control_bytes();
        FlowIndexCoverage {
            method: self.method(),
            physical_wire_bytes: self.source.as_bytes().len(),
            physical_records: self.source.records().len(),
            active_entries: self.active.len(),
            saved_entries: self.saved.len(),
            terminal_decoded_bytes,
            saved_decoded_bytes,
            ordinary_payloads_decoded: false,
            indexed_ordinary_targets_validated: false,
            complete_document_schema: false,
            achieved_byte_error: 0,
        }
    }
    fn control_bytes(&self) -> (usize, usize) {
        let terminal_decoded_bytes = self
            .terminal
            .as_ref()
            .map_or(0, |control| control.decoded_bytes().len());
        let saved_decoded_bytes = self
            .saved_control
            .as_ref()
            .map_or(0, |control| control.decoded_bytes().len());
        (terminal_decoded_bytes, saved_decoded_bytes)
    }
    /// Borrows native and semantic qualifications; e.g. `let exclusions = index.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowIndexDiagnostic] {
        use FlowIndexDiagnostic::*;
        &[
            FreshEmptyMapsAssumed,
            OrdinaryPayloadsNotDecoded,
            OrdinaryTargetsNotValidated,
            NativeRecoveryNotReproduced,
            InstalledTerminalPayloadNotVerified,
            CompleteDocumentSchemaNotChecked,
            NativeApplicationNotChecked,
        ]
    }
}
