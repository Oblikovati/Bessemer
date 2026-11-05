use super::*;
use crate::database_hash::{HashBufferCompletion, HashBufferHistory, HashBufferRelation};
use std::ops::Range;

/// Isolated immutable source/result, completion, exact coverage and ordered history.
#[derive(Clone, Debug)]
pub struct OverflowPayloadEdit {
    source: OverflowPayload,
    result: OverflowPayload,
    completion: HashBufferCompletion,
    history: Vec<OverflowPageHistory>,
}

impl OverflowPayload {
    /// Replaces complete same-length value bytes; e.g. `let edit = payload.edit_value(payload.value())?`.
    pub fn edit_value(&self, replacement: &[u8]) -> Result<OverflowPayloadEdit> {
        check_replacement_length(self.value.len(), replacement.len())?;
        let pages = replace_pages(self, replacement)?;
        let result = Self::from_pages(
            &pages,
            self.first,
            self.key.len() as u32,
            self.value.len() as u64,
            self.context,
        )?;
        verify_result(self, &result, replacement)?;
        Ok(publish_edit(self, result))
    }
}

fn check_replacement_length(expected: usize, actual: usize) -> Result<()> {
    if expected != actual {
        return Err(invalid(
            "value replacement",
            format!(
                "length {actual}; expected existing length {expected} for isolated storage edit"
            ),
        ));
    }
    Ok(())
}

fn verify_result(
    source: &OverflowPayload,
    result: &OverflowPayload,
    replacement: &[u8],
) -> Result<()> {
    if result.key != source.key
        || result.value.as_ref() != replacement
        || result.value_spans != source.value_spans
    {
        return Err(invalid(
            "replacement validation",
            "reparsed payload differs; expected unchanged key/spans and complete replacement",
        ));
    }
    Ok(())
}

fn publish_edit(source: &OverflowPayload, result: OverflowPayload) -> OverflowPayloadEdit {
    let completion = if result.pages == source.pages {
        HashBufferCompletion::Unchanged
    } else {
        HashBufferCompletion::Changed
    };
    let history = page_history(source, completion);
    OverflowPayloadEdit {
        source: source.clone(),
        result,
        completion,
        history,
    }
}

fn replace_pages(source: &OverflowPayload, replacement: &[u8]) -> Result<Vec<OverflowPageImage>> {
    let mut pages = source.pages.clone();
    let mut consumed = 0;
    for span in &source.value_spans {
        let index = pages
            .binary_search_by_key(&span.page, |page| page.identifier)
            .map_err(|_| {
                invalid(
                    span.page,
                    "missing value page; expected retained source identity",
                )
            })?;
        let end = consumed + span.range.len();
        pages[index].bytes[span.range.clone()].copy_from_slice(&replacement[consumed..end]);
        consumed = end;
    }
    Ok(pages)
}

fn page_history(
    source: &OverflowPayload,
    completion: HashBufferCompletion,
) -> Vec<OverflowPageHistory> {
    let spans: std::collections::BTreeMap<_, _> = source
        .value_spans
        .iter()
        .map(|span| (span.page, &span.range))
        .collect();
    let mut history = Vec::new();
    for page in &source.pages {
        let Some(selected) = spans.get(&page.identifier) else {
            history.push(history_span(page.identifier, 0..page.bytes.len(), false));
            continue;
        };
        append_value_history(&mut history, page, selected, completion);
    }
    history
}

fn append_value_history(
    history: &mut Vec<OverflowPageHistory>,
    page: &OverflowPageImage,
    selected: &Range<usize>,
    completion: HashBufferCompletion,
) {
    history.push(history_span(page.identifier, 0..selected.start, false));
    history.push(history_span(
        page.identifier,
        selected.clone(),
        completion == HashBufferCompletion::Changed,
    ));
    history.push(history_span(
        page.identifier,
        selected.end..page.bytes.len(),
        false,
    ));
}

fn history_span(identifier: NonZeroU64, range: Range<usize>, derived: bool) -> OverflowPageHistory {
    OverflowPageHistory {
        identifier,
        span: HashBufferHistory {
            parent: range.clone(),
            result: range,
            relation: if derived {
                HashBufferRelation::Derived
            } else {
                HashBufferRelation::Copied
            },
        },
    }
}

impl OverflowPayloadEdit {
    /// Borrows the immutable source checkpoint; e.g. `let bytes = edit.source().pages()`.
    pub fn source(&self) -> &OverflowPayload {
        &self.source
    }
    /// Borrows the revalidated complete result; e.g. `let value = edit.result().value()`.
    pub fn result(&self) -> &OverflowPayload {
        &self.result
    }
    /// Returns exact changed/no-op completion; e.g. `let completion = edit.completion()`.
    pub fn completion(&self) -> HashBufferCompletion {
        self.completion
    }
    /// Reports exact local coverage and zero achieved byte error; e.g. `let checked = edit.coverage()`.
    pub fn coverage(&self) -> OverflowCoverage {
        self.result.coverage
    }
    /// Borrows page-ID/byte-ordered relations covering every supplied byte; e.g. `let history = edit.history()`.
    pub fn history(&self) -> &[OverflowPageHistory] {
        &self.history
    }
    /// Returns provider and interpretation exclusions; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> [OverflowDiagnostic; 4] {
        [
            OverflowDiagnostic::ProviderAssumed,
            OverflowDiagnostic::NativeKeyReaderDiffers,
            OverflowDiagnostic::OpaqueValues,
            OverflowDiagnostic::FileAndAllocationNotValidated,
        ]
    }
}
