mod patch;
use super::*;

impl FlowLegacyBlockData {
    /// Replaces an existing field on a copy; e.g. `let edit = blocks.replace_field(FlowBlockFieldEdit::BitmapByte { index: 0, byte: 255 })?`.
    pub fn replace_field(&self, request: FlowBlockFieldEdit) -> Result<FlowBlockDataEdit> {
        let (offset, bytes) = patch::selected(self, request)?;
        let mut decoded = self.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(&bytes);
        let carrier_edit = edited_carrier(self, &decoded)?;
        let result = reopened(self, &carrier_edit, &decoded)?;
        Ok(FlowBlockDataEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn edited_carrier(body: &FlowLegacyBlockData, decoded: &[u8]) -> Result<FlowCarrierEdit> {
    if decoded == body.decoded.as_ref() {
        let original = super::super::payload_codec::record_payload(
            &body.source,
            &body.record,
            406,
            "Flow legacy block data",
        )?;
        return body.source.replace_payload(&body.record, original);
    }
    let payload = match body.options.compression {
        FlowBlockDataCompression::Raw => decoded.to_vec(),
        FlowBlockDataCompression::Zlib => {
            super::super::payload_codec::encode_zlib(decoded, "Flow legacy block data")?
        }
    };
    body.source.replace_payload(&body.record, &payload)
}

fn reopened(
    body: &FlowLegacyBlockData,
    edit: &FlowCarrierEdit,
    decoded: &[u8],
) -> Result<FlowLegacyBlockData> {
    let id = edit
        .result()
        .records()
        .nth(body.record.ordinal())
        .map(|record| record.id)
        .ok_or_else(|| {
            invalid_block(format!(
                "result occurrence {}; expected selected source ordinal retained",
                body.record.ordinal()
            ))
        })?;
    let result = edit.result().record_406_blocks(&id, body.options)?;
    if result.decoded_bytes() == decoded {
        return Ok(result);
    }
    Err(invalid_block(format!(
        "reopened {} decoded bytes; expected complete patched {} bytes",
        result.decoded.len(),
        decoded.len()
    )))
}
