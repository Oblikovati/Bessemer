use super::index::FileIndex;
use super::*;
mod edit;
mod types;
pub use edit::HashFileIndexEdit;
pub use types::*;

/// Immutable lossless saved index, distinct from an admitted complete file graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashFileIndexBuffer {
    context: HashFileContext,
    bytes: Box<[u8]>,
    index: FileIndex,
    coverage: HashIndexCoverage,
}

impl HashFileIndexBuffer {
    /// Reads complete map frames without graph admission; e.g. `HashFileIndexBuffer::from_bytes(&raw, context)?`.
    pub fn from_bytes(bytes: &[u8], context: HashFileContext) -> Result<Self> {
        read_saved_buffer(bytes, context).map_err(saved_index_read_error)
    }
    /// Borrows raw pairs in stored order, including duplicates; e.g. `let entries = buffer.mappings()`.
    pub fn mappings(&self) -> &[HashFileMapEntry] {
        &self.index.entries
    }
    /// Returns raw saved words without domain repair; e.g. `let split = buffer.header().split`.
    pub fn header(&self) -> HashIndexHeader {
        HashIndexHeader {
            free_head: self.index.free_head,
            split: self.index.split,
            maximum: self.index.maximum,
        }
    }
    /// Reconstructs every supplied byte, including non-index pages; e.g. `assert_eq!(buffer.to_bytes(), raw)`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Reports index-only checks and retained bytes; e.g. `let checked = buffer.coverage()`.
    pub fn coverage(&self) -> HashIndexCoverage {
        self.coverage
    }
}

fn read_saved_buffer(bytes: &[u8], context: HashFileContext) -> Result<HashFileIndexBuffer> {
    let mut reader = decode::bounded_file_reader(bytes, context)?;
    reader.claim(0, HashFilePageRole::Header)?;
    let index = index::read_saved(&mut reader)?;
    let coverage = saved_index_coverage(&reader, index.entries.len());
    Ok(HashFileIndexBuffer {
        context,
        bytes: bytes.into(),
        index,
        coverage,
    })
}

fn saved_index_read_error(error: BessemerError) -> BessemerError {
    BessemerError::new(
        "saved index read",
        &error.entity,
        error.class,
        format!("{}: {}", error.operation, error.cause),
    )
}

fn saved_index_coverage(reader: &index::FileReader<'_>, mappings: usize) -> HashIndexCoverage {
    HashIndexCoverage {
        mode: HashIndexCheckMode::ExactSavedIndexChain,
        buffer_bytes: reader.bytes.len(),
        interpreted_bytes: reader.interpreted,
        uninterpreted_bytes: reader.bytes.len() - reader.interpreted,
        index_pages: reader
            .roles
            .iter()
            .filter(|role| **role == Some(HashFilePageRole::Index))
            .count(),
        mappings,
        achieved_byte_error: 0,
    }
}
