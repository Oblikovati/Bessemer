//! Conditional complete default-provider hash file graphs and isolated value edits.
mod decode;
mod edit;
mod index;
mod types;
use crate::database_hash::HashWriterBucket;
use crate::{BessemerError, FailureClass, Result};
pub use edit::HashFileEdit;
use std::collections::BTreeMap;
pub use types::*;

/// Immutable owned file bytes with a complete classified and hash-checked graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashFileGraph {
    context: HashFileContext,
    bytes: Box<[u8]>,
    index: index::FileIndex,
    roles: Vec<HashFilePageRole>,
    buckets: BTreeMap<u64, HashWriterBucket>,
    coverage: HashFileCoverage,
}

impl HashFileGraph {
    /// Reads the explicitly selected complete file; e.g. `HashFileGraph::from_bytes(&raw, context)?`.
    pub fn from_bytes(bytes: &[u8], context: HashFileContext) -> Result<Self> {
        decode::file(bytes, context)
    }
    /// Reconstructs all bytes including opaque tails; e.g. `assert_eq!(graph.to_bytes(), raw)`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    /// Borrows persisted map pairs in stored order; e.g. `let mappings = graph.mappings()`.
    pub fn mappings(&self) -> &[HashFileMapEntry] {
        &self.index.entries
    }
    /// Borrows complete page roles in physical ID order; e.g. `let header = graph.page_roles()[0]`.
    pub fn page_roles(&self) -> &[HashFilePageRole] {
        &self.roles
    }
    /// Borrows a complete bucket by physical identity; e.g. `let bucket = graph.bucket(2)`.
    pub fn bucket(&self, physical: u64) -> Option<&HashWriterBucket> {
        self.buckets.get(&physical)
    }
    /// Reports exact framing and graph coverage; e.g. `let checked = graph.coverage()`.
    pub fn coverage(&self) -> HashFileCoverage {
        self.coverage
    }
}

fn invalid(entity: impl ToString, cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "database file graph",
        &entity.to_string(),
        FailureClass::Invalid,
        cause,
    )
}

fn qualify_bucket_error(physical: u64, error: BessemerError) -> BessemerError {
    BessemerError::new(
        "database file graph",
        &format!("bucket page {physical}"),
        error.class,
        format!("{} {}: {}", error.operation, error.entity, error.cause),
    )
}
