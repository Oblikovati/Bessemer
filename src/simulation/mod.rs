//! Stored Simulation images with actual paired MAS context and explicit semantic limits.
mod r#gen;
mod master;
mod out;
mod study;
mod wire;

use crate::{BessemerError, FailureClass, Result};
pub use r#gen::*;
pub use master::*;
pub use out::*;
pub use study::*;

fn invalid(entity: &str, cause: impl Into<String>) -> BessemerError {
    BessemerError::new("simulation images", entity, FailureClass::Invalid, cause)
}

fn unsupported(entity: &str, cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "simulation images",
        entity,
        FailureClass::Unsupported,
        cause,
    )
}

fn require(condition: bool, entity: &str, cause: impl Into<String>) -> Result<()> {
    if condition {
        return Ok(());
    }
    Err(invalid(entity, cause))
}
