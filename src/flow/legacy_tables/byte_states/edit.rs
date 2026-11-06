use super::*;
impl FlowByteStates {
    /// Replaces a raw signed header without dispatch; e.g. `let edit = states.replace_header(-1)?`.
    pub fn replace_header(&self, value: i32) -> Result<FlowSimpleTableEdit<Self>> {
        self.patch_offset(0, &value.to_le_bytes())
    }
    /// Replaces an existing byte on a copy; e.g. `let edit = states.replace_byte(0, 255)?`.
    pub fn replace_byte(&self, index: usize, value: u8) -> Result<FlowSimpleTableEdit<Self>> {
        let offset = wire::element_offset(316, 8, self.stored_count(), index, 1)?;
        self.patch_offset(offset, &[value])
    }
    fn patch_offset<const N: usize>(
        &self,
        offset: usize,
        bytes: &[u8; N],
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + N].copy_from_slice(bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_316_byte_states(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
