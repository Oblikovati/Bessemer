use super::*;

/// Complete non-destructive lexical edit, retaining both snapshots and every record parent.
#[derive(Debug, Clone)]
pub struct InspectionRecordEdit {
    /// Immutable source envelope and records anchoring all history parents.
    pub source: InspectionRecords,
    /// Original immutable lexical checkpoint, anchoring any additional replay parents.
    pub checkpoint: InspectionRecords,
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
        let replayed =
            changed && self.replay_checkpoint().xml.attribute(id.ordinal, local) == Some(value);
        let history = record_history(self, &project, id, changed, replayed)?;
        Ok(publish_record_edit(self, project, history, changed))
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
        let xml = self.replacement_xml(id, local, value)?;
        let mut project = Self::from_envelope(&self.envelope.with_xml(xml.bytes())?)?;
        // Retain the initial revision rather than chaining every intermediate snapshot (#194).
        project.checkpoint = Some(
            self.checkpoint
                .clone()
                .unwrap_or_else(|| Arc::new(self.clone())),
        );
        self.validate_replacement(&project, id, local, value)?;
        Ok(project)
    }

    fn replacement_xml(
        &self,
        id: &InspectionRecordId,
        local: &str,
        value: &str,
    ) -> Result<Utf8XmlDocument> {
        let checkpoint = self.replay_checkpoint();
        if checkpoint.xml.attribute(id.ordinal, local) == Some(value) {
            return self.xml.restore_attribute(
                id.ordinal,
                local,
                &checkpoint.xml,
                self.envelope.limits,
            );
        }
        self.xml
            .with_attribute(id.ordinal, local, value, self.envelope.limits)
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
    replayed: bool,
) -> Result<Vec<InspectionRecordHistory>> {
    source
        .records
        .iter()
        .zip(result.records.iter())
        .map(|(parent, record)| {
            record_relation(source, parent, record, selected, changed, replayed)
        })
        .collect()
}

fn record_relation(
    source: &InspectionRecords,
    parent: &InspectionRecord,
    result: &InspectionRecord,
    selected: &InspectionRecordId,
    changed: bool,
    replayed: bool,
) -> Result<InspectionRecordHistory> {
    let mut parents = vec![parent.id.clone()];
    if replayed && parent.id == *selected {
        let checkpoint = checkpoint_record(source, parent.id.ordinal)?;
        if checkpoint.id != parent.id {
            parents.push(checkpoint.id.clone());
        }
    }
    Ok(InspectionRecordHistory {
        parents: parents.into(),
        result: result.id.clone(),
        relation: if changed && parent.id == *selected {
            InspectionRecordRelation::Derived
        } else {
            InspectionRecordRelation::Copied
        },
    })
}

fn checkpoint_record(source: &InspectionRecords, ordinal: usize) -> Result<&InspectionRecord> {
    source
        .replay_checkpoint()
        .records
        .iter()
        .find(|record| record.id.ordinal == ordinal)
        .ok_or_else(|| {
            record_error(
                FailureClass::Integrity,
                format!(
                    "checkpoint ordinal {ordinal}; expected original record for lexical replay"
                ),
            )
        })
}

fn publish_record_edit(
    source: &InspectionRecords,
    project: InspectionRecords,
    history: Vec<InspectionRecordHistory>,
    changed: bool,
) -> InspectionRecordEdit {
    InspectionRecordEdit {
        source: source.clone(),
        checkpoint: source.replay_checkpoint().clone(),
        coverage: project.coverage(),
        diagnostics: project.diagnostics().into(),
        project,
        completion: if changed {
            InspectionRecordCompletion::Changed
        } else {
            InspectionRecordCompletion::Unchanged
        },
        history: history.into(),
    }
}
