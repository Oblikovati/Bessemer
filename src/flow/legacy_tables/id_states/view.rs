use super::*;

impl FlowIdStates {
    /// Borrows the complete original carrier; e.g. `let bytes = states.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = states.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows all raw payload bytes through exact EOF; e.g. `let bytes = states.payload_bytes()`.
    pub fn payload_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the uninterpreted unsigned header without a version gate; e.g. `let word = states.header_word()`.
    pub fn header_word(&self) -> u32 {
        self.wire.version as u32
    }
    /// Returns the original physical limits; e.g. `let limits = states.limits()`.
    pub fn limits(&self) -> FlowIdStateLimits {
        self.limits
    }
    /// Borrows a physical row, including duplicate identifiers; e.g. `let entry = states.entry(0)`.
    pub fn entry(&self, ordinal: usize) -> Option<FlowIdStateEntry<'_>> {
        (ordinal < self.count).then_some(FlowIdStateEntry {
            owner: self,
            ordinal,
        })
    }
    /// Enumerates all rows in wire order; e.g. `let count = states.entries().len()`.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = FlowIdStateEntry<'_>> {
        (0..self.count).map(|ordinal| FlowIdStateEntry {
            owner: self,
            ordinal,
        })
    }
    /// Reports complete raw framing coverage, without native dispatch; e.g. `let coverage = states.coverage()`.
    pub fn coverage(&self) -> FlowIdStateCoverage {
        FlowIdStateCoverage {
            payload_bytes: self.wire.decoded.len(),
            entries: self.count,
            framing_bytes: 8,
            row_bytes: self.count * 5,
            native_version_dispatch: false,
            achieved_error: 0,
        }
    }
    /// Borrows native and semantic exclusions; e.g. `let exclusions = states.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowIdStateDiagnostic] {
        &[
            FlowIdStateDiagnostic::IdentifierRolesNotResolved,
            FlowIdStateDiagnostic::NativeHashTableNotReproduced,
            FlowIdStateDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowIdStateEntry<'_> {
    /// Returns the physical ordinal independently of its signed identifier; e.g. `let ordinal = entry.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete five-byte row interval; e.g. `let interval = entry.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        let start = 8 + self.ordinal * 5;
        (start, start + 5)
    }
    /// Returns the unmodified signed key; e.g. `let identifier = entry.identifier()`.
    pub fn identifier(self) -> i32 {
        wire::signed_at(&self.owner.wire.decoded, self.payload_interval().0)
    }
    /// Returns all eight stored state bits without Boolean normalization; e.g. `let state = entry.state_byte()`.
    pub fn state_byte(self) -> u8 {
        self.owner.wire.decoded[self.payload_interval().0 + 4]
    }
}

impl FlowIdStateEdit {
    /// Borrows the original typed checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowIdStates {
        &self.source
    }
    /// Borrows the completely reopened typed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowIdStates {
        &self.result
    }
    /// Borrows all carrier checkpoints and physical history; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
