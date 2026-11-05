use super::*;
use std::ops::Range;

impl ElectricalIniDocument {
    /// Replaces one value occurrence while preserving every other lexical record; e.g. `document.edit_value(ordinal, b"value")?`.
    pub fn edit_value(&self, ordinal: usize, replacement: &[u8]) -> Result<ElectricalIniEdit> {
        let range = value_range(self, ordinal)?;
        let bytes = edited_bytes(self, &range, replacement)?;
        let result = Self::from_bytes(&bytes, self.limits)?
            .ok_or_else(|| ini_error(ordinal, "non-ASCII reparse; expected ASCII"))?;
        verify_records(self, &result, ordinal, replacement)?;
        Ok(publish_ini_edit(self, result, range, replacement.len()))
    }
}

fn publish_ini_edit(
    source: &ElectricalIniDocument,
    result: ElectricalIniDocument,
    range: Range<usize>,
    length: usize,
) -> ElectricalIniEdit {
    let completion = if source.bytes == result.bytes {
        ElectricalEditCompletion::Unchanged
    } else {
        ElectricalEditCompletion::Changed
    };
    let history = lexical_history(
        source.bytes.len(),
        result.bytes.len(),
        range,
        length,
        completion,
    );
    ElectricalIniEdit {
        source: source.clone(),
        result,
        completion,
        history,
    }
}

fn value_range(source: &ElectricalIniDocument, ordinal: usize) -> Result<Range<usize>> {
    let record = source.records.get(ordinal).ok_or_else(|| {
        BessemerError::new(
            "electrical INI edit",
            &ordinal.to_string(),
            FailureClass::Missing,
            format!(
                "record ordinal {ordinal}; expected 0..{}",
                source.records.len()
            ),
        )
    })?;
    record.value.clone().ok_or_else(|| {
        ini_error(
            ordinal,
            format!(
                "record kind {:?}; expected a Field value occurrence",
                record.kind
            ),
        )
    })
}

fn edited_bytes(
    source: &ElectricalIniDocument,
    range: &Range<usize>,
    replacement: &[u8],
) -> Result<Vec<u8>> {
    check_ascii_replacement(replacement)?;
    let length = edited_length(source, range, replacement.len())?;
    let mut bytes = Vec::with_capacity(length);
    bytes.extend_from_slice(&source.bytes[..range.start]);
    bytes.extend_from_slice(replacement);
    bytes.extend_from_slice(&source.bytes[range.end..]);
    Ok(bytes)
}

fn check_ascii_replacement(replacement: &[u8]) -> Result<()> {
    if let Some(position) = replacement.iter().position(|byte| !byte.is_ascii()) {
        return Err(ini_error(
            position,
            format!(
                "replacement byte {:#x}; expected ASCII",
                replacement[position]
            ),
        ));
    }
    Ok(())
}

fn edited_length(
    source: &ElectricalIniDocument,
    range: &Range<usize>,
    replacement: usize,
) -> Result<usize> {
    (source.bytes.len() - range.len())
        .checked_add(replacement)
        .filter(|length| *length <= source.limits.maximum_bytes)
        .ok_or_else(|| {
            ini_error(
                range.start,
                format!(
                    "replacement bytes {}; expected result <= {} bytes",
                    replacement, source.limits.maximum_bytes
                ),
            )
        })
}

fn verify_records(
    source: &ElectricalIniDocument,
    result: &ElectricalIniDocument,
    ordinal: usize,
    replacement: &[u8],
) -> Result<()> {
    if source.records.len() != result.records.len() {
        return Err(ini_error(
            ordinal,
            format!(
                "result records {}; expected {} unchanged lexical shapes",
                result.records.len(),
                source.records.len()
            ),
        ));
    }
    for index in 0..source.records.len() {
        verify_record(source, result, index, ordinal, replacement)?;
    }
    Ok(())
}

fn verify_record(
    source: &ElectricalIniDocument,
    result: &ElectricalIniDocument,
    index: usize,
    ordinal: usize,
    replacement: &[u8],
) -> Result<()> {
    let before = &source.records[index];
    let after = &result.records[index];
    let value = if index == ordinal {
        Some(replacement)
    } else {
        record_span(source, &before.value)
    };
    let same =
        matching_metadata(source, result, index) && value == record_span(result, &after.value);
    let untouched =
        index == ordinal || source.bytes[before.raw.clone()] == result.bytes[after.raw.clone()];
    if same && untouched {
        return Ok(());
    }
    let mismatch = format!(
        "result kind {:?}; expected source kind {:?}, same key/section/other records and exact selected value",
        after.kind, before.kind
    );
    Err(ini_error(index, mismatch))
}

fn matching_metadata(
    source: &ElectricalIniDocument,
    result: &ElectricalIniDocument,
    index: usize,
) -> bool {
    let before = &source.records[index];
    let after = &result.records[index];
    before.kind == after.kind
        && record_span(source, &before.section) == record_span(result, &after.section)
        && record_span(source, &before.key) == record_span(result, &after.key)
}

fn record_span<'a>(
    document: &'a ElectricalIniDocument,
    range: &Option<Range<usize>>,
) -> Option<&'a [u8]> {
    range.as_ref().map(|range| &document.bytes[range.clone()])
}

fn lexical_history(
    before: usize,
    after: usize,
    old: Range<usize>,
    replacement: usize,
    completion: ElectricalEditCompletion,
) -> [ElectricalByteHistory; 3] {
    let new_end = old.start + replacement;
    let changed = completion == ElectricalEditCompletion::Changed;
    [
        lexical_relation(0..old.start, 0..old.start, false),
        lexical_relation(old.clone(), old.start..new_end, changed),
        lexical_relation(old.end..before, new_end..after, false),
    ]
}

fn lexical_relation(
    parent: Range<usize>,
    result: Range<usize>,
    changed: bool,
) -> ElectricalByteHistory {
    ElectricalByteHistory {
        parent,
        result,
        relation: if changed {
            ElectricalByteRelation::Derived
        } else {
            ElectricalByteRelation::Copied
        },
    }
}

impl ElectricalIniEdit {
    /// Borrows the immutable source; e.g. `let raw = edit.source().to_bytes()`.
    pub fn source(&self) -> &ElectricalIniDocument {
        &self.source
    }
    /// Borrows the complete validated result; e.g. `let raw = edit.result().to_bytes()`.
    pub fn result(&self) -> &ElectricalIniDocument {
        &self.result
    }
    /// Reports changed versus completed no-op; e.g. `let status = edit.completion()`.
    pub fn completion(&self) -> ElectricalEditCompletion {
        self.completion
    }
    /// Reports exact resulting lexical coverage; e.g. `let checked = edit.coverage()`.
    pub fn coverage(&self) -> ElectricalIniCoverage {
        self.result.coverage()
    }
    /// Borrows full source/result intervals, including length changes; e.g. `let spans = edit.history()`.
    pub fn history(&self) -> &[ElectricalByteHistory; 3] {
        &self.history
    }
    /// Borrows value/native-load exclusions; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> &[ElectricalIniDiagnostic] {
        self.result.diagnostics()
    }
}
