use super::*;

impl FlowBaseGrid {
    /// Borrows the complete original checkpoint; e.g. `let wire = grid.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the revision-bound physical occurrence; e.g. `let ordinal = grid.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Borrows every decoded byte; e.g. `let raw = grid.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.decoded
    }
    /// Returns the original nonnegative signed dimensions; e.g. `let sizes = grid.dimensions()`.
    pub fn dimensions(&self) -> [i32; 3] {
        self.dimensions
    }
    /// Returns default index ranges, including empty zero dimensions; e.g. `let bounds = grid.default_tree_bounds()`.
    pub fn default_tree_bounds(&self) -> FlowTreeBounds {
        FlowTreeBounds::from_pairs(self.dimensions.map(|n| [0, n - 1]))
    }
    /// Returns the exact mathematical dimension product; e.g. `let roots = grid.root_grid_count()`.
    pub fn root_grid_count(&self) -> u128 {
        self.dimensions.into_iter().map(|n| n as u128).product()
    }
    /// Borrows a stored array without naming its physical axis; e.g. `let first = grid.boundary(0)`.
    pub fn boundary(&self, index: usize) -> Option<FlowGridBoundary<'_>> {
        let &(start, end) = self.spans.get(index)?;
        Some(FlowGridBoundary {
            bytes: &self.decoded[start..end],
            start,
        })
    }
    /// Returns the exact options; e.g. `let limit = grid.options().maximum_boundary_words`.
    pub fn options(&self) -> FlowGridOptions {
        self.options
    }
    /// Returns complete serialized coverage; e.g. `let checked = grid.coverage()`.
    pub fn coverage(&self) -> FlowGridCoverage {
        self.coverage
    }
    /// Borrows explicit acceptance exclusions; e.g. `let exclusions = grid.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowGridDiagnostic] {
        &[
            FlowGridDiagnostic::GeometryNotValidated,
            FlowGridDiagnostic::NativeRefinementNotReproduced,
            FlowGridDiagnostic::NativeObjectIndexingNotValidated,
            FlowGridDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowGridDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl<'a> FlowGridBoundary<'a> {
    /// Borrows all raw boundary bytes; e.g. `let raw = boundary.as_bytes()`.
    pub fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }
    /// Returns decoded start and exclusive end; e.g. `let span = boundary.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.start, self.start + self.bytes.len())
    }
    /// Returns the dimension plus one count; e.g. `let count = boundary.word_count()`.
    pub fn word_count(self) -> usize {
        self.bytes.len() / 8
    }
    /// Returns one complete raw word, retaining NaN payloads; e.g. `let bits = boundary.raw_bits(0)`.
    pub fn raw_bits(self, index: usize) -> Option<u64> {
        let start = index.checked_mul(8)?;
        let end = start.checked_add(8)?;
        let bytes: [u8; 8] = self.bytes.get(start..end)?.try_into().ok()?;
        Some(u64::from_le_bytes(bytes))
    }
    /// Returns an unnormalized binary64 view; e.g. `let value = boundary.value(0)`.
    pub fn value(self, index: usize) -> Option<f64> {
        self.raw_bits(index).map(f64::from_bits)
    }
}

impl FlowGridEdit {
    /// Borrows the immutable source grid; e.g. `let original = edit.source()`.
    pub fn source(&self) -> &FlowBaseGrid {
        &self.source
    }
    /// Borrows the fully reopened result; e.g. `let updated = edit.result()`.
    pub fn result(&self) -> &FlowBaseGrid {
        &self.result
    }
    /// Borrows complete carrier history and checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
