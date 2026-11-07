use super::*;
impl FlowResultDefinitions {
    /// Borrows the immutable carrier checkpoint; e.g. `let raw = definitions.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected physical occurrence identity; e.g. `let ordinal = definitions.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the validated outer byte version, independently of IMS versions; e.g. `assert_eq!(definitions.version(), 1)`.
    pub fn version(&self) -> u8 {
        self.wire.version as u8
    }
    /// Returns the independent caller bounds; e.g. `let bound = definitions.options().maximum_objects`.
    pub fn options(&self) -> FlowResultDefinitionOptions {
        self.options
    }
    /// Borrows the complete inflated body; e.g. `let bytes = definitions.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the actual primary-object count, including duplicates; e.g. `let count = definitions.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.objects.len()
    }
    /// Returns a complete collection's interval including its count; e.g. `let first = definitions.collection_interval(0)?`.
    pub fn collection_interval(&self, ordinal: usize) -> Result<(usize, usize)> {
        self.collections
            .get(ordinal)
            .map(|span| (span.start, span.end))
            .ok_or_else(|| table_index_error(500, "collection", ordinal, 5))
    }
    /// Borrows one checked physical IMS definition; e.g. `let object = definitions.definition(0)?`.
    pub fn definition(&self, ordinal: usize) -> Result<FlowImsDefinition<'_>> {
        let span = self
            .objects
            .get(ordinal)
            .ok_or_else(|| table_index_error(500, "definition", ordinal, self.objects.len()))?;
        Ok(FlowImsDefinition {
            owner: self,
            ordinal,
            span,
        })
    }
    /// Reports exact complete compressed and decoded coverage; e.g. `let proof = definitions.coverage()`.
    pub fn coverage(&self) -> FlowResultDefinitionCoverage {
        let arrays = self
            .objects
            .iter()
            .flat_map(|object| object.arrays.iter().flatten());
        FlowResultDefinitionCoverage {
            mode: wire::mode(&self.wire),
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            stored_objects: self.objects.len(),
            stored_arrays: arrays.clone().count(),
            empty_arrays: arrays.filter(|span| span.end - span.start == 12).count(),
            raw_words: self.values,
            achieved_error: 0,
        }
    }
    /// Qualifies engineering, native runtime and application exclusions; e.g. `let exclusions = definitions.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        use FlowSimpleTableDiagnostic::*;
        &[
            EngineeringRolesNotResolved,
            NativeRuntimeNotReproduced,
            NativeApplicationNotChecked,
        ]
    }
}
impl<'a> FlowImsDefinition<'a> {
    /// Returns physical order rather than an identifier-derived key; e.g. `let ordinal = object.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Reads the signed identifier without deduplication; e.g. `let identifier = object.identifier()`.
    pub fn identifier(self) -> i32 {
        wire::signed_at(&self.owner.wire.decoded, self.span.identifier_start)
    }
    /// Returns the identifier interval in the separate preceding block; e.g. `let span = object.identifier_interval()`.
    pub fn identifier_interval(self) -> (usize, usize) {
        (self.span.identifier_start, self.span.identifier_start + 4)
    }
    /// Borrows the complete validated serialized GUID; e.g. `let guid = object.guid_bytes()`.
    pub fn guid_bytes(self) -> &'a [u8] {
        &self.owner.wire.decoded[self.span.guid_start..self.span.guid_start + 16]
    }
    /// Returns the definition-body interval beginning at its signed version; e.g. `let span = object.body_interval()`.
    pub fn body_interval(self) -> (usize, usize) {
        (self.span.body.start, self.span.body.end)
    }
    /// Returns the normalized IMS version; e.g. `let version = object.version()`.
    pub fn version(self) -> FlowImsDefinitionVersion {
        self.span.version
    }
    /// Reads the physically present allocation byte; e.g. `let flag = object.allocation_byte()`.
    pub fn allocation_byte(self) -> Option<u8> {
        self.span
            .allocation
            .map(|offset| self.owner.wire.decoded[offset])
    }
    /// Returns the entire shared marker/count/value interval when present; e.g. `let span = object.shared_array_interval(FlowImsArrayKind::First)`.
    pub fn shared_array_interval(self, kind: FlowImsArrayKind) -> Option<(usize, usize)> {
        self.span.arrays[kind as usize].map(|span| (span.start, span.end))
    }
    /// Borrows only raw words, distinguishing absent from present-empty arrays; e.g. `let words = object.array(FlowImsArrayKind::First)`.
    pub fn array(self, kind: FlowImsArrayKind) -> Option<FlowBlockWordArray<'a>> {
        let span = self.span.arrays[kind as usize]?;
        Some(FlowBlockWordArray {
            bytes: &self.owner.wire.decoded[span.start + 12..span.end],
            start: span.start + 12,
        })
    }
}
