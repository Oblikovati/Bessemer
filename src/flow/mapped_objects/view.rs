use super::*;

impl FlowMappedObjects {
    /// Borrows the exact source checkpoint; e.g. `let raw = list.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = list.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Borrows the complete raw payload; e.g. `let raw = list.payload()`.
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    /// Returns the validated outer list word; e.g. `let version = list.version()`.
    pub fn version(&self) -> u32 {
        self.version
    }
    /// Borrows all physical entries, including nulls; e.g. `let entries = list.entries()`.
    pub fn entries(&self) -> &[FlowMappedEntry] {
        &self.entries
    }
    /// Borrows the explicit original context; e.g. `let context = list.context()`.
    pub fn context(&self) -> &FlowMappedContext {
        &self.context
    }
    /// Returns the accepted total limits; e.g. `let maximum = list.limits().maximum_pairs`.
    pub fn limits(&self) -> FlowMappedLimits {
        self.limits
    }
    /// Reports exact wire checks and external-context scope; e.g. `let checked = list.coverage()`.
    pub fn coverage(&self) -> FlowMappedCoverage {
        self.coverage
    }
    /// Borrows execution and semantic exclusions; e.g. `let exclusions = list.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowMappedDiagnostic] {
        &[
            FlowMappedDiagnostic::NativeObjectContextNotValidated,
            FlowMappedDiagnostic::EngineeringSemanticsNotEstablished,
            FlowMappedDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowMappedEntry {
    /// Returns the exact payload-relative interval; e.g. `let (start, end) = entry.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (self.start, self.end)
    }
    /// Returns the unchanged presence byte; e.g. `let raw = entry.presence_byte()`.
    pub fn presence_byte(&self) -> u8 {
        self.presence
    }
    /// Borrows the optional body, distinct from a null entry; e.g. `let body = entry.body()`.
    pub fn body(&self) -> Option<&FlowMappedBody> {
        self.body.as_ref()
    }
}

impl FlowMappedBody {
    /// Returns the independent validated body version; e.g. `let version = body.version()`.
    pub fn version(&self) -> u32 {
        self.version
    }
    /// Returns the full stored state byte; e.g. `let raw = body.state_byte()`.
    pub fn state_byte(&self) -> u8 {
        self.state
    }
    /// Borrows the nine-word prefix; e.g. `let bits = body.numeric().raw_bits()`.
    pub fn numeric(&self) -> &FlowMappedNumericBlock {
        &self.numeric
    }
    /// Borrows all physical identifier pairs, preserving duplicates; e.g. `let pairs = body.pairs()`.
    pub fn pairs(&self) -> &[FlowMappedIdentifierPair] {
        &self.pairs
    }
    /// Returns the neutral stored product factor; e.g. `let factor = body.stored_factor()`.
    pub fn stored_factor(&self) -> u32 {
        self.factor
    }
    /// Returns supplied context without native-validation credit; e.g. `let count = body.matched_count()`.
    pub fn matched_count(&self) -> Option<FlowMatchedObjectCount> {
        self.matched
    }
    /// Borrows exact product-framed values; e.g. `let mapped = body.mapped_values()`.
    pub fn mapped_values(&self) -> &FlowMappedNumericBlock {
        &self.mapped
    }
    /// Borrows the version-5 block when physically present; e.g. `let legacy = body.legacy()`.
    pub fn legacy(&self) -> Option<&FlowMappedLegacy> {
        self.legacy.as_ref()
    }
}

impl FlowMappedIdentifierPair {
    /// Returns the original source word's signed view; e.g. `let id = pair.source_identifier()`.
    pub fn source_identifier(self) -> i32 {
        self.source
    }
    /// Returns the original associated signed value; e.g. `let value = pair.associated_value()`.
    pub fn associated_value(self) -> i32 {
        self.associated
    }
}

impl FlowMappedNumericBlock {
    /// Returns the payload-relative first-word offset; e.g. `let offset = block.payload_offset()`.
    pub fn payload_offset(&self) -> usize {
        self.offset
    }
    /// Borrows all stored bits, including NaN payloads; e.g. `let bits = block.raw_bits()`.
    pub fn raw_bits(&self) -> &[u64] {
        &self.bits
    }
    /// Interprets each retained pattern as binary64 without normalization; e.g. `let values: Vec<f64> = block.numeric_values().collect()`.
    pub fn numeric_values(&self) -> impl ExactSizeIterator<Item = f64> + '_ {
        self.bits.iter().map(|bits| f64::from_bits(*bits))
    }
}

impl FlowMappedLegacy {
    /// Borrows the sixteen-word legacy prefix; e.g. `let bits = legacy.prefix().raw_bits()`.
    pub fn prefix(&self) -> &FlowMappedNumericBlock {
        &self.prefix
    }
    /// Borrows all three counted arrays in wire order; e.g. `let arrays = legacy.arrays()`.
    pub fn arrays(&self) -> &[FlowMappedNumericBlock; 3] {
        &self.arrays
    }
    /// Borrows all tripled-product words; e.g. `let bits = legacy.tripled_values().raw_bits()`.
    pub fn tripled_values(&self) -> &FlowMappedNumericBlock {
        &self.tripled
    }
}

impl FlowMappedEdit {
    /// Borrows the complete earlier typed checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowMappedObjects {
        &self.source
    }
    /// Borrows the fully reparsed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowMappedObjects {
        &self.result
    }
    /// Borrows complete physical occurrence history; e.g. `let parents = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
