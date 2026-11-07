use super::*;
/// Serialized object kind, normalized once for typed reference membership.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum FlowMediumKind {
    /// Complete kind-one body or reference body with a stored state byte.
    One = 1,
    /// Complete kind-two body or reference body without a stored state byte.
    Two = 2,
}
impl TryFrom<i32> for FlowMediumKind {
    type Error = BessemerError;
    fn try_from(value: i32) -> Result<Self> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(table_failure(
                424,
                FailureClass::Unsupported,
                format!("medium profile kind {value}; expected 1 or 2"),
            )),
        }
    }
}
impl FlowMediumKind {
    /// Returns the signed wire tag; e.g. `assert_eq!(FlowMediumKind::Two.as_i32(),2)`.
    pub fn as_i32(self) -> i32 {
        self as i32
    }
    pub(super) fn index(self) -> usize {
        self as usize - 1
    }
}
/// Mixed-kind limits without changing existing kind-one option literals.
#[derive(Clone, Copy, Debug)]
pub struct FlowMediumMixedOptions {
    /// Explicit fresh context, metadata, per-body bounds and global object/reference/wide/name bounds.
    pub profile: FlowMediumProfileOptions,
    /// Positive per-body maximum raw 32-bit model-five words.
    pub maximum_body_word_patterns: usize,
    /// Positive total maximum raw 32-bit model-five words across all bodies.
    pub maximum_total_word_patterns: usize,
}
/// Exact supported check mode; engineering validation is reported separately.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumMixedCheckMode {
    /// Both typed tables, ordered references and supported outer collections through EOF.
    ExactFreshMixedRecord,
}
/// Complete supported serialized coverage with explicit native exclusions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMediumMixedCoverage {
    /// Explicit exact mode.
    pub mode: FlowMediumMixedCheckMode,
    /// Complete payload bytes through EOF.
    pub payload_bytes: usize,
    /// Metadata bytes before the backend.
    pub metadata_bytes: usize,
    /// Complete backend bytes before outer references.
    pub backend_bytes: usize,
    /// Every physical first-table entry, including duplicates.
    pub first_objects: usize,
    /// Every physical reference entry, including duplicates.
    pub reference_objects: usize,
    /// All ordered inner and outer reference occurrences.
    pub references: usize,
    /// All raw 64-bit patterns.
    pub wide_words: usize,
    /// All raw model-five 32-bit patterns.
    pub word_patterns: usize,
    /// All stored body name units.
    pub name_units: usize,
    /// Both typed native maps were assumed initially empty.
    pub fresh_backend_assumed: bool,
    /// The complete supported record grammar was checked.
    pub complete_record_framing: bool,
    /// Engineering meanings and model evaluation remain unchecked.
    pub full_engineering_schema: bool,
    /// Generated kind-two fixtures do not establish installed-file acceptance.
    pub installed_kind_two_fixture_validated: bool,
    /// Exact serialized error without approximation.
    pub achieved_error: usize,
}
/// Exclusions retained alongside complete framing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumMixedDiagnostic {
    /// No installed kind-two fixture was validated.
    InstalledKindTwoFixtureNotValidated,
    /// Numeric engineering meanings and model evaluation remain unchecked.
    EngineeringAndEvaluationNotChecked,
    /// Native maps, allocation, duplicate-key lookup identity and runtime state were not reproduced.
    NativeRuntimeNotReproduced,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
/// Complete immutable nested body selected by its declared wire kind.
#[derive(Clone, Debug)]
pub enum FlowMediumBody {
    /// Complete kind-one body with borrowed raw-pattern views.
    KindOne(FlowMediumKindOne),
    /// Complete kind-two body with a typed model selector.
    KindTwo(FlowMediumKindTwo),
}
/// Immutable physical first-table occurrence with an independent table key.
#[derive(Clone, Debug)]
pub struct FlowMediumMixedEntry {
    pub(super) full: (usize, usize),
    pub(super) key: FlowGuid,
    pub(super) body: FlowMediumBody,
}
/// Immutable physical second-table occurrence preserving ordered typed references.
#[derive(Clone, Debug)]
pub struct FlowMediumMixedReference {
    pub(super) full: (usize, usize),
    pub(super) kind: FlowMediumKind,
    pub(super) key: FlowGuid,
    pub(super) state: Option<(usize, u8)>,
    pub(super) references_start: usize,
    pub(super) references: Arc<[FlowGuid]>,
}
/// Complete immutable fresh mixed-kind record and its validation coverage.
#[derive(Clone, Debug)]
pub struct FlowMediumMixedProfile {
    pub(super) backend: FlowMediumBackendPrefix,
    pub(super) options: FlowMediumMixedOptions,
    pub(super) first: Arc<[FlowMediumMixedEntry]>,
    pub(super) second: Arc<[FlowMediumMixedReference]>,
    pub(super) outer: OuterReferences,
    pub(super) backend_end: usize,
    pub(super) references: usize,
    pub(super) words: usize,
    pub(super) word_patterns: usize,
    pub(super) name_units: usize,
}
/// Completed isolated edit with complete reparsing and physical carrier history.
#[derive(Clone, Debug)]
pub struct FlowMediumMixedEdit {
    pub(super) source: FlowMediumMixedProfile,
    pub(super) result: FlowMediumMixedProfile,
    pub(super) prefix_edit: FlowMetadataPrefixEdit,
}
