use super::*;
impl FlowResultDefinitions {
    /// Replaces one existing signed identifier on an isolated copy; e.g. `let edit = definitions.replace_identifier(0, -7)?`.
    pub fn replace_identifier(
        &self,
        ordinal: usize,
        value: i32,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let offset = self.definition(ordinal)?.identifier_interval().0;
        self.patch(offset, &value.to_le_bytes())
    }
    /// Replaces a physically present allocation byte; e.g. `let edit = definitions.replace_allocation_byte(0, 255)?`.
    pub fn replace_allocation_byte(
        &self,
        ordinal: usize,
        value: u8,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let object = self.definition(ordinal)?;
        let offset = object.span.allocation.ok_or_else(|| invalid(format!("definition {ordinal} version {:?}, replacement allocation byte {value}; expected a physically present version-two/three allocation byte", object.version())))?;
        self.patch(offset, &[value])
    }
    /// Replaces one raw shared-array pattern without changing counts or presence; e.g. `let edit = definitions.replace_pattern(0, FlowImsArrayKind::First, 0, u32::MAX)?`.
    pub fn replace_pattern(
        &self,
        ordinal: usize,
        kind: FlowImsArrayKind,
        index: usize,
        bits: u32,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let object = self.definition(ordinal)?;
        let array = object.array(kind).ok_or_else(|| invalid(format!("definition {ordinal} version {:?}, array {kind:?}; expected a physically present shared array", object.version())))?;
        let offset = wire::element_offset(
            500,
            array.payload_interval().0,
            array.word_count(),
            index,
            4,
        )?;
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
        let result = carrier_edit
            .result()
            .record_500_ims_definitions(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        self.check_layout(&result)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn check_layout(&self, result: &Self) -> Result<()> {
        if self.collections != result.collections || self.objects != result.objects {
            return Err(table_failure(
                500,
                FailureClass::Integrity,
                format!(
                    "reopened {} definition layouts; expected identical {} source layouts and collection spans",
                    result.objects.len(),
                    self.objects.len()
                ),
            ));
        }
        Ok(())
    }
}
