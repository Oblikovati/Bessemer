use super::*;
pub(super) fn read(
    source: &FlowCarrier,
    id: &FlowRecordId,
    profile: FlowLegacyWordProfile,
    options: FlowLegacyFieldWordOptions,
) -> Result<FlowLegacyWordFields> {
    let raw = bounded_payload(
        source,
        id,
        profile.identifier(),
        options.maximum_payload_bytes,
    )?;
    let (count, first) = word_span(raw, profile, options.maximum_patterns)?;
    Ok(FlowLegacyWordFields {
        source: source.clone(),
        record: id.clone(),
        raw: Arc::from(raw),
        profile,
        options,
        count,
        first,
    })
}
fn word_span(raw: &[u8], profile: FlowLegacyWordProfile, maximum: usize) -> Result<(usize, usize)> {
    let mut cursor = ArchiveCursor::new(raw);
    let count = match profile {
        FlowLegacyWordProfile::Fixed105 => 320,
        FlowLegacyWordProfile::Counted113 => cursor.nonnegative_count(8, "record113 patterns")?,
    };
    admit_patterns(profile.identifier(), count, maximum)?;
    let first = raw.len() - cursor.remaining();
    cursor.take(count * 8)?;
    cursor.finish()?;
    Ok((count, first))
}
fn check_payload(fields: &FlowLegacyWordFields, expected: &[u8]) -> Result<()> {
    if fields.raw.as_ref() == expected {
        return Ok(());
    }
    Err(invalid(
        fields.profile.identifier(),
        "reparsed bytes differ; expected the complete independently edited raw payload".into(),
    ))
}
fn admit_patterns(identifier: u32, count: usize, maximum: usize) -> Result<()> {
    if count <= maximum {
        return Ok(());
    }
    Err(invalid(
        identifier,
        format!("stored pattern count {count}; expected at most configured bound {maximum}"),
    ))
}
impl FlowLegacyWordFields {
    /// Edits one existing raw pattern without changing framing; e.g. `let edit = fields.replace_pattern(7, 0x4081a80000000000)?`.
    pub fn replace_pattern(&self, index: usize, bits: u64) -> Result<FlowSimpleTableEdit<Self>> {
        let (start, end) = self.pattern_interval(index)?;
        let mut raw = self.raw.to_vec();
        raw[start..end].copy_from_slice(&bits.to_le_bytes());
        let carrier_edit = self.source.replace_payload(&self.record, &raw)?;
        let record = replacement_record(
            &self.record,
            carrier_edit.result(),
            self.profile.identifier(),
        )?;
        let result = read(carrier_edit.result(), &record, self.profile, self.options)?;
        check_payload(&result, &raw)?;
        Ok(FlowSimpleTableEdit {
            source: self.clone(),
            result,
            carrier_edit,
        })
    }
}
