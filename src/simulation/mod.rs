//! Stored Simulation images with actual paired MAS context and explicit semantic limits.
mod bodyranges;
mod egr;
mod elements;
mod elf;
mod r#gen;
mod lbc_property;
mod locator;
mod master;
mod metadata;
mod out;
mod persistence;
mod study;
mod wire;

use crate::{BessemerError, FailureClass, Result};
pub use bodyranges::*;
pub use egr::*;
pub use elements::*;
pub use elf::*;
pub use r#gen::*;
pub use lbc_property::*;
pub use locator::*;
pub use master::*;
pub use metadata::*;
pub use out::*;
pub use persistence::*;
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
