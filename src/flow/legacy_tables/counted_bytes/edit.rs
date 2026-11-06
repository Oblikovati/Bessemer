use super::*;
impl FlowCountedBytes {
    /// Replaces one raw byte on a copy with complete history; e.g. `let edit = values.replace_byte(0, 255)?`.
    pub fn replace_byte(&self, index: usize, value: u8) -> Result<FlowSimpleTableEdit<Self>> {
        let offset = wire::element_offset(12, 4, self.stored_count(), index, 1)?;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset] = value;
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_12_counted_bytes(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
