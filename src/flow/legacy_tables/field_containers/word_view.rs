use super::*;
impl FlowLegacyWordFields {
    /// Borrows the exact physical source checkpoint; e.g. `let source = fields.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = fields.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns the unversioned layout; e.g. `let identifier = fields.profile().identifier()`.
    pub fn profile(&self) -> FlowLegacyWordProfile {
        self.profile
    }
    /// Returns the original independent bounds; e.g. `let limit = fields.options().maximum_patterns`.
    pub fn options(&self) -> FlowLegacyFieldWordOptions {
        self.options
    }
    /// Borrows complete raw bytes including any count prefix; e.g. `let raw = fields.payload_bytes()`.
    pub fn payload_bytes(&self) -> &[u8] {
        &self.raw
    }
    /// Returns the actual stored-pattern count; e.g. `let count = fields.stored_count()`.
    pub fn stored_count(&self) -> usize {
        self.count
    }
    /// Returns one checked raw interval; e.g. `let span = fields.pattern_interval(7)?`.
    pub fn pattern_interval(&self, index: usize) -> Result<(usize, usize)> {
        let start =
            wire::element_offset(self.profile.identifier(), self.first, self.count, index, 8)?;
        Ok((start, start + 8))
    }
    /// Reads a complete unsigned pattern without float conversion; e.g. `let bits = fields.pattern(7)?`.
    pub fn pattern(&self, index: usize) -> Result<u64> {
        let (start, _) = self.pattern_interval(index)?;
        Ok(wire::wide_at(&self.raw, start))
    }
    /// Reports complete exact serialized coverage; e.g. `let coverage = fields.coverage()`.
    pub fn coverage(&self) -> FlowLegacyWordCoverage {
        FlowLegacyWordCoverage {
            mode: FlowLegacyTableCheckMode::ExactRawTable,
            profile: self.profile,
            payload_bytes: self.raw.len(),
            prefix_bytes: self.first,
            patterns: self.count,
            achieved_error: 0,
        }
    }
    /// Borrows semantic and native exclusions; e.g. `let exclusions = fields.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowLegacyFieldDiagnostic] {
        use FlowLegacyFieldDiagnostic::*;
        const NOTES: [FlowLegacyFieldDiagnostic; 5] = [
            EngineeringMeaningsNotAssigned,
            NativeRuntimeNotReproduced,
            NativeApplicationNotChecked,
            NativeFloatingPointEnvironmentNotReproduced,
            NativeVirtualMethodNotExecuted,
        ];
        &NOTES[..if self.profile == FlowLegacyWordProfile::Fixed105 {
            5
        } else {
            3
        }]
    }
}
