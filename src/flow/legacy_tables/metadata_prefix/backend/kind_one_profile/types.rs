use super::*;
/// Explicit initial native-state assumption for this complete framing profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumProfileContext {
    /// Both native maps start empty; only keys present in this record can resolve references.
    FreshBackend,
}
/// Exact check mode, independently of engineering and application acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumProfileCheckMode {
    /// Complete kind-one tables, references and supported outer collections through EOF.
    ExactFreshKindOneRecord,
}
/// Independent complete-record and nested-body limits.
#[derive(Clone, Copy, Debug)]
pub struct FlowMediumProfileOptions {
    /// Explicit metadata string mode and physical limits.
    pub metadata: FlowMetadataPrefixOptions,
    /// Per-body byte, wide-word and name-unit limits.
    pub body: FlowMediumBodyOptions,
    /// Explicit native initial-state assumption.
    pub context: FlowMediumProfileContext,
    /// Maximum physical first-table entries, including duplicate keys.
    pub maximum_first_objects: usize,
    /// Maximum physical second-table reference objects.
    pub maximum_reference_objects: usize,
    /// Maximum total second-table and outer references, including duplicates.
    pub maximum_references: usize,
    /// Maximum total wide patterns across all bodies.
    pub maximum_total_wide_words: usize,
    /// Maximum total stored body name units.
    pub maximum_total_name_units: usize,
}
/// Complete framing coverage under explicit fresh-state assumptions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMediumProfileCoverage {
    /// Exact supported check mode.
    pub mode: FlowMediumProfileCheckMode,
    /// Complete record payload through EOF.
    pub payload_bytes: usize,
    /// Complete preceding metadata prefix.
    pub metadata_bytes: usize,
    /// Complete backend bytes before outer references.
    pub backend_bytes: usize,
    /// Every physical first-table object.
    pub first_objects: usize,
    /// Every physical second-table object.
    pub reference_objects: usize,
    /// Ordered second-table and outer reference occurrences.
    pub references: usize,
    /// Complete raw body wide patterns.
    pub wide_words: usize,
    /// Complete stored body name units.
    pub name_units: usize,
    /// Both native maps were assumed initially empty.
    pub fresh_backend_assumed: bool,
    /// Complete supported record framing was validated.
    pub complete_record_framing: bool,
    /// Engineering meanings and evaluation remain unchecked.
    pub full_engineering_schema: bool,
    /// Exact serialized error, without approximation.
    pub achieved_error: usize,
}
/// Explicit exclusions from complete serialized framing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumProfileDiagnostic {
    /// Native allocation, hash order, duplicate-key lookup identity and load-time state were not reproduced.
    NativeRuntimeNotReproduced,
    /// Numeric engineering meanings and evaluation were not checked.
    EngineeringAndEvaluationNotChecked,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
#[derive(Clone, Debug)]
pub(super) struct FirstObject {
    pub full: (usize, usize),
    pub key: FlowGuid,
    pub body: FlowMediumKindOne,
}
#[derive(Clone, Debug)]
pub(super) struct ReferenceObject {
    pub full: (usize, usize),
    pub key: FlowGuid,
    pub state: usize,
    pub references_start: usize,
    pub references: Arc<[FlowGuid]>,
}
/// Complete immutable kind-one record; all physical duplicate-key occurrences remain distinct.
#[derive(Clone, Debug)]
pub struct FlowMediumKindOneProfile {
    pub(super) backend: FlowMediumBackendPrefix,
    pub(super) options: FlowMediumProfileOptions,
    pub(super) first: Arc<[FirstObject]>,
    pub(super) second: Arc<[ReferenceObject]>,
    pub(super) outer: OuterReferences,
    pub(super) backend_end: usize,
    pub(super) references: usize,
    pub(super) words: usize,
    pub(super) name_units: usize,
}
/// Borrowed physical first-table occurrence with an independent table key and complete body.
#[derive(Clone, Copy, Debug)]
pub struct FlowMediumProfileObject<'a> {
    pub(super) owner: &'a FlowMediumKindOneProfile,
    pub(super) ordinal: usize,
}
/// Borrowed second-table occurrence preserving every ordered reference.
#[derive(Clone, Copy, Debug)]
pub struct FlowMediumProfileReference<'a> {
    pub(super) owner: &'a FlowMediumKindOneProfile,
    pub(super) ordinal: usize,
}
/// Completed isolated profile edit with full record validation and physical history.
#[derive(Clone, Debug)]
pub struct FlowMediumProfileEdit {
    pub(super) source: FlowMediumKindOneProfile,
    pub(super) result: FlowMediumKindOneProfile,
    pub(super) prefix_edit: FlowMetadataPrefixEdit,
}
