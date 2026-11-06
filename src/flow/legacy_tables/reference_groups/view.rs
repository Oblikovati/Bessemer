mod checkpoints;
mod entries;
use super::*;

impl FlowReferenceGroups {
    /// Borrows one of exactly two physical groups; e.g. `let group = references.group(0)`.
    pub fn group(&self, ordinal: usize) -> Option<FlowReferenceGroup<'_>> {
        Some(FlowReferenceGroup {
            owner: self,
            ordinal,
            span: self.groups.get(ordinal)?,
        })
    }
    /// Enumerates both groups, retaining completed empty groups; e.g. `let count = references.groups().len()`.
    pub fn groups(&self) -> impl ExactSizeIterator<Item = FlowReferenceGroup<'_>> {
        self.groups
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowReferenceGroup {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Returns a stored header word when its version gate is active; e.g. `let header = references.header(FlowReferenceHeaderField::Stored)`.
    pub fn header(&self, field: FlowReferenceHeaderField) -> Option<i32> {
        self.headers[selection::header_index(field)]
            .map(|offset| wire::signed_at(&self.wire.decoded, offset))
    }
    /// Returns the native local default independently of the stored header; e.g. `let default = references.native_header_default()`.
    pub fn native_header_default(&self) -> i32 {
        2
    }
    /// Reports exact serialized totals and mode; e.g. `let coverage = references.coverage()`.
    pub fn coverage(&self) -> FlowReferenceGroupCoverage {
        FlowReferenceGroupCoverage {
            mode: wire::mode(&self.wire),
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            entries: self.entries.len(),
            reference_words: self.reference_words,
            first_words: self.word_counts[0],
            second_words: self.word_counts[1],
            mapping_words: self.mapping_words,
            wide_words: self.wide_words,
            achieved_error: 0,
        }
    }
    /// Borrows the runtime and engineering exclusions; e.g. `let exclusions = references.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowReferenceGroupDiagnostic] {
        use FlowReferenceGroupDiagnostic::*;
        &[
            MappingContextNotReconstructed,
            NativeRuntimeNotReproduced,
            ReferenceFixupsNotReproduced,
            RefinementPostprocessingNotReproduced,
            EngineeringRolesNotResolved,
            OwnerCompressionContextNotReconstructed,
            NativeApplicationNotChecked,
        ]
    }
}

impl<'a> FlowReferenceGroup<'a> {
    /// Returns the fixed physical ordinal; e.g. `let ordinal = group.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the full interval including its stored count; e.g. `let interval = group.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        self.span.full
    }
    /// Returns the physically stored entry count; e.g. `let count = group.entry_count()`.
    pub fn entry_count(self) -> usize {
        self.span.entries.1 - self.span.entries.0
    }
    /// Borrows an existing physical entry; e.g. `let entry = group.entry(0)`.
    pub fn entry(self, ordinal: usize) -> Option<FlowReferenceEntry<'a>> {
        if ordinal >= self.entry_count() {
            return None;
        }
        Some(FlowReferenceEntry {
            owner: self.owner,
            group: self.ordinal,
            ordinal,
            span: &self.owner.entries[self.span.entries.0 + ordinal],
        })
    }
    /// Enumerates entries in wire order without pointer fixups; e.g. `let count = group.entries().len()`.
    pub fn entries(self) -> impl ExactSizeIterator<Item = FlowReferenceEntry<'a>> {
        self.owner.entries[self.span.entries.0..self.span.entries.1]
            .iter()
            .enumerate()
            .map(move |(ordinal, span)| FlowReferenceEntry {
                owner: self.owner,
                group: self.ordinal,
                ordinal,
                span,
            })
    }
    /// Returns the stored level without executing native mapping shifts; e.g. `let level = group.refinement_level()`.
    pub fn refinement_level(self) -> Option<i32> {
        self.span
            .level
            .map(|offset| wire::signed_at(&self.owner.wire.decoded, offset))
    }
    /// Borrows present mapping words without signed normalization; e.g. `let mapping = group.mapping_words()`.
    pub fn mapping_words(self) -> Option<FlowBlockWordArray<'a>> {
        self.span
            .level
            .map(|_| word_array(&self.owner.wire, self.span.mapping))
    }
    /// Borrows an appended wide array only when physically present; e.g. `let wide = group.wide_words()`.
    pub fn wide_words(self) -> Option<FlowTableWideArray<'a>> {
        (self.span.wide.0 != self.span.wide.1).then(|| FlowTableWideArray {
            bytes: &self.owner.wire.decoded[self.span.wide.0..self.span.wide.1],
            start: self.span.wide.0,
        })
    }
}

fn word_array(wire: &TableWire, span: (usize, usize)) -> FlowBlockWordArray<'_> {
    FlowBlockWordArray {
        bytes: &wire.decoded[span.0..span.1],
        start: span.0,
    }
}
