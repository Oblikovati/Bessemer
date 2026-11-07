mod chunks;
mod construct;
mod edit;
mod filters;
mod header;
mod metadata;
mod order;
mod passes;
mod raster;
mod samples;
mod text;
mod types;
mod validate;
mod zlib;

use crate::{BessemerError, FailureClass, Result, archive_codec::PreservedContents};
use std::sync::Arc;
pub use types::{ChunkType, PngChunk, PngColor, PngContents, PngHeader, PngLimits, PngRaster};

/// Immutable static PNG archive with exact stored chunks and validated, unscaled samples.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PngPreview {
    contents: PreservedContents<PngContents>,
    raster: Arc<PngRaster>,
}

impl PngPreview {
    /// Constructs a new PNG from exact typed samples and an optional palette (empty for none).
    ///
    /// ```no_run
    /// # fn example(raster:bessemer::preview::PngRaster,limits:bessemer::preview::PngLimits)->bessemer::Result<()> {
    /// let preview=bessemer::preview::PngPreview::from_raster(raster,Vec::new(),limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_raster(
        raster: PngRaster,
        palette: Vec<[u8; 3]>,
        limits: PngLimits,
    ) -> Result<Self> {
        Self::new(construct::from_raster(&raster, &palette, limits)?, limits)
    }
    /// Checks complete framing, CRCs, metadata and pixels under caller-selected budgets.
    ///
    /// ```no_run
    /// # fn example(bytes:&[u8],limits:bessemer::preview::PngLimits)->bessemer::Result<()> {
    /// let preview=bessemer::preview::PngPreview::from_bytes(bytes,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(bytes: &[u8], limits: PngLimits) -> Result<Self> {
        let contents = chunks::decode(bytes, limits)?;
        let raster = Arc::new(validate::validate(&contents, limits)?);
        Ok(Self {
            contents: PreservedContents::decoded(bytes, contents),
            raster,
        })
    }

    /// Constructs and validates every stored chunk independently of an original encoded buffer.
    ///
    /// ```no_run
    /// # fn example(contents:bessemer::preview::PngContents,limits:bessemer::preview::PngLimits)->bessemer::Result<()> {
    /// let preview=bessemer::preview::PngPreview::new(contents,limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(contents: PngContents, limits: PngLimits) -> Result<Self> {
        let raster = Arc::new(validate::validate(&contents, limits)?);
        Ok(Self {
            contents: PreservedContents::constructed(contents),
            raster,
        })
    }

    /// Borrows immutable chunks, including their exact unknown ancillary payloads.
    ///
    /// ```no_run
    /// # fn example(preview:&bessemer::preview::PngPreview) {
    /// let chunks=&preview.contents().chunks;
    /// # }
    /// ```
    pub fn contents(&self) -> &PngContents {
        self.contents.current()
    }

    /// Borrows exact decoded channel values without applying gamma or ICC transforms.
    ///
    /// ```no_run
    /// # fn example(preview:&bessemer::preview::PngPreview) {
    /// let samples=&preview.raster().samples;
    /// # }
    /// ```
    pub fn raster(&self) -> &PngRaster {
        &self.raster
    }

    /// Expands palette/transparency and scales channels exactly to 16-bit RGBA, without color management.
    ///
    /// ```no_run
    /// # fn example(preview:&bessemer::preview::PngPreview) {
    /// let rgba=preview.rgba16();
    /// # }
    /// ```
    pub fn rgba16(&self) -> Vec<[u16; 4]> {
        raster::rgba16(&self.raster, self.contents())
    }

    /// Validates an owned edit and enforces PNG's unknown ancillary copy rules.
    ///
    /// ```no_run
    /// # fn example(preview:&bessemer::preview::PngPreview,limits:bessemer::preview::PngLimits)->bessemer::Result<()> {
    /// let changed=preview.with_contents(preview.contents().clone(),limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_contents(&self, contents: PngContents, limits: PngLimits) -> Result<Self> {
        let preserved = self.contents.edited(contents);
        if preserved.original_bytes().is_none() {
            edit::require_copy_policy(self.contents(), preserved.current())?;
        }
        let raster = Arc::new(validate::validate(preserved.current(), limits)?);
        Ok(Self {
            contents: preserved,
            raster,
        })
    }

    /// Replaces samples in the stored color/depth/interlace format with deterministic encoding.
    ///
    /// ```no_run
    /// # fn example(preview:&bessemer::preview::PngPreview,limits:bessemer::preview::PngLimits)->bessemer::Result<()> {
    /// let changed=preview.with_samples(preview.raster().samples.clone(),limits)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_samples(&self, samples: Vec<u16>, limits: PngLimits) -> Result<Self> {
        header::require_header(self.raster.header, limits)?;
        passes::expanded_size(self.raster.header, limits)?;
        if samples == self.raster.samples {
            return self.with_contents(self.contents().clone(), limits);
        }
        let raster = PngRaster {
            header: self.raster.header,
            samples,
        };
        raster::validate_samples(&raster, self.contents())?;
        self.with_contents(edit::replace_samples(self.contents(), &raster)?, limits)
    }

    /// Rebuilds changed chunks and preserves no-op, equivalent-edit and revert bytes exactly.
    ///
    /// ```no_run
    /// # fn example(preview:&bessemer::preview::PngPreview)->bessemer::Result<()> {
    /// let bytes=preview.to_bytes()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        if let Some(bytes) = self.contents.original_bytes() {
            return Ok(bytes.to_vec());
        }
        chunks::encode(self.contents())
    }
}

fn invalid(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "PNG preview",
        "PNG datastream",
        FailureClass::Invalid,
        cause,
    )
}
fn unsupported(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "PNG preview",
        "PNG datastream",
        FailureClass::Unsupported,
        cause,
    )
}
fn integrity(cause: impl Into<String>) -> BessemerError {
    BessemerError::new(
        "PNG preview",
        "PNG datastream",
        FailureClass::Integrity,
        cause,
    )
}
