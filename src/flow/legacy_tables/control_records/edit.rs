use super::*;
impl FlowSavedSpans {
    /// Replaces an existing raw key on a copy, retaining duplicate order; e.g. `let edit = saved.replace_key(0, u64::MAX)?`.
    pub fn replace_key(&self, ordinal: usize, bits: u64) -> Result<FlowSimpleTableEdit<Self>> {
        self.patch(self.entry(ordinal)?.start, &bits.to_le_bytes())
    }
    /// Replaces an existing raw value on a copy; e.g. `let edit = saved.replace_value(0, u64::MAX)?`.
    pub fn replace_value(&self, ordinal: usize, bits: u64) -> Result<FlowSimpleTableEdit<Self>> {
        self.patch(self.entry(ordinal)?.start + 8, &bits.to_le_bytes())
    }
    fn patch(&self, offset: usize, replacement: &[u8; 8]) -> Result<FlowSimpleTableEdit<Self>> {
        let decoded = patched(&self.wire, offset, replacement);
        let encoded = if decoded == self.wire.decoded.as_ref() {
            None
        } else {
            Some(transport::encode(&decoded)?)
        };
        let carrier_edit = replace(&self.wire, encoded.as_deref())?;
        let id = result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit.result().saved_span_writer(&id, self.options)?;
        check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
impl FlowTerminalIndex {
    /// Replaces a raw target identifier without adoption or reconciliation; e.g. `let edit = index.replace_identifier(0, u32::MAX)?`.
    pub fn replace_identifier(
        &self,
        ordinal: usize,
        identifier: u32,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        self.patch(self.entry(ordinal)?.start, &identifier.to_le_bytes())
    }
    /// Replaces a raw target start and recomputes the footer on a copy; e.g. `let edit = index.replace_record_start(0, u64::MAX)?`.
    pub fn replace_record_start(
        &self,
        ordinal: usize,
        start: u64,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        self.patch(self.entry(ordinal)?.start + 4, &start.to_le_bytes())
    }
    /// Replaces a raw target span and recomputes the footer on a copy; e.g. `let edit = index.replace_record_span(0, u64::MAX)?`.
    pub fn replace_record_span(
        &self,
        ordinal: usize,
        span: u64,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        self.patch(self.entry(ordinal)?.start + 12, &span.to_le_bytes())
    }
    fn patch<const N: usize>(
        &self,
        offset: usize,
        replacement: &[u8; N],
    ) -> Result<FlowSimpleTableEdit<Self>> {
        let decoded = patched(&self.wire, offset, replacement);
        let encoded = self.changed_payload(&decoded)?;
        let carrier_edit = replace(&self.wire, encoded.as_deref())?;
        let id = result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .terminal_index_writer(&id, self.width, self.options)?;
        check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn changed_payload(&self, decoded: &[u8]) -> Result<Option<Vec<u8>>> {
        if decoded == self.wire.decoded.as_ref() {
            return Ok(None);
        }
        transport::wrap_index(&transport::encode(decoded)?, self.width).map(Some)
    }
}
fn patched<const N: usize>(wire: &ControlWire, offset: usize, replacement: &[u8; N]) -> Vec<u8> {
    let mut decoded = wire.decoded.to_vec();
    decoded[offset..offset + N].copy_from_slice(replacement);
    decoded
}
fn replace(wire: &ControlWire, encoded: Option<&[u8]>) -> Result<FlowCarrierEdit> {
    let stored = match encoded {
        Some(stored) => stored,
        None => transport::payload(&wire.source, &wire.record, wire.identifier)?,
    };
    wire.source.replace_payload(&wire.record, stored)
}
fn result_id(wire: &ControlWire, result: &FlowCarrier) -> Result<FlowRecordId> {
    result
        .records()
        .nth(wire.record.ordinal())
        .map(|record| record.id)
        .ok_or_else(|| {
            table_index_error(
                wire.identifier,
                "result occurrence",
                wire.record.ordinal(),
                result.records().len(),
            )
        })
}
fn check_decoded(wire: &ControlWire, expected: &[u8]) -> Result<()> {
    if wire.decoded.as_ref() == expected {
        return Ok(());
    }
    Err(table_failure(
        wire.identifier,
        FailureClass::Integrity,
        format!(
            "reopened decoded body {} bytes; expected exactly {} patched bytes with unchanged writer structure",
            wire.decoded.len(),
            expected.len()
        ),
    ))
}
