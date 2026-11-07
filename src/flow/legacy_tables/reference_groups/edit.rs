use super::*;

impl FlowReferenceGroups {
    /// Replaces a stored header without changing the fixed group loop; e.g. `let edit = references.replace_header(FlowReferenceHeaderField::Stored, 99)?`.
    pub fn replace_header(
        &self,
        field: FlowReferenceHeaderField,
        value: i32,
    ) -> Result<FlowReferenceGroupEdit> {
        let offset = self.headers[selection::header_index(field)]
            .ok_or_else(|| reference_missing(format!("header {field:?}")))?;
        self.patch(offset, &value.to_le_bytes())
    }
    /// Replaces one existing typed entry field, retaining every parsed span; e.g. `let edit = references.replace_entry(0, 0, FlowReferenceEntryField::State, FlowBlockFieldValue::Byte(255))?`.
    pub fn replace_entry(
        &self,
        group: usize,
        ordinal: usize,
        field: FlowReferenceEntryField,
        value: FlowBlockFieldValue,
    ) -> Result<FlowReferenceGroupEdit> {
        let group = self.required_group(group)?;
        let entry = group
            .entry(ordinal)
            .ok_or_else(|| table_index_error(416, "entry", ordinal, group.entry_count()))?;
        let selected = selection::entry_field(entry, field)?;
        let bytes = encode(selected.kind, value).map_err(|error| table_boundary(error, 416))?;
        self.patch(selected.offset, &bytes)
    }
    /// Replaces a stored refinement level without applying runtime shifts; e.g. `let edit = references.replace_refinement_level(0, -3)?`.
    pub fn replace_refinement_level(
        &self,
        group: usize,
        value: i32,
    ) -> Result<FlowReferenceGroupEdit> {
        let group = self.required_group(group)?;
        let offset = group.span.level.ok_or_else(|| {
            reference_missing(format!("group {} refinement level", group.ordinal()))
        })?;
        self.patch(offset, &value.to_le_bytes())
    }
    /// Replaces an existing mapping word without pointer repair; e.g. `let edit = references.replace_mapping_word(0, 1, -1)?`.
    pub fn replace_mapping_word(
        &self,
        group: usize,
        index: usize,
        value: i32,
    ) -> Result<FlowReferenceGroupEdit> {
        let group = self.required_group(group)?;
        group
            .mapping_words()
            .ok_or_else(|| reference_missing(format!("group {} mapping words", group.ordinal())))?;
        let selected = selection::array(group.span.mapping, index, ScalarKind::Signed32)?;
        self.patch(selected.offset, &value.to_le_bytes())
    }
    /// Replaces an existing appended raw wide pattern; e.g. `let edit = references.replace_wide_word(1, 0, 0x8000000000000000)?`.
    pub fn replace_wide_word(
        &self,
        group: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowReferenceGroupEdit> {
        let group = self.required_group(group)?;
        let words = group
            .wide_words()
            .ok_or_else(|| reference_missing(format!("group {} wide array", group.ordinal())))?;
        if index >= words.word_count() {
            return Err(table_index_error(
                416,
                "wide word",
                index,
                words.word_count(),
            ));
        }
        self.patch(words.payload_interval().0 + index * 8, &bits.to_le_bytes())
    }
    fn required_group(&self, group: usize) -> Result<FlowReferenceGroup<'_>> {
        self.group(group)
            .ok_or_else(|| table_index_error(416, "group", group, 2))
    }
    fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowReferenceGroupEdit> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_416_references(&id, self.options)?;
        self.check_structure(&result, offset)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowReferenceGroupEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn check_structure(&self, result: &Self, offset: usize) -> Result<()> {
        if self.headers == result.headers
            && self.groups == result.groups
            && self.entries == result.entries
        {
            return Ok(());
        }
        Err(reference_invalid(format!(
            "edit at {offset} changed parsed reference-group shape; expected identical header, group, entry and array spans"
        )))
    }
}
