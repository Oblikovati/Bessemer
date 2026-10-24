use super::{invalid, unsupported};
use crate::Result;

/// Validated PNG four-letter chunk identity; the reserved bit is always zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    /// Parses a four-letter name once, rejecting the reserved lowercase third letter.
    ///
    /// ```
    /// assert_eq!(bessemer::preview::ChunkType::new(*b"IHDR")?.bytes(), *b"IHDR");
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub fn new(bytes: [u8; 4]) -> Result<Self> {
        if bytes.iter().all(u8::is_ascii_alphabetic) && bytes[2].is_ascii_uppercase() {
            return Ok(Self(bytes));
        }
        Err(invalid(format!(
            "chunk name {bytes:?}; expected four ASCII letters with uppercase third letter"
        )))
    }

    /// Returns the validated wire identity.
    ///
    /// ```
    /// # use bessemer::preview::ChunkType;
    /// assert_eq!(ChunkType::new(*b"IDAT")?.bytes(), *b"IDAT");
    /// # Ok::<(), bessemer::BessemerError>(())
    /// ```
    pub const fn bytes(self) -> [u8; 4] {
        self.0
    }

    pub(super) fn ancillary(self) -> bool {
        self.0[0].is_ascii_lowercase()
    }
    pub(super) fn safe_to_copy(self) -> bool {
        self.0[3].is_ascii_lowercase()
    }
}

/// One complete chunk; CRC and length are derived and verified rather than caller-supplied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PngChunk {
    /// Validated critical or ancillary chunk type.
    pub kind: ChunkType,
    /// Every payload byte, including unknown ancillary content.
    pub payload: Vec<u8>,
}

/// Every stored chunk in its physical order, from IHDR through IEND.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PngContents {
    /// Complete ordered chunk sequence.
    pub chunks: Vec<PngChunk>,
}

/// The five color models in the referenced static PNG specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PngColor {
    /// One gray sample.
    Grayscale,
    /// Red, green, blue samples.
    Rgb,
    /// One palette-index sample.
    Indexed,
    /// Gray and alpha samples.
    GrayscaleAlpha,
    /// Red, green, blue, alpha samples.
    Rgba,
}

impl PngColor {
    pub(super) fn wire(self) -> u8 {
        match self {
            Self::Grayscale => 0,
            Self::Rgb => 2,
            Self::Indexed => 3,
            Self::GrayscaleAlpha => 4,
            Self::Rgba => 6,
        }
    }
    pub(super) fn from_wire(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::Grayscale),
            2 => Ok(Self::Rgb),
            3 => Ok(Self::Indexed),
            4 => Ok(Self::GrayscaleAlpha),
            6 => Ok(Self::Rgba),
            other => Err(unsupported(format!(
                "PNG color type {other}; expected 0, 2, 3, 4 or 6"
            ))),
        }
    }
    pub(super) fn channels(self) -> usize {
        match self {
            Self::Grayscale | Self::Indexed => 1,
            Self::Rgb => 3,
            Self::GrayscaleAlpha => 2,
            Self::Rgba => 4,
        }
    }
}

/// Parsed IHDR values; compression and filter methods are validated to zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PngHeader {
    /// Positive width, at most 2^31-1.
    pub width: u32,
    /// Positive height, at most 2^31-1.
    pub height: u32,
    /// Stored sample depth (1, 2, 4, 8 or 16), validated for the color model.
    pub bit_depth: u8,
    /// Stored color model.
    pub color: PngColor,
    /// Whether samples use Adam7 interlacing.
    pub interlaced: bool,
}

/// Caller-selected limits at every untrusted PNG boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PngLimits {
    /// Maximum width times height before raster allocation.
    pub maximum_pixels: usize,
    /// Maximum cumulative image scanline and compressed ancillary expansion.
    pub maximum_inflated_bytes: usize,
    /// Maximum physical chunk count.
    pub maximum_chunks: usize,
}

/// Unscaled channel samples in top-to-bottom, left-to-right order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PngRaster {
    /// Source dimensions, depth, color model and interlace mode.
    pub header: PngHeader,
    /// All channels, including exact 16-bit samples or original palette indices.
    pub samples: Vec<u16>,
}
