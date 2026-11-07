use super::*;

impl FlowPointerBodies {
    /// Replaces an existing pool word on an isolated copy; e.g. `let edit = bodies.replace_pool_word(0, 1, 0x8000000000000000)?`.
    pub fn replace_pool_word(
        &self,
        pool: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowPointerBodyEdit> {
        let words = self
            .pool(pool)
            .ok_or_else(|| table_index_error(421, "pool", pool, self.pools.len()))?
            .wide_words();
        let offset = wide_offset(words, index)?;
        self.patch(offset, &bits.to_le_bytes())
    }
    /// Replaces an existing legacy/tagged body word; e.g. `let edit = bodies.replace_entry_word(2, 0, 0, 7)?`.
    pub fn replace_entry_word(
        &self,
        group: usize,
        entry: usize,
        index: usize,
        bits: u64,
    ) -> Result<FlowPointerBodyEdit> {
        let words = self
            .required_entry(group, entry)?
            .wide_words()
            .ok_or_else(|| missing_body(group, entry, "wide words"))?;
        self.patch(wide_offset(words, index)?, &bits.to_le_bytes())
    }
    /// Replaces an existing vector byte without editing framing; e.g. `let edit = bodies.replace_vector_byte(0, 0, 0, 1, 255)?`.
    pub fn replace_vector_byte(
        &self,
        group: usize,
        entry: usize,
        vector: usize,
        index: usize,
        value: u8,
    ) -> Result<FlowPointerBodyEdit> {
        let view = self.required_entry(group, entry)?;
        let span = view
            .vector_interval(vector)
            .ok_or_else(|| table_index_error(421, "vector", vector, view.vector_count()))?;
        self.patch(byte_offset(span, index)?, &[value])
    }
    /// Replaces one opaque tagged byte without decoding it; e.g. `let edit = bodies.replace_raw_byte(0, 0, 1, 255)?`.
    pub fn replace_raw_byte(
        &self,
        group: usize,
        entry: usize,
        index: usize,
        value: u8,
    ) -> Result<FlowPointerBodyEdit> {
        let span = self
            .required_entry(group, entry)?
            .raw_interval()
            .ok_or_else(|| missing_body(group, entry, "opaque tagged bytes"))?;
        self.patch(byte_offset(span, index)?, &[value])
    }
    /// Replaces a stored tag only when all parsed payload shapes stay identical; e.g. `let edit = bodies.replace_tag(0, 0, 255)?`.
    pub fn replace_tag(&self, group: usize, entry: usize, tag: u8) -> Result<FlowPointerBodyEdit> {
        let view = self.required_entry(group, entry)?;
        view.tag_byte()
            .ok_or_else(|| missing_body(group, entry, "tag byte"))?;
        self.patch(view.payload_interval().0, &[tag])
    }
    fn required_entry(&self, group: usize, entry: usize) -> Result<FlowPointerBodyEntry<'_>> {
        let count = self
            .entry_count(group)
            .ok_or_else(|| table_index_error(421, "group", group, 3))?;
        self.entry(group, entry)
            .ok_or_else(|| table_index_error(421, "entry", entry, count))
    }
    fn patch(&self, offset: usize, bytes: &[u8]) -> Result<FlowPointerBodyEdit> {
        let mut decoded = self.wire.decoded.to_vec();
        decoded[offset..offset + bytes.len()].copy_from_slice(bytes);
        let carrier_edit = wire::edited(&self.wire, &decoded)?;
        let id = wire::result_id(&self.wire, carrier_edit.result())?;
        let result = carrier_edit
            .result()
            .record_421_bodies(&id, self.options.clone())?;
        self.check_structure(&result, offset)?;
        wire::check_decoded(&result.wire, &decoded)?;
        Ok(FlowPointerBodyEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
    fn check_structure(&self, result: &Self, offset: usize) -> Result<()> {
        if self.pools == result.pools
            && self.groups == result.groups
            && self.entries == result.entries
            && self.vectors == result.vectors
        {
            return Ok(());
        }
        Err(body_invalid(format!(
            "edit at {offset} changed the parsed body shape; expected identical pool, collection, entry and vector spans"
        )))
    }
}

fn wide_offset(words: FlowTableWideArray<'_>, index: usize) -> Result<usize> {
    if index >= words.word_count() {
        return Err(table_index_error(
            421,
            "wide word",
            index,
            words.word_count(),
        ));
    }
    Ok(words.payload_interval().0 + index * 8)
}

fn byte_offset((start, end): (usize, usize), index: usize) -> Result<usize> {
    if index >= end - start {
        return Err(table_index_error(421, "payload byte", index, end - start));
    }
    Ok(start + index)
}

fn missing_body(group: usize, entry: usize, field: &str) -> BessemerError {
    table_failure(
        421,
        FailureClass::Missing,
        format!(
            "collection {group} entry {entry} has no {field}; expected an existing serialized field"
        ),
    )
}
