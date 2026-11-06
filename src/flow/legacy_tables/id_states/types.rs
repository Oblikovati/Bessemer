use super::*;

/// Independent physical limits for raw state rows.
#[derive(Clone, Copy, Debug)]
pub struct FlowIdStateLimits {
    /// Complete raw payload byte bound, at least eight.
    pub maximum_payload_bytes: usize,
    /// Positive maximum number of ordered rows, including duplicates.
    pub maximum_entries: usize,
}

/// Immutable complete raw state table, with no per-row allocation or implicit map application.
#[derive(Clone, Debug)]
pub struct FlowIdStates {
    pub(super) wire: TableWire,
    pub(super) limits: FlowIdStateLimits,
    pub(super) count: usize,
}

/// Borrowed physical row; identifiers and all eight state bits remain uninterpreted.
#[derive(Clone, Copy, Debug)]
pub struct FlowIdStateEntry<'a> {
    pub(super) owner: &'a FlowIdStates,
    pub(super) ordinal: usize,
}

/// Exact serialized scope, without native map or identifier-role acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowIdStateCoverage {
    /// Complete raw payload bytes through exact EOF.
    pub payload_bytes: usize,
    /// All ordered rows, retaining duplicate identifiers.
    pub entries: usize,
    /// Always eight header/count bytes.
    pub framing_bytes: usize,
    /// Always five bytes per checked row.
    pub row_bytes: usize,
    /// No native version dispatch is inferred from the raw header word.
    pub native_version_dispatch: bool,
    /// Exact serialized error, without approximation.
    pub achieved_error: usize,
}

/// Native-state exclusions accompanying complete wire coverage and map projections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowIdStateDiagnostic {
    /// Signed identifiers are not assumed to name carrier record types.
    IdentifierRolesNotResolved,
    /// Map results model assignments, not native allocation, hash ordering or other node state.
    NativeHashTableNotReproduced,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}

/// Completed isolated fixed-field edit with typed checkpoints and full physical history.
#[derive(Clone, Debug)]
pub struct FlowIdStateEdit {
    pub(super) source: FlowIdStates,
    pub(super) result: FlowIdStates,
    pub(super) carrier_edit: FlowCarrierEdit,
}
