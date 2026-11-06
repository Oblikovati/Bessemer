use super::*;
impl FlowMediumKindOne {
    pub(super) fn raw(&self) -> &[u8] {
        self.backend.metadata_prefix().payload_bytes()
    }
    /// Borrows the immutable backend checkpoint; e.g. `let backend = body.backend_prefix()`.
    pub fn backend_prefix(&self) -> &FlowMediumBackendPrefix {
        &self.backend
    }
    /// Returns the supplied independent bounds; e.g. `let limits = body.options()`.
    pub fn options(&self) -> FlowMediumBodyOptions {
        self.options
    }
    /// Returns the complete body interval in the record payload; e.g. `let span = body.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Borrows exactly the complete body; e.g. `let raw = body.body_bytes()`.
    pub fn body_bytes(&self) -> &[u8] {
        &self.raw()[self.span.start..self.span.end]
    }
    /// Borrows every following backend and outer-record byte; e.g. `let tail = body.opaque_tail()`.
    pub fn opaque_tail(&self) -> &[u8] {
        &self.raw()[self.span.end..]
    }
    /// Returns the uninterpreted base byte, without version dispatch; e.g. `let base = body.base_byte()`.
    pub fn base_byte(&self) -> u8 {
        self.raw()[self.span.start + 1]
    }
    /// Borrows the sixteen raw base key bytes; e.g. `let key = body.base_key_bytes()`.
    pub fn base_key_bytes(&self) -> &[u8] {
        &self.raw()[self.span.start + 2..self.span.start + 18]
    }
    /// Borrows the stored name and lossless native text projection; e.g. `let text = body.name()`.
    pub fn name(&self) -> FlowMetadataText<'_> {
        FlowMetadataText {
            owner: self.backend.metadata_prefix(),
            span: &self.span.name,
        }
    }
    /// Enumerates all seven curves in wire order; e.g. `let count = body.curves().len()`.
    pub fn curves(&self) -> impl ExactSizeIterator<Item = FlowMediumCurve<'_>> {
        (0..7).map(|ordinal| FlowMediumCurve {
            owner: self,
            ordinal,
        })
    }
    /// Borrows an existing curve; e.g. `let first = body.curve(0)`.
    pub fn curve(&self, ordinal: usize) -> Option<FlowMediumCurve<'_>> {
        (ordinal < 7).then_some(FlowMediumCurve {
            owner: self,
            ordinal,
        })
    }
    /// Returns all stored state bits, independently of native load-time replacement; e.g. `let state = body.state_byte()`.
    pub fn state_byte(&self) -> u8 {
        self.raw()[self.span.state]
    }
    /// Projects the documented native loaded value without mutating stored bytes; e.g. `let loaded = body.native_state_after_load()`.
    pub fn native_state_after_load(&self) -> u8 {
        1
    }
    /// Reports complete serialized-body scope and incomplete outer-record scope; e.g. `let coverage = body.coverage()`.
    pub fn coverage(&self) -> FlowMediumBodyCoverage {
        FlowMediumBodyCoverage {
            body_bytes: self.span.end - self.span.start,
            wide_words: self.span.words,
            name_units: self.name().stored_units(),
            body_framing_complete: true,
            full_record_schema: false,
            achieved_error: 0,
        }
    }
    /// Borrows semantic, runtime and following-record exclusions; e.g. `let exclusions = body.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowMediumBodyDiagnostic] {
        &[
            FlowMediumBodyDiagnostic::EngineeringAndEvaluationNotChecked,
            FlowMediumBodyDiagnostic::NativeRuntimeNotReproduced,
            FlowMediumBodyDiagnostic::FollowingRecordNotDecoded,
            FlowMediumBodyDiagnostic::NativeApplicationNotChecked,
        ]
    }
}
impl FlowMediumCurve<'_> {
    /// Returns the curve ordinal; e.g. `let ordinal = curve.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the full interval relative to the body; e.g. `let span = curve.body_interval()`.
    pub fn body_interval(self) -> (usize, usize) {
        let span = &self.owner.span.curves[self.ordinal];
        (
            span.start - self.owner.span.start,
            span.end - self.owner.span.start,
        )
    }
    /// Returns the declared array count; e.g. `let count = curve.value_count()`.
    pub fn value_count(self) -> usize {
        self.owner.span.curves[self.ordinal].count
    }
    /// Returns the unnormalized scalar pattern; e.g. `let bits = curve.scalar_pattern()`.
    pub fn scalar_pattern(self) -> u64 {
        wire::wide_at(
            self.owner.raw(),
            self.owner.span.curves[self.ordinal].scalar,
        )
    }
    /// Returns all extra-array flag bits; e.g. `let flag = curve.extra_flag_byte()`.
    pub fn extra_flag_byte(self) -> u8 {
        self.owner.raw()[self.owner.span.curves[self.ordinal].flag]
    }
    /// Distinguishes an active empty branch from an absent branch; e.g. `let active = curve.extra_arrays_active()`.
    pub fn extra_arrays_active(self) -> bool {
        self.extra_flag_byte() != 0
    }
}
impl FlowMediumBodyEdit {
    /// Borrows the original complete body; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowMediumKindOne {
        &self.source
    }
    /// Borrows the reparsed complete body; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowMediumKindOne {
        &self.result
    }
    /// Borrows all outer checkpoints and physical history; e.g. `let history = edit.prefix_edit().carrier_edit().history()`.
    pub fn prefix_edit(&self) -> &FlowMetadataPrefixEdit {
        &self.prefix_edit
    }
}
