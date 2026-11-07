use super::*;
impl FlowCheckpointComponent {
    /// Replaces one existing field on a copy under its original context and bounds; e.g. `let edit = view.replace_field(&field, FlowCheckpointValue::Pattern64(7))?`.
    pub fn replace_field(
        &self,
        field: &FlowCheckpointField,
        value: FlowCheckpointValue,
    ) -> Result<FlowCheckpointComponentEdit> {
        self.check_field(field)?;
        let encoded = field.encoded(value)?;
        let result = self.replaced(field, &encoded)?;
        let result_field = result.field(field.start, field.shape, field.rule);
        let history = byte_history(
            self.as_bytes().len(),
            field.payload_interval(),
            field.value()? != value,
        );
        Ok(FlowCheckpointComponentEdit {
            source: self.clone(),
            result,
            source_field: field.clone(),
            result_field,
            history,
        })
    }
    fn check_field(&self, field: &FlowCheckpointField) -> Result<()> {
        if Arc::ptr_eq(&self.revision, &field.revision) {
            return Ok(());
        }
        Err(invalid(
            "field identity",
            format!(
                "foreign or stale field {field:?}; expected a field from this immutable {:?} component revision",
                self.kind()
            ),
        ))
    }
    fn replaced(&self, field: &FlowCheckpointField, encoded: &[u8]) -> Result<Self> {
        let (start, end) = field.payload_interval();
        if self.as_bytes()[start..end] == *encoded {
            return Ok(self.clone());
        }
        let mut raw = self.as_bytes().to_vec();
        raw[start..end].copy_from_slice(encoded);
        let result = Self::from_bytes(&raw, self.context(), self.kind(), self.options())?;
        if result.revision.layout != self.revision.layout {
            return Err(invalid(
                "replacement layout",
                format!(
                    "field interval {start}..{end}; expected unchanged complete component hierarchy and field extents"
                ),
            ));
        }
        Ok(result)
    }
}
fn byte_history(
    length: usize,
    field: (usize, usize),
    changed: bool,
) -> Vec<FlowCheckpointByteHistory> {
    let selected = if changed {
        FlowPayloadRelation::Derived
    } else {
        FlowPayloadRelation::Copied
    };
    [
        (0, field.0, FlowPayloadRelation::Copied),
        (field.0, field.1, selected),
        (field.1, length, FlowPayloadRelation::Copied),
    ]
    .into_iter()
    .filter(|(start, end, _)| start < end)
    .map(|(start, end, relation)| FlowCheckpointByteHistory {
        source_interval: (start, end),
        result_interval: (start, end),
        relation,
    })
    .collect()
}
impl FlowCheckpointComponentEdit {
    /// Borrows the exact input checkpoint; e.g. `let original = edit.source().as_bytes()`.
    pub fn source(&self) -> &FlowCheckpointComponent {
        &self.source
    }
    /// Borrows the completely revalidated result; e.g. `let output = edit.result().as_bytes()`.
    pub fn result(&self) -> &FlowCheckpointComponent {
        &self.result
    }
    /// Borrows the exact input field identity; e.g. `let parent = edit.source_field()`.
    pub fn source_field(&self) -> &FlowCheckpointField {
        &self.source_field
    }
    /// Borrows the corresponding output field; e.g. `let undo = edit.result().replace_field(edit.result_field(), old)?`.
    pub fn result_field(&self) -> &FlowCheckpointField {
        &self.result_field
    }
    /// Borrows a complete ordered partition of source/result byte history; e.g. `let parents = edit.history()`.
    pub fn history(&self) -> &[FlowCheckpointByteHistory] {
        &self.history
    }
}
