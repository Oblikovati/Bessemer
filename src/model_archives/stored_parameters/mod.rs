//! Complete stored parameter snapshots and exact, non-destructive value edits (#202).
mod accessors;
mod edit;
mod names;
mod projection;
mod source;
mod types;

use super::*;
use crate::{BessemerError, FailureClass, Result};
pub use types::*;

fn parameter_error(actual: impl std::fmt::Debug) -> BessemerError {
    BessemerError::new(
        "stored parameter",
        "selected parameter",
        FailureClass::Invalid,
        format!(
            "{actual:?}; expected a complete physical parameter definition with its declared stored field types"
        ),
    )
}
