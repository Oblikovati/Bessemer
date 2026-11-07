use super::*;
impl FlowImsValues {
    /// Replaces an existing raw flag within its zero/nonzero presence class; e.g. `let edit = values.replace_flag(0, FlowImsValueFlag::Bl, 255)?`.
    pub fn replace_flag(
        &self,
        ordinal: usize,
        flag: FlowImsValueFlag,
        value: u8,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let object = self.object(ordinal)?;
        let before = object.flag(flag);
        if (before == 0) != (value == 0) {
            return Err(invalid(format!(
                "object {ordinal} flag {flag:?} replacement {value}, stored {before}; expected the same zero/nonzero presence class with unchanged body storage"
            )));
        }
        self.patch(object.flag_interval(flag).0, &[value])
    }
    /// Replaces an existing shared raw-wide pattern; e.g. `let edit = values.replace_shared_pattern(0, FlowImsValueArray::Main, 0, u64::MAX)?`.
    pub fn replace_shared_pattern(
        &self,
        ordinal: usize,
        kind: FlowImsValueArray,
        index: usize,
        bits: u64,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let object = self.object(ordinal)?;
        let array = object.array(kind).ok_or_else(|| {
            invalid(format!(
                "object {ordinal} array {kind:?}; expected a physically present shared array"
            ))
        })?;
        let offset = wire::element_offset(
            501,
            array.payload_interval().0,
            array.word_count(),
            index,
            8,
        )?;
        self.patch(offset, &bits.to_le_bytes())
    }
    /// Replaces one of the 18 raw patterns in an existing BL row; e.g. `let edit = values.replace_bl_pattern(0, 0, 17, u64::MAX)?`.
    pub fn replace_bl_pattern(
        &self,
        ordinal: usize,
        row: usize,
        column: usize,
        bits: u64,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let bl = self.object(ordinal)?.bl().ok_or_else(|| invalid(format!("object {ordinal} BL row {row}, column {column}; expected a physically present BL body")))?;
        let array = bl.row(row)?;
        let offset = wire::element_offset(501, array.payload_interval().0, 18, column, 8)?;
        self.patch(offset, &bits.to_le_bytes())
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
        let result =
            carrier_edit
                .result()
                .record_501_ims_values(&id, &self.context, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        self.check_layout(&result)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn check_layout(&self, result: &Self) -> Result<()> {
        if self.objects == result.objects
            && self.totals == result.totals
            && self.context == result.context
        {
            return Ok(());
        }
        Err(table_failure(
            501,
            FailureClass::Integrity,
            format!(
                "reopened {} value layouts; expected identical {} source layouts, totals and definition revision",
                result.objects.len(),
                self.objects.len()
            ),
        ))
    }
}
