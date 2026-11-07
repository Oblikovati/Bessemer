use super::*;
impl ZipEntryTimestamps {
    /// Edits both copies on an isolated file; e.g. `let edit = times.replace_timestamp(&id, request)?`.
    pub fn replace_timestamp(
        &self,
        id: &ZipTimestampEntryId,
        request: ZipTimestampRequest,
    ) -> Result<ZipTimestampEdit> {
        let entry = self.entry(id)?;
        let packed = request.packed_word();
        let result = self.replaced(entry, packed)?;
        let history = byte_history(self, &result, entry);
        Ok(ZipTimestampEdit {
            source: self.clone(),
            result,
            ordinal: id.ordinal,
            history,
        })
    }
    fn replaced(&self, entry: ZipTimestampEntry<'_>, packed: u32) -> Result<Self> {
        if entry.packed_word() == packed {
            return Ok(self.clone());
        }
        let mut raw = self.raw.to_vec();
        for (start, end) in [entry.span.local, entry.span.central] {
            raw[start..end].copy_from_slice(&packed.to_le_bytes());
        }
        let result = Self::from_bytes(&raw, self.maximum)?;
        check_layout(self, &result)?;
        Ok(result)
    }
}
fn check_layout(source: &ZipEntryTimestamps, result: &ZipEntryTimestamps) -> Result<()> {
    let equal = source.entries.len() == result.entries.len()
        && source
            .entries
            .iter()
            .zip(result.entries.iter())
            .all(|(before, after)| same_position(before, after));
    if equal {
        return Ok(());
    }
    Err(failure(
        "edited file",
        FailureClass::Invalid,
        format!(
            "result entries {}; expected the same {} names and header intervals",
            result.entries.len(),
            source.entries.len()
        ),
    ))
}
fn same_position(before: &TimestampSpan, after: &TimestampSpan) -> bool {
    before.name == after.name && before.local == after.local && before.central == after.central
}
fn byte_history(
    source: &ZipEntryTimestamps,
    result: &ZipEntryTimestamps,
    entry: ZipTimestampEntry<'_>,
) -> Vec<ZipTimestampByteHistory> {
    let cuts = [
        0,
        entry.span.local.0,
        entry.span.local.1,
        entry.span.central.0,
        entry.span.central.1,
        source.raw.len(),
    ];
    cuts.windows(2)
        .filter(|pair| pair[0] < pair[1])
        .map(|pair| history_interval(source, result, pair[0], pair[1]))
        .collect()
}
fn history_interval(
    source: &ZipEntryTimestamps,
    result: &ZipEntryTimestamps,
    start: usize,
    end: usize,
) -> ZipTimestampByteHistory {
    let relation = if source.raw[start..end] == result.raw[start..end] {
        ZipTimestampByteRelation::Copied
    } else {
        ZipTimestampByteRelation::Derived
    };
    ZipTimestampByteHistory {
        source_interval: (start, end),
        result_interval: (start, end),
        relation,
    }
}
impl ZipTimestampEdit {
    /// Borrows the complete immutable parent; e.g. `let before = edit.source().as_bytes()`.
    pub fn source(&self) -> &ZipEntryTimestamps {
        &self.source
    }
    /// Borrows the complete reparsed result; e.g. `let after = edit.result().as_bytes()`.
    pub fn result(&self) -> &ZipEntryTimestamps {
        &self.result
    }
    /// Returns exact completion status; e.g. `let changed = edit.completion()`.
    pub fn completion(&self) -> crate::display_lists::DisplayEditCompletion {
        if self.source.revision == self.result.revision {
            crate::display_lists::DisplayEditCompletion::Unchanged
        } else {
            crate::display_lists::DisplayEditCompletion::Changed
        }
    }
    /// Returns the original entry identity; e.g. `let parent = edit.source_entry()`.
    pub fn source_entry(&self) -> ZipTimestampEntryId {
        ZipTimestampEntryId {
            revision: self.source.revision.clone(),
            ordinal: self.ordinal,
        }
    }
    /// Returns the corresponding result identity; e.g. `let child = edit.result_entry()`.
    pub fn result_entry(&self) -> ZipTimestampEntryId {
        ZipTimestampEntryId {
            revision: self.result.revision.clone(),
            ordinal: self.ordinal,
        }
    }
    /// Returns complete result coverage and achieved error; e.g. `let proof = edit.coverage()`.
    pub fn coverage(&self) -> ZipTimestampCoverage {
        self.result.coverage()
    }
    /// Borrows unchecked responsibilities; e.g. `let notes = edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[ZipTimestampDiagnostic] {
        self.result.diagnostics()
    }
    /// Borrows every copied/derived parent interval; e.g. `let spans = edit.history()`.
    pub fn history(&self) -> &[ZipTimestampByteHistory] {
        &self.history
    }
}
