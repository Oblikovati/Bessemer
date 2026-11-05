use super::*;
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

struct SelectedCell {
    group: FlowArrayGroup,
    index: usize,
    offset: usize,
    before: i32,
    after: i32,
}

impl FlowMapArrays {
    /// Replaces one signed cell on a copy; e.g. `let edit = arrays.replace_value(FlowArrayGroup::Second, 0, -1)?`.
    pub fn replace_value(
        &self,
        group: FlowArrayGroup,
        index: usize,
        value: i32,
    ) -> Result<FlowArrayEdit> {
        let cell = cell_position(self, group, index, value)?;
        let carrier = edited_carrier(self, &cell)?;
        let result = result_arrays(self, &carrier)?;
        verify_decoded_edit(self, &result, cell.offset, value)?;
        let history = cell_history(self, &result, &cell);
        Ok(FlowArrayEdit {
            source: self.clone(),
            result,
            carrier,
            history,
        })
    }
}

fn cell_position(
    arrays: &FlowMapArrays,
    group: FlowArrayGroup,
    index: usize,
    after: i32,
) -> Result<SelectedCell> {
    let values = selected_values(arrays, group)?;
    let before = values.get(index).copied().ok_or_else(|| {
        array_failure(
            "array index",
            FailureClass::Invalid,
            format!("{group:?} index {index}; expected below {}", values.len()),
        )
    })?;
    let skip = if group == FlowArrayGroup::Second {
        arrays.first.as_ref().map_or(0, |first| first.len())
    } else {
        0
    };
    Ok(SelectedCell {
        group,
        index,
        offset: 8 + (skip + index) * 4,
        before,
        after,
    })
}

fn selected_values(arrays: &FlowMapArrays, group: FlowArrayGroup) -> Result<&[i32]> {
    let values = match group {
        FlowArrayGroup::First => arrays.first(),
        FlowArrayGroup::Second => Some(arrays.second()),
    };
    values.ok_or_else(|| {
        array_failure(
            "array group",
            FailureClass::Missing,
            format!(
                "{group:?} omitted for selector {}; expected a physically present array",
                arrays.selector
            ),
        )
    })
}

fn edited_carrier(arrays: &FlowMapArrays, cell: &SelectedCell) -> Result<FlowCarrierEdit> {
    if cell.before == cell.after {
        let payload = decode::record_payload(&arrays.source, &arrays.record)?;
        return arrays.source.replace_payload(&arrays.record, payload);
    }
    let mut decoded = arrays.decoded.to_vec();
    decoded[cell.offset..cell.offset + 4].copy_from_slice(&cell.after.to_le_bytes());
    let compressed = encode_zlib(&decoded)?;
    arrays.source.replace_payload(&arrays.record, &compressed)
}

fn result_arrays(source: &FlowMapArrays, carrier: &FlowCarrierEdit) -> Result<FlowMapArrays> {
    let ordinal = source.record.ordinal();
    let record = carrier.result().records().nth(ordinal).ok_or_else(|| {
        array_failure(
            "result occurrence",
            FailureClass::Integrity,
            format!("missing ordinal {ordinal}; expected unchanged record count"),
        )
    })?;
    carrier
        .result()
        .record_433_arrays(&record.id, source.options)
}

fn cell_history(
    source: &FlowMapArrays,
    result: &FlowMapArrays,
    cell: &SelectedCell,
) -> FlowArrayCellHistory {
    FlowArrayCellHistory {
        source_record: source.record.clone(),
        result_record: result.record.clone(),
        group: cell.group,
        index: cell.index,
        before: cell.before,
        after: cell.after,
        relation: if cell.before == cell.after {
            FlowPayloadRelation::Copied
        } else {
            FlowPayloadRelation::Derived
        },
    }
}

fn encode_zlib(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(bytes).map_err(encode_failure)?;
    encoder.finish().map_err(encode_failure)
}

fn encode_failure(error: std::io::Error) -> BessemerError {
    array_failure(
        "zlib encoding",
        FailureClass::Integrity,
        format!("{error}; expected complete RFC 1950 encoding"),
    )
}

fn verify_decoded_edit(
    source: &FlowMapArrays,
    result: &FlowMapArrays,
    offset: usize,
    value: i32,
) -> Result<()> {
    let mut expected = source.decoded.to_vec();
    expected[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    if result.decoded_bytes() == expected {
        return Ok(());
    }
    Err(array_failure(
        "result arrays",
        FailureClass::Integrity,
        format!(
            "{} result bytes differ from {} expected bytes; expected only selected four-byte cell change",
            result.decoded.len(),
            expected.len()
        ),
    ))
}
