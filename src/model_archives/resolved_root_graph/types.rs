use super::super::{ModelFieldHandle, ModelObjectId, ModelObjectTag, PartResolvedFeaturesArchive};
use std::sync::Arc;

/// Loaded target of a stored root occurrence; no external document is opened.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelResolvedRootTarget {
    /// The stored pointer is null.
    Null,
    /// The object was loaded by the complete configuration before this feature list.
    Inherited(ModelObjectId),
    /// The object has a physical definition in this complete feature stream.
    Local {
        /// Revision-scoped field of the defining object tag, which may be nested.
        definition: ModelFieldHandle,
    },
}

/// One physical root, including aliases and nulls, in persisted list order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelResolvedRootOccurrence {
    /// Zero-based position in the stored root list.
    pub ordinal: usize,
    /// Physical source occurrence, distinct from its target object's map identity.
    pub field: ModelFieldHandle,
    /// Exact pointer framing and resolved runtime class/schema/map identity.
    pub tag: ModelObjectTag,
    /// Loaded target classification; this is not a feature dependency edge.
    pub target: ModelResolvedRootTarget,
}

/// Unchanged physical root provenance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelResolvedRootCopy {
    /// Every copied parent; this projection copies one physical field.
    pub parents: [ModelFieldHandle; 1],
    /// Same-revision result occurrence.
    pub result: ModelFieldHandle,
}

/// Semantic limits of the complete stored-root projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelResolvedRootDiagnostic {
    /// Stored order and aliases do not establish feature dependency or evaluation order.
    FeatureDependenciesNotInferred,
    /// Neither geometry nor native application behavior has been checked.
    GeometryAndApplicationNotChecked,
    /// Inherited targets retain loaded identities; external documents were not resolved.
    ExternalDocumentsNotResolved,
}

/// Immutable complete root list with its source and physical provenance.
#[derive(Debug, Clone)]
pub struct ModelResolvedRootGraph {
    pub(super) source: PartResolvedFeaturesArchive,
    pub(super) roots: Arc<[ModelResolvedRootOccurrence]>,
    pub(super) copies: Arc<[ModelResolvedRootCopy]>,
}
