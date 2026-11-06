use super::*;

impl FlowFieldValues {
    /// Borrows the complete immutable original carrier; e.g. `let bytes = fields.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = fields.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the raw-prefix version; e.g. `let version = fields.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows the complete decoded body excluding the version; e.g. `let body = fields.body_bytes()`.
    pub fn body_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Borrows the original immutable context and bounds; e.g. `let options = fields.options()`.
    pub fn options(&self) -> &FlowFieldValueOptions {
        &self.options
    }
}

impl FlowFieldValueEdit {
    /// Borrows the original typed checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowFieldValues {
        &self.source
    }
    /// Borrows the fully reopened typed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowFieldValues {
        &self.result
    }
    /// Borrows all physical history and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
