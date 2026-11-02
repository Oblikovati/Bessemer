//! Stored dimension roles over the existing complete typed archive fields (#179).
mod accessors;
mod archive_ops;
mod projection;
#[cfg(test)]
#[path = "../../../tests/model_dimension_state/projection_unit.rs"]
mod projection_tests;

use super::{ModelFieldHandle, ModelFieldValue};
use crate::{BessemerError, FailureClass, Result};
use std::sync::Arc;

/// A stored dimension value request, independent of native setters and counter transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelDimensionStoredEdit {
    /// Replace only the bend-angle range flag without enforcing an angle range.
    EnforceBendAngleRange(bool),
    /// Replace bits 2 through 4 with zero through seven; larger requests fail.
    FilletRefPointChangeNumber(u8),
    /// Preserve the full signed redundant-identifier domain without resolving a handle.
    RedundantId(i32),
}

/// Owned stored dimension words with revision-bound handles in a complete part archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDimensionStoredState {
    mask: i32,
    redundant_id: i32,
    source_fields: [ModelFieldHandle; 2],
    path: Arc<str>,
}

impl ModelDimensionStoredState {
    fn replacement(
        &self,
        edit: ModelDimensionStoredEdit,
    ) -> Result<(&ModelFieldHandle, ModelFieldValue)> {
        let word = self.mask as u32;
        let value = match edit {
            ModelDimensionStoredEdit::EnforceBendAngleRange(enabled) => {
                (word & !2) | (u32::from(enabled) << 1)
            }
            ModelDimensionStoredEdit::FilletRefPointChangeNumber(number) if number <= 7 => {
                (word & !0x1c) | (u32::from(number) << 2)
            }
            ModelDimensionStoredEdit::FilletRefPointChangeNumber(number) => {
                return Err(self.counter_error(number));
            }
            ModelDimensionStoredEdit::RedundantId(identifier) => {
                return Ok((&self.source_fields[1], ModelFieldValue::I32(identifier)));
            }
        };
        Ok((&self.source_fields[0], ModelFieldValue::I32(value as i32)))
    }

    fn counter_error(&self, number: u8) -> BessemerError {
        BessemerError::new(
            "model dimension stored state",
            self.path.as_ref(),
            FailureClass::Invalid,
            format!(
                "fillet reference-point change number {number}; expected a stored value in 0..=7"
            ),
        )
    }
}
