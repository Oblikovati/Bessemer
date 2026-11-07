use super::*;
/// Explicit native string mode with independent complete-payload and transferred-unit bounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowTextSlotOptions {
    /// Required explicit mode, without owner-mode inference.
    pub string_mode: FlowMetadataStringMode,
    /// Complete payload bytes, at least 204 for header and fifty empty counts.
    pub maximum_payload_bytes: usize,
    /// All physically transferred UTF-16 units, including hidden suffixes and terminators.
    pub maximum_units: usize,
}
#[derive(Clone, Copy, Debug)]
pub(super) struct TextSlotSpan {
    pub start: usize,
    pub end: usize,
    pub units: usize,
    pub prefix: usize,
}
impl TextSlotSpan {
    pub(super) const EMPTY: Self = Self {
        start: 0,
        end: 0,
        units: 0,
        prefix: 0,
    };
}
/// Immutable complete raw slot carrier, retaining every stored unit.
#[derive(Clone, Debug)]
pub struct FlowTextSlots {
    pub(super) wire: TableWire,
    pub(super) options: FlowTextSlotOptions,
    pub(super) spans: Arc<[TextSlotSpan; SLOT_COUNT]>,
    pub(super) units: usize,
}
/// Borrowed physical text slot, with lossless raw units and a separate native prefix.
#[derive(Clone, Copy, Debug)]
pub struct FlowTextSlot<'a> {
    pub(super) owner: &'a FlowTextSlots,
    pub(super) ordinal: usize,
    pub(super) span: &'a TextSlotSpan,
}
/// Exact framing coverage under an explicit native UTF-16 assumption.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowTextSlotCoverage {
    /// Complete raw payload check mode; owner-mode provenance remains separate.
    pub mode: FlowLegacyTableCheckMode,
    /// Complete raw payload extent.
    pub payload_bytes: usize,
    /// Exactly fifty physically stored strings.
    pub slots: usize,
    /// All stored units, including hidden suffixes and terminators.
    pub stored_units: usize,
    /// Slots whose first-NUL prefix is empty and therefore retain previous text.
    pub empty_prefixes: usize,
    /// Zero for exact serialized checks, without geometric accuracy claims.
    pub achieved_error: usize,
}
/// Immutable owned fifty-value UTF-16 state; units need not form Unicode scalar strings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowTextSlotState {
    pub(super) slots: [Arc<[u16]>; SLOT_COUNT],
    pub(super) units: usize,
}
/// Physical-ordinal projection provenance; both parent checkpoints remain retained.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowTextSlotRelation {
    /// The native prefix was empty, so the complete previous text was retained.
    RetainedPrevious,
    /// A nonempty native prefix supplied the new text, even if equal to previous text.
    AppliedNativePrefix,
}
/// Completed pure projection with immutable record, previous-state and result checkpoints.
#[derive(Clone, Debug)]
pub struct FlowTextSlotProjection {
    pub(super) source: FlowTextSlots,
    pub(super) previous: FlowTextSlotState,
    pub(super) result: FlowTextSlotState,
    pub(super) history: [FlowTextSlotRelation; SLOT_COUNT],
    pub(super) maximum_units: usize,
}
/// Exact read/state projection counts, without executing native runtime updates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowTextSlotProjectionCoverage {
    /// Exact serialized source coverage under the declared text mode.
    pub source: FlowTextSlotCoverage,
    /// Total previous text units, including previous embedded NULs and suffixes.
    pub previous_units: usize,
    /// Total resulting text units.
    pub result_units: usize,
    /// Physical slots supplied by nonempty native prefixes.
    pub replaced_slots: usize,
    /// Physical slots retaining complete previous text.
    pub retained_slots: usize,
    /// Explicit output-state unit bound.
    pub maximum_result_units: usize,
}
/// Native mode, runtime and interpretation exclusions on reads, edits and projections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowTextSlotDiagnostic {
    /// UTF-16 mode was declared explicitly, without reconstructing owner-mode provenance.
    NativeUtf16ModeAssumed,
    /// Native runtime updates were not executed.
    NativeRuntimeNotReproduced,
    /// Engineering purposes for slot ordinals remain unresolved.
    EngineeringRolesNotResolved,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
