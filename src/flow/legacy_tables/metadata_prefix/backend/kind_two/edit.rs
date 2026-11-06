use super::*;
impl FlowMediumKindTwo {
    /// Replaces the raw base byte without version dispatch; e.g. `let edit=body.replace_base_byte(255)?`.
    pub fn replace_base_byte(&self, value: u8) -> Result<FlowMediumKindTwoEdit> {
        self.patch(self.span.start + 1, &[value])
    }
    /// Replaces the sixteen stored body-key bytes; e.g. `let edit=body.replace_base_key([0;16])?`.
    pub fn replace_base_key(&self, value: [u8; 16]) -> Result<FlowMediumKindTwoEdit> {
        self.patch(self.span.start + 2, &value)
    }
    /// Replaces every stored final state bit; e.g. `let edit=body.replace_state_byte(0)?`.
    pub fn replace_state_byte(&self, value: u8) -> Result<FlowMediumKindTwoEdit> {
        self.patch(self.span.state, &[value])
    }
    /// Replaces one existing unnormalized wide pattern; e.g. `let edit=body.replace_wide_word(FlowMediumKindTwoWideField::BodyScalar,0,1)?`.
    pub fn replace_wide_word(
        &self,
        field: FlowMediumKindTwoWideField,
        index: usize,
        bits: u64,
    ) -> Result<FlowMediumKindTwoEdit> {
        let (start, end) = self.wide_span(field)?;
        require_index(index, (end - start) / 8, "kind-two wide pattern")?;
        self.patch(start + index * 8, &bits.to_le_bytes())
    }
    /// Replaces a raw curve flag while preserving its zero/nonzero branch; e.g. `let edit=body.replace_curve_flag(FlowMediumCurveScope::Model,0,255)?`.
    pub fn replace_curve_flag(
        &self,
        scope: FlowMediumCurveScope,
        ordinal: usize,
        flag: u8,
    ) -> Result<FlowMediumKindTwoEdit> {
        let curve = self.required_curve(scope, ordinal)?;
        if (self.raw()[curve.flag] == 0) != (flag == 0) {
            return Err(metadata_invalid(format!(
                "{scope:?} curve {ordinal} replacement flag {flag}; expected unchanged extra-array presence {}",
                self.raw()[curve.flag] != 0
            )));
        }
        self.patch(curve.flag, &[flag])
    }
    /// Replaces one raw name unit while preserving its count and terminal NUL; e.g. `let edit=body.replace_name_unit(0,0xd800)?`.
    pub fn replace_name_unit(&self, index: usize, unit: u16) -> Result<FlowMediumKindTwoEdit> {
        require_index(index, self.name().stored_units(), "kind-two name unit")?;
        self.patch(
            self.span.prefix.name.start + 4 + index * 2,
            &unit.to_le_bytes(),
        )
    }
    /// Replaces an existing model state byte; e.g. `let edit=body.replace_model_state(0,128)?`.
    pub fn replace_model_state(&self, index: usize, state: u8) -> Result<FlowMediumKindTwoEdit> {
        let (start, end) = self.model_states_span();
        require_index(index, end - start, "model state byte")?;
        self.patch(start + index, &[state])
    }
    /// Replaces one raw model-five 32-bit word; e.g. `let edit=body.replace_model_word(0,u32::MAX)?`.
    pub fn replace_model_word(&self, index: usize, word: u32) -> Result<FlowMediumKindTwoEdit> {
        let (start, end) = self.table()?.words;
        require_index(index, (end - start) / 4, "model word pattern")?;
        self.patch(start + index * 4, &word.to_le_bytes())
    }
    /// Replaces the signed model-five member word; e.g. `let edit=body.replace_model_member_8c(i32::MIN)?`.
    pub fn replace_model_member_8c(&self, value: i32) -> Result<FlowMediumKindTwoEdit> {
        self.patch(self.table()?.member, &value.to_le_bytes())
    }
    fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowMediumKindTwoEdit> {
        let prefix_edit = self.backend.metadata_prefix().patch(offset, bytes)?;
        let backend = prefix_edit.result().medium_backend_prefix()?;
        let result = decode::read_at(&backend, self.span.start, self.options)
            .map_err(|error| table_boundary(error, 424))?;
        self.check_layout(&result)?;
        Ok(FlowMediumKindTwoEdit {
            source: self.clone(),
            result,
            prefix_edit,
        })
    }
    fn check_layout(&self, after: &Self) -> Result<()> {
        let a = &self.span;
        let b = &after.span;
        if a.start == b.start
            && a.end == b.end
            && a.scalar == b.scalar
            && a.model == b.model
            && a.state == b.state
            && a.prefix.name.start == b.prefix.name.start
            && a.prefix.name.end == b.prefix.name.end
            && a.prefix.curves == b.prefix.curves
            && a.prefix.arrays == b.prefix.arrays
        {
            return Ok(());
        }
        Err(metadata_invalid(format!(
            "edited kind-two body/model ends at {}/{}; expected unchanged {}/{} and all field layouts",
            b.end, b.model.end, a.end, a.model.end
        )))
    }
}
fn require_index(index: usize, count: usize, label: &str) -> Result<()> {
    if index >= count {
        return Err(table_index_error(424, label, index, count));
    }
    Ok(())
}
