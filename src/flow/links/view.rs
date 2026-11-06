use super::*;

impl FlowLinkCandidate {
    /// Retains literal caller context without filesystem access; e.g. `let candidate = FlowLinkCandidate::new(kind, "1.cpt", carrier)`.
    pub fn new(kind: FlowResultType, path: impl Into<Arc<str>>, carrier: FlowCarrier) -> Self {
        Self {
            kind,
            path: path.into(),
            carrier,
        }
    }
    /// Returns the explicit classification; e.g. `let kind = candidate.result_type()`.
    pub fn result_type(&self) -> FlowResultType {
        self.kind
    }
    /// Borrows the unchanged path, including empty paths; e.g. `let path = candidate.path()`.
    pub fn path(&self) -> &str {
        &self.path
    }
    /// Borrows the complete source checkpoint; e.g. `let wire = candidate.carrier().as_bytes()`.
    pub fn carrier(&self) -> &FlowCarrier {
        &self.carrier
    }
}

impl FlowGuidLookup {
    /// Borrows every supplied candidate in original order; e.g. `let inputs = lookup.candidates()`.
    pub fn candidates(&self) -> &[FlowLinkCandidate] {
        &self.candidates
    }
    /// Returns the requested type; e.g. `let kind = lookup.result_type()`.
    pub fn result_type(&self) -> FlowResultType {
        self.kind
    }
    /// Returns all sixteen query bytes; e.g. `let query = lookup.query()`.
    pub fn query(&self) -> FlowGuid {
        self.query
    }
    /// Returns the selected input position, or completed absence; e.g. `let index = lookup.selected_index()`.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }
    /// Borrows the selected immutable candidate; e.g. `let path = lookup.selected().map(FlowLinkCandidate::path)`.
    pub fn selected(&self) -> Option<&FlowLinkCandidate> {
        self.selected.map(|index| &self.candidates[index])
    }
    /// Reports exact visited-prefix coverage; e.g. `let checked = lookup.coverage()`.
    pub fn coverage(&self) -> FlowGuidLookupCoverage {
        FlowGuidLookupCoverage {
            mode: FlowGuidLookupCheckMode::ExactOrderedCandidatePrefix,
            supplied_candidates: self.candidates.len(),
            visited_candidates: self.visited,
            checked_eligible_candidates: self.eligible,
            achieved_byte_error: 0,
        }
    }
    /// Borrows execution exclusions; e.g. `let exclusions = lookup.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowGuidLookupDiagnostic] {
        &[
            FlowGuidLookupDiagnostic::FilesystemAndPathValidationNotPerformed,
            FlowGuidLookupDiagnostic::ReusedResultInfoNotModeled,
            FlowGuidLookupDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowRelatedCarriers {
    /// Borrows the seed checkpoint; e.g. `let wire = related.seed().as_bytes()`.
    pub fn seed(&self) -> &FlowCarrier {
        &self.seed
    }
    /// Borrows the complete first lookup; e.g. `let cpt = related.cpt_lookup().selected()`.
    pub fn cpt_lookup(&self) -> &FlowGuidLookup {
        &self.cpt
    }
    /// Borrows the optional second lookup, distinct from an executed empty result; e.g. `let attempted = related.settings_lookup().is_some()`.
    pub fn settings_lookup(&self) -> Option<&FlowGuidLookup> {
        self.settings.as_ref()
    }
    /// Returns the exact settings-stage gate; e.g. `let stage = related.settings_stage()`.
    pub fn settings_stage(&self) -> FlowRelatedSettingsStage {
        self.stage
    }
}
