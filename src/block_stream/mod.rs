//! Compressed-block carriers from SolidWorks specification clause 9.3.

pub(crate) mod decode;
mod encode;

use crate::{BessemerError, FailureClass, Result};

pub(crate) const CHUNK_GUID: [u8; 16] = [
    0x23, 0x1d, 0xd5, 0x71, 0xda, 0x81, 0x48, 0xa2, 0xa8, 0x58, 0x98, 0xb2, 0x1b, 0x89, 0xef, 0x99,
];
pub(crate) const BLOCK_LIMIT: usize = 1_048_576;

/// Whether each chunk carries its outer byte count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkFraming {
    /// Body partition streams: u32 chunk length followed by the GUID and blocks.
    LengthPrefixed,
    /// Streams named `__ZLB`: GUID followed directly by blocks.
    GuidOnly,
}

/// One validated chunk, retaining both its encoding and expanded content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompressedChunk {
    pub(crate) original: Vec<u8>,
    pub(crate) payload: Vec<u8>,
    pub(crate) replacement: Option<Vec<u8>>,
}

impl CompressedChunk {
    /// Returns the complete inflated chunk, including a pending replacement.
    ///
    /// ```no_run
    /// # let bytes = std::fs::read("carrier.bin")?;
    /// # let carrier = bessemer::block_stream::BlockStream::from_bytes(&bytes, bessemer::block_stream::ChunkFraming::GuidOnly, 1024)?;
    /// let payload = carrier.chunks()[0].payload();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn payload(&self) -> &[u8] {
        self.replacement.as_deref().unwrap_or(&self.payload)
    }
}

/// Owned, validated chunks with isolated replacement state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockStream {
    chunks: Vec<CompressedChunk>,
    framing: ChunkFraming,
    maximum_expanded: usize,
}

impl BlockStream {
    /// Decodes all chunks and checks zlib integrity under a total expansion limit.
    ///
    /// ```no_run
    /// use bessemer::block_stream::{BlockStream, ChunkFraming};
    /// let bytes = std::fs::read("partition.bin")?;
    /// let carrier = BlockStream::from_bytes(&bytes, ChunkFraming::LengthPrefixed, 64 * 1024 * 1024)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn from_bytes(
        bytes: &[u8],
        framing: ChunkFraming,
        maximum_expanded: usize,
    ) -> Result<Self> {
        let chunks = decode::chunks(bytes, framing, maximum_expanded)?;
        Ok(Self {
            chunks,
            framing,
            maximum_expanded,
        })
    }

    /// Returns validated chunks in their stored order.
    ///
    /// ```no_run
    /// # let bytes = std::fs::read("carrier.bin")?;
    /// # let carrier = bessemer::block_stream::BlockStream::from_bytes(&bytes, bessemer::block_stream::ChunkFraming::GuidOnly, 1024)?;
    /// for chunk in carrier.chunks() { println!("{} bytes", chunk.payload().len()); }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn chunks(&self) -> &[CompressedChunk] {
        &self.chunks
    }

    /// Replaces a nonempty chunk, keeping all other chunk encodings untouched.
    ///
    /// This validates carrier sizes, not the model semantics of the replacement.
    ///
    /// ```no_run
    /// # let bytes = std::fs::read("carrier.bin")?;
    /// # let mut carrier = bessemer::block_stream::BlockStream::from_bytes(&bytes, bessemer::block_stream::ChunkFraming::GuidOnly, 1024)?;
    /// carrier.replace_chunk(0, b"replacement".to_vec())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn replace_chunk(&mut self, index: usize, payload: Vec<u8>) -> Result<()> {
        let available = self.replacement_budget(index)?;
        validate_replacement(index, payload.len(), available)?;
        let replacement = (payload != self.chunks[index].payload).then_some(payload);
        self.chunks[index].replacement = replacement;
        Ok(())
    }

    fn replacement_budget(&self, index: usize) -> Result<usize> {
        let chunk = self.chunks.get(index).ok_or_else(|| {
            failure(
                index,
                FailureClass::Missing,
                format!("index {index}; expected < {}", self.chunks.len()),
            )
        })?;
        let total: usize = self.chunks.iter().map(|chunk| chunk.payload().len()).sum();
        Ok(self.maximum_expanded - (total - chunk.payload().len()))
    }

    /// Serializes chunks, preserving each unchanged chunk's original bytes.
    ///
    /// ```no_run
    /// # let bytes = std::fs::read("carrier.bin")?;
    /// # let carrier = bessemer::block_stream::BlockStream::from_bytes(&bytes, bessemer::block_stream::ChunkFraming::GuidOnly, 1024)?;
    /// std::fs::write("partition-copy.bin", carrier.to_bytes()?)?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut encoded = Vec::new();
        for (index, chunk) in self.chunks.iter().enumerate() {
            encode::append(&mut encoded, chunk, self.framing, index)?;
        }
        Ok(encoded)
    }
}

pub(crate) fn failure(index: usize, class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new(
        "compressed-block stream",
        &format!("chunk {index}"),
        class,
        cause,
    )
}

fn validate_replacement(index: usize, size: usize, available: usize) -> Result<()> {
    if size > 0 && size <= available {
        return Ok(());
    }
    Err(failure(
        index,
        FailureClass::Invalid,
        format!("replacement length {size}; expected 1..={available} bytes"),
    ))
}

pub(crate) fn invalid_chunk(cause: impl Into<String>) -> BessemerError {
    failure(0, FailureClass::Invalid, cause.into())
}
