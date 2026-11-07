use super::CompactTransform;
use crate::archive_codec::ArchiveString;

/// Per-operation limits for decoded view collections and their nested arrays.
#[derive(Debug, Clone, Copy)]
pub struct DisplayViewLimits {
    /// Maximum input and reconstructed bytes, checked before publication.
    pub maximum_bytes: usize,
    /// Maximum view records.
    pub maximum_views: usize,
    /// Maximum cumulative section planes and selection names.
    pub maximum_nested_entries: usize,
}

/// Named-view plane fields; their presence gates are defined by the named-view serializer.
pub type NamedSectionPlane = super::ViewSectionPlane;

/// Populated section-selection payload, without interpreting the native part flag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedSelectionValues {
    /// Unknown integer preceding the part flag.
    pub unknown_sel0: i32,
    /// Raw is-part flag.
    pub is_part: i32,
    /// Every stored selection name, in order.
    pub names: Vec<ArchiveString>,
}

/// Retained native presence flag plus its conditional payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedSectionSelections {
    /// Every nonzero value requires a payload; not normalized to a boolean.
    pub has_selections: i32,
    /// Absent exactly when the raw flag is zero.
    pub values: Option<NamedSelectionValues>,
}

/// Complete stored fields of one named view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedView {
    /// View name with stored CString encoding and prefix width.
    pub view_name: ArchiveString,
    /// Native compact model-to-view transform.
    pub model_to_view: CompactTransform,
    /// Native compact pan/zoom transform.
    pub pan_zoom: CompactTransform,
    /// Stored unknown string, without rebuilding its formatted content.
    pub unknown_string: ArchiveString,
    /// Absent up to version 0x1BF4; otherwise all section planes.
    pub section_planes: Option<Vec<NamedSectionPlane>>,
    /// Present only above 0x2003 with at least one section plane.
    pub selections: Option<NamedSectionSelections>,
    /// Stored two-word bitset above version 0x200D.
    pub unknown_bitset: Option<[i32; 2]>,
}
