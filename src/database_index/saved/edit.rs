use super::*;
use crate::database_hash::{HashBufferCompletion, HashBufferHistory, span_history};
use std::ops::Range;

/// Isolated saved-index operation with complete bytes, qualifications and history.
#[derive(Clone, Debug)]
pub struct HashFileIndexEdit {
    source: HashFileIndexBuffer,
    result: HashFileIndexBuffer,
    completion: HashBufferCompletion,
    history: [HashBufferHistory; 3],
}

impl HashFileIndexBuffer {
    /// Replaces raw saved words without graph repair; e.g. `buffer.edit_header(header)?`.
    pub fn edit_header(&self, replacement: HashIndexHeader) -> Result<HashFileIndexEdit> {
        let mut expected = self.index.clone();
        expected.free_head = replacement.free_head;
        expected.split = replacement.split;
        expected.maximum = replacement.maximum;
        let encoded = encoded_words(&[
            replacement.free_head,
            replacement.split,
            replacement.maximum,
        ]);
        let start = self.context.pager_extent + 8;
        saved_edit(self, expected, start..start + encoded.len(), &encoded)
    }
    /// Replaces one ordered pair by its saved ordinal; e.g. `buffer.edit_mapping(0, pair)?`.
    pub fn edit_mapping(
        &self,
        ordinal: usize,
        replacement: HashFileMapEntry,
    ) -> Result<HashFileIndexEdit> {
        let start = self.index.locations.get(ordinal).copied().ok_or_else(|| {
            BessemerError::new(
                "saved index edit",
                &ordinal.to_string(),
                FailureClass::Missing,
                format!(
                    "mapping ordinal {ordinal}; expected 0..{}",
                    self.index.entries.len()
                ),
            )
        })?;
        let mut expected = self.index.clone();
        expected.entries[ordinal] = replacement;
        let encoded = encoded_words(&[replacement.logical, replacement.physical]);
        saved_edit(self, expected, start..start + encoded.len(), &encoded)
    }
}

fn encoded_words(words: &[u64]) -> Vec<u8> {
    words.iter().flat_map(|word| word.to_be_bytes()).collect()
}

fn saved_edit(
    source: &HashFileIndexBuffer,
    expected: FileIndex,
    range: Range<usize>,
    replacement: &[u8],
) -> Result<HashFileIndexEdit> {
    let mut bytes = source.to_bytes();
    replace_saved_span(&mut bytes, range.clone(), replacement)?;
    let result = HashFileIndexBuffer::from_bytes(&bytes, source.context)?;
    check_saved_result(source, &result, &expected)?;
    let completion = if source.bytes == result.bytes {
        HashBufferCompletion::Unchanged
    } else {
        HashBufferCompletion::Changed
    };
    let history = span_history(bytes.len(), range, completion);
    Ok(HashFileIndexEdit {
        source: source.clone(),
        result,
        completion,
        history,
    })
}

fn check_saved_result(
    source: &HashFileIndexBuffer,
    result: &HashFileIndexBuffer,
    expected: &FileIndex,
) -> Result<()> {
    if result.index != *expected || result.coverage != source.coverage {
        return Err(invalid(
            "saved index edit",
            format!(
                "reparsed index {:?}, coverage {:?}; expected index {expected:?}, coverage {:?}",
                result.index, result.coverage, source.coverage
            ),
        ));
    }
    Ok(())
}

fn replace_saved_span(bytes: &mut [u8], range: Range<usize>, replacement: &[u8]) -> Result<()> {
    let length = bytes.len();
    let selected = bytes.get_mut(range.clone()).ok_or_else(|| {
        invalid(
            "saved index span",
            format!("range {range:?}; expected within {length} bytes"),
        )
    })?;
    if selected.len() != replacement.len() {
        return Err(invalid(
            "saved index span",
            format!(
                "replacement bytes {}; expected {} for {range:?}",
                replacement.len(),
                selected.len()
            ),
        ));
    }
    selected.copy_from_slice(replacement);
    Ok(())
}

impl HashFileIndexEdit {
    /// Borrows the isolated source checkpoint; e.g. `let raw = edit.source().to_bytes()`.
    pub fn source(&self) -> &HashFileIndexBuffer {
        &self.source
    }
    /// Borrows the complete revalidated saved index; e.g. `let raw = edit.result().to_bytes()`.
    pub fn result(&self) -> &HashFileIndexBuffer {
        &self.result
    }
    /// Distinguishes byte equality from changes; e.g. `let status = edit.completion()`.
    pub fn completion(&self) -> HashBufferCompletion {
        self.completion
    }
    /// Reports index-only checks and zero reparse error; e.g. `let checked = edit.coverage()`.
    pub fn coverage(&self) -> HashIndexCoverage {
        self.result.coverage
    }
    /// Borrows complete byte-ordered parent/result spans; e.g. `let history = edit.history()`.
    pub fn history(&self) -> &[HashBufferHistory; 3] {
        &self.history
    }
    /// Carries provider and graph/lookup exclusions; e.g. `let exclusions = edit.diagnostics()`.
    pub fn diagnostics(&self) -> [HashIndexDiagnostic; 4] {
        [
            HashIndexDiagnostic::ProviderAssumed,
            HashIndexDiagnostic::OpaqueRemainder,
            HashIndexDiagnostic::DuplicatePrecedenceNotInferred,
            HashIndexDiagnostic::FileGraphAndApplicationNotValidated,
        ]
    }
}
