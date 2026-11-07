use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ParallelSpan {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) count: FlowSimpleLegacyCount,
    pub(super) presence: Option<usize>,
    pub(super) arrays: Option<[(usize, usize); 4]>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct SimpleDeclaredCount {
    pub(super) value: FlowSimpleLegacyCount,
    pub(super) stored: bool,
}

/// Immutable complete two-group parallel table, retaining absent and present-empty arrays.
#[derive(Clone, Debug)]
pub struct FlowParallelTable {
    pub(super) wire: TableWire,
    pub(super) options: FlowParallelTableOptions,
    pub(super) groups: [ParallelSpan; 2],
    pub(super) wide_words: usize,
}

/// Borrowed exact group, retaining its stored count independently of array presence.
#[derive(Clone, Copy, Debug)]
pub struct FlowParallelGroup<'a> {
    pub(super) owner: &'a FlowParallelTable,
    pub(super) ordinal: usize,
    pub(super) span: &'a ParallelSpan,
}

/// Immutable per-leaf raw pairs, with no per-row allocation or tree-order assignment.
#[derive(Clone, Debug)]
pub struct FlowLeafPairs {
    pub(super) wire: TableWire,
    pub(super) options: FlowLeafPairOptions,
    pub(super) rows: usize,
}

/// Borrowed physical leaf pair; ordinal is wire order only.
#[derive(Clone, Copy, Debug)]
pub struct FlowLeafPairRow<'a> {
    pub(super) owner: &'a FlowLeafPairs,
    pub(super) ordinal: usize,
}

/// Immutable raw five-word rows or an explicitly inactive layout and count context.
#[derive(Clone, Debug)]
pub struct FlowFiveWordTable {
    pub(super) wire: TableWire,
    pub(super) options: FlowFiveWordTableOptions,
    pub(super) count: Option<SimpleDeclaredCount>,
    pub(super) start: usize,
    pub(super) rows: usize,
}

/// Borrowed raw five-word row, without assigning integer or floating-point field roles.
#[derive(Clone, Copy, Debug)]
pub struct FlowFiveWordRow<'a> {
    pub(super) owner: &'a FlowFiveWordTable,
    pub(super) ordinal: usize,
}

/// Immutable raw bytes and mandatory stored/external declared count.
#[derive(Clone, Debug)]
pub struct FlowRawByteTable {
    pub(super) wire: TableWire,
    pub(super) options: FlowRawByteTableOptions,
    pub(super) count: SimpleDeclaredCount,
    pub(super) start: usize,
}

/// Distinguishes absent count knowledge from an explicit external or stored count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSimpleCountOrigin {
    /// Inactive version-zero/one/two rows without supplied count knowledge.
    Unspecified,
    /// Explicit caller-supplied context, including a present zero.
    External,
    /// Physically stored nonnegative count.
    Stored,
}

/// Exact five-word storage branch, independently of the declared count.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFiveWordStorage {
    /// Legacy versions zero through two consume no rows.
    NoSerializedRows,
    /// Versions three/four use raw five-word rows, including an empty row array.
    RawRows,
    /// Later zero-count restored layouts consume no rows.
    InactiveRestored,
}

/// Complete exact serialized scope; no engineering or native runtime acceptance is implied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSimpleTableCoverage {
    /// Complete raw or checksum-valid zlib check mode.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete stored payload bytes.
    pub stored_bytes: usize,
    /// Complete decoded bytes through exact EOF.
    pub decoded_bytes: usize,
    /// Actual fixed-row tuples for records 437/411; zero for the other profiles.
    pub wire_rows: usize,
    /// Actual raw wide words across parallel arrays, leaf pairs or counted-wide profiles.
    pub wide_words: usize,
    /// Actual raw 32-bit words in five-word rows.
    pub raw_words: usize,
    /// Actual raw bytes in counted-byte and record-413 profiles.
    pub raw_bytes: usize,
    /// Exact serialized error, without approximation.
    pub achieved_error: usize,
}

/// Native, contextual and semantic exclusions accompanying complete serialized views.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSimpleTableDiagnostic {
    /// Engineering roles of raw fields remain unresolved.
    EngineeringRolesNotResolved,
    /// Native runtime state was not reconstructed.
    NativeRuntimeNotReproduced,
    /// Compression was supplied without reconstructing its owner.
    OwnerCompressionContextNotReconstructed,
    /// Leaf row order has not been assigned to tree traversal.
    TreeOrderNotAssigned,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}

/// Completed isolated simple-table edit with typed checkpoints and full physical history.
#[derive(Clone, Debug)]
pub struct FlowSimpleTableEdit<T: Send + Sync> {
    pub(in crate::flow::legacy_tables) source: T,
    pub(in crate::flow::legacy_tables) result: T,
    pub(in crate::flow::legacy_tables) carrier_edit: FlowCarrierEdit,
}
