use super::*;
impl FlowTaggedState {
    /// Replaces a tag while preserving body presence; e.g. `let edit = state.replace_tag(0, 255)?`.
    pub fn replace_tag(&self, ordinal: usize, tag: u8) -> Result<FlowTaggedStateEdit> {
        let row = self.existing_row(ordinal)?;
        if (row.tag() == 0) != (tag == 0) {
            return Err(tagged_invalid(format!(
                "row {ordinal} tag {} to {tag}; expected both tags zero or both nonzero to preserve body presence",
                row.tag()
            )));
        }
        self.edit_offset(row.tag_offset(), &[tag])
    }
    /// Replaces an existing raw wide value on a copy; e.g. `let edit = state.replace_wide(0, 1, 0x8000000000000000)?`.
    pub fn replace_wide(
        &self,
        ordinal: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowTaggedStateEdit> {
        let row = self.existing_row(ordinal)?;
        let array = row.values().ok_or_else(|| table_failure(129, FailureClass::Missing, format!("row {ordinal} tag {} wide array None; expected a present array after a nonzero tag", row.tag())))?;
        let offset = wire::element_offset(
            129,
            array.payload_interval().0,
            array.word_count(),
            index,
            8,
        )?;
        self.edit_offset(offset, &bits.to_le_bytes())
    }
    /// Replaces a signed tail only when rows exist; e.g. `let edit = state.replace_tail(-7)?`.
    pub fn replace_tail(&self, value: i32) -> Result<FlowTaggedStateEdit> {
        if self.tail().is_none() {
            return Err(table_failure(129, FailureClass::Missing, "row count 0 tail None; expected a physically stored signed tail after a positive row count".to_owned()));
        }
        self.edit_offset(self.wire.decoded.len() - 4, &value.to_le_bytes())
    }
    fn existing_row(&self, ordinal: usize) -> Result<FlowTaggedStateRow<'_>> {
        self.row(ordinal)
            .ok_or_else(|| table_index_error(129, "row", ordinal, self.rows.len()))
    }
    fn edit_offset<const N: usize>(
        &self,
        offset: usize,
        bytes: &[u8; N],
    ) -> Result<FlowTaggedStateEdit> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + N].copy_from_slice(bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_129_tagged_state(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowTaggedStateEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
