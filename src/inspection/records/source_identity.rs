use super::*;

/// Completion of an explicit source comparison, regardless of matching metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionSourceCompletion {
    /// Every supplied byte was hashed and both stored strings were compared.
    Completed,
}

/// Declared source-identity check scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionSourceCheckMode {
    /// Complete supplied bytes and exact, unnormalized stored metadata text.
    ExactCallerBytesAndStoredText,
}

/// Complete source-byte coverage without file resolution or format interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InspectionSourceCoverage {
    /// Exact versus sampled scope.
    pub mode: InspectionSourceCheckMode,
    /// Entire supplied source length, including an empty completed source.
    pub source_bytes: usize,
    /// Bytes left unprocessed; successful complete comparisons require zero.
    pub unread_source_bytes: usize,
}

/// Explicit exclusions from a completed compatibility comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectionSourceDiagnostic {
    /// The MD5 compatibility field does not authenticate the caller's source.
    ChecksumDoesNotAuthenticate,
    /// No stored SourcePath was opened or resolved.
    StoredPathsNotOpened,
    /// No application reload or source-file semantics were validated.
    ApplicationCompatibilityNotChecked,
}

/// Stored and actual identity observations; mismatches are completed results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectionSourceIdentityValues {
    /// Original ModelSize text, distinguishing missing and empty attributes.
    pub stored_size: Option<Arc<str>>,
    /// Length of the caller-supplied bytes, without consulting SourcePath.
    pub actual_size: usize,
    /// Exact comparison with the actual size's ordinary decimal spelling.
    pub size_matches: bool,
    /// Original ModelMD5HashData text, without case folding or hash parsing.
    pub stored_md5: Option<Arc<str>>,
    /// Lowercase checksum over every caller-supplied byte.
    pub actual_md5: Arc<str>,
    /// Exact stored-text comparison; absent or malformed text remains a mismatch.
    pub md5_matches: bool,
}

/// Immutable comparison and provenance, retaining the unchanged document snapshot.
#[derive(Debug, Clone)]
pub struct InspectionSourceComparison {
    /// Source project anchoring the document and all history parents.
    pub source: InspectionRecords,
    /// Validated physical Document occurrence, independent of stored Uid text.
    pub document: InspectionRecordId,
    /// Exact stored text, actual identity and separate comparison outcomes.
    pub values: InspectionSourceIdentityValues,
    /// Completed comparison even when either field is missing or mismatched.
    pub completion: InspectionSourceCompletion,
    /// Full byte and stored-text check scope.
    pub coverage: InspectionSourceCoverage,
    /// Unchanged source document relation; comparisons create no new revision.
    pub history: [InspectionRecordHistory; 1],
    /// Explicit limits carried with every successful comparison.
    pub diagnostics: [InspectionSourceDiagnostic; 3],
}

impl InspectionRecords {
    /// Compare explicitly supplied source bytes without resolving paths; e.g. `project.compare_document_source(&document.id, bytes)?`.
    pub fn compare_document_source(
        &self,
        document: &InspectionRecordId,
        bytes: &[u8],
    ) -> Result<InspectionSourceComparison> {
        let record = self.comparison_document(document)?;
        Ok(InspectionSourceComparison {
            source: self.clone(),
            document: record.id.clone(),
            values: comparison_values(record, bytes),
            completion: InspectionSourceCompletion::Completed,
            coverage: InspectionSourceCoverage {
                mode: InspectionSourceCheckMode::ExactCallerBytesAndStoredText,
                source_bytes: bytes.len(),
                unread_source_bytes: 0,
            },
            history: source_document_history(record),
            diagnostics: [
                InspectionSourceDiagnostic::ChecksumDoesNotAuthenticate,
                InspectionSourceDiagnostic::StoredPathsNotOpened,
                InspectionSourceDiagnostic::ApplicationCompatibilityNotChecked,
            ],
        })
    }

    fn comparison_document(&self, id: &InspectionRecordId) -> Result<&InspectionRecord> {
        let record = self.record(id).map_err(|mut error| {
            error.operation = "Inspection source identity";
            error
        })?;
        if record.kind != InspectionRecordKind::Document {
            return Err(BessemerError::new(
                "Inspection source identity",
                "document occurrence",
                FailureClass::Invalid,
                format!(
                    "record {id:?} kind {:?}; expected Document source-identity owner",
                    record.kind
                ),
            ));
        }
        Ok(record)
    }
}

fn source_document_history(record: &InspectionRecord) -> [InspectionRecordHistory; 1] {
    [InspectionRecordHistory {
        parents: vec![record.id.clone()].into(),
        result: record.id.clone(),
        relation: InspectionRecordRelation::Copied,
    }]
}

fn comparison_values(record: &InspectionRecord, source: &[u8]) -> InspectionSourceIdentityValues {
    let actual_size = source.len();
    let actual_md5: Arc<str> = super::source_md5::compatibility_md5_hex(source).into();
    let stored_size: Option<Arc<str>> = record.attribute("ModelSize").map(Arc::from);
    let stored_md5: Option<Arc<str>> = record.attribute("ModelMD5HashData").map(Arc::from);
    let size_matches = stored_size.as_deref() == Some(actual_size.to_string().as_str());
    let md5_matches = stored_md5.as_deref() == Some(actual_md5.as_ref());
    InspectionSourceIdentityValues {
        stored_size,
        actual_size,
        size_matches,
        stored_md5,
        actual_md5,
        md5_matches,
    }
}
