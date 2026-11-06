use super::*;
impl FlowMediumKindOne {
    /// Replaces the uninterpreted base byte; e.g. `let edit = body.replace_base_byte(255)?`.
    pub fn replace_base_byte(&self, value: u8) -> Result<FlowMediumBodyEdit> {
        self.patch(self.span.start + 1, &[value])
    }
    /// Replaces all sixteen existing base-key bytes; e.g. `let edit = body.replace_base_key([0;16])?`.
    pub fn replace_base_key(&self, value: [u8; 16]) -> Result<FlowMediumBodyEdit> {
        self.patch(self.span.start + 2, &value)
    }
    /// Replaces a complete stored state byte without applying native load-time replacement; e.g. `let edit = body.replace_state_byte(0)?`.
    pub fn replace_state_byte(&self, value: u8) -> Result<FlowMediumBodyEdit> {
        self.patch(self.span.state, &[value])
    }
    /// Replaces one existing raw wide pattern; e.g. `let edit = body.replace_wide_word(FlowMediumWideField::FinalScalars,0,1)?`.
    pub fn replace_wide_word(
        &self,
        field: FlowMediumWideField,
        index: usize,
        bits: u64,
    ) -> Result<FlowMediumBodyEdit> {
        let (start, end) = self.wide_span(field)?;
        let count = (end - start) / 8;
        if index >= count {
            return Err(table_index_error(424, "medium wide word", index, count));
        }
        self.patch(start + index * 8, &bits.to_le_bytes())
    }
    /// Replaces an extra-array flag without changing its presence branch; e.g. `let edit = body.replace_curve_flag(0,255)?`.
    pub fn replace_curve_flag(&self, ordinal: usize, flag: u8) -> Result<FlowMediumBodyEdit> {
        let curve = self.required_curve(ordinal)?;
        if (self.raw()[curve.flag] == 0) != (flag == 0) {
            return Err(metadata_invalid(format!(
                "curve {ordinal} replacement flag {flag}; expected unchanged extra-array presence {}",
                self.raw()[curve.flag] != 0
            )));
        }
        self.patch(curve.flag, &[flag])
    }
    /// Replaces a stored UTF-16 unit while preserving count and terminator; e.g. `let edit = body.replace_name_unit(0,0xd800)?`.
    pub fn replace_name_unit(&self, index: usize, unit: u16) -> Result<FlowMediumBodyEdit> {
        let count = self.name().stored_units();
        if index >= count {
            return Err(table_index_error(424, "medium name unit", index, count));
        }
        self.patch(self.span.name.start + 4 + index * 2, &unit.to_le_bytes())
    }
    fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowMediumBodyEdit> {
        let prefix_edit = self.backend.metadata_prefix().patch(offset, bytes)?;
        let result = prefix_edit
            .result()
            .medium_backend_prefix()?
            .first_kind_one_body(self.options)?;
        self.check_layout(&result)?;
        Ok(FlowMediumBodyEdit {
            source: self.clone(),
            result,
            prefix_edit,
        })
    }
    fn check_layout(&self, after: &Self) -> Result<()> {
        let before = &self.span;
        let result = &after.span;
        if before.start == result.start
            && before.end == result.end
            && before.curves == result.curves
            && before.arrays == result.arrays
            && before.scalars == result.scalars
            && before.state == result.state
            && before.name.start == result.name.start
            && before.name.end == result.name.end
        {
            return Ok(());
        }
        Err(metadata_invalid(format!(
            "edited kind-one body ends at {} with name interval {:?}; expected unchanged boundary {} and {:?}",
            result.end,
            (result.name.start, result.name.end),
            before.end,
            (before.name.start, before.name.end)
        )))
    }
}
