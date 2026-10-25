use super::{
    AnnotationViewContents, ArchiveString, CompactTransform, DisplayVersion, FixedTransform,
    NamedView, SpatialView, StoredDouble,
};
use crate::archive_codec::{HeaderBox, StoredFloat};
use std::{collections::BTreeSet, sync::Arc};

/// Explicit document kind supplied by the application; the stream does not store it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayDocumentKind {
    /// Part display lists.
    Part,
    /// Assembly display lists.
    Assembly,
}

/// Native archive context, separate from stored view flags and add-in version strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayArchiveContext {
    /// Unsigned display-list version.
    pub version: DisplayVersion,
    /// Application-declared document kind.
    pub kind: DisplayDocumentKind,
    /// Application-supplied unsigned current-file version, independent of display-list version.
    /// Tangent bucket lists are absent below `0x224`; the stream does not store this context.
    pub file_version: u32,
    /// Exact component IDs from the parsed external face directory; empty when absent.
    pub face_tessellation_components: Arc<BTreeSet<String>>,
}

/// Operation-local byte, field, object, nesting and aggregate collection limits.
#[derive(Debug, Clone, Copy)]
pub struct DisplayArchiveLimits {
    /// Maximum physical and reconstructed bytes.
    pub maximum_bytes: usize,
    /// Maximum typed fields.
    pub maximum_fields: usize,
    /// Maximum newly loaded objects, excluding class-map entries.
    pub maximum_objects: usize,
    /// Maximum nested record scopes.
    pub maximum_depth: usize,
    /// Maximum aggregate collection entries.
    pub maximum_entries: usize,
}

/// Exact new-class descriptor carried by an MFC object tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayClassDescriptor {
    /// Stored class schema.
    pub schema: u16,
    /// Nonempty ASCII native class name.
    pub name: String,
}

/// Nonzero native object-map identity; this is a persisted map index, not a snapshot handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayObjectId(u32);

impl DisplayObjectId {
    /// Validates a native map index without accepting the null slot or class flag bit.
    ///
    /// ```
    /// assert_eq!(bessemer::display_lists::DisplayObjectId::new(2)?.value(),2);
    /// # Ok::<(),bessemer::BessemerError>(())
    /// ```
    pub fn new(value: u32) -> crate::Result<Self> {
        if (1..=0x7fff_ffff).contains(&value) {
            return Ok(Self(value));
        }
        Err(super::archive_error::invalid(format!(
            "object identity {value}; expected 1..=0x7FFFFFFF"
        )))
    }
    /// Returns the stored map identity for explicit graph inspection.
    ///
    /// ```
    /// assert_eq!(bessemer::display_lists::DisplayObjectId::new(7)?.value(),7);
    /// # Ok::<(),bessemer::BessemerError>(())
    /// ```
    pub fn value(self) -> u32 {
        self.0
    }
}

/// Validated meaning of an object pointer in the shared MFC map.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayObjectPointer {
    /// Null pointer.
    Null,
    /// A newly loaded object, mapped before its body is read.
    New(DisplayObjectId),
    /// An alias of an earlier mapped object.
    Reference(DisplayObjectId),
}

/// Exact MFC tag framing; map semantics are validated by the shared archive reader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayObjectTag {
    /// Validated shared-map meaning, checked again during construction.
    pub target: DisplayObjectPointer,
    /// Resolved class for new and aliased objects, absent for null.
    pub runtime_class: Option<DisplayClassDescriptor>,
    /// First native tag word.
    pub tag: u16,
    /// Extended tag only when the first word is 0x7FFF.
    pub extended: Option<u32>,
    /// Descriptor only when the first word is 0xFFFF.
    pub new_class: Option<DisplayClassDescriptor>,
}

/// Typed stored field value; unresolved class bodies are never replaced by raw remainders.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DisplayFieldValue {
    /// Stored unsigned byte.
    U8(u8),
    /// Stored unsigned word.
    U16(u16),
    /// Stored signed word.
    I16(i16),
    /// Stored signed integer.
    I32(i32),
    /// Stored unsigned integer.
    U32(u32),
    /// Exact single-precision bits.
    F32(StoredFloat),
    /// Exact double-precision bits.
    F64(StoredDouble),
    /// CString text and retained physical framing.
    Text(ArchiveString),
    /// Exact native compact transform.
    Transform(CompactTransform),
    /// Exact native fixed transform.
    FixedTransform(FixedTransform),
    /// Stored ten-double bounding box.
    Box(HeaderBox),
    /// MFC class/object/null/reference tag.
    ObjectTag(DisplayObjectTag),
    /// Complete named-view body.
    NamedViews(Option<Vec<NamedView>>),
    /// Complete annotation-view body.
    AnnotationViews(Option<AnnotationViewContents>),
    /// Complete 3D-view body.
    SpatialViews(Option<Vec<SpatialView>>),
    /// Complete inline facet container, using the shared tessellation codec.
    Facet(Box<crate::tessellation::FacetContainer>),
    /// An omitted inline facet supplied by the declared component's external face block.
    ExternalFacet {
        /// Component identity from the external directory.
        component_id: Arc<ArchiveString>,
        /// Zero-based face position consumed within this component's part tessellation.
        index: u32,
    },
}

/// One ordered field with a parser-derived scope path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayField {
    /// Stable structural path within this decoded layout.
    pub path: Arc<str>,
    /// Explicit stored type and value.
    pub value: DisplayFieldValue,
}

/// Complete ordered wire fields; construction revalidates types, paths and the full stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisplayArchiveContents {
    /// Every consumed field in wire order.
    pub fields: Vec<DisplayField>,
}
