use super::*;
impl FlowObjectRecords {
    /// Replaces a signed field on a copy; e.g. `let edit = objects.replace_signed(&id, -7)?`.
    pub fn replace_signed(
        &self,
        id: &FlowObjectFieldId,
        value: i32,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        self.check_field(id, 4)?;
        self.replace_scalar(id.offset, &value.to_le_bytes())
    }
    /// Replaces an opaque or state byte on a copy; e.g. `let edit = objects.replace_byte(&id, 255)?`.
    pub fn replace_byte(
        &self,
        id: &FlowObjectFieldId,
        value: u8,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        self.check_field(id, 1)?;
        self.replace_scalar(id.offset, &[value])
    }
    fn replace_scalar(
        &self,
        offset: usize,
        replacement: &[u8],
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + replacement.len()].copy_from_slice(replacement);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_304_objects(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
