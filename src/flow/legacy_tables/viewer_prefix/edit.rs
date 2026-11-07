use super::*;
impl FlowViewerPrefix {
    /// Replaces an existing signed prefix field on a copy; e.g. `let edit = prefix.replace_signed(FlowViewerSignedField::StoredSecond, 1)?`.
    pub fn replace_signed(
        &self,
        field: FlowViewerSignedField,
        value: i32,
    ) -> Result<FlowViewerPrefixEdit> {
        let offset = match field {
            FlowViewerSignedField::IgnoredInitial => 0,
            FlowViewerSignedField::StoredSecond => 4,
            FlowViewerSignedField::FinalWord => self.layout.triplet_start + 24,
        };
        self.patch(offset, &value.to_le_bytes())
    }
    /// Replaces one existing raw array pattern, preserving counts and opaque bytes; e.g. `let edit = prefix.replace_array_pattern(0, 0, u64::MAX)?`.
    pub fn replace_array_pattern(
        &self,
        ordinal: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowViewerPrefixEdit> {
        let array = self.array(ordinal)?;
        let offset = wire::element_offset(
            121,
            array.payload_interval().0,
            array.word_count(),
            index,
            8,
        )?;
        self.patch(offset, &bits.to_le_bytes())
    }
    /// Replaces one of the three discarded raw patterns; e.g. `let edit = prefix.replace_discarded_pattern(0, u64::MAX)?`.
    pub fn replace_discarded_pattern(
        &self,
        index: usize,
        bits: u64,
    ) -> Result<FlowViewerPrefixEdit> {
        let offset = wire::element_offset(121, self.layout.triplet_start, 3, index, 8)?;
        self.patch(offset, &bits.to_le_bytes())
    }
    fn patch<const N: usize>(
        &self,
        offset: usize,
        replacement: &[u8; N],
    ) -> Result<FlowViewerPrefixEdit> {
        let mut body = self.wire.decoded.to_vec();
        body[offset..offset + N].copy_from_slice(replacement);
        let carrier_edit = wire::edited_versioned(&self.wire, &body, OPERATION)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_121_viewer_prefix(&id, self.options)?;
        wire::check_decoded(&result.wire, &body)?;
        self.check_layout(&result)?;
        Ok(FlowViewerPrefixEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn check_layout(&self, result: &Self) -> Result<()> {
        if result.layout == self.layout {
            return Ok(());
        }
        Err(table_failure(
            121,
            FailureClass::Integrity,
            format!(
                "reopened layout {:?}; expected unchanged source layout {:?}",
                result.layout, self.layout
            ),
        ))
    }
}
