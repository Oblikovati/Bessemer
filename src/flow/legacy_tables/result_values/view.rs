use super::*;
impl FlowImsValues {
    /// Borrows the immutable value-carrier checkpoint; e.g. `let raw = values.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the revision-bound value occurrence; e.g. `let ordinal = values.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Borrows exact retained definition provenance and the fresh declaration; e.g. `let definitions = values.context().definitions()`.
    pub fn context(&self) -> &FlowImsValueContext {
        &self.context
    }
    /// Returns the validated outer byte version; e.g. `assert_eq!(values.version(), 2)`.
    pub fn version(&self) -> u8 {
        self.wire.version as u8
    }
    /// Returns the original independent bounds; e.g. `let bound = values.options().maximum_bl_rows`.
    pub fn options(&self) -> FlowImsValueOptions {
        self.options
    }
    /// Borrows all inflated bytes through complete EOF; e.g. `let raw = values.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns actual objects inherited from the retained definitions; e.g. `let count = values.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.objects.len()
    }
    /// Borrows one checked physical value object; e.g. `let object = values.object(0)?`.
    pub fn object(&self, ordinal: usize) -> Result<FlowImsValue<'_>> {
        let span = self
            .objects
            .get(ordinal)
            .ok_or_else(|| table_index_error(501, "value object", ordinal, self.objects.len()))?;
        Ok(FlowImsValue {
            owner: self,
            ordinal,
            span,
        })
    }
    /// Reports complete exact framing and definition provenance; e.g. `let proof = values.coverage()`.
    pub fn coverage(&self) -> FlowImsValueCoverage {
        let counts = self.shape_counts();
        FlowImsValueCoverage {
            mode: wire::mode(&self.wire),
            definition_record: self.context.definitions().record().clone(),
            fresh_objects: self.context.declaration(),
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            stored_objects: self.objects.len(),
            shared_arrays: counts[0],
            empty_shared_arrays: counts[1],
            shared_values: self.totals.shared,
            bl_bodies: counts[2],
            empty_bl_bodies: counts[3],
            bl_rows: self.totals.bl_rows,
            bl_values: self.totals.bl_values,
            achieved_error: 0,
        }
    }
    fn shape_counts(&self) -> [usize; 4] {
        let arrays = self
            .objects
            .iter()
            .flat_map(|object| object.arrays.iter().flatten());
        let bl = self.objects.iter().filter_map(|object| object.bl);
        [
            arrays.clone().count(),
            arrays.filter(|span| span.end - span.start == 12).count(),
            bl.clone().count(),
            bl.filter(|span| span.end - span.start == 12).count(),
        ]
    }
    /// Qualifies fresh-state, pairing, runtime and engineering assumptions; e.g. `let exclusions = values.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowImsValueDiagnostic] {
        use FlowImsValueDiagnostic::*;
        &[
            FreshObjectsAssumed,
            DefinitionPairingAssumed,
            NativeRuntimeNotReproduced,
            EngineeringColumnsNotAssigned,
            NativeApplicationNotChecked,
        ]
    }
}
impl<'a> FlowImsValue<'a> {
    /// Returns physical order rather than an identifier-derived key; e.g. `let ordinal = object.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the signed identifier inherited from this context; e.g. `let identifier = object.identifier()`.
    pub fn identifier(self) -> i32 {
        self.span.identifier
    }
    /// Borrows the exact corresponding retained definition; e.g. `let definition = object.definition()?`.
    pub fn definition(self) -> Result<FlowImsDefinition<'a>> {
        self.owner.context.definitions().definition(self.ordinal)
    }
    /// Returns the admitted signed IMS-value version; e.g. `assert_eq!(object.version(), 3)`.
    pub fn version(self) -> i32 {
        3
    }
    /// Returns the complete object interval beginning at its version; e.g. `let span = object.body_interval()`.
    pub fn body_interval(self) -> (usize, usize) {
        (self.span.body.start, self.span.body.end)
    }
    /// Reads a raw flag without normalizing nonzero values; e.g. `let raw = object.flag(FlowImsValueFlag::Bl)`.
    pub fn flag(self, flag: FlowImsValueFlag) -> u8 {
        self.owner.wire.decoded[self.span.flags[flag as usize]]
    }
    /// Returns the physical one-byte flag interval; e.g. `let span = object.flag_interval(FlowImsValueFlag::First)`.
    pub fn flag_interval(self, flag: FlowImsValueFlag) -> (usize, usize) {
        let start = self.span.flags[flag as usize];
        (start, start + 1)
    }
    /// Returns a complete present shared-header/value interval; e.g. `let span = object.shared_array_interval(FlowImsValueArray::Main)`.
    pub fn shared_array_interval(self, kind: FlowImsValueArray) -> Option<(usize, usize)> {
        self.span.arrays[kind as usize].map(|span| (span.start, span.end))
    }
    /// Borrows exact raw-wide values, distinguishing absent from present-empty arrays; e.g. `let array = object.array(FlowImsValueArray::Main)`.
    pub fn array(self, kind: FlowImsValueArray) -> Option<FlowTableWideArray<'a>> {
        let span = self.span.arrays[kind as usize]?;
        Some(FlowTableWideArray {
            bytes: &self.owner.wire.decoded[span.start + 12..span.end],
            start: span.start + 12,
        })
    }
    /// Borrows the complete BL body when present, including zero-row bodies; e.g. `let bl = object.bl()`.
    pub fn bl(self) -> Option<FlowImsBlRows<'a>> {
        self.span.bl.map(|span| FlowImsBlRows {
            owner: self.owner,
            span,
        })
    }
}
impl<'a> FlowImsBlRows<'a> {
    /// Returns the admitted BL version; e.g. `assert_eq!(bl.version(), 0)`.
    pub fn version(self) -> i32 {
        0
    }
    /// Returns the complete BL header and row interval; e.g. `let span = bl.body_interval()`.
    pub fn body_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Returns actual serialized rows; e.g. `let count = bl.row_count()`.
    pub fn row_count(self) -> usize {
        (self.span.end - self.span.start - 12) / 144
    }
    /// Borrows exactly 18 checked raw-wide values; e.g. `let row = bl.row(0)?`.
    pub fn row(self, ordinal: usize) -> Result<FlowTableWideArray<'a>> {
        let start =
            wire::element_offset(501, self.span.start + 12, self.row_count(), ordinal, 144)?;
        Ok(FlowTableWideArray {
            bytes: &self.owner.wire.decoded[start..start + 144],
            start,
        })
    }
    /// Reports the reference post-load state observation without executing native loading; e.g. `assert_eq!(bl.native_state_after_load(), 0)`.
    pub fn native_state_after_load(self) -> u8 {
        0
    }
}
