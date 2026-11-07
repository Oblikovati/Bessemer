use super::*;
impl FlowMediumKindTwo {
    pub(super) fn raw(&self) -> &[u8] {
        self.backend.metadata_prefix().payload_bytes()
    }
    /// Borrows the immutable backend checkpoint; e.g. `let backend=body.backend_prefix()`.
    pub fn backend_prefix(&self) -> &FlowMediumBackendPrefix {
        &self.backend
    }
    /// Returns the independent parsing limits; e.g. `let options=body.options()`.
    pub fn options(&self) -> FlowMediumKindTwoOptions {
        self.options
    }
    /// Returns the complete body interval in payload coordinates; e.g. `let span=body.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Borrows every checked body byte; e.g. `let raw=body.body_bytes()`.
    pub fn body_bytes(&self) -> &[u8] {
        &self.raw()[self.span.start..self.span.end]
    }
    /// Borrows every following undecoded byte; e.g. `let tail=body.opaque_tail()`.
    pub fn opaque_tail(&self) -> &[u8] {
        &self.raw()[self.span.end..]
    }
    /// Returns the uninterpreted base byte; e.g. `let base=body.base_byte()`.
    pub fn base_byte(&self) -> u8 {
        self.raw()[self.span.start + 1]
    }
    /// Borrows exactly sixteen body base-key bytes, independently of the table key; e.g. `let key=body.base_key_bytes()`.
    pub fn base_key_bytes(&self) -> &[u8] {
        &self.raw()[self.span.start + 2..self.span.start + 18]
    }
    /// Borrows the lossless stored name and first-NUL text; e.g. `let name=body.name()`.
    pub fn name(&self) -> FlowMetadataText<'_> {
        FlowMetadataText {
            owner: self.backend.metadata_prefix(),
            span: &self.span.prefix.name,
        }
    }
    /// Returns the normalized signed model selector; e.g. `let kind=body.model_kind()`.
    pub fn model_kind(&self) -> FlowMediumModelKind {
        self.span.model.kind
    }
    /// Returns the complete version/model interval relative to the body; e.g. `let span=body.model_interval()`.
    pub fn model_interval(&self) -> (usize, usize) {
        (
            self.span.model.start - self.span.start,
            self.span.model.end - self.span.start,
        )
    }
    /// Enumerates common or model curves, including completed empty model collections; e.g. `let curves=body.curves(FlowMediumCurveScope::Model)`.
    pub fn curves(
        &self,
        scope: FlowMediumCurveScope,
    ) -> impl ExactSizeIterator<Item = FlowMediumCurve<'_>> {
        self.curve_spans(scope)
            .iter()
            .enumerate()
            .map(|(ordinal, span)| {
                FlowMediumCurve::from_span(self.raw(), span, self.span.start, ordinal)
            })
    }
    /// Borrows an existing curve in its explicit namespace; e.g. `let curve=body.curve(FlowMediumCurveScope::Body,0)`.
    pub fn curve(
        &self,
        scope: FlowMediumCurveScope,
        ordinal: usize,
    ) -> Option<FlowMediumCurve<'_>> {
        self.curve_spans(scope)
            .get(ordinal)
            .map(|span| FlowMediumCurve::from_span(self.raw(), span, self.span.start, ordinal))
    }
    /// Borrows every stored model state byte; e.g. `let states=body.model_state_bytes()`.
    pub fn model_state_bytes(&self) -> &[u8] {
        let (start, end) = self.model_states_span();
        &self.raw()[start..end]
    }
    /// Returns model-five counts independently, including zero; e.g. `let counts=body.model_counts()`.
    pub fn model_counts(&self) -> Option<[usize; 3]> {
        match &self.span.model.payload {
            ModelPayload::Tables(table) => Some(table.counts),
            _ => None,
        }
    }
    /// Borrows model-five raw 32-bit words, distinguishing absent from present empty; e.g. `let words=body.model_word_patterns()`.
    pub fn model_word_patterns(&self) -> Option<FlowBlockWordArray<'_>> {
        let ModelPayload::Tables(table) = &self.span.model.payload else {
            return None;
        };
        let (start, end) = table.words;
        Some(FlowBlockWordArray {
            bytes: &self.raw()[start..end],
            start,
        })
    }
    /// Returns the complete signed model-five member word; e.g. `let member=body.model_member_8c()`.
    pub fn model_member_8c(&self) -> Option<i32> {
        match &self.span.model.payload {
            ModelPayload::Tables(table) => Some(wire::signed_at(self.raw(), table.member)),
            _ => None,
        }
    }
    /// Returns every stored final state bit; e.g. `let state=body.state_byte()`.
    pub fn state_byte(&self) -> u8 {
        self.raw()[self.span.state]
    }
    /// Projects the documented native loaded value without changing stored bytes; e.g. `let loaded=body.native_state_after_load()`.
    pub fn native_state_after_load(&self) -> u8 {
        1
    }
    /// Reports exact body scope and explicit installed/full-record exclusions; e.g. `let coverage=body.coverage()`.
    pub fn coverage(&self) -> FlowMediumKindTwoCoverage {
        FlowMediumKindTwoCoverage {
            body_bytes: self.span.end - self.span.start,
            wide_words: self.span.words,
            word_patterns: self
                .model_word_patterns()
                .map_or(0, |words| words.word_count()),
            name_units: self.name().stored_units(),
            model_kind: self.model_kind(),
            body_framing_complete: true,
            full_record_schema: false,
            installed_fixture_validated: false,
            achieved_error: 0,
        }
    }
    /// Borrows installed, engineering, runtime and following-record exclusions; e.g. `let exclusions=body.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowMediumKindTwoDiagnostic] {
        &[
            FlowMediumKindTwoDiagnostic::InstalledFixtureNotValidated,
            FlowMediumKindTwoDiagnostic::EngineeringAndEvaluationNotChecked,
            FlowMediumKindTwoDiagnostic::NativeRuntimeNotReproduced,
            FlowMediumKindTwoDiagnostic::FollowingRecordNotDecoded,
            FlowMediumKindTwoDiagnostic::NativeApplicationNotChecked,
        ]
    }
}
impl FlowMediumKindTwoEdit {
    /// Borrows the unchanged typed source; e.g. `let before=edit.source()`.
    pub fn source(&self) -> &FlowMediumKindTwo {
        &self.source
    }
    /// Borrows the completely reparsed typed result; e.g. `let after=edit.result()`.
    pub fn result(&self) -> &FlowMediumKindTwo {
        &self.result
    }
    /// Borrows every carrier checkpoint and physical history relation; e.g. `let history=edit.prefix_edit().carrier_edit().history()`.
    pub fn prefix_edit(&self) -> &FlowMetadataPrefixEdit {
        &self.prefix_edit
    }
}
