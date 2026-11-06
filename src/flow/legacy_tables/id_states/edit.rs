use super::*;

impl FlowIdStates {
    /// Replaces the raw header without introducing version dispatch; e.g. `let edit = states.replace_header_word(u32::MAX)?`.
    pub fn replace_header_word(&self, word: u32) -> Result<FlowIdStateEdit> {
        self.patch(0, &word.to_le_bytes())
    }
    /// Replaces an existing signed identifier while preserving row order; e.g. `let edit = states.replace_identifier(0, -1)?`.
    pub fn replace_identifier(&self, ordinal: usize, identifier: i32) -> Result<FlowIdStateEdit> {
        let entry = self.required_entry(ordinal)?;
        self.patch(entry.payload_interval().0, &identifier.to_le_bytes())
    }
    /// Replaces an existing complete state byte; e.g. `let edit = states.replace_state(0, 255)?`.
    pub fn replace_state(&self, ordinal: usize, state: u8) -> Result<FlowIdStateEdit> {
        let entry = self.required_entry(ordinal)?;
        self.patch(entry.payload_interval().0 + 4, &[state])
    }
    fn required_entry(&self, ordinal: usize) -> Result<FlowIdStateEntry<'_>> {
        self.entry(ordinal)
            .ok_or_else(|| table_index_error(410, "state row", ordinal, self.count))
    }
    fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowIdStateEdit> {
        let mut raw = self.wire.decoded.to_vec();
        raw[offset..offset + bytes.len()].copy_from_slice(bytes);
        let carrier_edit = wire::edited(&self.wire, &raw)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit.result().record_410_states(&id, self.limits)?;
        if self.count != result.count {
            return Err(state_invalid(format!(
                "edit at {offset} changed row count to {}; expected {}",
                result.count, self.count
            )));
        }
        wire::check_decoded(&result.wire, &raw)?;
        Ok(FlowIdStateEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
