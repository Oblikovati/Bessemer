use super::*;
impl FlowTextSlots {
    /// Replaces the arbitrary signed header on a copy; e.g. `let edit = slots.replace_header(-7)?`.
    pub fn replace_header(&self, value: i32) -> Result<FlowSimpleTableEdit<Self>> {
        self.patch(0, &value.to_le_bytes())
    }
    /// Replaces an existing unit, revalidating terminal NUL and native prefixes; e.g. `let edit = slots.replace_unit(0, 0, 65)?`.
    pub fn replace_unit(
        &self,
        ordinal: usize,
        index: usize,
        value: u16,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let slot = self.slot(ordinal)?;
        let offset = wire::element_offset(311, slot.span.start + 4, slot.stored_units(), index, 2)?;
        self.patch(offset, &value.to_le_bytes())
    }
    fn patch<const N: usize>(
        &self,
        offset: usize,
        replacement: &[u8; N],
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + N].copy_from_slice(replacement);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_311_text_slots(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
