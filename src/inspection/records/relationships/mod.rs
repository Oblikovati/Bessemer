//! Explicit-policy Inspection relationship load and writer-filter projections.
mod projection;
mod types;
use super::*;
use std::sync::Arc;
pub use types::*;

impl InspectionRecords {
    /// Project one sheet's relationships without changing bytes; e.g. `project.relationship_views(&sheet.id, policy)?`.
    pub fn relationship_views(
        &self,
        sheet: &InspectionRecordId,
        policy: InspectionRelationshipPolicy,
    ) -> Result<InspectionRelationshipViews> {
        self.require_relationship_sheet(sheet)?;
        let attempts = projection::attempts(&self.xml, &self.revision, sheet.ordinal, policy);
        let entries = projection::first_insertions(&attempts);
        let writer_entries = entries
            .iter()
            .filter(|entry| !entry.key.is_empty())
            .cloned()
            .collect();
        Ok(publish(
            self,
            sheet,
            policy,
            attempts,
            entries,
            writer_entries,
        ))
    }

    fn require_relationship_sheet(&self, sheet: &InspectionRecordId) -> Result<()> {
        let record = self.record(sheet)?;
        if record.kind == InspectionRecordKind::Sheet {
            return Ok(());
        }
        Err(record_error(
            FailureClass::Invalid,
            format!(
                "record {sheet:?} kind {:?}; expected Sheet for relationship traversal",
                record.kind
            ),
        ))
    }
}

fn publish(
    source: &InspectionRecords,
    sheet: &InspectionRecordId,
    policy: InspectionRelationshipPolicy,
    attempts: Vec<InspectionRelationshipAttempt>,
    entries: Vec<InspectionRelationshipAttempt>,
    writer_entries: Arc<[InspectionRelationshipAttempt]>,
) -> InspectionRelationshipViews {
    InspectionRelationshipViews {
        source: source.clone(),
        sheet: sheet.clone(),
        policy,
        attempts: attempts.into(),
        entries: entries.into(),
        writer_entries,
        completion: InspectionRelationshipCompletion::Completed,
        coverage: source.coverage(),
        history: [InspectionRecordHistory {
            parents: vec![sheet.clone()].into(),
            result: sheet.clone(),
            relation: InspectionRecordRelation::Copied,
        }],
        diagnostics: RELATIONSHIP_DIAGNOSTICS,
    }
}

const RELATIONSHIP_DIAGNOSTICS: [InspectionRelationshipDiagnostic; 3] = [
    InspectionRelationshipDiagnostic::NativeDomPolicyUnknown,
    InspectionRelationshipDiagnostic::NativeMapOrderUnknown,
    InspectionRelationshipDiagnostic::GraphAndApplicationSemanticsNotChecked,
];
