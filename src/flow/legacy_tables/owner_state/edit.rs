use super::*;
impl FlowOwnerState {
    /// Replaces one existing raw word, including the legacy gate, on a copy; e.g. `let edit = state.replace_word(0, u32::MAX)?`.
    pub fn replace_word(&self, index: usize, bits: u32) -> Result<FlowSimpleTableEdit<Self>> {
        let offset = wire::element_offset(
            self.kind.identifier(),
            self.kind.words_start(),
            self.kind.word_count(),
            index,
            4,
        )?;
        self.patch(offset, &bits.to_le_bytes())
    }
    /// Replaces one existing signed modern member; e.g. `let edit = state.replace_modern_member(0, -7)?`.
    pub fn replace_modern_member(
        &self,
        index: usize,
        value: i32,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        self.require_modern("signed member")?;
        self.replace_word(index, value as u32)
    }
    /// Replaces the existing unsigned modern byte; e.g. `let edit = state.replace_state_byte(255)?`.
    pub fn replace_state_byte(&self, value: u8) -> Result<FlowSimpleTableEdit<Self>> {
        self.require_modern("state byte")?;
        self.patch(28, &[value])
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
        let result = read::read(carrier_edit.result(), &id, self.kind, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
