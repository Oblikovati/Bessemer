//! Packed DIB preview reading, immutable editing, and lossless archive preservation.
mod codec;
mod header;
mod pixels;
mod rle;
mod types;

use crate::{BessemerError, FailureClass, Result, archive_codec::PreservedContents};
pub use types::{BitmapInfoHeader, DibContents, DibPixels, PaletteEntry};

/// Immutable packed DIB archive; a raster budget is mandatory at untrusted boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DibPreview(PreservedContents<DibContents>);

impl DibPreview {
    /// Decodes and validates the complete packed stream and its pixel commands.
    ///
    /// ```no_run
    /// # fn example(bytes: &[u8]) -> bessemer::Result<()> {
    /// let preview = bessemer::preview::DibPreview::from_bytes(bytes, 16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(bytes: &[u8], maximum_pixels: usize) -> Result<Self> {
        let contents = codec::decode(bytes, maximum_pixels)?;
        Ok(Self(PreservedContents::decoded(bytes, contents)))
    }

    /// Constructs a validated preview independently of an original encoded stream.
    ///
    /// ```no_run
    /// # fn example(contents: bessemer::preview::DibContents) -> bessemer::Result<()> {
    /// let preview = bessemer::preview::DibPreview::new(contents, 16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(contents: DibContents, maximum_pixels: usize) -> Result<Self> {
        codec::validate(&contents, maximum_pixels)?;
        Ok(Self(PreservedContents::constructed(contents)))
    }

    /// Borrows every immutable archived header, palette, and image byte.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview) {
    /// let header = &preview.contents().header;
    /// # }
    /// ```
    pub fn contents(&self) -> &DibContents {
        self.0.current()
    }

    /// Validates an owned edit before publication, leaving this preview isolated.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview) -> bessemer::Result<()> {
    /// let edited = preview.with_contents(preview.contents().clone(), 16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_contents(&self, contents: DibContents, maximum_pixels: usize) -> Result<Self> {
        codec::validate(&contents, maximum_pixels)?;
        Ok(Self(self.0.edited(contents)))
    }

    /// Preserves equivalent edits and reverts byte for byte; rebuilds changed stored fields.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview) -> bessemer::Result<()> {
    /// let bytes = preview.to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.0.original_bytes() {
            return Ok(bytes.to_vec());
        }
        codec::encode(self.contents())
    }
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new("DIB preview", "packed bitmap", FailureClass::Invalid, cause)
}
fn unsupported(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "DIB preview",
        "packed bitmap",
        FailureClass::Unsupported,
        cause,
    )
}
