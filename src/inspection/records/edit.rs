use super::*;

/// Complete non-destructive lexical edit, retaining both snapshots and every record parent.
#[derive(Debug, Clone)]
pub struct InspectionRecordEdit {
    /// Immutable source envelope and records anchoring all history parents.
    pub source: InspectionRecords,
    /// Complete reparsed immutable result, ready for envelope encoding.
    pub project: InspectionRecords,
    /// Changed or unchanged successful operation.
    pub completion: InspectionRecordCompletion,
    /// Every result record in physical order with its source parent.
    pub history: Arc<[InspectionRecordHistory]>,
    /// Complete XML and literal-role validation scope.
    pub coverage: InspectionRecordCoverage,
    /// Semantic exclusions accompanying this result.
    pub diagnostics: Arc<[InspectionRecordDiagnostic]>,
}

impl InspectionRecords {
    /// Edit an existing unqualified attribute; e.g. `records.edit_attribute(&id, "Value", "Pass")?`.
    pub fn edit_attribute(
        &self,
        id: &InspectionRecordId,
        local: &str,
        value: &str,
    ) -> Result<InspectionRecordEdit> {
        let changed = self.existing_value(id, local)? != value;
        let project = self.replaced_project(id, local, value, changed)?;
        let history = record_history(self, &project, id, changed);
        Ok(InspectionRecordEdit {
            source: self.clone(),
            coverage: project.coverage(),
            diagnostics: project.diagnostics().into(),
            project,
            completion: if changed {
                InspectionRecordCompletion::Changed
            } else {
                InspectionRecordCompletion::Unchanged
            },
            history: history.into(),
        })
    }

    fn existing_value(&self, id: &InspectionRecordId, local: &str) -> Result<&str> {
        self.record(id)?.attribute(local).ok_or_else(|| {
            record_error(
                FailureClass::Missing,
                format!(
                    "attribute {local:?} on record {id:?}; expected existing unqualified attribute"
                ),
            )
        })
    }

    fn replaced_project(
        &self,
        id: &InspectionRecordId,
        local: &str,
        value: &str,
        changed: bool,
    ) -> Result<Self> {
        if !changed {
            return Ok(self.clone());
        }
        let xml = self
            .xml
            .with_attribute(id.ordinal, local, value, self.envelope.limits)?;
        let project = Self::from_envelope(&self.envelope.with_xml(xml.bytes())?)?;
        self.validate_replacement(&project, id, local, value)?;
        Ok(project)
    }

    fn validate_replacement(
        &self,
        project: &Self,
        id: &InspectionRecordId,
        local: &str,
        value: &str,
    ) -> Result<()> {
        let actual = project
            .records
            .iter()
            .find(|record| record.id.ordinal == id.ordinal)
            .and_then(|record| record.attribute(local));
        if actual != Some(value) || project.records.len() != self.records.len() {
            return Err(record_error(
                FailureClass::Integrity,
                format!(
                    "reparsed attribute {actual:?} and {} records; expected {value:?} and {} records",
                    project.records.len(),
                    self.records.len()
                ),
            ));
        }
        Ok(())
    }
}

fn record_history(
    source: &InspectionRecords,
    result: &InspectionRecords,
    selected: &InspectionRecordId,
    changed: bool,
) -> Vec<InspectionRecordHistory> {
    source
        .records
        .iter()
        .zip(result.records.iter())
        .map(|(parent, record)| InspectionRecordHistory {
            parents: vec![parent.id.clone()].into(),
            result: record.id.clone(),
            relation: if changed && parent.id == *selected {
                InspectionRecordRelation::Derived
            } else {
                InspectionRecordRelation::Copied
            },
        })
        .collect()
}
