use super::*;
impl FlowObjectRecords {
    /// Borrows the original immutable checkpoint; e.g. `let raw = objects.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the physical occurrence identity; e.g. `let ordinal = objects.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the admitted version; e.g. `let version = objects.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns the resource policy used to open the view; e.g. `let bound = objects.options().maximum_rows`.
    pub fn options(&self) -> FlowObjectRecordOptions {
        self.options
    }
    /// Borrows every raw payload byte; e.g. `let raw = objects.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the serialized row count, excluding the trailer; e.g. `let count = objects.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.count
    }
    /// Borrows a checked stored row; e.g. `let row = objects.row(0)?`.
    pub fn row(&self, ordinal: usize) -> Result<FlowObjectRow<'_>> {
        let start = wire::element_offset(304, 8, self.count, ordinal, self.stride)?;
        Ok(FlowObjectRow {
            owner: self,
            ordinal,
            start,
        })
    }
    /// Borrows the physically present uncounted trailer; e.g. `let trailer = objects.legacy_trailer()`.
    pub fn legacy_trailer(&self) -> Option<FlowObjectTrailer<'_>> {
        (self.count > 0 && self.version() < 5).then(|| FlowObjectTrailer {
            owner: self,
            start: 8 + self.count * self.stride,
        })
    }
    /// Reports exact raw framing coverage; e.g. `let coverage = objects.coverage()`.
    pub fn coverage(&self) -> FlowObjectRecordCoverage {
        FlowObjectRecordCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            stored_rows: self.count,
            legacy_trailer: self.legacy_trailer().is_some(),
            achieved_error: 0,
        }
    }
    /// Borrows runtime, binding and application exclusions; e.g. `let exclusions = objects.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowObjectRecordDiagnostic] {
        use FlowObjectRecordDiagnostic::*;
        &[
            NativeIdentityBindingNotVerified,
            NativeRuntimePopulationNotReproduced,
            LegacyFlagLifecycleNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
    /// Reads an existing signed field identity; e.g. `let member = objects.signed(&id)?`.
    pub fn signed(&self, id: &FlowObjectFieldId) -> Result<i32> {
        self.check_field(id, 4)?;
        Ok(wire::signed_at(&self.wire.decoded, id.offset))
    }
    /// Reads an existing raw byte identity; e.g. `let byte = objects.byte(&id)?`.
    pub fn byte(&self, id: &FlowObjectFieldId) -> Result<u8> {
        self.check_field(id, 1)?;
        Ok(self.wire.decoded[id.offset])
    }
}
