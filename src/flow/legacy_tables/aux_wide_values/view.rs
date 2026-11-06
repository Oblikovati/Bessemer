use super::*;
impl FlowAuxWideValues {
    /// Borrows the immutable original checkpoint; e.g. `let raw = values.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the revision-bound occurrence; e.g. `let ordinal = values.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the admitted version; e.g. `let version = values.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Returns the original independent limits; e.g. `let bound = values.options().maximum_values`.
    pub fn options(&self) -> FlowAuxWideOptions {
        self.options
    }
    /// Borrows the complete decoded body; e.g. `let raw = values.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Retains the physically stored version-zero discarded word; e.g. `let legacy = values.discarded_legacy()`.
    pub fn discarded_legacy(&self) -> Option<i32> {
        (self.version() == 0).then(|| wire::signed_at(&self.wire.decoded, 4))
    }
    /// Returns the actual stored value count; e.g. `let count = values.stored_count()`.
    pub fn stored_count(&self) -> usize {
        (self.wire.decoded.len() - self.start) / 8
    }
    /// Borrows all raw patterns without floating-point conversion; e.g. `let bits = values.values().raw_bits(0)`.
    pub fn values(&self) -> FlowTableWideArray<'_> {
        FlowTableWideArray {
            bytes: &self.wire.decoded[self.start..],
            start: self.start,
        }
    }
    /// Reports the reference native discard observation without dropping serialized values; e.g. `assert!(values.native_values_discarded())`.
    pub fn native_values_discarded(&self) -> bool {
        true
    }
    /// Reports exact compressed and decoded framing; e.g. `let proof = values.coverage()`.
    pub fn coverage(&self) -> FlowSimpleTableCoverage {
        FlowSimpleTableCoverage {
            mode: wire::mode(&self.wire),
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            wire_rows: 0,
            wide_words: self.stored_count(),
            raw_words: 0,
            raw_bytes: 0,
            achieved_error: 0,
        }
    }
    /// Qualifies native runtime, engineering and application acceptance; e.g. `let exclusions = values.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        use FlowSimpleTableDiagnostic::*;
        &[
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
}
