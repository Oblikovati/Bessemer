use super::{
    DibPixels, DibPreview, codec, invalid, rle,
    types::{DibEncoding, ImageShape},
    unsupported,
};
use crate::Result;

impl DibPreview {
    /// Decodes top-to-bottom pixels without inventing values for unwritten RLE positions.
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
        if shape.encoding == DibEncoding::Bgrx32 {
            return Ok(DibPixels::Bgrx(decode_bgrx(&contents.pixel_bytes, shape)));
        }
        let mut pixels = vec![None; shape.count];
        rle::walk_rle(
            &contents.pixel_bytes,
            shape,
            contents.palette.len(),
            Some(&mut pixels),
        )?;
        Ok(DibPixels::Indexed(pixels))
    }

    /// Replaces every RLE8 pixel with checked top-to-bottom indices and deterministic runs.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview, indices: &[u8]) -> bessemer::Result<()> {
    /// let edited = preview.with_indexed_pixels(indices, 16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_indexed_pixels(&self, indices: &[u8], maximum_pixels: usize) -> Result<Self> {
        let shape = self.replacement_shape(DibEncoding::Rle8, indices.len(), maximum_pixels)?;
        if let Some(index) = indices
            .iter()
            .find(|index| **index as usize >= self.contents().palette.len())
        {
            return Err(invalid(format!(
                "replacement index {index}; expected below {}",
                self.contents().palette.len()
            )));
        }
        self.with_pixel_bytes(rle::encode_indices(indices, shape), maximum_pixels)
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

    fn replacement_shape(
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
        if count != shape.count {
            return Err(invalid(format!(
                "replacement pixel count {count}; expected {}",
                shape.count
            )));
        }
        Ok(shape)
    }

    fn with_pixel_bytes(&self, bytes: Vec<u8>, maximum_pixels: usize) -> Result<Self> {
        let mut contents = self.contents().clone();
        contents.header.image_size = u32::try_from(bytes.len())
            .map_err(|_| invalid(format!("replacement bytes {}; expected u32", bytes.len())))?;
        contents.pixel_bytes = bytes;
        self.with_contents(contents, maximum_pixels)
    }
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

fn stored_row(row: usize, shape: ImageShape) -> usize {
    if shape.top_down {
        return row;
    }
    shape.height - 1 - row
}
