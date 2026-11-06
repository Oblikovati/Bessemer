mod checkpoints;
mod rows;
use super::*;

impl FlowParallelTable {
    /// Borrows a physical group, including absent arrays; e.g. `let first = table.group(0)`.
    pub fn group(&self, ordinal: usize) -> Option<FlowParallelGroup<'_>> {
        Some(FlowParallelGroup {
            owner: self,
            ordinal,
            span: self.groups.get(ordinal)?,
        })
    }
    /// Enumerates exactly two physical groups; e.g. `let count = table.groups().len()`.
    pub fn groups(&self) -> impl ExactSizeIterator<Item = FlowParallelGroup<'_>> {
        self.groups
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowParallelGroup {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Reports complete serialized counts, independently of declared absent rows; e.g. `let coverage = table.coverage()`.
    pub fn coverage(&self) -> FlowSimpleTableCoverage {
        coverage(&self.wire, 0, self.wide_words, 0, 0)
    }
    /// Borrows native and semantic exclusions; e.g. `let exclusions = table.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        diagnostics(false)
    }
}

impl<'a> FlowParallelGroup<'a> {
    /// Returns its physical ordinal, zero or one; e.g. `let ordinal = group.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete decoded group interval; e.g. `let span = group.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Returns the declared count even when arrays are absent; e.g. `let count = group.declared_count()`.
    pub fn declared_count(self) -> FlowSimpleLegacyCount {
        self.span.count
    }
    /// Returns the physically stored signed flag or absence in later versions; e.g. `let flag = group.presence_word()`.
    pub fn presence_word(self) -> Option<i32> {
        self.span
            .presence
            .map(|offset| wire::signed_at(&self.owner.wire.decoded, offset))
    }
    /// Distinguishes absent arrays from four present empty arrays; e.g. `let present = group.arrays_present()`.
    pub fn arrays_present(self) -> bool {
        self.span.arrays.is_some()
    }
    /// Borrows one existing raw wide array; e.g. `let first = group.array(0)`.
    pub fn array(self, ordinal: usize) -> Option<FlowTableWideArray<'a>> {
        let &(start, end) = self.span.arrays.as_ref()?.get(ordinal)?;
        Some(FlowTableWideArray {
            bytes: &self.owner.wire.decoded[start..end],
            start,
        })
    }
}

impl FlowLeafPairs {
    /// Borrows a physical row without assigning tree order; e.g. `let first = pairs.row(0)`.
    pub fn row(&self, ordinal: usize) -> Option<FlowLeafPairRow<'_>> {
        (ordinal < self.rows).then_some(FlowLeafPairRow {
            owner: self,
            ordinal,
        })
    }
    /// Enumerates every physical row without allocating per row; e.g. `let count = pairs.rows().len()`.
    pub fn rows(&self) -> impl ExactSizeIterator<Item = FlowLeafPairRow<'_>> {
        (0..self.rows).map(|ordinal| FlowLeafPairRow {
            owner: self,
            ordinal,
        })
    }
    /// Borrows every raw pair byte after the version; e.g. `let raw = pairs.raw_bytes()`.
    pub fn raw_bytes(&self) -> &[u8] {
        &self.wire.decoded[4..]
    }
    /// Reports exact wire rows and wide words; e.g. `let coverage = pairs.coverage()`.
    pub fn coverage(&self) -> FlowSimpleTableCoverage {
        coverage(&self.wire, self.rows, self.rows * 2, 0, 0)
    }
    /// Borrows explicit tree-order and native exclusions; e.g. `let exclusions = pairs.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        diagnostics(true)
    }
}

