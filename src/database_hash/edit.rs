use super::*;
use std::ops::Range;

/// Immutable source/result snapshots and exact local coverage/history of a value replacement.
#[derive(Clone, Debug)]
pub struct HashBufferEdit {
    source: HashPageBuffer,
    buffer: HashPageBuffer,
    completion: HashBufferCompletion,
    history: [HashBufferHistory; 3],
}

impl HashPageBuffer {
    /// Replaces one complete value without moving cells; e.g. `let edit = buffer.edit_value(cell.offset(), cell.value())?`.
    pub fn edit_value(&self, cell_offset: u16, value: &[u8]) -> Result<HashBufferEdit> {
        let range = self.replacement_range(cell_offset, value.len())?;
        let mut bytes = self.to_bytes();
        bytes[range.clone()].copy_from_slice(value);
        let buffer = Self::from_bytes(&bytes, self.context)?;
        verify_replacement(self, &buffer, cell_offset, value)?;
        Ok(publish(self, buffer, range))
    }

    fn cell_at(&self, cell_offset: u16) -> Result<&HashInlineCell> {
        self.cells
            .iter()
            .find(|cell| cell.offset == cell_offset)
            .ok_or_else(|| {
                invalid(format!(
                    "cell offset {cell_offset}; expected an existing decoded cell"
                ))
            })
    }

    fn replacement_range(&self, cell_offset: u16, length: usize) -> Result<Range<usize>> {
        let cell = self.cell_at(cell_offset)?;
        if cell.value.len() != length {
            return Err(invalid(format!(
                "value length {}; expected {} bytes at cell {cell_offset} for an isolated fixed-extent replacement",
                length,
                cell.value.len()
            )));
        }
        Ok(cell.value_range())
    }
}

fn publish(source: &HashPageBuffer, buffer: HashPageBuffer, range: Range<usize>) -> HashBufferEdit {
    let completion = if source.bytes == buffer.bytes {
        HashBufferCompletion::Unchanged
    } else {
        HashBufferCompletion::Changed
    };
    let history = span_history(source.bytes.len(), range, completion);
    HashBufferEdit {
        source: source.clone(),
        buffer,
        completion,
        history,
    }
}

fn verify_replacement(
    source: &HashPageBuffer,
    result: &HashPageBuffer,
    offset: u16,
    value: &[u8],
) -> Result<()> {
    let mut expected = source.cells.clone();
    let cell = expected
        .iter_mut()
        .find(|cell| cell.offset == offset)
        .ok_or_else(|| invalid(format!("cell {offset}; expected retained identity")))?;
    cell.value = value.into();
    if source.header != result.header || source.free != result.free || expected != result.cells {
        return Err(invalid(format!(
            "replacement at cell {offset}; expected exact other fields and complete new value"
        )));
    }
    Ok(())
}

pub(crate) fn span_history(
    length: usize,
    value: Range<usize>,
    completion: HashBufferCompletion,
) -> [HashBufferHistory; 3] {
    let changed = completion == HashBufferCompletion::Changed;
    [
        span(0..value.start, false),
        span(value.clone(), changed),
        span(value.end..length, false),
    ]
}

pub(super) fn span(range: Range<usize>, derived: bool) -> HashBufferHistory {
    HashBufferHistory {
        parent: range.clone(),
        result: range,
        relation: if derived {
            HashBufferRelation::Derived
        } else {
            HashBufferRelation::Copied
        },
    }
}

impl HashBufferEdit {
    /// Borrows the isolated source checkpoint; e.g. `let original = edit.source().to_bytes()`.
    pub fn source(&self) -> &HashPageBuffer {
        &self.source
    }
    /// Borrows the revalidated result; e.g. `let bytes = edit.buffer().to_bytes()`.
    pub fn buffer(&self) -> &HashPageBuffer {
        &self.buffer
    }
    /// Returns changed versus exact no-op completion; e.g. `let completion = edit.completion()`.
    pub fn completion(&self) -> HashBufferCompletion {
        self.completion
    }
    /// Reports exact local checks and zero achieved reparse error; e.g. `let coverage = edit.coverage()`.
    pub fn coverage(&self) -> HashBufferCoverage {
        self.buffer.coverage
    }
    /// Returns ordered full-span parent/result relations; e.g. `let spans = edit.history()`.
    pub fn history(&self) -> &[HashBufferHistory; 3] {
        &self.history
    }
    /// Carries the explicit interpretation/provider exclusions; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> [HashBufferDiagnostic; 4] {
        [
            HashBufferDiagnostic::OpaqueValues,
            HashBufferDiagnostic::FreeSizesNotAllocatorValidation,
            HashBufferDiagnostic::PageGraphNotValidated,
            HashBufferDiagnostic::ProviderAssumed,
        ]
    }
}
