use super::*;
impl FlowFixedState {
    /// Replaces an existing raw leading member on a copy; e.g. `let edit = state.replace_wide_member(0, FlowFixedStateWideMember::Member600, 7)?`.
    pub fn replace_wide_member(
        &self,
        ordinal: usize,
        member: FlowFixedStateWideMember,
        bits: u64,
    ) -> Result<FlowFixedStateEdit> {
        let row = self.existing_row(ordinal)?;
        self.edit_offset(
            row.interval.0 + member.relative_offset(),
            &bits.to_le_bytes(),
        )
    }
    /// Replaces one existing same-kind settings slot; e.g. `let edit = state.replace_setting(0, FlowFixedSettingsKind::Integer, 1, FlowFixedSlot::Integer(0xffffffff))?`.
    pub fn replace_setting(
        &self,
        ordinal: usize,
        kind: FlowFixedSettingsKind,
        index: usize,
        value: FlowFixedSlot,
    ) -> Result<FlowFixedStateEdit> {
        let row = self.existing_row(ordinal)?;
        let bytes = super::super::super::fixed_settings::replacement_bytes(kind, value)
            .map_err(|error| table_boundary(error, 139))?;
        let (start, _) = row
            .settings_interval(kind)
            .ok_or_else(|| missing_settings(row, kind))?;
        let offset = wire::element_offset(139, start, kind.slot_count(), index, bytes.len())?;
        self.edit_offset(offset, &bytes)
    }
    /// Replaces a physically present final signed member; e.g. `let edit = state.replace_final_member(0, -7)?`.
    pub fn replace_final_member(&self, ordinal: usize, value: i32) -> Result<FlowFixedStateEdit> {
        let row = self.existing_row(ordinal)?;
        if row.final_member().is_none() {
            return Err(table_failure(
                139,
                FailureClass::Missing,
                format!(
                    "row {ordinal} version {} final member None; expected a physically stored signed member in version three or four",
                    self.version()
                ),
            ));
        }
        self.edit_offset(row.interval.1 - 4, &value.to_le_bytes())
    }
    fn existing_row(&self, ordinal: usize) -> Result<FlowFixedStateRow<'_>> {
        self.row(ordinal)
            .ok_or_else(|| table_index_error(139, "row", ordinal, self.row_count()))
    }
    fn edit_offset(&self, offset: usize, bytes: &[u8]) -> Result<FlowFixedStateEdit> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_139_fixed_state(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowFixedStateEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
fn missing_settings(row: FlowFixedStateRow<'_>, kind: FlowFixedSettingsKind) -> BessemerError {
    table_failure(
        139,
        FailureClass::Missing,
        format!(
            "row {} version {} settings {kind:?} None; expected physically stored settings in versions two through four",
            row.ordinal,
            row.owner.version()
        ),
    )
}
