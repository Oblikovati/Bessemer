use super::{UsdArchive, UsdRecordId, UsdRevision};
use std::sync::Arc;

/// Exact record checks, separate from native loader or application validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdCheckMode {
    /// Every count, header, payload extent and complete-stream boundary.
    ExactCompleteCountedRecords,
    /// Complete framing plus literal preservation of every unchanged record.
    ExactCompleteRecordsAndUnchangedRecordBytes,
    /// Complete Unicode record grammar and every documented writer literal.
    ExactUnicodeWriterGrammarAndLiterals,
}

/// Successful framing completion, including a completed empty stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdReadCompletion {
    /// No records were supplied; the empty span was checked completely.
    CompletedEmpty,
    /// Every supplied record was framed completely.
    CompletedRecords,
}

/// Responsibilities outside raw record framing and edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdDiagnostic {
    /// Narrow bytes are not decoded with an invented source code page.
    NarrowCodePageUnassigned,
    /// Framing alone does not establish the separate labeled writer grammar.
    WriterLayoutNotImplied,
    /// No native dispatch, conversion, loader or application validation occurred.
    NativeLoaderNotExecuted,
}

/// Exact complete-stream coverage, with semantic limits retained.
#[derive(Debug, Clone, Copy)]
pub struct UsdFramingCoverage {
    /// Empty versus populated successful completion.
    pub completion: UsdReadCompletion,
    /// Every record boundary is checked exactly.
    pub check_mode: UsdCheckMode,
    /// All complete records, including text after #EOF.
    pub checked_records: usize,
    /// All supplied encoded bytes.
    pub checked_bytes: usize,
    /// Native/semantic exclusions.
    pub diagnostics: [UsdDiagnostic; 3],
}

/// Byte change status independent of semantic or native loader success.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdEditCompletion {
    /// All bytes were already identical; original identities survived.
    Unchanged,
    /// One exact encoded record changed on a validated isolated image.
    Changed,
}

/// Exact parent relationship for one record in a completed edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsdRecordRelation {
    /// Complete original header and payload survived verbatim.
    Copied,
    /// This record derives from its original position and supplied replacement.
    Derived,
}

/// A revision-bound record relationship, including unchanged records.
#[derive(Debug, Clone)]
pub struct UsdRecordHistory {
    /// Original complete record identity.
    pub parent: UsdRecordId,
    /// Validated resulting record identity.
    pub result: UsdRecordId,
    /// Copied versus explicitly replaced.
    pub relation: UsdRecordRelation,
}

/// Completed isolated record edit with complete sources and byte-preservation history.
#[derive(Debug, Clone)]
pub struct UsdRecordEdit {
    /// Retained immutable original stream.
    pub source: UsdArchive,
    /// Validated complete resulting stream.
    pub archive: UsdArchive,
    /// Complete supplied replacement image, retaining its own original framing.
    pub replacement: super::UsdStringRecord,
    /// Requested source position.
    pub parent: UsdRecordId,
    /// Resulting position.
    pub result: UsdRecordId,
    /// Original complete-stream identity.
    pub parent_revision: UsdRevision,
    /// Result complete-stream identity, identical on byte no-op.
    pub result_revision: UsdRevision,
    /// Changed versus authentic byte no-op.
    pub completion: UsdEditCompletion,
    /// Complete output framing with the stronger edit preservation mode.
    pub coverage: UsdFramingCoverage,
    /// Every original record contributes one ordered relationship.
    pub history: Arc<[UsdRecordHistory]>,
    /// Achieved requested encoded-pattern error, zero.
    pub requested_pattern_error_bits: u32,
}

impl UsdArchive {
    /// Exact complete-stream scope and limits, e.g. `archive.coverage()`; no native acceptance is implied.
    pub fn coverage(&self) -> UsdFramingCoverage {
        UsdFramingCoverage {
            completion: if self.spans.is_empty() {
                UsdReadCompletion::CompletedEmpty
            } else {
                UsdReadCompletion::CompletedRecords
            },
            check_mode: UsdCheckMode::ExactCompleteCountedRecords,
            checked_records: self.spans.len(),
            checked_bytes: self.bytes.len(),
            diagnostics: [
                UsdDiagnostic::NarrowCodePageUnassigned,
                UsdDiagnostic::WriterLayoutNotImplied,
                UsdDiagnostic::NativeLoaderNotExecuted,
            ],
        }
    }
}
