use super::*;
impl FlowSelectionContext {
    /// Borrows the retained link checkpoint; e.g. `let id = context.links().record()`.
    pub fn links(&self) -> &FlowLinkRecords {
        &self.links
    }
    /// Borrows the retained state checkpoint; e.g. `let raw = context.states().raw_bytes()`.
    pub fn states(&self) -> &FlowByteStates {
        &self.states
    }
    /// Returns the declared assumption and bounds; e.g. `let policy = context.options()`.
    pub fn options(&self) -> FlowSelectionContextOptions {
        self.options
    }
    /// Returns the physical link/map count; e.g. `let count = context.link_count()`.
    pub fn link_count(&self) -> usize {
        self.dense_indices.len()
    }
    /// Returns the derived wide-value count; e.g. `let count = context.selected_count()`.
    pub fn selected_count(&self) -> usize {
        self.ordinals.len()
    }
    /// Borrows selected physical ordinals in total order; e.g. `let selected = context.selected_ordinals()`.
    pub fn selected_ordinals(&self) -> &[usize] {
        &self.ordinals
    }
    /// Resolves an input entry to a dense value or absence; e.g. `let index = context.dense_index(0)?`.
    pub fn dense_index(&self, ordinal: usize) -> Result<Option<usize>> {
        self.dense_indices
            .get(ordinal)
            .copied()
            .ok_or_else(|| table_index_error(112, "input link", ordinal, self.link_count()))
    }
    /// Reports exact supplied-input/fresh-map coverage; e.g. `let proof = context.coverage()`.
    pub fn coverage(&self) -> FlowSelectionContextCoverage {
        FlowSelectionContextCoverage {
            mode: FlowSelectionCheckMode::ExactFreshMapFromSerializedInputs,
            link_mode: self.links.coverage().mode,
            state_mode: self.states.coverage().mode,
            links: self.link_count(),
            state_bytes: self.states.stored_count(),
            selected_values: self.selected_count(),
            absent_entries: self.link_count() - self.selected_count(),
            assumption: self.options.assumption,
            achieved_error: 0,
        }
    }
    /// Borrows required native and application qualifications; e.g. `let exclusions = context.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSelectionDiagnostic] {
        diagnostics()
    }
}
