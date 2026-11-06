use super::*;
impl FlowStoredState {
    /// Replaces an existing stored member on a copy; e.g. `let edit = state.replace_member(FlowStoredStateMember::Member5E0, -7)?`.
    pub fn replace_member(
        &self,
        member: FlowStoredStateMember,
        value: i32,
    ) -> Result<FlowStoredStateEdit> {
        let offset = self.stored_offset(member)?;
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_432_stored_state(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowStoredStateEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn stored_offset(&self, member: FlowStoredStateMember) -> Result<usize> {
        self.member(member)
            .map(|_| member.payload_offset())
            .ok_or_else(|| {
                table_failure(
                    432,
                    FailureClass::Missing,
                    format!(
                        "version {} member {member:?}; expected a physically stored signed member",
                        self.version()
                    ),
                )
            })
    }
}
