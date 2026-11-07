use super::*;

/// Explicit bounds on projection metadata and full child-path bytes.
#[derive(Clone, Copy, Debug)]
pub struct FlowRootIndexLimits {
    /// Positive maximum root records.
    pub maximum_roots: usize,
    /// Positive maximum local leaf records across all roots.
    pub maximum_leaves: usize,
    /// Positive maximum total child-path bytes, without depth truncation.
    pub maximum_path_bytes: usize,
}

/// Native nonnegative signed storage index, derived once from validated dimensions/indices.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlowRootStorageIndex(pub(super) i32);

#[derive(Clone, Debug)]
pub(super) struct RootSpan {
    pub(super) indices: [i32; 3],
    pub(super) storage_index: FlowRootStorageIndex,
    pub(super) first_leaf: usize,
    pub(super) leaf_end: usize,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct LeafSpan {
    pub(super) node: usize,
    pub(super) path_start: usize,
    pub(super) path_end: usize,
}

/// Immutable complete root/local-leaf projection with both original source checkpoints.
#[derive(Clone, Debug)]
pub struct FlowRootIndexProjection {
    pub(super) context: FlowDerivedLeafCount,
    pub(super) roots: Arc<[RootSpan]>,
    pub(super) leaves: Arc<[LeafSpan]>,
    pub(super) paths: Arc<[u8]>,
    pub(super) limits: FlowRootIndexLimits,
    pub(super) coverage: FlowRootIndexCoverage,
}

/// One borrowed root with local leaves; no global block-vector ordinal is assigned.
#[derive(Clone, Copy, Debug)]
pub struct FlowProjectedRoot<'a> {
    pub(super) owner: &'a FlowRootIndexProjection,
    pub(super) span: &'a RootSpan,
}

/// One borrowed local leaf with complete wire child path and original node interval.
#[derive(Clone, Copy, Debug)]
pub struct FlowProjectedLeaf<'a> {
    pub(super) owner: &'a FlowRootIndexProjection,
    pub(super) ordinal: usize,
    pub(super) span: &'a LeafSpan,
}

/// Complete serialized-root and local-path projection check mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowRootIndexCheckMode {
    /// All root indices, strides and all wire-local leaves/paths were checked exactly.
    ExactRootStorageAndLocalPaths,
}

/// Exact completed projection coverage, without native context or scan acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowRootIndexCoverage {
    /// Explicit exact check scope.
    pub mode: FlowRootIndexCheckMode,
    /// Every complete global root position.
    pub checked_roots: usize,
    /// Every local wire leaf across all roots.
    pub checked_leaves: usize,
    /// Complete path bytes, including depths 255/256 without modulo wrapping.
    pub child_path_bytes: usize,
    /// True maximum leaf path depth.
    pub maximum_path_depth: u16,
    /// No approximation in the exact integer/byte projection.
    pub achieved_error: u64,
}

/// Exclusions from the derived wire projection's acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowRootIndexDiagnostic {
    /// The caller assumed fresh runtime state; construction was not executed.
    NativeContextNotExecuted,
    /// Local wire leaves are not assigned to global block-vector ordinals.
    GlobalBlockOrdinalsNotAssigned,
    /// Native parallel scan offsets are not established by root storage arithmetic.
    ParallelScanOffsetsNotValidated,
    /// No native application or geometry acceptance.
    NativeApplicationNotChecked,
}
