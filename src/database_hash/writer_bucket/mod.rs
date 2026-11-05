use super::*;
use crate::database_overflow::{
    OverflowDiagnostic, OverflowPageHistory, OverflowPageImage, OverflowPayload,
    OverflowStorageContext,
};
use std::num::NonZeroU64;
mod decode;
mod edit;
mod types;
pub use edit::HashWriterEdit;
use types::WriterPayload;
pub use types::*;

/// Immutable complete selected bucket and uniquely owned overflow payload chains.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashWriterBucket {
    context: OverflowStorageContext,
    maximum_entries: usize,
    bytes: Box<[u8]>,
    header: HashBufferHeader,
    free: Vec<HashFreeRecord>,
    cells: Vec<HashWriterCell>,
    coverage: HashWriterCoverage,
}

impl HashWriterBucket {
    /// Reads the explicit writer layout; e.g. `HashWriterBucket::from_bytes(&bucket, &pages, context, 100)?`.
    pub fn from_bytes(
        bytes: &[u8],
        pages: &[OverflowPageImage],
        context: OverflowStorageContext,
        maximum_entries: usize,
    ) -> Result<Self> {
        decode::bucket(bytes, pages, context, maximum_entries)
    }
    /// Borrows complete cells in stored chain order; e.g. `let first = bucket.cells().first()`.
    pub fn cells(&self) -> &[HashWriterCell] {
        &self.cells
    }
    /// Returns raw bucket metadata; outgoing buckets are not traversed; e.g. `let next = bucket.header().next_page`.
    pub fn header(&self) -> HashBufferHeader {
        self.header
    }
    /// Borrows ordered raw free records without allocator claims; e.g. `let free = bucket.free_records()`.
    pub fn free_records(&self) -> &[HashFreeRecord] {
        &self.free
    }
    /// Reports complete local coverage; e.g. `let checked = bucket.coverage()`.
    pub fn coverage(&self) -> HashWriterCoverage {
        self.coverage
    }
    /// Reconstructs the original bucket image; e.g. `assert_eq!(bucket.to_bytes(), original)`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Reconstructs owned overflow images in page-ID order; e.g. `let pages = bucket.to_overflow_pages()`.
    pub fn to_overflow_pages(&self) -> Vec<OverflowPageImage> {
        let mut pages = Vec::new();
        for cell in &self.cells {
            if let WriterPayload::Overflow { payload, .. } = &cell.payload {
                pages.extend_from_slice(payload.pages());
            }
        }
        pages.sort_by_key(|page| page.identifier);
        pages
    }
}
