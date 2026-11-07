use super::*;
impl FlowFixedState {
    /// Borrows the immutable original carrier; e.g. `let raw = state.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = state.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the observed supported version; e.g. `let version = state.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows the complete raw payload; e.g. `let raw = state.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns admitted raw byte and row bounds; e.g. `let maximum = state.options().maximum_rows`.
    pub fn options(&self) -> FlowFixedStateOptions {
        self.options
    }
    /// Retains version-zero count absence separately from stored zero; e.g. `let count = state.declared_count()`.
    pub fn declared_count(&self) -> Option<u32> {
        self.count
    }
    /// Returns actual complete row count; e.g. `let count = state.row_count()`.
    pub fn row_count(&self) -> usize {
        self.count.map_or(0, |count| count as usize)
    }
    /// Borrows one existing physical row; e.g. `let first = state.row(0)`.
    pub fn row(&self, ordinal: usize) -> Option<FlowFixedStateRow<'_>> {
        (ordinal < self.row_count()).then(|| self.row_at(ordinal))
    }
    /// Enumerates actual rows without per-row allocation; e.g. `let count = state.rows().len()`.
    pub fn rows(&self) -> impl ExactSizeIterator<Item = FlowFixedStateRow<'_>> {
        (0..self.row_count()).map(|ordinal| self.row_at(ordinal))
    }
    fn row_at(&self, ordinal: usize) -> FlowFixedStateRow<'_> {
        let width = self.layout.stride();
        let start = 8 + ordinal * width;
        FlowFixedStateRow {
            owner: self,
            ordinal,
            interval: (start, start + width),
        }
    }
    /// Reports complete raw framing and stored scalar counts; e.g. `let coverage = state.coverage()`.
    pub fn coverage(&self) -> FlowFixedStateCoverage {
        let rows = self.row_count();
        let [wides, words, finals] = self.value_totals();
        FlowFixedStateCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            declared_count: self.count,
            rows,
            row_bytes: self.count.map(|_| self.layout.stride()),
            wide_patterns: wides,
            settings_words: words,
            final_members: finals,
            stored_values: 1 + usize::from(self.count.is_some()) + wides + words + finals,
            achieved_error: 0,
        }
    }
    fn value_totals(&self) -> [usize; 3] {
        let rows = self.row_count();
        let wides = rows
            * (2 + if self.layout.has_settings() {
                FlowFixedSettingsKind::Numeric.slot_count()
            } else {
                0
            });
        let words = if self.layout.has_settings() {
            rows * FlowFixedSettingsKind::Integer.slot_count()
        } else {
            0
        };
        let finals = if self.layout.has_final() { rows } else { 0 };
        [wides, words, finals]
    }
    /// Borrows native settings, engineering and application exclusions; e.g. `let exclusions = state.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowFixedStateDiagnostic] {
        use FlowFixedStateDiagnostic::*;
        &[
            NativeSettingsApplicationNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
}
impl FlowFixedStateWideMember {
    /// Returns the member's byte offset relative to its row; e.g. `let offset = FlowFixedStateWideMember::Discarded.relative_offset()`.
    pub fn relative_offset(self) -> usize {
        match self {
            Self::Member600 => 0,
            Self::Discarded => 8,
        }
    }
}
impl<'a> FlowFixedStateRow<'a> {
    /// Returns physical row order; e.g. `let ordinal = row.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete row interval; e.g. `let span = row.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.interval
    }
    /// Reads an exact raw leading member; e.g. `let first = row.wide_member(FlowFixedStateWideMember::Member600)`.
    pub fn wide_member(self, member: FlowFixedStateWideMember) -> u64 {
        wire::wide_at(
            &self.owner.wire.decoded,
            self.interval.0 + member.relative_offset(),
        )
    }
    /// Borrows 320 raw settings patterns or version-one absence; e.g. `let settings = row.settings_wides()`.
    pub fn settings_wides(self) -> Option<FlowTableWideArray<'a>> {
        let (start, end) = self.settings_interval(FlowFixedSettingsKind::Numeric)?;
        Some(FlowTableWideArray {
            bytes: &self.owner.wire.decoded[start..end],
            start,
        })
    }
    /// Borrows 100 raw settings words or version-one absence; e.g. `let settings = row.settings_words()`.
    pub fn settings_words(self) -> Option<FlowBlockWordArray<'a>> {
        let (start, end) = self.settings_interval(FlowFixedSettingsKind::Integer)?;
        Some(FlowBlockWordArray {
            bytes: &self.owner.wire.decoded[start..end],
            start,
        })
    }
    /// Reads the final signed member in versions three/four, retaining absence otherwise; e.g. `let final_value = row.final_member()`.
    pub fn final_member(self) -> Option<i32> {
        self.owner
            .layout
            .has_final()
            .then(|| wire::signed_at(&self.owner.wire.decoded, self.interval.1 - 4))
    }
    pub(super) fn settings_interval(self, kind: FlowFixedSettingsKind) -> Option<(usize, usize)> {
        if !self.owner.layout.has_settings() {
            return None;
        }
        let wide_bytes = FlowFixedSettingsKind::Numeric.slot_count() * 8;
        let (offset, length) = match kind {
            FlowFixedSettingsKind::Numeric => (16, wide_bytes),
            FlowFixedSettingsKind::Integer => (
                16 + wide_bytes,
                FlowFixedSettingsKind::Integer.slot_count() * 4,
            ),
        };
        let start = self.interval.0 + offset;
        Some((start, start + length))
    }
}
impl FlowFixedStateEdit {
    /// Borrows the validated original checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowFixedState {
        &self.source
    }
    /// Borrows the completely reopened result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowFixedState {
        &self.result
    }
    /// Borrows physical history and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