impl FlowFiveWordTable {
    /// Returns count knowledge, including inactive external counts and None; e.g. `let count = table.declared_count()`.
    pub fn declared_count(&self) -> Option<FlowSimpleLegacyCount> {
        self.count.map(|count| count.value)
    }
    /// Distinguishes unspecified, external and stored counts; e.g. `let origin = table.count_origin()`.
    pub fn count_origin(&self) -> FlowSimpleCountOrigin {
        origin(self.count)
    }
    /// Returns the actual serialized storage branch; e.g. `let storage = table.storage()`.
    pub fn storage(&self) -> FlowFiveWordStorage {
        match self.wire.version {
            0..=2 => FlowFiveWordStorage::NoSerializedRows,
            3..=4 => FlowFiveWordStorage::RawRows,
            _ => FlowFiveWordStorage::InactiveRestored,
        }
    }
    /// Returns the exact row-array offsets, including empty layouts; e.g. `let span = table.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (self.start, self.wire.decoded.len())
    }
    /// Borrows every actual row byte; inactive contexts remain empty, e.g. `let raw = table.raw_bytes()`.
    pub fn raw_bytes(&self) -> &[u8] {
        &self.wire.decoded[self.start..]
    }
    /// Borrows a physically stored row only; e.g. `let first = table.row(0)`.
    pub fn row(&self, ordinal: usize) -> Option<FlowFiveWordRow<'_>> {
        (ordinal < self.rows).then_some(FlowFiveWordRow {
            owner: self,
            ordinal,
        })
    }
    /// Enumerates actual wire rows, independently of declared context; e.g. `let count = table.rows().len()`.
    pub fn rows(&self) -> impl ExactSizeIterator<Item = FlowFiveWordRow<'_>> {
        (0..self.rows).map(|ordinal| FlowFiveWordRow {
            owner: self,
            ordinal,
        })
    }
    /// Reports actual serialized rows and words; e.g. `let coverage = table.coverage()`.
    pub fn coverage(&self) -> FlowSimpleTableCoverage {
        coverage(&self.wire, self.rows, 0, self.rows * 5, 0)
    }
    /// Borrows native and semantic exclusions; e.g. `let exclusions = table.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        diagnostics(false)
    }
}

impl FlowRawByteTable {
    /// Returns the mandatory admitted stored/external count; e.g. `let count = table.declared_count()`.
    pub fn declared_count(&self) -> FlowSimpleLegacyCount {
        self.count.value
    }
    /// Returns whether count knowledge was external or stored; e.g. `let origin = table.count_origin()`.
    pub fn count_origin(&self) -> FlowSimpleCountOrigin {
        origin(Some(self.count))
    }
    /// Returns exact raw-array offsets, including inactive empty arrays; e.g. `let span = table.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (self.start, self.wire.decoded.len())
    }
    /// Borrows every unmodified byte, without text conversion; e.g. `let raw = table.raw_bytes()`.
    pub fn raw_bytes(&self) -> &[u8] {
        &self.wire.decoded[self.start..]
    }
    /// Reads a checked existing byte; e.g. `let first = table.byte(0)`.
    pub fn byte(&self, index: usize) -> Option<u8> {
        self.raw_bytes().get(index).copied()
    }
    /// Reports actual raw bytes independently of count origin; e.g. `let coverage = table.coverage()`.
    pub fn coverage(&self) -> FlowSimpleTableCoverage {
        coverage(&self.wire, 0, 0, 0, self.raw_bytes().len())
    }
    /// Borrows native and semantic exclusions; e.g. `let exclusions = table.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSimpleTableDiagnostic] {
        diagnostics(false)
    }
}

fn origin(count: Option<SimpleDeclaredCount>) -> FlowSimpleCountOrigin {
    match count {
        None => FlowSimpleCountOrigin::Unspecified,
        Some(count) if count.stored => FlowSimpleCountOrigin::Stored,
        Some(_) => FlowSimpleCountOrigin::External,
    }
}

fn coverage(
    wire: &TableWire,
    rows: usize,
    wide: usize,
    words: usize,
    bytes: usize,
) -> FlowSimpleTableCoverage {
    FlowSimpleTableCoverage {
        mode: wire::mode(wire),
        stored_bytes: wire.stored_bytes,
        decoded_bytes: wire.decoded.len(),
        wire_rows: rows,
        wide_words: wide,
        raw_words: words,
        raw_bytes: bytes,
        achieved_error: 0,
    }
}

fn diagnostics(leaf_pairs: bool) -> &'static [FlowSimpleTableDiagnostic] {
    use FlowSimpleTableDiagnostic::*;
    if leaf_pairs {
        return &[
            EngineeringRolesNotResolved,
            NativeRuntimeNotReproduced,
            OwnerCompressionContextNotReconstructed,
            TreeOrderNotAssigned,
            NativeApplicationNotChecked,
        ];
    }
    &[
        EngineeringRolesNotResolved,
        NativeRuntimeNotReproduced,
        OwnerCompressionContextNotReconstructed,
        NativeApplicationNotChecked,
    ]
}
