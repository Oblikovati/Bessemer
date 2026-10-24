/// Complete forty-byte BITMAPINFOHEADER, with signed dimensions and raw observations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitmapInfoHeader {
    /// Header length; supported value is 40.
    pub header_size: u32,
    /// Positive image width.
    pub width: i32,
    /// Positive bottom-up height, or negative top-down height for uncompressed images.
    pub height: i32,
    /// Plane count; must be one.
    pub planes: u16,
    /// Bits per pixel: eight for RLE8, 32 for BGRX.
    pub bit_count: u16,
    /// Windows BI_RLE8 (1) or BI_RGB (0).
    pub compression: u32,
    /// Declared stored pixel-byte count.
    pub image_size: u32,
    /// Signed horizontal resolution observation.
    pub x_pixels_per_metre: i32,
    /// Signed vertical resolution observation.
    pub y_pixels_per_metre: i32,
    /// Explicit palette count; zero selects the bit-depth default.
    pub colors_used: u32,
    /// Stored important-colour observation, without normalization.
    pub colors_important: u32,
}

/// One stored RGBQUAD, including its uninterpreted reserved byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaletteEntry {
    /// Blue intensity.
    pub blue: u8,
    /// Green intensity.
    pub green: u8,
    /// Red intensity.
    pub red: u8,
    /// Stored reserved byte; this is not inferred to be alpha.
    pub reserved: u8,
}

/// All packed DIB fields; the outer stream-length word is derived on writing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DibContents {
    /// Complete stored header.
    pub header: BitmapInfoHeader,
    /// Stored palette, including optional optimization colours for BGRX.
    pub palette: Vec<PaletteEntry>,
    /// Complete declared image bytes, including bytes after an RLE end marker.
    pub pixel_bytes: Vec<u8>,
}

/// Decoded pixels in top-to-bottom row order; dimensions are in the archive header.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DibPixels {
    /// RLE8 indices; None means no opcode wrote this pixel, not palette entry zero.
    Indexed(Vec<Option<u8>>),
    /// Four raw BGRX bytes per pixel; the fourth byte is preserved without alpha semantics.
    Bgrx(Vec<[u8; 4]>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DibEncoding {
    Rle8,
    Bgrx32,
}

#[derive(Clone, Copy)]
pub(super) struct ImageShape {
    pub width: usize,
    pub height: usize,
    pub count: usize,
    pub top_down: bool,
    pub encoding: DibEncoding,
}
