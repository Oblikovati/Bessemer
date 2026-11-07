//! Exact version-7.9 packed-array wire operations and explicit software-profile values.
mod binary32;
mod indices;
mod types;
mod value_types;
mod values;
mod vectors;

use crate::{BessemerError, FailureClass, Result};
pub use indices::ComposerIndexBlock;
pub(super) use indices::read_index_prefix;
pub use types::*;
pub use value_types::*;
pub use values::ComposerVectorValues;
pub use vectors::ComposerVectorBlock;
pub(super) use vectors::read_vector_prefix;

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "composer packed array",
        "complete block",
        FailureClass::Invalid,
        cause,
    )
}

fn bounded_size(count: usize, width: usize, maximum: usize) -> Result<usize> {
    let size = count.checked_mul(width).ok_or_else(|| {
        invalid(format!(
            "count {count} * width {width}; expected representable packed size"
        ))
    })?;
    require_budget(size, maximum)?;
    Ok(size)
}

fn require_budget(size: usize, maximum: usize) -> Result<()> {
    if size <= maximum {
        return Ok(());
    }
    Err(invalid(format!(
        "wire bytes {size}; expected at most {maximum}"
    )))
}

fn read_error(mut error: BessemerError) -> BessemerError {
    error.operation = "composer packed array";
    error
}

fn require_length(actual: usize, expected: usize, context: impl std::fmt::Display) -> Result<()> {
    if actual == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "{context}: length {actual}; expected {expected}"
    )))
}

fn bounded_total(first: usize, second: usize, maximum: usize) -> Result<usize> {
    let total = first.checked_add(second).ok_or_else(|| {
        invalid(format!(
            "sizes {first} + {second}; expected representable complete wire size"
        ))
    })?;
    require_budget(total, maximum)?;
    Ok(total)
}

fn require_reconstruction(actual: &[u8], expected: &[u8]) -> Result<()> {
    if actual == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "reconstructed {} bytes differ; expected exact original {} bytes",
        actual.len(),
        expected.len()
    )))
}

fn packed_values(bytes: &[u8], width: usize) -> impl ExactSizeIterator<Item = u32> + '_ {
    bytes.chunks_exact(width).map(move |chunk| {
        let mut word = [0; 4];
        word[..width].copy_from_slice(chunk);
        u32::from_le_bytes(word)
    })
}

fn edit_result<T>(
    parent: &ComposerArrayRevision,
    revision: &ComposerArrayRevision,
    block: T,
    coverage: ComposerArrayCoverage,
) -> ComposerArrayEdit<T> {
    ComposerArrayEdit {
        block,
        coverage,
        fidelity: ComposerArrayFidelity::ExactStoredValues,
        completion: edit_completion(parent == revision),
        history: edit_history(parent, revision),
        diagnostics: ARRAY_DIAGNOSTICS,
    }
}

fn edit_completion(unchanged: bool) -> ComposerArrayCompletion {
    if unchanged {
        return ComposerArrayCompletion::Unchanged;
    }
    ComposerArrayCompletion::Changed
}

fn edit_history(
    parent: &ComposerArrayRevision,
    revision: &ComposerArrayRevision,
) -> ComposerArrayHistory {
    if parent == revision {
        return ComposerArrayHistory::Unchanged(parent.clone());
    }
    ComposerArrayHistory::Derived {
        parent: parent.clone(),
        result: revision.clone(),
    }
}

const ARRAY_DIAGNOSTICS: [ComposerArrayDiagnostic; 3] = [
    ComposerArrayDiagnostic::FloatingReconstructionNotChecked,
    ComposerArrayDiagnostic::GeometryNotChecked,
    ComposerArrayDiagnostic::NativeApplicationNotChecked,
];
