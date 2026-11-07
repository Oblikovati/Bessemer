use super::*;
/// Independent complete raw payload and actual row bounds; zero rows admits empty layouts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFixedStateOptions {
    /// Maximum complete raw bytes, at least four.
    pub maximum_payload_bytes: usize,
    /// Maximum physically stored rows, without allocating one object per row.
    pub maximum_rows: usize,
}
/// Immutable complete fixed-state payload, without native settings application.
#[derive(Clone, Debug)]
pub struct FlowFixedState {
    pub(super) wire: TableWire,
    pub(super) options: FlowFixedStateOptions,
    pub(super) layout: FixedStateLayout,
    pub(super) count: Option<u32>,
}
/// Borrowed complete physical row with version-dependent optional arrays and final member.
#[derive(Clone, Copy, Debug)]
pub struct FlowFixedStateRow<'a> {
    pub(super) owner: &'a FlowFixedState,
    pub(super) ordinal: usize,
    pub(super) interval: (usize, usize),
}
/// Two always-present raw wide members in every physical row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFixedStateWideMember {
    /// Stored member at native offset 0x600, without applying settings semantics.
    Member600,
    /// Stored wide member discarded by the native reader.
    Discarded,
}
/// Complete exact raw framing and actual stored-value accounting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFixedStateCoverage {
    /// Always exact raw framing through EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete raw bytes, including all structural prefixes.
    pub payload_bytes: usize,
    /// None in version zero, distinct from a stored zero count in versions one through four.
    pub declared_count: Option<u32>,
    /// Actual complete rows.
    pub rows: usize,
    /// None in version zero; otherwise the supported 16/2976/2980-byte row width.
    pub row_bytes: Option<usize>,
    /// Raw wide patterns, including both leading members and all settings arrays.
    pub wide_patterns: usize,
    /// Raw settings words, excluding framing and final signed members.
    pub settings_words: usize,
    /// Physically present final signed members.
    pub final_members: usize,
    /// Every stored scalar, including version and optional count prefixes.
    pub stored_values: usize,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
/// Exclusions accompanying complete serialized fixed-state results.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFixedStateDiagnostic {
    /// Native application of the stored settings was not executed.
    NativeSettingsApplicationNotReproduced,
    /// Engineering roles and units remain unresolved.
    EngineeringRolesNotResolved,
    /// No acceptance check was performed in SolidWorks.
    NativeApplicationNotChecked,
}
/// Completed isolated edit with typed source/result checkpoints and physical carrier history.
#[derive(Clone, Debug)]
pub struct FlowFixedStateEdit {
    pub(super) source: FlowFixedState,
    pub(super) result: FlowFixedState,
    pub(super) carrier_edit: FlowCarrierEdit,
}
