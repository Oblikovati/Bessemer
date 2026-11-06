use super::*;

impl FlowParallelTable {
    /// Borrows the complete original carrier; e.g. `let raw = table.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = table.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows all decoded bytes; e.g. `let raw = table.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the observed nonnegative version; e.g. `let version = table.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns original limits and transport; e.g. `let options = table.options()`.
    pub fn options(&self) -> FlowParallelTableOptions {
        self.options
    }
}

impl FlowLeafPairs {
    /// Borrows the complete original carrier; e.g. `let raw = pairs.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = pairs.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows all decoded bytes; e.g. `let raw = pairs.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the observed zero version; e.g. `let version = pairs.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns original explicit leaf context and limits; e.g. `let options = pairs.options()`.
    pub fn options(&self) -> FlowLeafPairOptions {
        self.options
    }
}

impl FlowFiveWordTable {
    /// Borrows the complete original carrier; e.g. `let raw = table.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = table.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows all decoded bytes; e.g. `let raw = table.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the observed nonnegative version; e.g. `let version = table.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns original explicit count context and limits; e.g. `let options = table.options()`.
    pub fn options(&self) -> FlowFiveWordTableOptions {
        self.options
    }
}

impl FlowRawByteTable {
    /// Borrows the complete original carrier; e.g. `let raw = table.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = table.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows all decoded bytes; e.g. `let raw = table.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the observed nonnegative version; e.g. `let version = table.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns original explicit count context and limits; e.g. `let options = table.options()`.
    pub fn options(&self) -> FlowRawByteTableOptions {
        self.options
    }
}

impl<T: Send + Sync> FlowSimpleTableEdit<T> {
    /// Borrows the immutable original typed checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &T {
        &self.source
    }
    /// Borrows the completely reopened typed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &T {
        &self.result
    }
    /// Borrows all physical history relations and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
