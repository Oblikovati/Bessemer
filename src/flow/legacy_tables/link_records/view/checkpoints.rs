use super::*;

impl FlowLinkRecords {
    /// Borrows the original complete carrier checkpoint; e.g. `let raw = links.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the revision-bound selected occurrence; e.g. `let ordinal = links.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows every decoded byte; e.g. `let raw = links.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the observed legacy version; e.g. `let version = links.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns the original typed transport/context/limits; e.g. `let options = links.options()`.
    pub fn options(&self) -> FlowLinkRecordOptions {
        self.options
    }
}

impl FlowLinkRecordEdit {
    /// Borrows the original immutable link view; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowLinkRecords {
        &self.source
    }
    /// Borrows the fully reopened result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowLinkRecords {
        &self.result
    }
    /// Borrows complete physical carrier history; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
