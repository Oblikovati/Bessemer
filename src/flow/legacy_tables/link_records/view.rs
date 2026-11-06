mod arrays;
mod checkpoints;
mod tails;
use super::*;

impl FlowLinkRecords {
    /// Borrows one physical entry; e.g. `let first = links.entry(0)`.
    pub fn entry(&self, ordinal: usize) -> Option<FlowLinkEntry<'_>> {
        Some(FlowLinkEntry {
            owner: self,
            ordinal,
            span: self.entries.get(ordinal)?,
        })
    }
    /// Enumerates entries in physical order, retaining duplicates; e.g. `let count = links.entries().len()`.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = FlowLinkEntry<'_>> {
        self.entries
            .iter()
            .enumerate()
            .map(|(ordinal, span)| FlowLinkEntry {
                owner: self,
                ordinal,
                span,
            })
    }
    /// Borrows the complete versioned tail; e.g. `let tail = links.tail()`.
    pub fn tail(&self) -> FlowLinkTail<'_> {
        FlowLinkTail { owner: self }
    }
    /// Reports exact wire scope and explicit context; e.g. `let coverage = links.coverage()`.
    pub fn coverage(&self) -> FlowLinkRecordCoverage {
        FlowLinkRecordCoverage {
            mode: wire::mode(&self.wire),
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            entries: self.entries.len(),
            variable_integers: self.variable_integers,
            array_elements: self.array_elements,
            pointer_range: self.options.global_pointer_count,
            achieved_error: 0,
        }
    }
    /// Borrows native and semantic exclusions; e.g. `let exclusions = links.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowLinkRecordDiagnostic] {
        &[
            FlowLinkRecordDiagnostic::NativeRuntimeNotReproduced,
            FlowLinkRecordDiagnostic::EngineeringRolesNotResolved,
            FlowLinkRecordDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowLinkRecordDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl<'a> FlowLinkEntry<'a> {
    /// Returns its physical ordinal; e.g. `let ordinal = entry.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns exact decoded entry bounds; e.g. `let interval = entry.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.start, self.span.end)
    }
    /// Returns both raw references, retaining the -1 sentinel; e.g. `let refs = entry.reference_indices()`.
    pub fn reference_indices(self) -> [i32; 2] {
        std::array::from_fn(|i| wire::signed_at(&self.owner.wire.decoded, self.span.start + i * 4))
    }
    /// Borrows all seven raw binary64 words; e.g. `let bits = entry.wide_words().raw_bits(0)`.
    pub fn wide_words(self) -> FlowTableWideArray<'a> {
        let start = self.span.start + 8;
        FlowTableWideArray {
            bytes: &self.owner.wire.decoded[start..start + 56],
            start,
        }
    }
    /// Returns the serialized byte before any runtime conversion; e.g. `let byte = entry.code_byte()`.
    pub fn code_byte(self) -> u8 {
        self.owner.wire.decoded[self.span.start + 64]
    }
    /// Returns the documented version-zero conversion; e.g. `let code = entry.native_code_byte()`.
    pub fn native_code_byte(self) -> u8 {
        let code = self.code_byte();
        let band = code / 10;
        if self.owner.wire.version == 0 && (3..=6).contains(&band) && code % 10 < 6 {
            return band * 10 + (code % 10) / 2;
        }
        code
    }
    /// Borrows every byte-counted signed integer; e.g. `let values = entry.variable_integers()`.
    pub fn variable_integers(self) -> FlowLinkIntegers<'a> {
        integers(self.owner, self.span.start + 66, self.span.end)
    }
}

fn integers(owner: &FlowLinkRecords, start: usize, end: usize) -> FlowLinkIntegers<'_> {
    FlowLinkIntegers {
        bytes: &owner.wire.decoded[start..end],
        start,
    }
}

fn wide(owner: &FlowLinkRecords, start: usize, count: usize) -> FlowTableWideArray<'_> {
    FlowTableWideArray {
        bytes: &owner.wire.decoded[start..start + count * 8],
        start,
    }
}
