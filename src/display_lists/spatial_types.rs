use super::{ArchiveString, CompactTransform, ViewSectionPlane};

/// Stored Win32 BITMAP framing, distinct from a packed DIB preview stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewBitmap {
    /// Raw stored width; it does not determine the byte count.
    pub width: i32,
    /// Height used with row_bytes to frame the stored bitmap bytes.
    pub height: i32,
    /// Raw bytes per row.
    pub row_bytes: i32,
    /// Stored plane count, without inferred restrictions.
    pub planes: u16,
    /// Stored bits per pixel, without decoding or rescaling.
    pub bits_per_pixel: u16,
    /// Exactly height times row_bytes bytes, subject to the native size bound.
    pub bits: Vec<u8>,
}

/// Complete stored fields of one 3D view, with explicit conditional slots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpatialView {
    /// View name including its stored configuration separator text.
    pub view_name: ArchiveString,
    /// Unknown active-view integer above version 0x46C9.
    pub unknown_active: Option<i32>,
    /// Compact model-to-view transform.
    pub model_to_view: CompactTransform,
    /// Compact pan/zoom transform.
    pub pan_zoom: CompactTransform,
    /// Stored unknown string.
    pub unknown_string: ArchiveString,
    /// Win32 BITMAP body above version 0x1C1E.
    pub preview_bitmap: Option<ViewBitmap>,
    /// Stored native ID-list text above version 0x1C42, without interpretation.
    pub unknown_ids_string: Option<ArchiveString>,
    /// All stored planes; both extensions are present only above version 0x2012.
    pub section_planes: Vec<ViewSectionPlane>,
    /// Unknown integer above 0x2012 when at least one plane exists.
    pub unknown_after_planes: Option<i32>,
    /// Unknown integer above version 0x1FB5.
    pub unknown_468: Option<i32>,
    /// Stored two-word bitset above version 0x1FB5.
    pub unknown_bitset: Option<[i32; 2]>,
    /// Stored unknown text above version 0x3331.
    pub unknown_text_string: Option<ArchiveString>,
}
