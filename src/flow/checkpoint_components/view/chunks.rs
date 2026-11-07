use super::*;
/// Borrowed complete encoded chunk framing without a codec interpretation.
#[derive(Clone, Copy, Debug)]
pub struct FlowCheckpointChunks<'a> {
    owner: &'a FlowCheckpointComponent,
    chunks: &'a [ChunkLayout],
}
/// Borrowed opaque encoded chunk; empty chunks remain present.
#[derive(Clone, Copy, Debug)]
pub struct FlowCheckpointChunk<'a> {
    owner: &'a FlowCheckpointComponent,
    layout: &'a ChunkLayout,
    ordinal: usize,
}
impl FlowCheckpointComponent {
    /// Borrows version-zero chunk framing; e.g. `let chunk = view.chunk_buffer()?.chunk(0)?`.
    pub fn chunk_buffer(&self) -> Result<FlowCheckpointChunks<'_>> {
        let Layout::Chunks(chunks) = &self.revision.layout else {
            return Err(wrong_profile(self, "ChunkBuffer"));
        };
        Ok(FlowCheckpointChunks {
            owner: self,
            chunks,
        })
    }
}
impl<'a> FlowCheckpointChunks<'a> {
    /// Returns the validated stored version; e.g. `assert_eq!(chunks.version(), 0)`.
    pub fn version(self) -> i32 {
        0
    }
    /// Returns the unvalidated declared raw-size field; e.g. `let declared = chunks.declared_raw_bytes().value()?`.
    pub fn declared_raw_bytes(self) -> FlowCheckpointField {
        self.owner.field(8, ValueShape::Pattern64, ValueRule::Any)
    }
    /// Returns the positive stored chunk count; e.g. `let count = chunks.len()`.
    pub fn len(self) -> usize {
        self.chunks.len()
    }
    /// Returns whether the chunk list is empty; complete buffers always contain at least one chunk.
    /// Example: `assert!(!chunks.is_empty())`.
    pub fn is_empty(self) -> bool {
        self.chunks.is_empty()
    }
    /// Borrows a checked opaque chunk; e.g. `let encoded = chunks.chunk(0)?.encoded_bytes()`.
    pub fn chunk(self, index: usize) -> Result<FlowCheckpointChunk<'a>> {
        let ordinal = checked_index(index, self.chunks.len(), "encoded chunk")?;
        Ok(FlowCheckpointChunk {
            owner: self.owner,
            layout: &self.chunks[ordinal],
            ordinal,
        })
    }
}
impl<'a> FlowCheckpointChunk<'a> {
    /// Returns stored chunk order; e.g. `let ordinal = chunk.ordinal()`.
    pub fn ordinal(self) -> usize {
        self.ordinal
    }
    /// Returns the complete length-prefixed interval; e.g. `let span = chunk.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.layout.start, self.layout.end)
    }
    /// Returns the exact nonnegative encoded length; e.g. `let count = chunk.stored_bytes()`.
    pub fn stored_bytes(self) -> usize {
        self.layout.encoded.count
    }
    /// Returns the opaque encoded interval without its prefix; e.g. `let span = chunk.encoded_interval()`.
    pub fn encoded_interval(self) -> (usize, usize) {
        (self.layout.encoded.start, self.layout.encoded.end())
    }
    /// Borrows opaque bytes without decompression; e.g. `let raw = chunk.encoded_bytes()`.
    pub fn encoded_bytes(self) -> &'a [u8] {
        &self.owner.as_bytes()[self.layout.encoded.start..self.layout.encoded.end()]
    }
    /// Borrows editable existing bytes without changing encoded length; e.g. `let field = chunk.encoded_fields().field(0)?`.
    pub fn encoded_fields(self) -> FlowCheckpointPatternArray<'a> {
        array(self.owner, self.layout.encoded)
    }
}
