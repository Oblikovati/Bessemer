use super::*;
/// Raw word profile; neither format has a stored version word.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLegacyWordProfile {
    /// Exactly 320 uncounted raw patterns.
    Fixed105,
    /// Nonnegative signed count followed by complete raw patterns.
    Counted113,
}
impl FlowLegacyWordProfile {
    /// Returns the exact ordinary record identifier; e.g. `assert_eq!(FlowLegacyWordProfile::Fixed105.identifier(), 105)`.
    pub fn identifier(self) -> u32 {
        match self {
            Self::Fixed105 => 105,
            Self::Counted113 => 113,
        }
    }
}
/// Independent complete raw payload and actual stored-pattern bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLegacyFieldWordOptions {
    /// Complete raw payload bytes, including record113's count prefix.
    pub maximum_payload_bytes: usize,
    /// Actual stored patterns; record105 requires 320, while record113 may be empty.
    pub maximum_patterns: usize,
}
/// Immutable complete raw word payload with its exact physical parent.
#[derive(Clone, Debug)]
pub struct FlowLegacyWordFields {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) raw: Arc<[u8]>,
    pub(super) profile: FlowLegacyWordProfile,
    pub(super) options: FlowLegacyFieldWordOptions,
    pub(super) count: usize,
    pub(super) first: usize,
}
/// Exact serialized framing of the selected unversioned word profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowLegacyWordCoverage {
    /// Always exact raw framing through EOF.
    pub mode: FlowLegacyTableCheckMode,
    /// Selected layout.
    pub profile: FlowLegacyWordProfile,
    /// Complete raw bytes.
    pub payload_bytes: usize,
    /// Count prefix bytes, zero or four.
    pub prefix_bytes: usize,
    /// Actual stored patterns.
    pub patterns: usize,
    /// Zero serialized approximation error.
    pub achieved_error: usize,
}
/// Exclusions from complete stored-field framing and scalar decisions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowLegacyFieldDiagnostic {
    /// Raw patterns have not been assigned engineering meaning or units.
    EngineeringMeaningsNotAssigned,
    /// Native runtime conversion and object state were not reproduced.
    NativeRuntimeNotReproduced,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
    /// Scalar decisions model completed branches rather than floating-point faults or environment state.
    NativeFloatingPointEnvironmentNotReproduced,
    /// A selected virtual branch does not execute its native method or supply its result.
    NativeVirtualMethodNotExecuted,
    /// No installed raw record155 payload has been authenticated.
    NativeRecord155PayloadNotVerified,
    /// Native inverse-map construction was not reconstructed.
    NativeInverseMapsNotReproduced,
}
/// Modeled slot-seven comparison outcome; no native virtual method is executed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow105Slot7Decision {
    /// The completed comparisons select virtual slot0x78, including NaNs.
    VirtualMethod78,
    /// Value exceeds 565, giving precheck status -1.
    Above565,
    /// Value is at or below 253, giving precheck status -2.
    AtOrBelow253,
}
/// Pure record105 scalar projection, separate from native method results and floating-point faults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flow105ScalarQueries {
    /// True for every slot-ten pattern except the two signed zeros.
    pub slot10_nonzero: bool,
    /// Modeled completed slot-seven branch.
    pub slot7: Flow105Slot7Decision,
    /// Always false: native fault state was not reproduced.
    pub native_fp_faults_reproduced: bool,
    /// Always false: the native virtual method was not called.
    pub native_virtual_result_reproduced: bool,
}
