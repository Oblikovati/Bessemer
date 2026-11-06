use super::*;

/// Shared explicit raw/zlib owner context; e.g. `let mode = FlowTreeCompression::Zlib`.
pub type FlowTreeCompression = FlowGroupCompression;

/// Three inclusive signed index ranges, including reversed empty ranges.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowTreeBounds([[i32; 2]; 3]);

impl FlowTreeBounds {
    /// Retains explicit first/second/third index pairs; e.g. `let bounds = FlowTreeBounds::from_pairs([[0,0];3])`.
    pub const fn from_pairs(pairs: [[i32; 2]; 3]) -> Self {
        Self(pairs)
    }
    /// Borrows all original signed ranges; e.g. `let first = bounds.pairs()[0]`.
    pub const fn pairs(&self) -> &[[i32; 2]; 3] {
        &self.0
    }
    /// Returns exact nonnegative inclusive widths; e.g. `let widths = bounds.widths()`.
    pub fn widths(self) -> [u64; 3] {
        self.0
            .map(|[lo, hi]| (i64::from(hi) - i64::from(lo) + 1).max(0) as u64)
    }
    /// Returns the exact mathematical product, including 2^96; e.g. `let positions = bounds.grid_count()`.
    pub fn grid_count(self) -> u128 {
        self.widths().into_iter().map(u128::from).product()
    }
}

/// Explicit compression/bounds plus independent decoded-byte, root, node and depth bounds.
#[derive(Clone, Copy, Debug)]
pub struct FlowTreeOptions {
    /// Owner-selected transport, never guessed from byte signatures.
    pub compression: FlowTreeCompression,
    /// Required only for versions zero/one; stored-bound versions reject unused context.
    pub legacy_bounds: Option<FlowTreeBounds>,
    /// Maximum complete decoded payload bytes, at least four.
    pub maximum_decoded_bytes: usize,
    /// Positive maximum physical root positions in an active legacy forest.
    pub maximum_roots: usize,
    /// Positive maximum total nodes across all roots.
    pub maximum_nodes: usize,
    /// Inclusive maximum wire depth, from zero through 256.
    pub maximum_depth: u16,
}

/// Exact child link in wire order; an optional discarded prefix never implies absence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowTreeChild {
    pub(super) prefix: Option<u8>,
    pub(super) node: usize,
}

/// One immutable flat-preorder node, retaining all otherwise discarded wire fields.
#[derive(Clone, Debug)]
pub struct FlowTreeNode {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) marker: u8,
    pub(super) depth: u16,
    pub(super) word: Option<u32>,
    pub(super) children: Arc<[FlowTreeChild]>,
}

/// One first-index-fast root position, including physically absent trees.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowTreeRoot {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) indices: [i32; 3],
    pub(super) presence: Option<u8>,
    pub(super) node: Option<usize>,
}

/// Complete exact wire forest or inactive delegated prefix, with immutable checkpoints.
#[derive(Clone, Debug)]
pub struct FlowBlockTrees {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) decoded: Arc<[u8]>,
    pub(super) version: i32,
    pub(super) bounds: FlowTreeBounds,
    pub(super) roots: Option<Arc<[FlowTreeRoot]>>,
    pub(super) nodes: Arc<[FlowTreeNode]>,
    pub(super) options: FlowTreeOptions,
    pub(super) coverage: FlowTreeCoverage,
}

/// Exact serialized check scope, distinct from native runtime ordering or geometry acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowTreeCheckMode {
    /// Exact raw legacy forest or inactive delegated prefix through full payload EOF.
    ExactRawForestFrames,
    /// One exact RFC1950 stream plus exact decoded forest/prefix EOF.
    ExactZlibForestFrames,
}

/// Complete serialized coverage, retaining inactive large-grid counts outside allocation counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowTreeCoverage {
    /// Exact serialized check scope.
    pub mode: FlowTreeCheckMode,
    /// Complete original stored payload bytes.
    pub stored_bytes: usize,
    /// Complete decoded payload extent.
    pub decoded_bytes: usize,
    /// Physical active-forest root positions, zero for inactive prefixes.
    pub checked_roots: usize,
    /// All wire nodes across all present roots.
    pub checked_nodes: usize,
    /// Nodes with exactly eight children.
    pub checked_splits: usize,
    /// Every physical leaf node.
    pub checked_leaves: usize,
    /// Complete retained child-prefix bytes.
    pub discarded_child_bytes: usize,
    /// Complete retained version-2 leaf-word bytes.
    pub discarded_leaf_word_bytes: usize,
    /// Maximum observed wire depth, zero for an empty forest/prefix.
    pub maximum_wire_depth: u16,
    /// Exact stored/decoded disagreement, zero for completed operations.
    pub achieved_byte_error: usize,
}

/// Work outside exact raw-tree framing and isolated field editing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowTreeDiagnostic {
    /// Wire preorder was not equated with native parallel block-vector order.
    NativeBlockVectorOrderNotProven,
    /// Root bounds were not bound to native global base-grid dimensions/allocation state.
    GlobalGridBindingNotChecked,
    /// The explicit owner-selected compression context was not reconstructed.
    OwnerCompressionContextNotReconstructed,
    /// Native loaders, solver state and geometry/application acceptance were not executed.
    NativeApplicationNotChecked,
}

/// Existing fixed-width wire edits; node/root cardinality and split structure stay fixed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowTreeFieldEdit {
    /// Changes one leaf's complete non-split marker.
    LeafMarker {
        /// Existing flat wire-preorder node position.
        node: usize,
        /// Replacement byte in zero through 254.
        byte: u8,
    },
    /// Changes an existing discarded child-prefix byte, including zero or 255.
    ChildPrefix {
        /// Existing parent node position.
        node: usize,
        /// Existing child in zero through seven.
        child: usize,
        /// Complete replacement raw prefix byte.
        byte: u8,
    },
    /// Changes one physically stored version-2 leaf word.
    DiscardedLeafWord {
        /// Existing flat leaf-node position.
        node: usize,
        /// Complete replacement u32 bits.
        word: u32,
    },
    /// Changes a stored root presence byte while retaining its null/non-null branch.
    RootPresence {
        /// Existing active-forest root position.
        root: usize,
        /// Replacement byte with the same zero/nonzero status.
        byte: u8,
    },
}

/// Completed isolated field edit with both typed views and complete physical occurrence history.
#[derive(Clone, Debug)]
pub struct FlowTreeEdit {
    pub(super) source: FlowBlockTrees,
    pub(super) result: FlowBlockTrees,
    pub(super) carrier_edit: FlowCarrierEdit,
}
