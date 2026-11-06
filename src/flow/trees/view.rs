use super::*;

impl FlowBlockTrees {
    /// Borrows the original complete carrier checkpoint; e.g. `let original = trees.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = trees.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Borrows every decoded byte; e.g. `let wire = trees.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.decoded
    }
    /// Returns the observed nonnegative signed version; e.g. `let version = trees.version()`.
    pub fn version(&self) -> i32 {
        self.version
    }
    /// Returns stored or explicit legacy bounds; e.g. `let ranges = *trees.bounds().pairs()`.
    pub fn bounds(&self) -> FlowTreeBounds {
        self.bounds
    }
    /// Reports physical bound presence; e.g. `let stored = trees.bounds_stored()`.
    pub fn bounds_stored(&self) -> bool {
        self.version >= 2
    }
    /// Distinguishes active legacy forests from inactive delegated prefixes; e.g. `let active = trees.body_present()`.
    pub fn body_present(&self) -> bool {
        self.roots.is_some()
    }
    /// Returns the inactive stored zero byte, or absence in legacy versions; e.g. `let flag = trees.body_presence_byte()`.
    pub fn body_presence_byte(&self) -> Option<u8> {
        (self.version >= 4).then_some(0)
    }
    /// Borrows roots in first-index-fast wire order; inactive bodies return None, e.g. `let roots = trees.roots()`.
    pub fn roots(&self) -> Option<&[FlowTreeRoot]> {
        self.roots.as_deref()
    }
    /// Borrows every node in flat wire preorder; e.g. `let count = trees.nodes().len()`.
    pub fn nodes(&self) -> &[FlowTreeNode] {
        &self.nodes
    }
    /// Returns the exact supplied context and limits; e.g. `let depth = trees.options().maximum_depth`.
    pub fn options(&self) -> FlowTreeOptions {
        self.options
    }
    /// Reports complete serialized validation; e.g. `let checked = trees.coverage()`.
    pub fn coverage(&self) -> FlowTreeCoverage {
        self.coverage
    }
    /// Borrows native-context exclusions; e.g. `let exclusions = trees.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowTreeDiagnostic] {
        &[
            FlowTreeDiagnostic::NativeBlockVectorOrderNotProven,
            FlowTreeDiagnostic::GlobalGridBindingNotChecked,
            FlowTreeDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowTreeDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowTreeRoot {
    /// Returns the exact decoded interval; e.g. `let span = root.payload_interval()`.
    pub fn payload_interval(self) -> (usize, usize) {
        (self.start, self.end)
    }
    /// Returns first/second/third indices without inferred axes; e.g. `let indices = root.indices()`.
    pub fn indices(self) -> [i32; 3] {
        self.indices
    }
    /// Returns the full stored presence byte or its legacy absence; e.g. `let raw = root.presence_byte()`.
    pub fn presence_byte(self) -> Option<u8> {
        self.presence
    }
    /// Returns the root's flat node position, distinct from absent tree; e.g. `let node = root.node_index()`.
    pub fn node_index(self) -> Option<usize> {
        self.node
    }
}

impl FlowTreeNode {
    /// Returns the exact decoded subtree interval; e.g. `let span = node.payload_interval()`.
    pub fn payload_interval(&self) -> (usize, usize) {
        (self.start, self.end)
    }
    /// Returns the full split/leaf marker; e.g. `let marker = node.marker_byte()`.
    pub fn marker_byte(&self) -> u8 {
        self.marker
    }
    /// Returns the true bounded wire depth; e.g. `let depth = node.wire_depth()`.
    pub fn wire_depth(&self) -> u16 {
        self.depth
    }
    /// Returns the native level's modulo-256 view; e.g. `let level = node.native_level_byte()`.
    pub fn native_level_byte(&self) -> u8 {
        (self.depth & 255) as u8
    }
    /// Returns the physically stored version-2 leaf word; e.g. `let bits = node.discarded_leaf_word()`.
    pub fn discarded_leaf_word(&self) -> Option<u32> {
        self.word
    }
    /// Borrows all eight children for a split, or an empty leaf slice; e.g. `let children = node.children()`.
    pub fn children(&self) -> &[FlowTreeChild] {
        &self.children
    }
}

impl FlowTreeChild {
    /// Returns the discarded raw byte, never a presence flag; e.g. `let raw = child.discarded_prefix_byte()`.
    pub fn discarded_prefix_byte(self) -> Option<u8> {
        self.prefix
    }
    /// Returns the child's flat node position; e.g. `let index = child.node_index()`.
    pub fn node_index(self) -> usize {
        self.node
    }
}

impl FlowTreeEdit {
    /// Borrows the earlier typed checkpoint, including original wire; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowBlockTrees {
        &self.source
    }
    /// Borrows the fully reparsed result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowBlockTrees {
        &self.result
    }
    /// Borrows complete physical occurrence history; e.g. `let parents = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
