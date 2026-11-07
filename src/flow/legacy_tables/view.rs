mod checkpoints;
use super::*;

impl FlowPointerTables {
    /// Borrows one exact group, preserving empty headers; e.g. `let first = table.group(0)`.
    pub fn group(&self, index: usize) -> Option<FlowPointerTableGroup<'_>> {
        Some(FlowPointerTableGroup {
            owner: self,
            ordinal: index,
            span: self.groups.get(index)?,
        })
    }
    /// Enumerates both groups in physical order; e.g. `let count = table.groups().len()`.
    pub fn groups(&self) -> impl ExactSizeIterator<Item = FlowPointerTableGroup<'_>> {
        self.groups
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowPointerTableGroup {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Reports complete serialized coverage and range context separately; e.g. `let checked = table.coverage()`.
    pub fn coverage(&self) -> FlowLegacyTableCoverage {
        coverage(
            &self.wire,
            self.pair_count,
            self.wide_count,
            0,
            self.options.global_pointer_count,
            None,
        )
    }
    /// Borrows native-pointer and application exclusions; e.g. `let exclusions = table.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowLegacyTableDiagnostic] {
        &[
            FlowLegacyTableDiagnostic::NativePointerPopulationNotReproduced,
            FlowLegacyTableDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowLegacyTableDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl<'a> FlowPointerTableGroup<'a> {
    /// Returns physical group zero or one; e.g. `let ordinal = group.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns exact decoded group offsets; e.g. `let span = group.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Returns the physically stored pair count; e.g. `let pairs = group.count()`.
    pub fn count(self) -> usize {
        self.span.count
    }
    /// Returns the stored version-one count discard or its absence; e.g. `let raw = group.discarded_count_word()`.
    pub fn discarded_count_word(self) -> Option<i32> {
        self.span
            .discarded_count
            .map(|offset| wire::signed_at(&self.owner.wire.decoded, offset))
    }
    /// Borrows populated fields; empty groups return None, e.g. `let optional = group.body()`.
    pub fn body(self) -> Option<FlowPointerGroupBody<'a>> {
        Some(FlowPointerGroupBody {
            owner: self.owner,
            span: self.span.body.as_ref()?,
        })
    }
}

impl<'a> FlowPointerGroupBody<'a> {
    /// Returns all stored member components; e.g. `let members = body.member_triplet()`.
    pub fn member_triplet(self) -> [i32; 3] {
        std::array::from_fn(|index| {
            wire::signed_at(&self.owner.wire.decoded, self.span.members + index * 4)
        })
    }
    /// Returns the three validated nonnegative stored lengths; e.g. `let lengths = body.array_lengths()`.
    pub fn array_lengths(self) -> [usize; 3] {
        self.span.lengths
    }
    /// Enumerates every discarded triplet without native interpretation; e.g. `let count = body.discarded_triplets().len()`.
    pub fn discarded_triplets(self) -> impl ExactSizeIterator<Item = [i32; 3]> + 'a {
        let raw = &self.owner.wire.decoded[self.span.discarded.0..self.span.discarded.1];
        raw.as_chunks::<12>()
            .0
            .iter()
            .map(|triplet| std::array::from_fn(|index| wire::signed_at(triplet, index * 4)))
    }
    /// Borrows an existing wide array, including a present empty one; e.g. `let first = body.array(0)`.
    pub fn array(self, index: usize) -> Option<FlowTableWideArray<'a>> {
        let &(start, end) = self.span.arrays.get(index)?;
        Some(FlowTableWideArray {
            bytes: &self.owner.wire.decoded[start..end],
            start,
        })
    }
    /// Returns the physically stored pre-pair discard or its absence; e.g. `let raw = body.discarded_pre_pairs_word()`.
    pub fn discarded_pre_pairs_word(self) -> Option<i32> {
        self.span
            .pre_pairs
            .map(|offset| wire::signed_at(&self.owner.wire.decoded, offset))
    }
    /// Enumerates all pairs in wire order, retaining duplicates; e.g. `let count = body.pairs().len()`.
    pub fn pairs(self) -> impl ExactSizeIterator<Item = [i32; 2]> + 'a {
        let raw = &self.owner.wire.decoded[self.span.pairs.0..self.span.pairs.1];
        raw.as_chunks::<8>()
            .0
            .iter()
            .map(|pair| [wire::signed_at(pair, 0), wire::signed_at(pair, 4)])
    }
    /// Returns one exact pair with checked indexing; e.g. `let first = body.pair(0)`.
    pub fn pair(self, index: usize) -> Option<[i32; 2]> {
        let start = index.checked_mul(8)?;
        let end = start.checked_add(8)?;
        let raw = self.owner.wire.decoded[self.span.pairs.0..self.span.pairs.1].get(start..end)?;
        Some([wire::signed_at(raw, 0), wire::signed_at(raw, 4)])
    }
}

