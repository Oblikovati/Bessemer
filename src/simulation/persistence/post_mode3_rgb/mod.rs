mod array;
mod group;
mod plan;
mod storage;
mod types;
mod views;
use crate::{BessemerError, FailureClass, Result};
pub use array::*;
pub use plan::*;
pub use storage::*;
pub use types::*;

fn mode3_required<T: Copy + std::fmt::Debug>(
    value: Option<T>,
    entity: &str,
    expected: &str,
) -> Result<T> {
    value.ok_or_else(|| {
        mode3_failure(
            entity,
            FailureClass::Missing,
            format!("fact {value:?}; expected {expected} at the reached local stage"),
        )
    })
}
fn mode3_failure(entity: &str, class: FailureClass, cause: String) -> BessemerError {
    BessemerError::new("mode3 RGB operation", entity, class, cause)
}
