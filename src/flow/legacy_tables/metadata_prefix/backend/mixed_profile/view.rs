use super::*;
impl FlowMediumMixedProfile {
    /// Borrows the immutable complete carrier; e.g. `let wire=profile.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        self.metadata_prefix().source()
    }
    /// Borrows the revision-bound physical occurrence; e.g. `let ordinal=profile.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        self.metadata_prefix().record()
    }
    /// Borrows complete payload bytes through exact EOF; e.g. `let raw=profile.payload_bytes()`.
    pub fn payload_bytes(&self) -> &[u8] {
        self.metadata_prefix().payload_bytes()
    }
    /// Borrows the decoded metadata checkpoint; e.g. `let prefix=profile.metadata_prefix()`.
    pub fn metadata_prefix(&self) -> &FlowMetadataPrefix {
        self.backend.metadata_prefix()
    }
    /// Returns explicit context and independent limits; e.g. `let limits=profile.options()`.
    pub fn options(&self) -> FlowMediumMixedOptions {
        self.options
    }
    /// Returns the complete backend interval before outer references; e.g. `let span=profile.backend_interval()`.
    pub fn backend_interval(&self) -> (usize, usize) {
        (
            self.metadata_prefix().coverage().known_prefix_bytes,
            self.backend_end,
        )
    }
    /// Borrows all ordered physical first entries, including duplicates; e.g. `let count=profile.first_objects().len()`.
    pub fn first_objects(&self) -> &[FlowMediumMixedEntry] {
        &self.first
    }
    /// Borrows all ordered physical reference entries; e.g. `let count=profile.reference_objects().len()`.
    pub fn reference_objects(&self) -> &[FlowMediumMixedReference] {
        &self.second
    }
    /// Borrows ordered outer second-table keys; e.g. `let keys=profile.outer_references()`.
    pub fn outer_references(&self) -> &[FlowGuid] {
        &self.outer.keys
    }
    /// Borrows signed attributes in outer-reference order; e.g. `let words=profile.outer_attributes()`.
    pub fn outer_attributes(&self) -> &[i32] {
        &self.outer.attributes
    }
    /// Returns the outer reference-key interval; e.g. `let span=profile.outer_references_interval()`.
    pub fn outer_references_interval(&self) -> (usize, usize) {
        (
            self.outer.start,
            self.outer.start + self.outer.keys.len() * 16,
        )
    }
    /// Returns the raw signed-attribute interval; e.g. `let span=profile.outer_attributes_interval()`.
    pub fn outer_attributes_interval(&self) -> (usize, usize) {
        (
            self.outer.attributes_start,
            self.outer.attributes_start + self.outer.attributes.len() * 4,
        )
    }
    /// Reports exact mixed framing separately from engineering and native acceptance; e.g. `let coverage=profile.coverage()`.
    pub fn coverage(&self) -> FlowMediumMixedCoverage {
        FlowMediumMixedCoverage {
            mode: FlowMediumMixedCheckMode::ExactFreshMixedRecord,
            payload_bytes: self.payload_bytes().len(),
            metadata_bytes: self.backend_interval().0,
            backend_bytes: self.backend_end - self.backend_interval().0,
            first_objects: self.first.len(),
            reference_objects: self.second.len(),
            references: self.references,
            wide_words: self.words,
            word_patterns: self.word_patterns,
            name_units: self.name_units,
            fresh_backend_assumed: true,
            complete_record_framing: true,
            full_engineering_schema: false,
            installed_kind_two_fixture_validated: false,
            achieved_error: 0,
        }
    }
    /// Borrows the explicit acceptance exclusions; e.g. `let exclusions=profile.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowMediumMixedDiagnostic] {
        &[
            FlowMediumMixedDiagnostic::InstalledKindTwoFixtureNotValidated,
            FlowMediumMixedDiagnostic::EngineeringAndEvaluationNotChecked,
            FlowMediumMixedDiagnostic::NativeRuntimeNotReproduced,
            FlowMediumMixedDiagnostic::NativeApplicationNotChecked,
        ]
    }
}
