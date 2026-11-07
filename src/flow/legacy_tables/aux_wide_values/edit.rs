use super::*;
impl FlowAuxWideValues {
    /// Replaces an existing raw pattern on a copy; e.g. `let edit = values.replace_pattern(0, u64::MAX)?`.
    pub fn replace_pattern(&self, index: usize, bits: u64) -> Result<FlowSimpleTableEdit<Self>> {
        let offset = wire::element_offset(444, self.start, self.stored_count(), index, 8)?;
        self.patch(offset, &bits.to_le_bytes())
    }
    /// Replaces the physically stored version-zero discarded word; e.g. `let edit = values.replace_discarded_legacy(-7)?`.
    pub fn replace_discarded_legacy(&self, value: i32) -> Result<FlowSimpleTableEdit<Self>> {
        if self.discarded_legacy().is_none() {
            return Err(table_failure(
                444,
                FailureClass::Missing,
                format!(
                    "version {}; expected version zero with a physically stored discarded word",
                    self.version()
                ),
            ));
        }
        self.patch(4, &value.to_le_bytes())
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
            .record_444_aux_wides(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