impl<'a> FlowTableWideArray<'a> {
    /// Borrows every raw array byte; e.g. `let raw = array.as_bytes()`.
    pub fn as_bytes(self) -> &'a [u8] {
        self.bytes
    }
    /// Returns exact decoded offsets; e.g. `let span = array.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.start, self.start + self.bytes.len())
    }
    /// Returns the complete raw-word count; e.g. `let count = array.word_count()`.
    pub fn word_count(self) -> usize {
        self.bytes.len() / 8
    }
    /// Returns one unnormalized wide pattern; e.g. `let bits = array.raw_bits(0)`.
    pub fn raw_bits(self, index: usize) -> Option<u64> {
        let start = index.checked_mul(8)?;
        let end = start.checked_add(8)?;
        Some(wire::wide_at(self.bytes.get(start..end)?, 0))
    }
}

impl FlowLegacyWords {
    /// Returns whether the count is physically stored; e.g. `let stored = words.count_stored()`.
    pub fn count_stored(&self) -> bool {
        self.wire.version == 1
    }
    /// Returns the complete word count; e.g. `let count = words.word_count()`.
    pub fn word_count(&self) -> usize {
        self.count
    }
    /// Returns exact decoded word offsets; e.g. `let span = words.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (self.words_start, self.wire.decoded.len())
    }
    /// Borrows every raw word byte; e.g. `let raw = words.raw_bytes()`.
    pub fn raw_bytes(&self) -> &[u8] {
        &self.wire.decoded[self.words_start..]
    }
    /// Returns one exact unsigned word; e.g. `let raw = words.raw_word(0)`.
    pub fn raw_word(&self, index: usize) -> Option<u32> {
        if index >= self.count {
            return None;
        }
        Some(wire::unsigned_at(
            &self.wire.decoded,
            self.words_start + index * 4,
        ))
    }
    /// Reports exact word coverage and legacy context separately; e.g. `let checked = words.coverage()`.
    pub fn coverage(&self) -> FlowLegacyTableCoverage {
        coverage(
            &self.wire,
            0,
            0,
            self.count,
            None,
            self.options.legacy_counts,
        )
    }
    /// Borrows unresolved-role and application exclusions; e.g. `let exclusions = words.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowLegacyTableDiagnostic] {
        &[
            FlowLegacyTableDiagnostic::WordRolesNotResolved,
            FlowLegacyTableDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowLegacyTableDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

fn coverage(
    wire: &TableWire,
    pairs: usize,
    wide: usize,
    raw: usize,
    pointer_range: Option<FlowGlobalPointerCount>,
    external_word_counts: Option<FlowLegacyWordCounts>,
) -> FlowLegacyTableCoverage {
    FlowLegacyTableCoverage {
        mode: wire::mode(wire),
        stored_bytes: wire.stored_bytes,
        decoded_bytes: wire.decoded.len(),
        pointer_pairs: pairs,
        wide_words: wide,
        raw_words: raw,
        pointer_range,
        external_word_counts,
        achieved_error: 0,
    }
}
