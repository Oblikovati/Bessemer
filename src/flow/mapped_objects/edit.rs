use super::*;

impl FlowMappedObjects {
    /// Edits one existing fixed-width body field; e.g. `let edit = list.replace_field(0, FlowMappedFieldEdit::State(255))?`.
    pub fn replace_field(&self, entry: usize, edit: FlowMappedFieldEdit) -> Result<FlowMappedEdit> {
        let selected = self
            .entries
            .get(entry)
            .ok_or_else(|| index_error("entry", entry, self.entries.len()))?;
        let body = selected.body.as_ref().ok_or_else(|| {
            mapped_failure(
                FailureClass::Missing,
                format!("entry {entry} is null; expected a populated body"),
            )
        })?;
        let (offset, bytes) = field_bytes(selected, body, edit)?;
        let mut payload = self.payload.to_vec();
        payload[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = self.source.replace_payload(&self.record, &payload)?;
        let result = checked_result(&carrier_edit, self, &payload)?;
        Ok(FlowMappedEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn field_bytes(
    entry: &FlowMappedEntry,
    body: &FlowMappedBody,
    edit: FlowMappedFieldEdit,
) -> Result<(usize, Vec<u8>)> {
    match edit {
        FlowMappedFieldEdit::State(value) => Ok((entry.start + 5, vec![value])),
        FlowMappedFieldEdit::PairSource { index, value } => pair_bytes(body, index, value, 0),
        FlowMappedFieldEdit::PairValue { index, value } => pair_bytes(body, index, value, 4),
        FlowMappedFieldEdit::Numeric { field, index, bits } => {
            numeric_bytes(body, field, index, bits)
        }
    }
}

fn pair_bytes(
    body: &FlowMappedBody,
    index: usize,
    value: i32,
    offset: usize,
) -> Result<(usize, Vec<u8>)> {
    if index >= body.pairs.len() {
        return Err(index_error("pair", index, body.pairs.len()));
    }
    Ok((
        body.pair_offset + index * 8 + offset,
        value.to_le_bytes().to_vec(),
    ))
}

fn numeric_bytes(
    body: &FlowMappedBody,
    field: FlowMappedNumericField,
    index: usize,
    bits: u64,
) -> Result<(usize, Vec<u8>)> {
    let block = numeric_block(body, field).ok_or_else(|| {
        mapped_failure(
            FailureClass::Missing,
            format!(
                "numeric field {field:?} in body version {}; expected a physically present block",
                body.version
            ),
        )
    })?;
    if index >= block.bits.len() {
        return Err(index_error("numeric word", index, block.bits.len()));
    }
    Ok((block.offset + index * 8, bits.to_le_bytes().to_vec()))
}

fn numeric_block(
    body: &FlowMappedBody,
    field: FlowMappedNumericField,
) -> Option<&FlowMappedNumericBlock> {
    use FlowMappedNumericField::*;
    match field {
        Prefix => Some(&body.numeric),
        Mapped => Some(&body.mapped),
        LegacyPrefix => body.legacy.as_ref().map(|legacy| &legacy.prefix),
        LegacyFirst => body.legacy.as_ref().map(|legacy| &legacy.arrays[0]),
        LegacySecond => body.legacy.as_ref().map(|legacy| &legacy.arrays[1]),
        LegacyThird => body.legacy.as_ref().map(|legacy| &legacy.arrays[2]),
        LegacyTripled => body.legacy.as_ref().map(|legacy| &legacy.tripled),
    }
}

fn checked_result(
    carrier_edit: &FlowCarrierEdit,
    source: &FlowMappedObjects,
    payload: &[u8],
) -> Result<FlowMappedObjects> {
    let result = carrier_edit
        .result()
        .mapped_objects(source.limits, source.context.clone())?
        .ok_or_else(|| {
            mapped_failure(
                FailureClass::Missing,
                "edited record 144 absent; expected complete selected mapped list".to_owned(),
            )
        })?;
    if result.payload() == payload {
        return Ok(result);
    }
    Err(mapped_failure(
        FailureClass::Invalid,
        format!(
            "reopened payload has {} bytes; expected complete patched {} bytes",
            result.payload().len(),
            payload.len()
        ),
    ))
}

fn index_error(label: &str, index: usize, length: usize) -> BessemerError {
    mapped_failure(
        FailureClass::Invalid,
        format!("{label} index {index}; expected below {length} in this immutable selected list"),
    )
}
