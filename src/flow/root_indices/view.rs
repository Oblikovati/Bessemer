use super::*;

impl FlowRootStorageIndex {
    /// Returns the validated native storage index; e.g. `let index = root.storage_index().value()`.
    pub fn value(self) -> i32 {
        self.0
    }
    /// Returns the native 56-byte root-storage offset; e.g. `let offset = index.byte_offset()`.
    pub fn byte_offset(self) -> u64 {
        self.0 as u64 * 56
    }
}

impl FlowRootIndexProjection {
    /// Borrows the explicit fresh-context derivation and both checkpoints; e.g. `let context = projection.source_context()`.
    pub fn source_context(&self) -> &FlowDerivedLeafCount {
        &self.context
    }
    /// Returns exact projection limits; e.g. `let cap = projection.limits().maximum_path_bytes`.
    pub fn limits(&self) -> FlowRootIndexLimits {
        self.limits
    }
    /// Returns complete exact projection coverage; e.g. `let checked = projection.coverage()`.
    pub fn coverage(&self) -> FlowRootIndexCoverage {
        self.coverage
    }
    /// Borrows native/runtime exclusions; e.g. `let exclusions = projection.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowRootIndexDiagnostic] {
        &[
            FlowRootIndexDiagnostic::NativeContextNotExecuted,
            FlowRootIndexDiagnostic::GlobalBlockOrdinalsNotAssigned,
            FlowRootIndexDiagnostic::ParallelScanOffsetsNotValidated,
            FlowRootIndexDiagnostic::NativeApplicationNotChecked,
        ]
    }
    /// Borrows one projected root by wire position; e.g. `let first = projection.root(0)`.
    pub fn root(&self, index: usize) -> Option<FlowProjectedRoot<'_>> {
        Some(FlowProjectedRoot {
            owner: self,
            span: self.roots.get(index)?,
        })
    }
    /// Enumerates all projected roots in wire order; e.g. `let count = projection.roots().len()`.
    pub fn roots(&self) -> impl ExactSizeIterator<Item = FlowProjectedRoot<'_>> {
        self.roots
            .iter()
            .map(|span| FlowProjectedRoot { owner: self, span })
    }
}

impl<'a> FlowProjectedRoot<'a> {
    /// Returns original root coordinates without inferred engineering axes; e.g. `let indices = root.indices()`.
    pub fn indices(self) -> [i32; 3] {
        self.span.indices
    }
    /// Returns the typed native root-storage index; e.g. `let offset = root.storage_index().byte_offset()`.
    pub fn storage_index(self) -> FlowRootStorageIndex {
        self.span.storage_index
    }
    /// Enumerates local leaves in depth-first child order; e.g. `let count = root.leaves().len()`.
    pub fn leaves(self) -> impl ExactSizeIterator<Item = FlowProjectedLeaf<'a>> {
        self.owner.leaves[self.span.first_leaf..self.span.leaf_end]
            .iter()
            .enumerate()
            .map(move |(ordinal, span)| FlowProjectedLeaf {
                owner: self.owner,
                ordinal,
                span,
            })
    }
}

impl<'a> FlowProjectedLeaf<'a> {
    /// Returns the ordinal within this root only; e.g. `let local = leaf.local_leaf_ordinal()`.
    pub fn local_leaf_ordinal(self) -> usize {
        self.ordinal
    }
    /// Borrows every child slot, without modulo-depth wrapping; e.g. `let path = leaf.child_path()`.
    pub fn child_path(self) -> &'a [u8] {
        &self.owner.paths[self.span.path_start..self.span.path_end]
    }
    /// Returns the original decoded node interval; e.g. `let interval = leaf.wire_interval()`.
    pub fn wire_interval(self) -> (usize, usize) {
        self.owner.context.trees().nodes()[self.span.node].payload_interval()
    }
}
