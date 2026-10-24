mod cursor;
mod objects;
mod preserved;
mod text;

pub(crate) use cursor::ArchiveCursor;
pub(crate) use objects::{ArchiveClass, ArchiveMapIndex, ObjectReader, ObjectRecord, ObjectWriter};
pub(crate) use preserved::PreservedContents;
pub use text::{ArchiveString, TextEncoding};

use crate::{BessemerError, FailureClass};

pub(crate) fn malformed(cause: impl Into<String>) -> BessemerError {
    BessemerError::new("archive codec", "archive", FailureClass::Invalid, cause)
}
