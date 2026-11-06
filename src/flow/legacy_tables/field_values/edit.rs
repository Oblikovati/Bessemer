use super::*;

impl FlowFieldValues {
    /// Replaces an existing global-object pattern; e.g. `let edit = fields.replace_object_word(0, 1, 0x8000000000000000)?`.
    pub fn replace_object_word(
        &self,
        object: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowFieldValueEdit> {
        let object = self
            .object(object)
            .ok_or_else(|| table_index_error(108, "global object", object, self.objects().len()))?;
        self.patch_wide(object.wide_words(), index, bits)
    }
    /// Replaces an existing active-region pattern; e.g. `let edit = fields.replace_region_word(0, 1, 0x7ff8000000001234)?`.
    pub fn replace_region_word(
        &self,
        region: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowFieldValueEdit> {
        let region = self
            .region(region)
            .ok_or_else(|| table_index_error(108, "region", region, self.regions().len()))?;
        self.patch_wide(region.wide_words(), index, bits)
    }
    /// Replaces an existing signed table word subject to supplied expected checks; e.g. `let edit = fields.replace_width_table_word(0, -1)?`.
    pub fn replace_width_table_word(&self, index: usize, value: i32) -> Result<FlowFieldValueEdit> {
        let table = self.width_table().ok_or_else(|| {
            field_invalid(
                "region layout has no legacy width table; expected a global-object layout".into(),
            )
        })?;
        if index >= table.word_count() {
            return Err(table_index_error(
                108,
                "width-table word",
                index,
                table.word_count(),
            ));
        }
        self.patch(index * 4, &value.to_le_bytes())
    }
    fn patch_wide(
        &self,
        words: FlowTableWideArray<'_>,
        index: usize,
        bits: u64,
    ) -> Result<FlowFieldValueEdit> {
        if index >= words.word_count() {
            return Err(table_index_error(
                108,
                "wide pattern",
                index,
                words.word_count(),
            ));
        }
        self.patch(words.payload_interval().0 + index * 8, &bits.to_le_bytes())
    }
    fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowFieldValueEdit> {
        let mut body = self.wire.decoded.to_vec();
        body[offset..offset + bytes.len()].copy_from_slice(bytes);
        let carrier_edit = edited_body(&self.wire, &body)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_108_values(&id, self.options.clone())?;
        self.check_structure(&result, offset)?;
        wire::check_decoded(&result.wire, &body)?;
        Ok(FlowFieldValueEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn check_structure(&self, result: &Self, offset: usize) -> Result<()> {
        if self.spans != result.spans
            || self.table_words != result.table_words
            || self.version() != result.version()
        {
            return Err(field_invalid(format!(
                "edit at body offset {offset} changed structure; expected identical version, table length and item spans"
            )));
        }
        Ok(())
    }
}

fn edited_body(wire: &TableWire, body: &[u8]) -> Result<FlowCarrierEdit> {
    if body == wire.decoded.as_ref() {
        let stored = super::super::super::payload_codec::record_payload(
            &wire.source,
            &wire.record,
            108,
            "Flow field values",
        )?;
        return wire.source.replace_payload(&wire.record, stored);
    }
    let encoded = match wire.compression {
        FlowLegacyTableTransport::Raw => body.to_vec(),
        FlowLegacyTableTransport::Zlib => {
            super::super::super::payload_codec::encode_zlib(body, "Flow field values")?
        }
    };
    let mut payload = wire.version.to_le_bytes().to_vec();
    payload.extend_from_slice(&encoded);
    wire.source.replace_payload(&wire.record, &payload)
}
