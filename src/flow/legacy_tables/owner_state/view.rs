use super::*;
impl FlowOwnerState {
    /// Borrows the immutable original carrier; e.g. `let raw = state.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected physical checkpoint identity; e.g. `let ordinal = state.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the normalized serialized layout; e.g. `let kind = state.kind()`.
    pub fn kind(&self) -> FlowOwnerStateKind {
        self.kind
    }
    /// Retains legacy version absence; e.g. `let version = state.version()`.
    pub fn version(&self) -> Option<i32> {
        (self.kind == FlowOwnerStateKind::Modern434).then_some(self.wire.version)
    }
    /// Returns the original payload bound; e.g. `let bound = state.options().maximum_payload_bytes`.
    pub fn options(&self) -> FlowOwnerStateOptions {
        self.options
    }
    /// Borrows every validated raw byte; e.g. `let raw = state.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Borrows all physically stored words as raw patterns; e.g. `let bits = state.words().raw_word(0)`.
    pub fn words(&self) -> FlowBlockWordArray<'_> {
        let start = self.kind.words_start();
        let end = start + self.kind.word_count() * 4;
        FlowBlockWordArray {
            bytes: &self.wire.decoded[start..end],
            start,
        }
    }
    /// Reads one signed modern member, rejecting legacy layout or invalid indices; e.g. `let value = state.modern_member(0)?`.
    pub fn modern_member(&self, index: usize) -> Result<i32> {
        self.require_modern("signed member")?;
        let offset = wire::element_offset(434, 4, 6, index, 4)?;
        Ok(wire::signed_at(&self.wire.decoded, offset))
    }
    /// Reads the unsigned modern byte while retaining legacy absence; e.g. `let byte = state.state_byte()`.
    pub fn state_byte(&self) -> Option<u8> {
        (self.kind == FlowOwnerStateKind::Modern434).then(|| self.wire.decoded[28])
    }
    /// Reports exact physical scope and enabled projection; e.g. `let proof = state.coverage()`.
    pub fn coverage(&self) -> FlowOwnerStateCoverage {
        let enabled = self.updates_enabled();
        FlowOwnerStateCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            raw_words: self.kind.word_count(),
            updates_enabled: enabled,
            projected_updates: if enabled { 7 } else { 0 },
            achieved_error: 0,
        }
    }
    /// Borrows native-runtime, engineering and application exclusions; e.g. `let exclusions = state.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        use FlowSimpleTableDiagnostic::*;
        &[
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
    pub(super) fn require_modern(&self, field: &str) -> Result<()> {
        if self.kind == FlowOwnerStateKind::Modern434 {
            return Ok(());
        }
        Err(table_failure(
            122,
            FailureClass::Missing,
            format!(
                "layout {:?}, field {field}; expected a physically stored modern record434 field",
                self.kind
            ),
        ))
    }
}
