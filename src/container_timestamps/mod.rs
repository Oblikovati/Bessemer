//! Explicit native ZIP timestamp arithmetic and isolated existing-entry metadata edits.
mod arithmetic;
mod edit;
mod read;
mod types;
mod view;
use crate::archive_codec::ArchiveRevision;
use crate::{BessemerError, Document, FailureClass, Result};
use std::sync::Arc;
pub use types::*;

fn failure(entity: &str, class: FailureClass, cause: impl Into<String>) -> BessemerError {
    BessemerError::new("ZIP entry timestamps", entity, class, cause)
}
