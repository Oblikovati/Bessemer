use std::ops::Range;
use std::sync::Arc;

/// Namespace-expanded XML identity; prefixes are lexical aliases, not identities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpandedName {
    /// Namespace URI, or None for unqualified names.
    pub namespace: Option<Arc<str>>,
    /// Local name without its prefix.
    pub local: Arc<str>,
}

/// One ordinary XML attribute, with decoded but uninterpreted stored text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlAttribute {
    /// Namespace-expanded attribute name; default namespaces do not qualify attributes.
    pub name: ExpandedName,
    /// XML-decoded and XML-normalized value, without semantic conversions.
    pub value: String,
}

/// Physical Unicode encoding of a metadata stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlEncoding {
    /// UTF-8, with optional BOM recorded separately.
    Utf8,
    /// Little-endian UTF-16.
    Utf16Le,
    /// Big-endian UTF-16.
    Utf16Be,
}

/// Physical XML framing independent of namespace and property semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XmlFormat {
    /// Unicode encoding used by the stored bytes.
    pub encoding: XmlEncoding,
    /// Whether an encoding BOM precedes the XML declaration/root.
    pub bom: bool,
    /// Opaque leading byte for KeyWords, outside the XML payload.
    pub keyword_prefix: Option<u8>,
}

/// Complete lexical XML source and physical framing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlSource {
    /// Physical Unicode/BOM/keyword framing.
    pub format: XmlFormat,
    /// Exact decoded source, or None for a completed empty stream.
    pub document: Option<String>,
}

/// Explicit resource budgets for metadata parsing and immutable edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XmlLimits {
    /// Maximum physical bytes and decoded UTF-8 source bytes, checked separately.
    pub maximum_bytes: usize,
    /// Maximum lexical events, including the synthetic document frame.
    pub maximum_nodes: u32,
    /// Maximum attributes, including namespace declarations.
    pub maximum_attributes: usize,
    /// Maximum element depth, with the document element at depth one.
    pub maximum_depth: usize,
}

/// Revision-scoped element identity; old snapshots cannot address a changed document.
#[derive(Debug, Clone)]
pub struct XmlElementId {
    pub(super) ordinal: usize,
    pub(super) revision: Arc<()>,
}

impl PartialEq for XmlElementId {
    fn eq(&self, other: &Self) -> bool {
        self.ordinal == other.ordinal && Arc::ptr_eq(&self.revision, &other.revision)
    }
}
impl Eq for XmlElementId {}

#[derive(Debug)]
pub(super) struct AttributeRecord {
    pub visible: XmlAttribute,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub(super) struct ElementRecord {
    pub name: ExpandedName,
    pub attributes: Vec<AttributeRecord>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub direct_text: String,
    pub span: Range<usize>,
}

#[derive(Debug, Default, Clone, Copy)]
pub(super) struct XmlMetrics {
    pub depth: usize,
    pub physical_bytes: usize,
    pub source_bytes: usize,
    pub nodes: u32,
    pub attributes: usize,
}

#[derive(Debug, Default)]
pub(super) struct XmlModel {
    pub elements: Vec<ElementRecord>,
    pub metrics: XmlMetrics,
}
