//! Complete headerless named bitmap resource arrays (11.1661 / C-3345 / #190).
mod codec;
mod edit;
mod types;
pub use types::*;

use crate::{BessemerError, Document, FailureClass, Result, archive_codec::PreservedContents};

/// Immutable count/payload archive with an operation-local original encoding checkpoint.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitmapResourceArray {
    contents: PreservedContents<Vec<u8>>,
    preferred_width: codec::CountWidth,
    maximum_payload: usize,
}

impl BitmapResourceArray {
    /// Reads the entire logical array under a payload budget; e.g. `BitmapResourceArray::from_bytes(&[0, 0], 1024)?`.
    pub fn from_bytes(bytes: &[u8], maximum_payload: usize) -> Result<Self> {
        let (payload, preferred_width) =
            codec::decode(bytes, maximum_payload).map_err(|mut error| {
                error.operation = "bitmap resource array";
                error
            })?;
        Ok(Self {
            contents: PreservedContents::decoded(bytes, payload),
            preferred_width,
            maximum_payload,
        })
    }

    /// Reads an explicitly selected logical stream after container checks; e.g. `BitmapResourceArray::from_document(&document, "Contents/SketchBitmap0", 1_000_000)?`.
    pub fn from_document(
        document: &Document,
        stream: &str,
        maximum_payload: usize,
    ) -> Result<Self> {
        let bytes = document.read_stream_limited(stream, maximum_payload.saturating_add(6))?;
        Self::from_bytes(&bytes, maximum_payload).map_err(|mut error| {
            error.entity = stream.to_owned();
            error
        })
    }

    /// Constructs an independent array using the shortest count header; e.g. `BitmapResourceArray::new(vec![1, 2], 1024)?`.
    pub fn new(payload: Vec<u8>, maximum_payload: usize) -> Result<Self> {
        codec::validate_payload(payload.len(), maximum_payload)?;
        let preferred_width = codec::CountWidth::canonical(payload.len());
        Ok(Self {
            contents: PreservedContents::constructed(payload),
            preferred_width,
            maximum_payload,
        })
    }

    /// Borrows opaque bytes without image assumptions; e.g. `let first = resource.payload().first();`.
    pub fn payload(&self) -> &[u8] {
        self.contents.current()
    }

    /// Retains original no-op/revert bytes and count-width preference; e.g. `let logical_bytes = resource.to_bytes()?;`.
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(original) = self.contents.original_bytes() {
            return Ok(original.to_vec());
        }
        codec::encode(self)
    }
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "bitmap resource array",
        "logical count/payload archive",
        FailureClass::Invalid,
        cause,
    )
}
