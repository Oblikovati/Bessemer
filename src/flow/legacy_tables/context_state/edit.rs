use super::*;
impl FlowContextState {
    /// Replaces one physically stored raw word on a copy; e.g. `let edit = state.replace_index(0, 0xffffffff)?`.
    pub fn replace_index(&self, index: usize, bits: u32) -> Result<FlowContextStateEdit> {
        let count = (self.index_end - 8) / 4;
        if index >= count {
            return Err(table_index_error(138, "index word", index, count));
        }
        self.edit_offset(8 + index * 4, &bits.to_le_bytes())
    }
    /// Replaces an existing signed tail, retaining its presence; e.g. `let edit = state.replace_tail(-7)?`.
    pub fn replace_tail(&self, value: i32) -> Result<FlowContextStateEdit> {
        if self.tail().is_none() {
            return Err(table_failure(
                138,
                FailureClass::Missing,
                format!(
                    "version {} tail None; expected a physically stored signed tail in version two or three",
                    self.version()
                ),
            ));
        }
        self.edit_offset(self.index_end, &value.to_le_bytes())
    }
    fn edit_offset(&self, offset: usize, bytes: &[u8]) -> Result<FlowContextStateEdit> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + 4].copy_from_slice(bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_138_context_state(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowContextStateEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
