use super::*;

impl FlowMetadataPrefix {
    /// Replaces a version only when every prefix boundary remains unchanged; e.g. `let edit = prefix.replace_version_byte(3)?`.
    pub fn replace_version_byte(&self, version: u8) -> Result<FlowMetadataPrefixEdit> {
        self.patch(0, &[version])
    }
    /// Replaces one existing signed vector value; e.g. `let edit = prefix.replace_vector_value(0, 0, -1)?`.
    pub fn replace_vector_value(
        &self,
        vector: usize,
        index: usize,
        value: i32,
    ) -> Result<FlowMetadataPrefixEdit> {
        let span = self
            .vectors
            .get(vector)
            .ok_or_else(|| table_index_error(424, "vector", vector, self.vectors.len()))?;
        if index >= span.count {
            return Err(table_index_error(424, "vector value", index, span.count));
        }
        self.patch(span.start + 4 + index * 4, &value.to_le_bytes())
    }
    /// Replaces every stored object-state bit; e.g. `let edit = prefix.replace_object_state(0, 255)?`.
    pub fn replace_object_state(&self, object: usize, state: u8) -> Result<FlowMetadataPrefixEdit> {
        self.patch(self.required_object(object)?.state, &[state])
    }
    /// Replaces a signed member-C4 word; e.g. `let edit = prefix.replace_member_c4(0, -1)?`.
    pub fn replace_member_c4(&self, object: usize, value: i32) -> Result<FlowMetadataPrefixEdit> {
        self.patch(
            self.required_object(object)?.state + 1,
            &value.to_le_bytes(),
        )
    }
    /// Replaces one stored UTF-16 code unit while preserving string framing; e.g. `let edit = prefix.replace_string_unit(0, FlowMetadataStringRole::First, 0, 0xd800)?`.
    pub fn replace_string_unit(
        &self,
        object: usize,
        role: FlowMetadataStringRole,
        index: usize,
        unit: u16,
    ) -> Result<FlowMetadataPrefixEdit> {
        let strings = &self.required_object(object)?.strings;
        let span = strings[items::role_index(role)].as_ref().ok_or_else(|| {
            metadata_invalid(format!(
                "object {object} string {role:?}; expected a stored string"
            ))
        })?;
        let units = (span.end - span.start - 4) / 2;
        if index >= units {
            return Err(table_index_error(424, "UTF-16 unit", index, units));
        }
        self.patch(span.start + 4 + index * 2, &unit.to_le_bytes())
    }
    fn required_object(&self, ordinal: usize) -> Result<&ObjectSpan> {
        self.objects
            .get(ordinal)
            .ok_or_else(|| table_index_error(424, "object", ordinal, self.objects.len()))
    }
    pub(super) fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowMetadataPrefixEdit> {
        let mut raw = self.raw.to_vec();
        raw[offset..offset + bytes.len()].copy_from_slice(bytes);
        let carrier_edit = self.source.replace_payload(&self.record, &raw)?;
        let id = self.edited_id(carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_424_metadata_prefix(&id, self.options)?;
        self.check_layout(&result, &raw)?;
        Ok(FlowMetadataPrefixEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn edited_id(&self, result: &FlowCarrier) -> Result<FlowRecordId> {
        result
            .records()
            .nth(self.record.ordinal())
            .map(|record| record.id)
            .ok_or_else(|| {
                table_index_error(
                    424,
                    "result occurrence",
                    self.record.ordinal(),
                    result.records().len(),
                )
            })
    }
    fn check_layout(&self, result: &Self, raw: &[u8]) -> Result<()> {
        let object_layout = self
            .objects
            .iter()
            .zip(result.objects.iter())
            .all(|(before, after)| same_object_layout(before, after));
        if self.end == result.end
            && self.vectors == result.vectors
            && self.objects.len() == result.objects.len()
            && object_layout
            && result.raw.as_ref() == raw
        {
            return Ok(());
        }
        Err(layout_error(self, result))
    }
}

fn same_object_layout(before: &ObjectSpan, after: &ObjectSpan) -> bool {
    before.start == after.start
        && before.end == after.end
        && before.state == after.state
        && before
            .strings
            .iter()
            .zip(after.strings.iter())
            .all(|(left, right)| {
                left.as_ref().map(|span| (span.start, span.end))
                    == right.as_ref().map(|span| (span.start, span.end))
            })
}

fn layout_error(before: &FlowMetadataPrefix, after: &FlowMetadataPrefix) -> BessemerError {
    metadata_invalid(format!(
        "edited prefix ends at {} with {} objects; expected identical boundary {} and {} object layouts",
        after.end,
        after.objects.len(),
        before.end,
        before.objects.len()
    ))
}
