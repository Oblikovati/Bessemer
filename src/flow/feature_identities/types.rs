use super::*;

/// Positive allocation bounds and optional external legacy-tail context.
#[derive(Clone, Copy, Debug)]
pub struct FlowFeatureOptions {
    /// Maximum physically stored objects.
    pub maximum_objects: usize,
    /// Maximum total retained UTF-16 code units, including terminators and hidden suffixes.
    pub maximum_text_units: usize,
    /// Maximum physically stored tail values; virtual legacy tails allocate no vector.
    pub maximum_tail_values: usize,
    /// Explicit runtime count for versions zero through three; negative values derive zero.
    pub legacy_tail_count: Option<i32>,
}

/// Established native factory class, separate from original type-code bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFeatureClass {
    /// Native SBoundaryCondition, type zero.
    BoundaryCondition,
    /// Native SVSource, type five.
    VSource,
    /// Native SSSource, type nine.
    SSSource,
    /// Native SFan, type ten.
    Fan,
    /// Native SUnknownFeature fallback, including type 39.
    Unknown,
}

/// One complete immutable physical identity entry, including every raw string unit.
#[derive(Clone, Debug)]
pub struct FlowFeatureIdentity {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) header_version: u32,
    pub(super) type_code: u32,
    pub(super) body_version: u32,
    pub(super) text: Arc<[u16]>,
    pub(super) state: u8,
}

/// Stored tail or explicit virtual legacy-tail description; none are guessed to be equivalent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FlowFeatureTail {
    /// Physically stored signed values, including a completed empty array.
    Stored(Arc<[i32]>),
    /// Missing external count in versions zero through three.
    RequiresRuntimeCount,
    /// Virtual runtime fill, without allocating or serializing a vector.
    DerivedRuntime {
        /// Original signed caller context.
        runtime_count: i32,
        /// Nonnegative count after native negative-count handling.
        count: u32,
        /// Native fill value, always two in this profile.
        fill_value: i32,
    },
}

/// Complete selected physical list, original revision and exact stored fields.
#[derive(Clone, Debug)]
pub struct FlowFeatureIdentities {
    pub(super) source: FlowCarrier,
    pub(super) record: FlowRecordId,
    pub(super) payload: Arc<[u8]>,
    pub(super) version: u32,
    pub(super) objects: Arc<[FlowFeatureIdentity]>,
    pub(super) tail: FlowFeatureTail,
    pub(super) total_text_units: usize,
    pub(super) options: FlowFeatureOptions,
}

/// One existing object's wire field; text contains full counted units, including any terminator.
#[derive(Clone, Debug)]
pub enum FlowFeatureFieldEdit {
    /// Independent stored object-header word.
    HeaderVersion(u32),
    /// Original factory code; unknown codes keep their fallback grammar.
    TypeCode(u32),
    /// Independent stored identity-body word.
    BodyVersion(u32),
    /// Full state byte, without Boolean normalization.
    State(u8),
    /// Full wire units: empty or terminated, preserving surrogates and suffixes after NUL.
    Text(Arc<[u16]>),
}

/// Completed edit with full source/result views and complete physical occurrence history.
#[derive(Clone, Debug)]
pub struct FlowFeatureEdit {
    pub(super) source: FlowFeatureIdentities,
    pub(super) result: FlowFeatureIdentities,
    pub(super) carrier_edit: FlowCarrierEdit,
}

/// Exact raw identity-list check scope, without native runtime object acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFeatureCheckMode {
    /// All counts, terminated wire strings, object intervals, tail and payload EOF checked.
    ExactRawIdentityList,
}

/// Complete stored-field coverage; unresolved virtual context stays explicit in the tail result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFeatureCoverage {
    /// Exact check scope.
    pub mode: FlowFeatureCheckMode,
    /// Entire selected payload extent.
    pub checked_bytes: usize,
    /// Every physical object, including empty identity strings.
    pub checked_objects: usize,
    /// Every raw counted code unit, including hidden suffixes and terminators.
    pub checked_text_units: usize,
    /// Physically stored tail values only.
    pub checked_tail_values: usize,
    /// Exact stored-byte disagreement, zero for completed views.
    pub achieved_byte_error: usize,
}

/// Work outside raw identity parsing and isolated wire editing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFeatureDiagnostic {
    /// Existing-object matching, registration and virtual merge calls were not executed.
    RuntimeObjectMatchingNotExecuted,
    /// Only the ordinary little-endian mode-zero terminated-string profile was checked.
    AlternateStringModesNotChecked,
    /// No native application or solver accepted these results.
    NativeApplicationNotChecked,
}
