//! Ordered literal standalone Inspection records, with lossless existing-attribute edits.
mod edit;
mod load;
mod projection;
pub mod relationships;
mod source_identity;
mod source_md5;
mod types;

use super::InspectionEnvelope;
use crate::archive_codec::ArchiveRevision;
use crate::metadata::Utf8XmlDocument;
use crate::{BessemerError, FailureClass, Result};
pub use edit::InspectionRecordEdit;
pub use load::{InspectionLoadedCustomProperty, InspectionLoadedMeasurement};
pub use source_identity::{
    InspectionSourceCheckMode, InspectionSourceComparison, InspectionSourceCompletion,
    InspectionSourceCoverage, InspectionSourceDiagnostic, InspectionSourceIdentityValues,
};
use std::sync::Arc;
pub use types::*;

/// Immutable envelope and ordered lexical records; no filesystem or runtime resolution.
#[derive(Debug, Clone)]
pub struct InspectionRecords {
    envelope: InspectionEnvelope,
    xml: Utf8XmlDocument,
    revision: ArchiveRevision,
    records: Arc<[InspectionRecord]>,
    checkpoint: Option<Arc<InspectionRecords>>,
}

impl InspectionRecords {
    /// Index an already validated envelope; e.g. `InspectionRecords::from_envelope(&envelope)?`.
    pub fn from_envelope(envelope: &InspectionEnvelope) -> Result<Self> {
        let xml = Utf8XmlDocument::from_bytes_for(
            envelope.xml_bytes(),
            envelope.limits,
            "Inspection XML",
        )?;
        let revision = ArchiveRevision::default();
        let records = projection::collect(&xml, &revision).into();
        Ok(Self {
            envelope: envelope.clone(),
            xml,
            revision,
            records,
            checkpoint: None,
        })
    }

    /// Borrow the exact envelope for encryption; e.g. `records.envelope().to_bytes()?`.
    pub fn envelope(&self) -> &InspectionEnvelope {
        &self.envelope
    }

    /// Borrow all named occurrences in physical XML order; e.g. `records.records().len()`.
    pub fn records(&self) -> &[InspectionRecord] {
        &self.records
    }

    /// Resolve a same-revision physical occurrence; e.g. `records.record(&record.id)?`.
    pub fn record(&self, id: &InspectionRecordId) -> Result<&InspectionRecord> {
        if id.revision != self.revision {
            return Err(record_error(
                FailureClass::Invalid,
                format!("record {id:?}; expected identity from this project revision"),
            ));
        }
        self.records
            .iter()
            .find(|record| record.id.ordinal == id.ordinal)
            .ok_or_else(|| {
                record_error(
                    FailureClass::Missing,
                    format!("record {id:?}; expected indexed physical occurrence"),
                )
            })
    }

    /// Return complete XML/record check coverage; e.g. `records.coverage().records`.
    pub fn coverage(&self) -> InspectionRecordCoverage {
        InspectionRecordCoverage {
            mode: InspectionRecordCheckMode::ExactLexicalXml,
            xml_bytes: self.envelope.xml_bytes().len(),
            elements: self.xml.element_count(),
            records: self.records.len(),
            achieved_value_error: 0,
        }
    }

    /// Return semantic exclusions; e.g. `records.diagnostics().len()`.
    pub fn diagnostics(&self) -> &[InspectionRecordDiagnostic] {
        use InspectionRecordDiagnostic::*;
        &[
            ValueSemanticsNotEvaluated,
            ExternalSourcesNotResolved,
            ApplicationReloadNotChecked,
        ]
    }

    fn replay_checkpoint(&self) -> &Self {
        self.checkpoint.as_deref().unwrap_or(self)
    }
}

fn record_error(class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new("Inspection records", "standalone project XML", class, cause)
}

#[cfg(test)]
#[path = "../../../tests/inspection_relationships/unit_capture.rs"]
mod relationship_capture_tests;
