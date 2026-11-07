//! Exact lexical references and explicit supplied-state lookup (14.92–14.93).
mod paths;
mod server;
mod text;
mod types;

use crate::{BessemerError, FailureClass, Result};
pub use server::ComposerGeodeServer;
pub use text::ComposerReferenceText;
pub use types::*;

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "composer supplied references",
        "reference/context",
        FailureClass::Invalid,
        cause,
    )
}
