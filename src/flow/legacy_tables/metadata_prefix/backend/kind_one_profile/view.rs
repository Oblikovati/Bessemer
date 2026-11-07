use super::*;
impl FlowMediumKindOneProfile {
    /// Borrows the immutable complete carrier; e.g. `let raw=profile.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        self.backend.metadata_prefix().source()
    }
    /// Borrows the revision-bound occurrence; e.g. `let ordinal=profile.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        self.backend.metadata_prefix().record()
    }
    /// Borrows the complete raw payload through exact EOF; e.g. `let raw=profile.payload_bytes()`.
    pub fn payload_bytes(&self) -> &[u8] {
        self.backend.metadata_prefix().payload_bytes()
    }
    /// Borrows the independently decoded metadata prefix; e.g. `let metadata=profile.metadata_prefix()`.
    pub fn metadata_prefix(&self) -> &FlowMetadataPrefix {
        self.backend.metadata_prefix()
    }
    /// Returns the explicit context and resource bounds; e.g. `let options=profile.options()`.
    pub fn options(&self) -> FlowMediumProfileOptions {
        self.options
    }
    /// Returns the complete backend interval before outer references; e.g. `let span=profile.backend_interval()`.
    pub fn backend_interval(&self) -> (usize, usize) {
        (
            self.metadata_prefix().coverage().known_prefix_bytes,
            self.backend_end,
        )
    }
    /// Enumerates every first-table occurrence, including duplicate keys; e.g. `let count=profile.first_objects().len()`.
    pub fn first_objects(&self) -> impl ExactSizeIterator<Item = FlowMediumProfileObject<'_>> {
        (0..self.first.len()).map(|ordinal| FlowMediumProfileObject {
            owner: self,
            ordinal,
        })
    }
    /// Borrows one physical first-table occurrence; e.g. `let first=profile.first_object(0)`.
    pub fn first_object(&self, ordinal: usize) -> Option<FlowMediumProfileObject<'_>> {
        (ordinal < self.first.len()).then_some(FlowMediumProfileObject {
            owner: self,
            ordinal,
        })
    }
    /// Enumerates every second-table occurrence; e.g. `let count=profile.reference_objects().len()`.
    pub fn reference_objects(
        &self,
    ) -> impl ExactSizeIterator<Item = FlowMediumProfileReference<'_>> {
        (0..self.second.len()).map(|ordinal| FlowMediumProfileReference {
            owner: self,
            ordinal,
        })
    }
    /// Borrows one second-table occurrence; e.g. `let reference=profile.reference_object(0)`.
    pub fn reference_object(&self, ordinal: usize) -> Option<FlowMediumProfileReference<'_>> {
        (ordinal < self.second.len()).then_some(FlowMediumProfileReference {
            owner: self,
            ordinal,
        })
    }
    /// Borrows ordered outer keys without deduplication; e.g. `let keys=profile.outer_references()`.
    pub fn outer_references(&self) -> &[FlowGuid] {
        &self.outer.keys
    }
    /// Borrows signed outer attributes in the same order; e.g. `let attributes=profile.outer_attributes()`.
    pub fn outer_attributes(&self) -> &[i32] {
        &self.outer.attributes
    }
    /// Returns the raw outer-key interval in payload coordinates; e.g. `let span=profile.outer_references_interval()`.
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
    /// Reports exact supported record framing under fresh-state assumptions; e.g. `let coverage=profile.coverage()`.
    pub fn coverage(&self) -> FlowMediumProfileCoverage {
        FlowMediumProfileCoverage {
            mode: FlowMediumProfileCheckMode::ExactFreshKindOneRecord,
            payload_bytes: self.payload_bytes().len(),
            metadata_bytes: self.backend_interval().0,
            backend_bytes: self.backend_end - self.backend_interval().0,
            first_objects: self.first.len(),
            reference_objects: self.second.len(),
            references: self.references,
            wide_words: self.words,
            name_units: self.name_units,
            fresh_backend_assumed: true,
            complete_record_framing: true,
            full_engineering_schema: false,
            achieved_error: 0,
        }
    }
    /// Borrows engineering/runtime/application exclusions; e.g. `let exclusions=profile.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowMediumProfileDiagnostic] {
        &[
            FlowMediumProfileDiagnostic::NativeRuntimeNotReproduced,
            FlowMediumProfileDiagnostic::EngineeringAndEvaluationNotChecked,
            FlowMediumProfileDiagnostic::NativeApplicationNotChecked,
        ]
    }
}
impl<'a> FlowMediumProfileObject<'a> {
    /// Returns the physical ordinal independently of duplicate keys; e.g. `let ordinal=object.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete kind/key/body interval; e.g. `let span=object.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.owner.first[self.ordinal].full
    }
    /// Returns the exact typed binary table key; e.g. `let key=object.key()`.
    pub fn key(self) -> FlowGuid {
        self.owner.first[self.ordinal].key
    }
    /// Borrows the complete nested body; e.g. `let body=object.body()`.
    pub fn body(self) -> &'a FlowMediumKindOne {
        &self.owner.first[self.ordinal].body
    }
}
impl<'a> FlowMediumProfileReference<'a> {
    /// Returns the physical second-table ordinal; e.g. `let ordinal=reference.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete kind/reference-body interval; e.g. `let span=reference.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.owner.second[self.ordinal].full
    }
    /// Returns the exact second-table key; e.g. `let key=reference.key()`.
    pub fn key(self) -> FlowGuid {
        self.owner.second[self.ordinal].key
    }
    /// Returns every stored state bit; e.g. `let state=reference.state_byte()`.
    pub fn state_byte(self) -> u8 {
        self.owner.payload_bytes()[self.owner.second[self.ordinal].state]
    }
    /// Projects native load-time replacement without mutating stored bytes; e.g. `let state=reference.native_state_after_load()`.
    pub fn native_state_after_load(self) -> u8 {
        1
    }
    /// Borrows all ordered typed first-table references; e.g. `let keys=reference.references()`.
    pub fn references(self) -> &'a [FlowGuid] {
        &self.owner.second[self.ordinal].references
    }
    /// Returns the exact stored-reference interval; e.g. `let span=reference.references_interval()`.
    pub fn references_interval(self) -> (usize, usize) {
        let span = &self.owner.second[self.ordinal];
        (
            span.references_start,
            span.references_start + span.references.len() * 16,
        )
    }
}
impl FlowMediumProfileEdit {
    /// Borrows the unchanged complete input profile; e.g. `let before=edit.source()`.
    pub fn source(&self) -> &FlowMediumKindOneProfile {
        &self.source
    }
    /// Borrows the completely reparsed result; e.g. `let after=edit.result()`.
    pub fn result(&self) -> &FlowMediumKindOneProfile {
        &self.result
    }
    /// Borrows all carrier checkpoints and physical history; e.g. `let history=edit.prefix_edit().carrier_edit().history()`.
    pub fn prefix_edit(&self) -> &FlowMetadataPrefixEdit {
        &self.prefix_edit
    }
}
