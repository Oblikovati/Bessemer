use super::*;
impl FlowPairedState {
    /// Replaces one existing raw index on a copy; e.g. `let edit = state.replace_index(0, 1, 0xffffffff)?`.
    pub fn replace_index(
        &self,
        group: usize,
        index: usize,
        bits: u32,
    ) -> Result<FlowPairedStateEdit> {
        let array = self.existing_group(group)?.indices();
        let offset = wire::element_offset(
            417,
            array.payload_interval().0,
            array.word_count(),
            index,
            4,
        )?;
        self.edit_offset(offset, &bits.to_le_bytes())
    }
    /// Replaces one existing wide pattern without floating conversion; e.g. `let edit = state.replace_wide(1, 0, 0x8000000000000000)?`.
    pub fn replace_wide(
        &self,
        group: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowPairedStateEdit> {
        let array = self.existing_group(group)?.wides().ok_or_else(|| {
            table_failure(417, FailureClass::Missing, format!("group {group} wide array None; expected a present wide array after a positive index count"))
        })?;
        let offset = wire::element_offset(
            417,
            array.payload_interval().0,
            array.word_count(),
            index,
            8,
        )?;
        self.edit_offset(offset, &bits.to_le_bytes())
    }
    fn existing_group(&self, ordinal: usize) -> Result<FlowPairedStateGroup<'_>> {
        self.group(ordinal)
            .ok_or_else(|| table_index_error(417, "group", ordinal, 2))
    }
    fn edit_offset<const N: usize>(
        &self,
        offset: usize,
        bytes: &[u8; N],
    ) -> Result<FlowPairedStateEdit> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + N].copy_from_slice(bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_417_paired_state(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowPairedStateEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
