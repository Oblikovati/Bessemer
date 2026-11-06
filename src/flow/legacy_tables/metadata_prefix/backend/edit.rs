use super::*;
impl FlowMediumBackendPrefix {
    /// Replaces a count while preserving empty/populated framing; e.g. `let edit = backend.replace_declared_count(0, 2)?`.
    pub fn replace_declared_count(
        &self,
        ordinal: usize,
        count: i32,
    ) -> Result<FlowMediumBackendEdit> {
        let span = self.required_table(ordinal)?;
        if count < 0 || (count == 0) != (span.count == 0) {
            return Err(metadata_invalid(format!(
                "table {ordinal} replacement count {count}; expected nonnegative count retaining empty={} framing",
                span.count == 0
            )));
        }
        self.patch(span.start + 1, &count.to_le_bytes())
    }
    /// Replaces a populated kind only when its conditional key shape is preserved; e.g. `let edit = backend.replace_first_kind(1, -1)?`.
    pub fn replace_first_kind(&self, ordinal: usize, kind: i32) -> Result<FlowMediumBackendEdit> {
        let span = self.required_table(ordinal)?;
        if span.kind.is_none() || (ordinal == 0 && (kind == 1) != span.key) {
            return Err(metadata_invalid(format!(
                "table {ordinal} replacement kind {kind}; expected populated kind preserving key presence {}",
                span.key
            )));
        }
        self.patch(span.start + 5, &kind.to_le_bytes())
    }
    /// Replaces an existing sixteen-byte first-table key; e.g. `let edit = backend.replace_first_key(0, [0;16])?`.
    pub fn replace_first_key(
        &self,
        ordinal: usize,
        key: [u8; 16],
    ) -> Result<FlowMediumBackendEdit> {
        let span = self.required_table(ordinal)?;
        if !span.key {
            return Err(metadata_invalid(format!(
                "table {ordinal}; expected existing first-table kind-one key"
            )));
        }
        self.patch(span.start + 9, &key)
    }
    fn required_table(&self, ordinal: usize) -> Result<&BackendTableSpan> {
        self.tables.get(ordinal).ok_or_else(|| {
            table_index_error(424, "reached backend table", ordinal, self.tables.len())
        })
    }
    fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowMediumBackendEdit> {
        let prefix_edit = self.prefix.patch(self.prefix.end + offset, bytes)?;
        let result = prefix_edit.result().medium_backend_prefix()?;
        self.check_layout(&result)?;
        Ok(FlowMediumBackendEdit {
            source: self.clone(),
            result,
            prefix_edit,
        })
    }
    fn check_layout(&self, after: &Self) -> Result<()> {
        let same = self.tables.len() == after.tables.len()
            && self
                .tables
                .iter()
                .zip(after.tables.iter())
                .all(|(left, right)| same_table_layout(left, right));
        if same && self.end == after.end && self.stop() == after.stop() {
            return Ok(());
        }
        Err(metadata_invalid(format!(
            "backend edit ends at {} with {} tables; expected identical boundary {} and {} table layouts",
            after.end,
            after.tables.len(),
            self.end,
            self.tables.len()
        )))
    }
}

fn same_table_layout(left: &BackendTableSpan, right: &BackendTableSpan) -> bool {
    left.start == right.start
        && left.end == right.end
        && left.key == right.key
        && left.kind.is_some() == right.kind.is_some()
}
