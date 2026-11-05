//! Conditional installed inline hash-page buffers (9.2 / C-3595 / C-3601–3603).
mod decode;
mod edit;
mod page_reader;
mod types;
use crate::{BessemerError, FailureClass, Result};
pub use edit::HashBufferEdit;
pub use types::*;

/// Immutable complete buffer, ordered chains and owned inline cell payloads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashPageBuffer {
    context: HashBufferContext,
    bytes: Box<[u8]>,
    header: HashBufferHeader,
    free: Vec<HashFreeRecord>,
    cells: Vec<HashInlineCell>,
    coverage: HashBufferCoverage,
}

impl HashPageBuffer {
    /// Reads one explicitly selected engine buffer; e.g. `HashPageBuffer::from_bytes(&bytes, context)?`.
    pub fn from_bytes(bytes: &[u8], context: HashBufferContext) -> Result<Self> {
        decode::buffer(bytes, context)
    }
    /// Returns raw page metadata; e.g. `let next = buffer.header().next_page`.
    pub fn header(&self) -> HashBufferHeader {
        self.header
    }
    /// Borrows free records in native chain order; e.g. `let first = buffer.free_records().first()`.
    pub fn free_records(&self) -> &[HashFreeRecord] {
        &self.free
    }
    /// Borrows complete cells in native chain order; e.g. `let first = buffer.cells().first()`.
    pub fn cells(&self) -> &[HashInlineCell] {
        &self.cells
    }
    /// Reproduces the reader's modulo16 sum without allocation guarantees; e.g. `let total = buffer.free_size_sum()`.
    pub fn free_size_sum(&self) -> u16 {
        self.free
            .iter()
            .fold(0u16, |sum, record| sum.wrapping_add(record.size))
    }
    /// Reports local interpretation versus retained bytes; e.g. `let checked = buffer.coverage()`.
    pub fn coverage(&self) -> HashBufferCoverage {
        self.coverage
    }
    /// Reconstructs all bytes, including unreferenced/free contents; e.g. `assert_eq!(buffer.to_bytes(), bytes)`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "database hash buffer",
        "selected page image",
        FailureClass::Invalid,
        cause,
    )
}
