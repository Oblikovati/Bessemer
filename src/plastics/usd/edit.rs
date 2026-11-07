use super::{
    Result, UsdArchive, UsdCheckMode, UsdEditCompletion, UsdRecordEdit, UsdRecordHistory,
    UsdRecordId, UsdRecordRelation, UsdStringRecord, invalid,
};

impl UsdArchive {
    /// Replace one exact encoded record on an isolated stream, e.g. `archive.edit_record(&id, &replacement)?`.
    pub fn edit_record(
        &self,
        id: &UsdRecordId,
        replacement: &UsdStringRecord,
    ) -> Result<UsdRecordEdit> {
        let index = self.require_record(id)?;
        let archive = self.replaced_record(index, replacement)?;
        let history = record_history(self, &archive, index)?;
        edit_result(self, archive, id, replacement, history)
    }

    fn replaced_record(&self, index: usize, replacement: &UsdStringRecord) -> Result<Self> {
        let span = self.spans[index];
        if replacement.bytes[replacement.span.offset..replacement.span.end]
            == self.bytes[span.offset..span.end]
        {
            return Ok(self.clone());
        }
        let bytes = replace_bytes(self, index, replacement)?;
        let archive = Self::from_bytes(&bytes, self.limits)?;
        verify_preservation(self, &archive, index, replacement)?;
        Ok(archive)
    }
}

fn replace_bytes(
    source: &UsdArchive,
    index: usize,
    replacement: &UsdStringRecord,
) -> Result<Vec<u8>> {
    let span = source.spans[index];
    let size = replacement_size(source, index, replacement)?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(size).map_err(|error| {
        invalid(
            "replacement extent",
            format!("{size} bytes, allocation {error}; expected available complete output storage"),
        )
    })?;
    bytes.extend_from_slice(&source.bytes[..span.offset]);
    bytes.extend_from_slice(&replacement.bytes[replacement.span.offset..replacement.span.end]);
    bytes.extend_from_slice(&source.bytes[span.end..]);
    Ok(bytes)
}

fn verify_preservation(
    source: &UsdArchive,
    result: &UsdArchive,
    index: usize,
    replacement: &UsdStringRecord,
) -> Result<()> {
    if source.spans.len() != result.spans.len() {
        return Err(invalid(
            "replacement record count",
            format!("{}; expected {}", result.spans.len(), source.spans.len()),
        ));
    }
    for ordinal in 0..source.spans.len() {
        verify_record(source, result, ordinal, index, replacement)?;
    }
    Ok(())
}

fn record_history(
    source: &UsdArchive,
    result: &UsdArchive,
    index: usize,
) -> Result<Vec<UsdRecordHistory>> {
    let changed = source.revision != result.revision;
    (0..source.spans.len())
        .map(|ordinal| {
            Ok(UsdRecordHistory {
                parent: source.record_id(ordinal)?,
                result: result.record_id(ordinal)?,
                relation: if changed && ordinal == index {
                    UsdRecordRelation::Derived
                } else {
                    UsdRecordRelation::Copied
                },
            })
        })
        .collect()
}

fn edit_result(
    source: &UsdArchive,
    archive: UsdArchive,
    id: &UsdRecordId,
    replacement: &UsdStringRecord,
    history: Vec<UsdRecordHistory>,
) -> Result<UsdRecordEdit> {
    let mut coverage = archive.coverage();
    coverage.check_mode = UsdCheckMode::ExactCompleteRecordsAndUnchangedRecordBytes;
    Ok(UsdRecordEdit {
        source: source.clone(),
        replacement: replacement.clone(),
        parent: id.clone(),
        result: archive.record_id(id.index)?,
        parent_revision: source.revision.clone(),
        result_revision: archive.revision.clone(),
        completion: if archive.revision == source.revision {
            UsdEditCompletion::Unchanged
        } else {
            UsdEditCompletion::Changed
        },
        archive,
        coverage,
        history: history.into(),
        requested_pattern_error_bits: 0,
    })
}

fn replacement_size(
    source: &UsdArchive,
    index: usize,
    replacement: &UsdStringRecord,
) -> Result<usize> {
    if replacement.payload().len() > source.limits.maximum_payload_bytes {
        return Err(invalid(
            "replacement payload",
            format!(
                "{} bytes; expected at most {}",
                replacement.payload().len(),
                source.limits.maximum_payload_bytes
            ),
        ));
    }
    let span = source.spans[index];
    let original = source.bytes.len() - (span.end - span.offset);
    original.checked_add(replacement.span.end - replacement.span.offset).ok_or_else(|| invalid("replacement extent", format!("{original} retained bytes plus replacement; expected a representable complete extent")))
}

fn verify_record(
    source: &UsdArchive,
    result: &UsdArchive,
    ordinal: usize,
    index: usize,
    replacement: &UsdStringRecord,
) -> Result<()> {
    let expected = if ordinal == index {
        replacement.clone()
    } else {
        source.record(&source.record_id(ordinal)?)?
    };
    if result.record(&result.record_id(ordinal)?)?.to_bytes() != expected.to_bytes() {
        return Err(invalid(
            "replacement byte preservation",
            format!(
                "record {ordinal}; expected exact supplied replacement or unchanged original encoded record"
            ),
        ));
    }
    Ok(())
}
