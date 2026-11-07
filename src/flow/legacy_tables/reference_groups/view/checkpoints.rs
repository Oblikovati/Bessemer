use super::*;

impl FlowReferenceGroups {
    /// Borrows the complete original carrier; e.g. `let bytes = references.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = references.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the observed legacy version; e.g. `let version = references.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows every decoded byte through exact EOF; e.g. `let bytes = references.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the explicit mapping context and original bounds; e.g. `let options = references.options()`.
    pub fn options(&self) -> FlowReferenceGroupOptions {
        self.options
    }
}

impl FlowReferenceGroupEdit {
    /// Borrows the original typed checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowReferenceGroups {
        &self.source
    }
    /// Borrows the completely reopened result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowReferenceGroups {
        &self.result
    }
    /// Borrows all physical history and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
