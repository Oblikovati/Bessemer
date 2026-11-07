use super::*;

impl FlowBaseGrid {
    /// Replaces an existing raw boundary word on a copy; e.g. `let edit = grid.replace_boundary_word(0, 0, 0x8000000000000000)?`.
    pub fn replace_boundary_word(
        &self,
        array: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowGridEdit> {
        let offset = selected_offset(self, array, index)?;
        let mut decoded = self.decoded.to_vec();
        decoded[offset..offset + 8].copy_from_slice(&bits.to_le_bytes());
        let carrier_edit = edited_carrier(self, &decoded)?;
        let result = reopened(self, &carrier_edit, &decoded)?;
        Ok(FlowGridEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn selected_offset(grid: &FlowBaseGrid, array: usize, index: usize) -> Result<usize> {
    let boundary = grid.boundary(array).ok_or_else(|| {
        grid_failure(format!(
            "boundary array {array}; expected index 0 through 2"
        ))
    })?;
    if index < boundary.word_count() {
        return Ok(boundary.start + index * 8);
    }
    Err(grid_failure(format!(
        "boundary array {array} word {index}; expected index below {}",
        boundary.word_count()
    )))
}

fn edited_carrier(grid: &FlowBaseGrid, decoded: &[u8]) -> Result<FlowCarrierEdit> {
    if decoded == grid.decoded.as_ref() {
        let original = super::super::payload_codec::record_payload(
            &grid.source,
            &grid.record,
            101,
            "Flow base grid",
        )?;
        return grid.source.replace_payload(&grid.record, original);
    }
    let payload = match grid.options.compression {
        FlowGridCompression::Raw => decoded.to_vec(),
        FlowGridCompression::Zlib => {
            super::super::payload_codec::encode_zlib(decoded, "Flow base grid")?
        }
    };
    grid.source.replace_payload(&grid.record, &payload)
}

fn reopened(grid: &FlowBaseGrid, edit: &FlowCarrierEdit, decoded: &[u8]) -> Result<FlowBaseGrid> {
    let id = edit
        .result()
        .records()
        .nth(grid.record.ordinal())
        .map(|record| record.id)
        .ok_or_else(|| {
            grid_failure(format!(
                "result occurrence {}; expected selected source ordinal retained",
                grid.record.ordinal()
            ))
        })?;
    let result = edit.result().record_101_grid(&id, grid.options)?;
    if result.decoded_bytes() == decoded {
        return Ok(result);
    }
    Err(grid_failure(format!(
        "reopened {} decoded bytes; expected complete patched {} bytes",
        result.decoded.len(),
        decoded.len()
    )))
}
