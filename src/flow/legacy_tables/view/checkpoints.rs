use super::*;

impl FlowPointerTables {
    /// Borrows the complete original checkpoint; e.g. `let raw = table.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = table.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows every decoded byte; e.g. `let raw = table.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the observed legacy version; e.g. `let version = table.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns exact transport/context/limit options; e.g. `let bound = table.options().global_pointer_count`.
    pub fn options(&self) -> FlowPointerTableOptions {
        self.options
    }
}

impl FlowLegacyWords {
    /// Borrows the complete original checkpoint; e.g. `let raw = words.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = words.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows every decoded byte; e.g. `let raw = words.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the observed legacy version; e.g. `let version = words.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns the original context and limits; e.g. `let legacy = words.options().legacy_counts`.
    pub fn options(&self) -> FlowLegacyWordOptions {
        self.options
    }
}

impl FlowPointerTableEdit {
    /// Borrows the exact immutable source; e.g. `let original = edit.source()`.
    pub fn source(&self) -> &FlowPointerTables {
        &self.source
    }
    /// Borrows the complete reopened result; e.g. `let updated = edit.result()`.
    pub fn result(&self) -> &FlowPointerTables {
        &self.result
    }
    /// Borrows complete carrier checkpoints/history; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}

impl FlowLegacyWordEdit {
    /// Borrows the exact immutable source; e.g. `let original = edit.source()`.
    pub fn source(&self) -> &FlowLegacyWords {
        &self.source
    }
    /// Borrows the complete reopened result; e.g. `let updated = edit.result()`.
    pub fn result(&self) -> &FlowLegacyWords {
        &self.result
    }
    /// Borrows complete carrier checkpoints/history; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
