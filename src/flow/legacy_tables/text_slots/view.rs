use super::*;
impl FlowTextSlots {
    /// Borrows the immutable carrier checkpoint; e.g. `let raw = slots.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected occurrence identity; e.g. `let ordinal = slots.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the arbitrary signed header without version dispatch; e.g. `let header = slots.header()`.
    pub fn header(&self) -> i32 {
        self.wire.version
    }
    /// Returns the admitted independent bounds and explicit mode; e.g. `let bound = slots.options().maximum_units`.
    pub fn options(&self) -> FlowTextSlotOptions {
        self.options
    }
    /// Borrows the complete raw payload; e.g. `let raw = slots.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Borrows a checked physical slot, including stored-empty strings; e.g. `let slot = slots.slot(0)?`.
    pub fn slot(&self, ordinal: usize) -> Result<FlowTextSlot<'_>> {
        let span = self
            .spans
            .get(ordinal)
            .ok_or_else(|| table_index_error(311, "text slot", ordinal, SLOT_COUNT))?;
        Ok(FlowTextSlot {
            owner: self,
            ordinal,
            span,
        })
    }
    /// Reports exact framing and empty-prefix coverage; e.g. `let coverage = slots.coverage()`.
    pub fn coverage(&self) -> FlowTextSlotCoverage {
        FlowTextSlotCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            slots: SLOT_COUNT,
            stored_units: self.units,
            empty_prefixes: self.spans.iter().filter(|span| span.prefix == 0).count(),
            achieved_error: 0,
        }
    }
    /// Borrows mode, native-runtime and engineering/application exclusions; e.g. `let exclusions = slots.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowTextSlotDiagnostic] {
        use FlowTextSlotDiagnostic::*;
        &[
            NativeUtf16ModeAssumed,
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
}
impl FlowTextSlot<'_> {
    /// Returns the fixed physical ordinal; e.g. `let ordinal = slot.ordinal()`.
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
    /// Returns count-prefix and complete text interval; e.g. `let span = slot.interval()`.
    pub fn interval(&self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Returns the physical stored unit count, including terminal NUL; e.g. `let count = slot.stored_units()`.
    pub fn stored_units(&self) -> usize {
        self.span.units
    }
    /// Borrows every stored UTF-16LE byte without normalization; e.g. `let raw = slot.raw_bytes()`.
    pub fn raw_bytes(&self) -> &[u8] {
        &self.owner.wire.decoded[self.span.start + 4..self.span.end]
    }
    /// Borrows the first-NUL native prefix as lossless UTF-16LE; e.g. `let raw = slot.native_prefix_bytes()`.
    pub fn native_prefix_bytes(&self) -> &[u8] {
        &self.raw_bytes()[..self.span.prefix * 2]
    }
    /// Reads one checked stored unit, including terminators and hidden text; e.g. `let unit = slot.unit(0)`.
    pub fn unit(&self, index: usize) -> Option<u16> {
        (index < self.span.units).then(|| wire::unit_at(self.raw_bytes(), index * 2))
    }
}
