use super::{
    DibPreview, codec, pixels::stored_row, rows::row_stride, types::DibEncoding, unsupported,
};
use crate::Result;

impl DibPreview {
    /// Extracts top-to-bottom RGB555 words, preserving the unused high bit without alpha semantics.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview) -> bessemer::Result<()> {
    /// let words = preview.decode_bgr555_words(16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn decode_bgr555_words(&self, maximum_pixels: usize) -> Result<Vec<u16>> {
        let shape = codec::validate(self.contents(), maximum_pixels)?;
        require_bgr555(shape.encoding)?;
        let stride = row_stride(shape)?;
        let mut words = Vec::with_capacity(shape.count);
        for row in 0..shape.height {
            let offset = stored_row(row, shape) * stride;
            let bytes = &self.contents().pixel_bytes[offset..offset + shape.width * 2];
            words.extend(
                bytes
                    .as_chunks::<2>()
                    .0
                    .iter()
                    .map(|word| u16::from_le_bytes(*word)),
            );
        }
        Ok(words)
    }

    /// Replaces top-to-bottom raw words, preserving padding, palettes and size declarations.
    ///
    /// ```no_run
    /// # fn example(preview: &bessemer::preview::DibPreview, words: &[u16]) -> bessemer::Result<()> {
    /// let edited = preview.with_bgr555_words(words, 16_000_000)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_bgr555_words(&self, words: &[u16], maximum_pixels: usize) -> Result<Self> {
        let shape = self.replacement_shape(DibEncoding::Bgr555, words.len(), maximum_pixels)?;
        let stride = row_stride(shape)?;
        let mut bytes = self.contents().pixel_bytes.clone();
        for row in 0..shape.height {
            let source = row * shape.width;
            let offset = stored_row(row, shape) * stride;
            let target = &mut bytes[offset..offset + shape.width * 2];
            for (chunk, word) in target
                .as_chunks_mut::<2>()
                .0
                .iter_mut()
                .zip(&words[source..source + shape.width])
            {
                *chunk = word.to_le_bytes();
            }
        }
        self.with_pixel_bytes(bytes, maximum_pixels)
    }
}

fn require_bgr555(encoding: DibEncoding) -> Result<()> {
    if encoding == DibEncoding::Bgr555 {
        return Ok(());
    }
    Err(unsupported(format!(
        "RGB555 extraction from {encoding:?}; expected Bgr555"
    )))
}
