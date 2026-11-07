use super::*;
use crate::database_hash::{
    HashBufferCompletion, HashBufferHistory, HashBufferRelation, HashWriterEdit, HashWriterHistory,
};
use std::num::NonZeroU64;

/// Immutable complete file operation with validated result and full ordered history.
#[derive(Clone, Debug)]
pub struct HashFileEdit {
    source: HashFileGraph,
    result: HashFileGraph,
    completion: HashBufferCompletion,
    history: Vec<HashBufferHistory>,
}

impl HashFileGraph {
    /// Replaces one same-length value by physical cell identity; e.g. `graph.edit_value(page, 12, &value)?`.
    pub fn edit_value(
        &self,
        physical: NonZeroU64,
        offset: u16,
        replacement: &[u8],
    ) -> Result<HashFileEdit> {
        let identity = physical.get();
        let bucket = bucket_at(self, identity)?;
        let edit = bucket
            .edit_value(offset, replacement)
            .map_err(|error| qualify_bucket_error(identity, error))?;
        let bytes = write_bucket(self, identity, edit.result());
        let result = Self::from_bytes(&bytes, self.context)?;
        verify(self, &result, identity, edit.result())?;
        let history = file_history(self, identity, &edit);
        Ok(HashFileEdit {
            source: self.clone(),
            result,
            completion: edit.completion(),
            history,
        })
    }
}

fn bucket_at(source: &HashFileGraph, identity: u64) -> Result<&HashWriterBucket> {
    source.buckets.get(&identity).ok_or_else(|| {
        BessemerError::new(
            "database file edit",
            &identity.to_string(),
            FailureClass::Missing,
            format!("page {identity}; expected existing bucket"),
        )
    })
}

fn write_bucket(source: &HashFileGraph, physical: u64, bucket: &HashWriterBucket) -> Vec<u8> {
    let mut bytes = source.to_bytes();
    let extent = source.context.pager_extent;
    let start = physical as usize * extent;
    bytes[start..start + extent].copy_from_slice(&bucket.to_bytes());
    for page in bucket.to_overflow_pages() {
        let start = page.identifier.get() as usize * extent;
        bytes[start..start + extent].copy_from_slice(&page.bytes);
    }
    bytes
}

fn verify(
    source: &HashFileGraph,
    result: &HashFileGraph,
    physical: u64,
    bucket: &HashWriterBucket,
) -> Result<()> {
    let mut expected = source.buckets.clone();
    expected.insert(physical, bucket.clone());
    if source.index != result.index
        || source.roles != result.roles
        || source.coverage != result.coverage
        || result.buckets != expected
    {
        return Err(invalid(
            physical,
            "reparsed replacement; expected identical index/roles/coverage and all unrelated cells",
        ));
    }
    Ok(())
}

fn file_history(
    source: &HashFileGraph,
    physical: u64,
    edit: &HashWriterEdit,
) -> Vec<HashBufferHistory> {
    let mut selected: BTreeMap<u64, Vec<HashBufferHistory>> = BTreeMap::new();
    for relation in edit.history() {
        let (identity, span) = match relation {
            HashWriterHistory::Bucket(span) => (physical, span),
            HashWriterHistory::Overflow(page) => (page.identifier.get(), &page.span),
        };
        selected.entry(identity).or_default().push(span.clone());
    }
    let mut history = Vec::new();
    for identity in 0..source.roles.len() {
        let spans = selected
            .remove(&(identity as u64))
            .unwrap_or_else(|| vec![copied(source.context.pager_extent)]);
        history.extend(
            spans
                .into_iter()
                .map(|span| absolute(span, identity * source.context.pager_extent)),
        );
    }
    history
}

fn copied(length: usize) -> HashBufferHistory {
    HashBufferHistory {
        parent: 0..length,
        result: 0..length,
        relation: HashBufferRelation::Copied,
    }
}

fn absolute(span: HashBufferHistory, base: usize) -> HashBufferHistory {
    HashBufferHistory {
        parent: base + span.parent.start..base + span.parent.end,
        result: base + span.result.start..base + span.result.end,
        relation: span.relation,
    }
}

impl HashFileEdit {
    /// Borrows the immutable source checkpoint; e.g. `let bytes = edit.source().to_bytes()`.
    pub fn source(&self) -> &HashFileGraph {
        &self.source
    }
    /// Borrows the complete revalidated result; e.g. `let bytes = edit.result().to_bytes()`.
    pub fn result(&self) -> &HashFileGraph {
        &self.result
    }
    /// Distinguishes exact no-op completion from change; e.g. `let status = edit.completion()`.
    pub fn completion(&self) -> HashBufferCompletion {
        self.completion
    }
    /// Reports exact supplied graph coverage; e.g. `let checked = edit.coverage()`.
    pub fn coverage(&self) -> HashFileCoverage {
        self.result.coverage
    }
    /// Borrows complete physical-byte-ordered history; e.g. `let relations = edit.history()`.
    pub fn history(&self) -> &[HashBufferHistory] {
        &self.history
    }
    /// Carries provider and semantic exclusions; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> [HashFileDiagnostic; 4] {
        [
            HashFileDiagnostic::ProviderAssumed,
            HashFileDiagnostic::OpaqueValues,
            HashFileDiagnostic::NativeKeyReaderDiffers,
            HashFileDiagnostic::AllocationTransactionAndApplicationNotValidated,
        ]
    }
}
