/// Explicit native map state required by the documented selection profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSelectionMapAssumption {
    /// Assume the destination map was empty before deriving physical entry selections.
    FreshEmptyMap,
}
/// Exact serialized-input selection scope, distinct from native owner/scatter acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSelectionCheckMode {
    /// Check every supplied typed input entry and derive its fresh-map selection.
    ExactFreshMapFromSerializedInputs,
}
/// Mandatory qualifications for fresh serialized selection and raw-value acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSelectionDiagnostic {
    /// An empty prior native map is an explicit assumption.
    FreshMapAssumed,
    /// Supplied input revisions do not establish native file/object pairing.
    NativeOwnerBindingNotVerified,
    /// No native scatter, callbacks or scheduling were performed.
    NativeRuntimeScatterNotReproduced,
    /// No engineering meaning was assigned to raw wide patterns or byte states.
    EngineeringRolesNotResolved,
    /// Acceptance was not checked in SolidWorks.
    NativeApplicationNotChecked,
}
