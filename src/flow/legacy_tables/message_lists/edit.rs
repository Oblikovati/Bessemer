use super::*;
impl FlowMessageLists {
    /// Replaces one raw member on a copy; e.g. `let edit = messages.replace_member(0, 0x80000000)?`.
    pub fn replace_member(&self, ordinal: usize, pattern: u32) -> Result<FlowMessageListEdit> {
        let item = self
            .item(ordinal)
            .ok_or_else(|| table_index_error(301, "item", ordinal, self.items.len()))?;
        self.edit_offset(item.payload_interval().0, &pattern.to_le_bytes())
    }
    /// Replaces one legacy byte without interpreting text; e.g. `let edit = messages.replace_text_byte(&id, 0, 255)?`.
    pub fn replace_text_byte(
        &self,
        id: &FlowMessageTextId,
        index: usize,
        value: u8,
    ) -> Result<FlowMessageListEdit> {
        let offset = self.text_offset(id, index, FlowMessageTextEncoding::LegacyBytes)?;
        self.edit_offset(offset, &[value])
    }
    /// Replaces a UTF16 unit and revalidates the terminal NUL; e.g. `let edit = messages.replace_text_unit(&id, 0, 65)?`.
    pub fn replace_text_unit(
        &self,
        id: &FlowMessageTextId,
        index: usize,
        value: u16,
    ) -> Result<FlowMessageListEdit> {
        let offset = self.text_offset(id, index, FlowMessageTextEncoding::Utf16Units)?;
        self.edit_offset(offset, &value.to_le_bytes())
    }
    fn text_offset(
        &self,
        id: &FlowMessageTextId,
        index: usize,
        expected: FlowMessageTextEncoding,
    ) -> Result<usize> {
        let text = self.text(id)?;
        if text.encoding() != expected {
            return Err(message_invalid(format!(
                "text encoding {:?}; expected {expected:?}",
                text.encoding()
            )));
        }
        wire::element_offset(
            301,
            text.payload_interval().0 + 4,
            text.stored_count(),
            index,
            expected.element_width(),
        )
    }
    fn edit_offset<const N: usize>(
        &self,
        offset: usize,
        bytes: &[u8; N],
    ) -> Result<FlowMessageListEdit> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + N].copy_from_slice(bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_301_message_lists(&id, self.options)?;
        self.check_layout(&result)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowMessageListEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn check_layout(&self, result: &Self) -> Result<()> {
        let same_texts = self.texts.len() == result.texts.len()
            && self
                .texts
                .iter()
                .zip(result.texts.iter())
                .all(|(before, after)| {
                    before.role == after.role
                        && before.interval == after.interval
                        && before.count == after.count
                });
        if self.items == result.items && self.encoding == result.encoding && same_texts {
            return Ok(());
        }
        Err(message_invalid("edited message layout changed; expected identical item/list/string intervals and counts".to_owned()))
    }
}
