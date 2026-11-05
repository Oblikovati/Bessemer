use super::{
    DibContents, DibPixels, DibPreview, codec, indexed_rgb, invalid, rle,
    types::{DibEncoding, ImageShape},
    unsupported,
};
use crate::Result;

impl DibPreview {
    /// Decodes top-to-bottom pixels without inventing values for unwritten RLE positions.
    /// For 24-bit BGR, use `decode_bgr24_pixels` to retain three-byte pixel values.
    /// For 16-bit RGB555, use `decode_bgr555_words` to retain every stored word bit.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview) -> bessemer::Result<()> {
    /// let pixels = preview.decode_pixels(16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn decode_pixels(&self, maximum_pixels: usize) -> Result<DibPixels> {
        let contents = self.contents();
        let shape = codec::validate(contents, maximum_pixels)?;
        match shape.encoding {
            DibEncoding::Bgrx32 => Ok(DibPixels::Bgrx(decode_bgrx(&contents.pixel_bytes, shape))),
            DibEncoding::Rle8 => decode_rle(contents, shape),
            DibEncoding::IndexedRgb(bits) => Ok(DibPixels::Indexed(indexed_rgb::decode(
                &contents.pixel_bytes,
                shape,
                bits,
            )?)),
            DibEncoding::Bgr24 => Err(unsupported(
                "24-bit BI_RGB pixels; expected decode_bgr24_pixels for three-byte BGR values",
            )),
            DibEncoding::Bgr555 => Err(unsupported(
                "16-bit BI_RGB pixels; expected decode_bgr555_words for raw RGB555 words",
            )),
        }
    }

    /// Replaces indexed pixels, using deterministic RLE8 runs or preserving RGB packing bits.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview, indices: &[u8]) -> bessemer::Result<()> {
    /// let edited = preview.with_indexed_pixels(indices, 16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_indexed_pixels(&self, indices: &[u8], maximum_pixels: usize) -> Result<Self> {
        let shape = self.indexed_replacement_shape(indices.len(), maximum_pixels)?;
        require_palette_indices(indices, self.contents().palette.len())?;
        let bytes = if let DibEncoding::IndexedRgb(bits) = shape.encoding {
            indexed_rgb::replace(&self.contents().pixel_bytes, shape, bits, indices)?
        } else {
            rle::encode_indices(indices, shape)
        };
        self.with_pixel_bytes(bytes, maximum_pixels)
    }

    /// Replaces BGRX pixels in top-to-bottom order, preserving their fourth bytes.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview, pixels: &[[u8; 4]]) -> bessemer::Result<()> {
    /// let edited = preview.with_bgrx_pixels(pixels, 16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_bgrx_pixels(&self, pixels: &[[u8; 4]], maximum_pixels: usize) -> Result<Self> {
        let shape = self.replacement_shape(DibEncoding::Bgrx32, pixels.len(), maximum_pixels)?;
        let mut bytes = Vec::new();
        for row in 0..shape.height {
            let offset = stored_row(row, shape) * shape.width;
            bytes.extend(pixels[offset..offset + shape.width].iter().flatten());
        }
        self.with_pixel_bytes(bytes, maximum_pixels)
    }

    pub(super) fn replacement_shape(
        &self,
        expected: DibEncoding,
        count: usize,
        maximum_pixels: usize,
    ) -> Result<ImageShape> {
        let shape = codec::validate(self.contents(), maximum_pixels)?;
        if shape.encoding != expected {
            return Err(unsupported(format!(
                "replacement format {expected:?}; expected source format {:?}",
                shape.encoding
            )));
        }
        require_replacement_count(count, shape.count)?;
        Ok(shape)
    }

    fn indexed_replacement_shape(&self, count: usize, maximum_pixels: usize) -> Result<ImageShape> {
        let shape = codec::validate(self.contents(), maximum_pixels)?;
        if !matches!(
            shape.encoding,
            DibEncoding::Rle8 | DibEncoding::IndexedRgb(_)
        ) {
            return Err(unsupported(format!(
                "indexed replacement from {:?}; expected Rle8 or IndexedRgb",
                shape.encoding
            )));
        }
        require_replacement_count(count, shape.count)?;
        Ok(shape)
    }

    pub(super) fn with_pixel_bytes(&self, bytes: Vec<u8>, maximum_pixels: usize) -> Result<Self> {
        let mut contents = self.contents().clone();
        // Preserve legal omitted BI_RGB sizes across fresh edit/reversal checkpoints (#208).
        if contents.header.compression != 0 || contents.header.image_size != 0 {
            contents.header.image_size = u32::try_from(bytes.len())
                .map_err(|_| invalid(format!("replacement bytes {}; expected u32", bytes.len())))?;
        }
        contents.pixel_bytes = bytes;
        self.with_contents(contents, maximum_pixels)
    }
}

fn require_palette_indices(indices: &[u8], palette: usize) -> Result<()> {
    if let Some(index) = indices.iter().find(|index| **index as usize >= palette) {
        return Err(invalid(format!(
            "replacement index {index}; expected below {palette}"
        )));
    }
    Ok(())
}

fn require_replacement_count(count: usize, expected: usize) -> Result<()> {
    if count == expected {
        return Ok(());
    }
    Err(invalid(format!(
        "replacement pixel count {count}; expected {expected}"
    )))
}

fn decode_rle(contents: &DibContents, shape: ImageShape) -> Result<DibPixels> {
    let mut pixels = vec![None; shape.count];
    rle::walk_rle(
        &contents.pixel_bytes,
        shape,
        contents.palette.len(),
        Some(&mut pixels),
    )?;
    Ok(DibPixels::Indexed(pixels))
}

fn decode_bgrx(bytes: &[u8], shape: ImageShape) -> Vec<[u8; 4]> {
    let stored = bytes.as_chunks::<4>().0;
    let mut pixels = Vec::new();
    for row in 0..shape.height {
        let offset = stored_row(row, shape) * shape.width;
        pixels.extend_from_slice(&stored[offset..offset + shape.width]);
    }
    pixels
}

pub(super) fn stored_row(row: usize, shape: ImageShape) -> usize {
    if shape.top_down {
        return row;
    }
    shape.height - 1 - row
}
