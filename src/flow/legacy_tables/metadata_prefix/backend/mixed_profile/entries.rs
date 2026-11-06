use super::*;
impl FlowMediumBody {
    /// Returns the declared kind without inferring it from bytes; e.g. `let kind=body.kind()`.
    pub fn kind(&self) -> FlowMediumKind {
        match self {
            Self::KindOne(_) => FlowMediumKind::One,
            Self::KindTwo(_) => FlowMediumKind::Two,
        }
    }
    /// Returns the complete body interval in payload coordinates; e.g. `let span=body.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        match self {
            Self::KindOne(body) => body.payload_interval(),
            Self::KindTwo(body) => body.payload_interval(),
        }
    }
    pub(super) fn bytes(&self) -> &[u8] {
        match self {
            Self::KindOne(body) => body.body_bytes(),
            Self::KindTwo(body) => body.body_bytes(),
        }
    }
    pub(super) fn totals(&self) -> (usize, usize, usize) {
        match self {
            Self::KindOne(body) => {
                let coverage = body.coverage();
                (coverage.wide_words, 0, coverage.name_units)
            }
            Self::KindTwo(body) => {
                let coverage = body.coverage();
                (
                    coverage.wide_words,
                    coverage.word_patterns,
                    coverage.name_units,
                )
            }
        }
    }
}
impl FlowMediumMixedEntry {
    /// Returns the complete physical kind/key/body interval; e.g. `let span=entry.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        self.full
    }
    /// Returns the raw typed table key; e.g. `let key=entry.key()`.
    pub fn key(&self) -> FlowGuid {
        self.key
    }
    /// Borrows the complete nested typed body; e.g. `let body=entry.body()`.
    pub fn body(&self) -> &FlowMediumBody {
        &self.body
    }
}
impl FlowMediumMixedReference {
    /// Returns the complete physical reference interval; e.g. `let span=reference.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        self.full
    }
    /// Returns the selected first-table key domain; e.g. `let kind=reference.kind()`.
    pub fn kind(&self) -> FlowMediumKind {
        self.kind
    }
    /// Returns the second-table key; e.g. `let key=reference.key()`.
    pub fn key(&self) -> FlowGuid {
        self.key
    }
    /// Returns stored kind-one state, or absence for kind two; e.g. `let state=reference.state_byte()`.
    pub fn state_byte(&self) -> Option<u8> {
        self.state.map(|(_, value)| value)
    }
    /// Projects kind-one native loaded state without altering stored bytes; e.g. `let state=reference.native_state_after_load()`.
    pub fn native_state_after_load(&self) -> Option<u8> {
        self.state.map(|_| 1)
    }
    /// Borrows every ordered typed first-table reference; e.g. `let keys=reference.references()`.
    pub fn references(&self) -> &[FlowGuid] {
        &self.references
    }
    /// Returns the raw reference-key interval; e.g. `let span=reference.references_interval()`.
    pub fn references_interval(&self) -> (usize, usize) {
        (
            self.references_start,
            self.references_start + self.references.len() * 16,
        )
    }
}
impl FlowMediumMixedEdit {
    /// Borrows the unchanged input profile; e.g. `let before=edit.source()`.
    pub fn source(&self) -> &FlowMediumMixedProfile {
        &self.source
    }
    /// Borrows the completely revalidated result; e.g. `let after=edit.result()`.
    pub fn result(&self) -> &FlowMediumMixedProfile {
        &self.result
    }
    /// Borrows complete carrier checkpoints and physical history; e.g. `let history=edit.prefix_edit().carrier_edit().history()`.
    pub fn prefix_edit(&self) -> &FlowMetadataPrefixEdit {
        &self.prefix_edit
    }
}
