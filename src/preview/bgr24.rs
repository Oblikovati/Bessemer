use super::{
    DibPreview, codec, invalid,
    pixels::stored_row,
    types::{DibEncoding, ImageShape},
    unsupported,
};
use crate::Result;

impl DibPreview {
    /// Extracts top-to-bottom BGR triples while retaining stored row padding in the archive.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview) -> bessemer::Result<()> {
    /// let pixels = preview.decode_bgr24_pixels(16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn decode_bgr24_pixels(&self, maximum_pixels: usize) -> Result<Vec<[u8; 3]>> {
        let shape = codec::validate(self.contents(), maximum_pixels)?;
        if shape.encoding != DibEncoding::Bgr24 {
            return Err(unsupported(format!(
                "BGR24 extraction from {:?}; expected Bgr24",
                shape.encoding
            )));
        }
        let stride = row_stride(shape)?;
        let mut pixels = Vec::with_capacity(shape.count);
        for row in 0..shape.height {
            let offset = stored_row(row, shape) * stride;
            let bytes = &self.contents().pixel_bytes[offset..offset + shape.width * 3];
            pixels.extend_from_slice(bytes.as_chunks::<3>().0);
        }
        Ok(pixels)
    }

    /// Replaces top-to-bottom BGR triples, preserving padding, palettes and size declarations.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview, pixels: &[[u8; 3]]) -> bessemer::Result<()> {
    /// let edited = preview.with_bgr24_pixels(pixels, 16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_bgr24_pixels(&self, pixels: &[[u8; 3]], maximum_pixels: usize) -> Result<Self> {
        let shape = self.replacement_shape(DibEncoding::Bgr24, pixels.len(), maximum_pixels)?;
        let stride = row_stride(shape)?;
        let mut bytes = self.contents().pixel_bytes.clone();
        for row in 0..shape.height {
            let source = row * shape.width;
            let offset = stored_row(row, shape) * stride;
            let target = &mut bytes[offset..offset + shape.width * 3];
            for (chunk, pixel) in target
                .as_chunks_mut::<3>()
                .0
                .iter_mut()
                .zip(&pixels[source..source + shape.width])
            {
                chunk.copy_from_slice(pixel);
            }
        }
        self.with_pixel_bytes(bytes, maximum_pixels)
    }
}

pub(super) fn row_stride(shape: ImageShape) -> Result<usize> {
    shape
        .width
        .checked_mul(3)
        .and_then(|bytes| bytes.checked_add(3))
        .map(|bytes| bytes & !3)
        .ok_or_else(|| {
            invalid(format!(
                "BGR24 width {}; expected aligned row byte size in usize",
                shape.width
            ))
        })
}

pub(super) fn byte_count(shape: ImageShape) -> Result<usize> {
    let stride = row_stride(shape)?;
    stride.checked_mul(shape.height).ok_or_else(|| {
        invalid(format!(
            "BGR24 stride {stride} and height {}; expected stored byte size in usize",
            shape.height
        ))
    })
}
