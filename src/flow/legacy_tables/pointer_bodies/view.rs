mod checkpoints;
mod entries;
use super::*;

impl FlowPointerBodies {
    /// Borrows one physical leaf pool; e.g. `let pool = bodies.pool(0)`.
    pub fn pool(&self, ordinal: usize) -> Option<FlowPointerBodyPool<'_>> {
        Some(FlowPointerBodyPool {
            owner: self,
            span: self.pools.get(ordinal)?,
        })
    }
    /// Enumerates complete leaf pools in wire order; e.g. `let count = bodies.pools().len()`.
    pub fn pools(&self) -> impl ExactSizeIterator<Item = FlowPointerBodyPool<'_>> {
        self.pools
            .iter()
            .map(|span| FlowPointerBodyPool { owner: self, span })
    }
    /// Returns one of three complete collection intervals; e.g. `let interval = bodies.group_interval(0)`.
    pub fn group_interval(&self, group: usize) -> Option<(usize, usize)> {
        self.groups
            .get(group)
            .map(|span| (span.full.start, span.full.end))
    }
    /// Returns the physical entry count in one collection; e.g. `let count = bodies.entry_count(2)`.
    pub fn entry_count(&self, group: usize) -> Option<usize> {
        self.groups
            .get(group)
            .map(|span| span.entries.end - span.entries.start)
    }
    /// Borrows one context-selected body; e.g. `let entry = bodies.entry(2, 0)`.
    pub fn entry(&self, group: usize, ordinal: usize) -> Option<FlowPointerBodyEntry<'_>> {
        let entries = self.groups.get(group)?.entries;
        if ordinal >= entries.end - entries.start {
            return None;
        }
        Some(FlowPointerBodyEntry {
            owner: self,
            group,
            ordinal,
            span: &self.entries[entries.start + ordinal],
        })
    }
    /// Reports complete exact serialized coverage; e.g. `let coverage = bodies.coverage()`.
    pub fn coverage(&self) -> FlowPointerBodyCoverage {
        FlowPointerBodyCoverage {
            mode: wire::mode(&self.wire),
            stored_bytes: self.wire.stored_bytes,
            decoded_bytes: self.wire.decoded.len(),
            pools: self.pools.len(),
            entries: self.entries.len(),
            vectors: self.vectors.len(),
            wide_words: self.wide_words,
            raw_bytes: self.raw_bytes,
            achieved_error: 0,
        }
    }
    /// Borrows the exclusions from serialized coverage; e.g. `let exclusions = bodies.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowPointerBodyDiagnostic] {
        use FlowPointerBodyDiagnostic::*;
        &[
            NativeRuntimeNotReproduced,
            OrientationPostprocessingNotReproduced,
            TreeOrderNotAssigned,
            EngineeringRolesNotResolved,
            PackedPayloadNotDecoded,
            OwnerCompressionContextNotReconstructed,
            NativeApplicationNotChecked,
        ]
    }
}

impl<'a> FlowPointerBodyPool<'a> {
    /// Returns the complete pool interval including its count; e.g. `let interval = pool.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.span.full.start, self.span.full.end)
    }
    /// Returns the stored triplet count, including zero; e.g. `let count = pool.triplet_count()`.
    pub fn triplet_count(self) -> u8 {
        ((self.span.wide.end - self.span.wide.start) / 24) as u8
    }
    /// Borrows all raw wide patterns without float normalization; e.g. `let bits = pool.wide_words().raw_bits(0)`.
    pub fn wide_words(self) -> FlowTableWideArray<'a> {
        body_wide(&self.owner.wire, self.span.wide)
    }
}

fn body_wide(wire: &TableWire, span: BodySpan) -> FlowTableWideArray<'_> {
    FlowTableWideArray {
        bytes: &wire.decoded[span.start..span.end],
        start: span.start,
    }
}
