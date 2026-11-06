use super::*;
impl FlowSelectedValues {
    /// Replaces one checked raw wide pattern on a copy; e.g. `let edit = values.replace_value(&id, 0x8000000000000000)?`.
    pub fn replace_value(
        &self,
        id: &FlowSelectedValueId,
        bits: u64,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let offset = self.checked_index(id)? * 8;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + 8].copy_from_slice(&bits.to_le_bytes());
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let result_id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit.result().record_112_selected_values(
            &result_id,
            &self.context,
            self.options,
        )?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
