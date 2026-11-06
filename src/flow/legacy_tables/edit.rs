mod select;
use super::*;

impl FlowPointerTables {
    /// Replaces one existing field without changing structure; e.g. `let edit = table.replace_field(FlowPointerTableField::Member { group: 0, component: 1 }, FlowPointerTableValue::Signed32(-1))?`.
    pub fn replace_field(
        &self,
        field: FlowPointerTableField,
        value: FlowPointerTableValue,
    ) -> Result<FlowPointerTableEdit> {
        let (offset, wide) = select::offset(self, field)?;
        let bytes = field_bytes(field, value, wide)?;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_401_pointer_table(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowPointerTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

impl FlowLegacyWords {
    /// Replaces an existing raw word on an isolated copy; e.g. `let edit = words.replace_word(0, 0xffffffff)?`.
    pub fn replace_word(&self, index: usize, word: u32) -> Result<FlowLegacyWordEdit> {
        if index >= self.count {
            return Err(table_index_error(405, "word", index, self.count));
        }
        let mut decoded = self.wire.decoded.to_vec();
        let offset = self.words_start + index * 4;
        decoded[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit.result().record_405_words(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowLegacyWordEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn field_bytes(
    field: FlowPointerTableField,
    value: FlowPointerTableValue,
    wide: bool,
) -> Result<Vec<u8>> {
    match (wide, value) {
        (true, FlowPointerTableValue::WideBits(bits)) => Ok(bits.to_le_bytes().to_vec()),
        (false, FlowPointerTableValue::Signed32(word)) => Ok(word.to_le_bytes().to_vec()),
        _ => Err(table_failure(
            401,
            FailureClass::Invalid,
            format!(
                "field {field:?} value {value:?}; expected {}",
                if wide {
                    "WideBits(u64)"
                } else {
                    "Signed32(i32)"
                }
            ),
        )),
    }
}
