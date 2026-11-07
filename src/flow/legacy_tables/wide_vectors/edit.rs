use super::*;
impl FlowWideVectors {
    /// Replaces one existing raw pattern on a copy; e.g. `let edit = vectors.replace_pattern(0, 0, u64::MAX)?`.
    pub fn replace_pattern(
        &self,
        vector: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let array = self.vector(vector)?;
        let offset = wire::element_offset(
            148,
            array.payload_interval().0,
            array.word_count(),
            index,
            8,
        )?;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + 8].copy_from_slice(&bits.to_le_bytes());
        self.reopen(&decoded)
    }
    fn reopen(&self, decoded: &[u8]) -> Result<FlowSimpleTableEdit<Self>> {
        let carrier_edit = wire::edited(&self.wire, decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_148_wide_vectors(&id, self.options)?;
        wire::check_decoded(&result.wire, decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
