use super::*;

impl FlowPointerBodies {
    /// Borrows the complete immutable carrier checkpoint; e.g. `let bytes = bodies.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = bodies.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the observed legacy version; e.g. `let version = bodies.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows every decoded byte through exact EOF; e.g. `let bytes = bodies.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Borrows the original immutable runtime context and limits; e.g. `let options = bodies.options()`.
    pub fn options(&self) -> &FlowPointerBodyOptions {
        &self.options
    }
}

impl FlowPointerBodyEdit {
    /// Borrows the original typed checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowPointerBodies {
        &self.source
    }
    /// Borrows the completely reopened typed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowPointerBodies {
        &self.result
    }
    /// Borrows carrier checkpoints and all physical history relations; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
