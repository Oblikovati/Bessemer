use super::{ArchiveString, CompactTransform, StoredDouble};

/// Fixed annotation section-plane fields; stored values are not geometry validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationSectionPlane {
    /// Stored plane root point.
    pub root_point: [StoredDouble; 3],
    /// Stored normal without normalization.
    pub normal: [StoredDouble; 3],
    /// Unknown stored double.
    pub unknown_double: StoredDouble,
    /// Unknown stored integer.
    pub unknown_integer: i32,
}

/// Stored annotation-view identity, without assuming uniqueness or a restricted domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnnotationViewId(i32);

impl AnnotationViewId {
    /// Wraps the native signed identity once at the codec boundary.
    ///
    /// ```
    /// assert_eq!(bessemer::display_lists::AnnotationViewId::new(-99).value(),-99);
    /// ```
    pub fn new(value: i32) -> Self {
        Self(value)
    }

    /// Returns the exact stored identity without interpreting reserved values.
    ///
    /// ```
    /// assert_eq!(bessemer::display_lists::AnnotationViewId::new(0xc6).value(),0xc6);
    /// ```
    pub fn value(self) -> i32 {
        self.0
    }
}

/// Complete stored fields of an annotation view, including raw visibility flags.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationView {
    /// Stored view name and CString framing.
    pub view_name: ArchiveString,
    /// Raw view identity, without imposing observed domains.
    pub view_id: AnnotationViewId,
    /// Raw native shown flag, without boolean normalization.
    pub is_shown: i32,
    /// Raw native current-view flag.
    pub is_current: i32,
    /// Unknown integer present above version 0x1C40.
    pub unknown_flag: Option<i32>,
    /// Stored second name, without recreating its native suffix.
    pub unknown_name2: ArchiveString,
    /// Compact model-to-view transform.
    pub model_to_view: CompactTransform,
    /// Compact pan/zoom transform.
    pub pan_zoom: CompactTransform,
    /// Stored unknown string.
    pub unknown_string: ArchiveString,
    /// Every stored section plane in order.
    pub section_planes: Vec<AnnotationSectionPlane>,
}

/// Complete active annotation-view body; empty views can still have a trailing bitset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnotationViewContents {
    /// All ordered view records, retaining repeated IDs.
    pub views: Vec<AnnotationView>,
    /// Collection-level bitset above version 0x1FA8.
    pub unknown_bitset: Option<[i32; 2]>,
}
