use super::*;

impl FlowParallelTable {
    /// Replaces an existing raw wide word on an isolated copy; e.g. `let edit = table.replace_word(0, 2, 1, 0x8000000000000000)?`.
    pub fn replace_word(
        &self,
        group: usize,
        array: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        if array >= 4 {
            return Err(table_index_error(407, "array", array, 4));
        }
        let span = self
            .groups
            .get(group)
            .ok_or_else(|| table_index_error(407, "group", group, 2))?;
        let arrays = span
            .arrays
            .as_ref()
            .ok_or_else(|| missing_parallel(group, "arrays"))?;
        let count = (arrays[array].1 - arrays[array].0) / 8;
        if index >= count {
            return Err(table_index_error(407, "wide word", index, count));
        }
        self.edit_offset(arrays[array].0 + index * 8, &bits.to_le_bytes())
    }
    /// Replaces an existing presence word while retaining array presence; e.g. `let edit = table.replace_presence(0, -3)?`.
    pub fn replace_presence(&self, group: usize, value: i32) -> Result<FlowSimpleTableEdit<Self>> {
        let span = self
            .groups
            .get(group)
            .ok_or_else(|| table_index_error(407, "group", group, 2))?;
        let offset = span
            .presence
            .ok_or_else(|| missing_parallel(group, "presence word"))?;
        self.edit_offset(offset, &value.to_le_bytes())
    }
    fn edit_offset(&self, offset: usize, bytes: &[u8]) -> Result<FlowSimpleTableEdit<Self>> {
        let (carrier_edit, decoded) = patched_carrier(&self.wire, offset, bytes)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit.result().record_407_arrays(&id, self.options)?;
        if self.groups != result.groups {
            return Err(simple_invalid(
                407,
                format!(
                    "field offset {offset} changed array presence; expected identical group and array structure"
                ),
            ));
        }
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

impl FlowLeafPairs {
    /// Replaces one existing pair component without tree-order assumptions; e.g. `let edit = pairs.replace_word(0, 1, 0x7ff8000000001234)?`.
    pub fn replace_word(
        &self,
        ordinal: usize,
        component: usize,
        bits: u64,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        if component >= 2 {
            return Err(table_index_error(437, "component", component, 2));
        }
        let row = self
            .row(ordinal)
            .ok_or_else(|| table_index_error(437, "wire row", ordinal, self.rows))?;
        let offset = row.payload_interval().0 + component * 8;
        let (carrier_edit, decoded) = patched_carrier(&self.wire, offset, &bits.to_le_bytes())?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_437_leaf_pairs(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

impl FlowFiveWordTable {
    /// Replaces one physically stored raw word, without allocating inactive rows; e.g. `let edit = table.replace_word(0, 4, 0xffffffff)?`.
    pub fn replace_word(
        &self,
        ordinal: usize,
        component: usize,
        bits: u32,
    ) -> Result<FlowSimpleTableEdit<Self>> {
        if component >= 5 {
            return Err(table_index_error(411, "component", component, 5));
        }
        let row = self
            .row(ordinal)
            .ok_or_else(|| table_index_error(411, "wire row", ordinal, self.rows))?;
        let offset = row.payload_interval().0 + component * 4;
        let (carrier_edit, decoded) = patched_carrier(&self.wire, offset, &bits.to_le_bytes())?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit.result().record_411_rows(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

impl FlowRawByteTable {
    /// Replaces one existing byte without text conversion; e.g. `let edit = table.replace_byte(0, 255)?`.
    pub fn replace_byte(&self, index: usize, value: u8) -> Result<FlowSimpleTableEdit<Self>> {
        if index >= self.raw_bytes().len() {
            return Err(table_index_error(
                413,
                "raw byte",
                index,
                self.raw_bytes().len(),
            ));
        }
        let (carrier_edit, decoded) = patched_carrier(&self.wire, self.start + index, &[value])?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit.result().record_413_bytes(&id, self.options)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}

fn patched_carrier(
    wire: &TableWire,
    offset: usize,
    bytes: &[u8],
) -> Result<(FlowCarrierEdit, Vec<u8>)> {
    let mut decoded = wire.decoded.to_vec();
    let end = offset.checked_add(bytes.len()).filter(|end| *end <= decoded.len()).ok_or_else(|| {
        simple_invalid(wire.identifier, format!("patch offset {offset}, length {}; expected complete span within {} decoded bytes", bytes.len(), decoded.len()))
    })?;
    decoded[offset..end].copy_from_slice(bytes);
    let edit = wire::edited(wire, &decoded)?;
    Ok((edit, decoded))
}

fn missing_parallel(group: usize, field: &str) -> BessemerError {
    table_failure(
        407,
        FailureClass::Missing,
        format!("group {group} {field} absent; expected an existing serialized legacy field"),
    )
}
