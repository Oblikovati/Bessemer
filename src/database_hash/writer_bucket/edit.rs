use super::super::edit::{span as history_span, span_history};
use super::*;
use std::{collections::BTreeMap, ops::Range};

/// Immutable complete bucket edit with source, result, coverage and ordered page history.
#[derive(Clone, Debug)]
pub struct HashWriterEdit {
    source: HashWriterBucket,
    result: HashWriterBucket,
    completion: HashBufferCompletion,
    history: Vec<HashWriterHistory>,
}

impl HashWriterBucket {
    /// Replaces one same-length cell value without relocating storage; e.g. `let edit = bucket.edit_value(offset, &replacement)?`.
    pub fn edit_value(&self, offset: u16, replacement: &[u8]) -> Result<HashWriterEdit> {
        let index = self
            .cells
            .iter()
            .position(|c| c.offset() == offset)
            .ok_or_else(|| invalid(format!("cell {offset}; expected existing bucket cell")))?;
        let (cell, spans) = edited_cell(&self.cells[index], replacement)?;
        let mut bytes = self.to_bytes();
        let mut pages = self.to_overflow_pages();
        write_cell(&cell, &mut bytes, &mut pages)?;
        let result = Self::from_bytes(&bytes, &pages, self.context, self.maximum_entries)?;
        verify_result(self, &result, index, &cell)?;
        Ok(publish(self, result, &cell, spans))
    }
}

fn publish(
    source: &HashWriterBucket,
    result: HashWriterBucket,
    cell: &HashWriterCell,
    spans: Vec<OverflowPageHistory>,
) -> HashWriterEdit {
    let completion = if source.bytes == result.bytes && source.cells == result.cells {
        HashBufferCompletion::Unchanged
    } else {
        HashBufferCompletion::Changed
    };
    let history = history(source, cell, spans, completion);
    HashWriterEdit {
        source: source.clone(),
        result,
        completion,
        history,
    }
}

fn edited_cell(
    source: &HashWriterCell,
    replacement: &[u8],
) -> Result<(HashWriterCell, Vec<OverflowPageHistory>)> {
    check_replacement(source, replacement)?;
    let mut cell = source.clone();
    let spans = match &mut cell.payload {
        WriterPayload::Inline(p) => {
            p.value = replacement.into();
            Vec::new()
        }
        WriterPayload::Overflow { payload, .. } => {
            let edit = payload.edit_value(replacement)?;
            let spans = edit.history().to_vec();
            *payload = edit.result().clone();
            spans
        }
    };
    Ok((cell, spans))
}

fn check_replacement(source: &HashWriterCell, replacement: &[u8]) -> Result<()> {
    if source.value().len() != replacement.len() {
        return Err(invalid(format!(
            "cell {} replacement length {}; expected {}",
            source.offset(),
            replacement.len(),
            source.value().len()
        )));
    }
    Ok(())
}

fn inline_range(cell: &HashWriterCell) -> Range<usize> {
    let start = usize::from(cell.offset()) + 26 + cell.key().len();
    start..start + cell.value().len()
}

fn write_cell(
    cell: &HashWriterCell,
    bytes: &mut [u8],
    pages: &mut [OverflowPageImage],
) -> Result<()> {
    match &cell.payload {
        WriterPayload::Inline(p) => bytes[inline_range(cell)].copy_from_slice(&p.value),
        WriterPayload::Overflow { payload, .. } => {
            for page in payload.pages() {
                let index = pages
                    .binary_search_by_key(&page.identifier, |p| p.identifier)
                    .map_err(|_| {
                        invalid(format!(
                            "page {}; expected retained source identity",
                            page.identifier
                        ))
                    })?;
                pages[index].clone_from(page);
            }
        }
    }
    Ok(())
}

fn verify_result(
    source: &HashWriterBucket,
    result: &HashWriterBucket,
    index: usize,
    cell: &HashWriterCell,
) -> Result<()> {
    let mut expected = source.cells.clone();
    expected[index] = cell.clone();
    if result.cells != expected
        || result.header != source.header
        || result.free != source.free
        || result.coverage != source.coverage
    {
        return Err(invalid(format!(
            "cell {}; expected complete replacement and unchanged metadata/other cells/coverage",
            cell.offset()
        )));
    }
    Ok(())
}

fn history(
    source: &HashWriterBucket,
    cell: &HashWriterCell,
    selected: Vec<OverflowPageHistory>,
    completion: HashBufferCompletion,
) -> Vec<HashWriterHistory> {
    let mut rows = bucket_history(source, cell, completion);
    let mut grouped: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for span in selected {
        grouped.entry(span.identifier).or_default().push(span);
    }
    for page in source.to_overflow_pages() {
        if let Some(spans) = grouped.remove(&page.identifier) {
            rows.extend(spans.into_iter().map(HashWriterHistory::Overflow));
            continue;
        }
        let span = history_span(0..page.bytes.len(), false);
        rows.push(HashWriterHistory::Overflow(OverflowPageHistory {
            identifier: page.identifier,
            span,
        }));
    }
    rows
}

fn bucket_history(
    source: &HashWriterBucket,
    cell: &HashWriterCell,
    completion: HashBufferCompletion,
) -> Vec<HashWriterHistory> {
    if cell.overflow_identifier().is_some() {
        return vec![HashWriterHistory::Bucket(history_span(
            0..source.bytes.len(),
            false,
        ))];
    }
    span_history(source.bytes.len(), inline_range(cell), completion)
        .into_iter()
        .map(HashWriterHistory::Bucket)
        .collect()
}

impl HashWriterEdit {
    /// Borrows the immutable source; e.g. `let original = edit.source().to_bytes()`.
    pub fn source(&self) -> &HashWriterBucket {
        &self.source
    }
    /// Borrows the complete revalidated result; e.g. `let pages = edit.result().to_overflow_pages()`.
    pub fn result(&self) -> &HashWriterBucket {
        &self.result
    }
    /// Reports changed versus exact no-op completion; e.g. `let status = edit.completion()`.
    pub fn completion(&self) -> HashBufferCompletion {
        self.completion
    }
    /// Reports exact local coverage and zero reparse error; e.g. `let checked = edit.coverage()`.
    pub fn coverage(&self) -> HashWriterCoverage {
        self.result.coverage
    }
    /// Borrows complete bucket/page-ordered copied and derived relations; e.g. `let spans = edit.history()`.
    pub fn history(&self) -> &[HashWriterHistory] {
        &self.history
    }
    /// Carries explicit provider/value/file/allocator qualifications; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> [OverflowDiagnostic; 4] {
        [
            OverflowDiagnostic::ProviderAssumed,
            OverflowDiagnostic::NativeKeyReaderDiffers,
            OverflowDiagnostic::OpaqueValues,
            OverflowDiagnostic::FileAndAllocationNotValidated,
        ]
    }
}
