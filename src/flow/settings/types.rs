use super::*;

/// Complete optional static-settings identity with its exact parent checkpoint.
#[derive(Clone, Debug)]
pub struct FlowSettingsIdentity {
    pub(super) source: FlowCarrier,
    pub(super) record: Option<FlowRecordId>,
    pub(super) guid: FlowGuid,
}

/// Native cache-selection branch, separate from loader execution or success.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsAction {
    /// Nonzero source-precedence byte retains FLD-sourced settings.
    RetainFldSettings,
    /// Equal complete binary GUIDs skip the loader sequence, including equal nil keys.
    SkipCachedIdentity,
    /// Differing keys require the native loader sequence before the projected copy.
    InvokeLoaderSequence,
}

/// Exact supplied-context projection with original inputs and proposed normal-return state retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSettingsSelection {
    pub(super) incoming: FlowGuid,
    pub(super) cached_before: FlowGuid,
    pub(super) from_fld: u8,
    pub(super) action: FlowSettingsAction,
    pub(super) cached_after_normal_return: FlowGuid,
    pub(super) overwrite_warning: bool,
}

/// Exact identity/projection check scopes; neither establishes settings-body acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsCheckMode {
    /// Header-123 gate and optional complete record-435 identity field.
    ExactOptionalHeaderIdentity,
    /// Complete binary-key equality and precedence selection under normal-return semantics.
    ExactNormalReturnSelection,
}

/// Selected optional-field coverage with absence distinguished from a stored nil GUID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSettingsIdentityCoverage {
    /// Required check scope.
    pub mode: FlowSettingsCheckMode,
    /// Sixteen when the field is physically present, otherwise zero.
    pub identity_bytes: usize,
    /// Exact stored identity-byte disagreement; completed reads report zero.
    pub achieved_byte_error: usize,
}

/// Exact supplied-context branch coverage, without an executed-loader claim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSettingsSelectionCoverage {
    /// Required check scope.
    pub mode: FlowSettingsCheckMode,
    /// False when FLD precedence short-circuited the binary GUID comparison.
    pub guid_equality_considered: bool,
    /// Exact projected-cache byte disagreement; complete projections report zero.
    pub achieved_byte_error: usize,
}

/// Native application work not established by identity reads or branch projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsDiagnostic {
    /// Selection did not call loaders, inspect their results or mutate a native cache/model.
    LoaderSequenceNotExecuted,
    /// Settings bodies, source-state provenance and native application acceptance remain separate.
    NativeApplicationNotChecked,
}
