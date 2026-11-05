//! Complete explicitly supplied installed-writer overflow storage (#229).
mod decode;
mod edit;
mod types;
use crate::{BessemerError, FailureClass, Result};
pub use edit::OverflowPayloadEdit;
use std::num::NonZeroU64;
pub use types::*;

/// Immutable complete key/value payload and owned, ID-ordered page snapshots.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OverflowPayload {
    context: OverflowStorageContext,
    first: NonZeroU64,
    pages: Vec<OverflowPageImage>,
    key: Box<[u8]>,
    value: Box<[u8]>,
    value_spans: Vec<types::PayloadSpan>,
    coverage: OverflowCoverage,
}

impl OverflowPayload {
    /// Reads exactly one supplied chain; e.g. `OverflowPayload::from_pages(&pages, first, 36, 304, context)?`.
    pub fn from_pages(
        pages: &[OverflowPageImage],
        first: NonZeroU64,
        key_length: u32,
        value_length: u64,
        context: OverflowStorageContext,
    ) -> Result<Self> {
        decode::payload(pages, first, key_length, value_length, context)
    }
    /// Borrows the exact raw key; e.g. `let key = payload.key()`.
    pub fn key(&self) -> &[u8] {
        &self.key
    }
    /// Borrows the complete opaque value; e.g. `let value = payload.value()`.
    pub fn value(&self) -> &[u8] {
        &self.value
    }
    /// Borrows complete pages in ID order; e.g. `let original = payload.pages().to_vec()`.
    pub fn pages(&self) -> &[OverflowPageImage] {
        &self.pages
    }
    /// Reports exact local coverage and zero achieved error; e.g. `let coverage = payload.coverage()`.
    pub fn coverage(&self) -> OverflowCoverage {
        self.coverage
    }
}

fn invalid(entity: impl ToString, cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "database overflow",
        &entity.to_string(),
        FailureClass::Invalid,
        cause,
    )
}
