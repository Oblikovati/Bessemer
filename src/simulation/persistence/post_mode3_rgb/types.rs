use super::{super::SimulationRgbaColor, SimulationMode3RgbTriples};

/// Canonical file facts; unavailable older words/colors remain unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationMode3RgbInputs {
    /// Signed stored1B8, without an initialization default.
    pub mode_1b8_raw: Option<i32>,
    /// Complete original1DC RGBA, copied even for non3 modes.
    pub color_1dc: Option<SimulationRgbaColor>,
    /// Parent170 code; unknown for a standalone style.
    pub plot_type_raw: Option<i32>,
}
/// Explicit already-reached traversal pass; whole-caller admission remains external.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationMode3RgbPass {
    /// Applies the plot54/selector1-or2 suppression after group RGB selection.
    Primary,
    /// Does not apply the primary plot/selector suppression.
    Reflected,
}
/// One local group context; unknown facts are required only when reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationMode3RgbGroup {
    /// Supplied reached traversal pass.
    pub pass: SimulationMode3RgbPass,
    /// Group visibility/status word; only zero reaches the item test.
    pub status_raw: Option<i32>,
    /// External item test; any nonzero value admits the group.
    pub item_test_raw: Option<i32>,
    /// Numeric face argument, without UI interpretation.
    pub face_raw: Option<i32>,
    /// Both colors are copied when group replacement is reached.
    pub first_color: Option<SimulationRgbaColor>,
    /// Complete other color, even when face404 selects the first.
    pub other_color: Option<SimulationRgbaColor>,
    /// External group selector used only for primary plot54 filtering.
    pub selector_raw: Option<i32>,
}
/// Exact local RGB derivation; alpha remains in copied parents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationMode3RgbOrigin {
    /// Initial raw0.5 triple.
    DefaultHalf,
    /// Original1DC first three words.
    Stored1dc,
    /// First copied group color, selected by face404.
    GroupFirst,
    /// Other copied group color.
    GroupOther,
}
/// Furthest local group stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationMode3RgbStage {
    /// Nonzero group status skipped further group work.
    GroupHidden,
    /// External item test returned zero.
    GroupEmpty,
    /// Selected RGB retained, but primary plot54/selector1-or2 suppressed submission.
    PrimaryFiltered,
    /// One count1/index5 local array request was admitted.
    ArrayRequested,
}
/// Completion remains local to selected words and a separately bounded array write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationMode3RgbCompletion {
    /// No array request or write was admitted.
    CompletedEmpty,
    /// An exact RGB selection and local array request were produced.
    CompletedSelection,
    /// Three raw words were copied into an isolated destination snapshot.
    CompletedArrayWrite,
}
/// Exact local scope; no geometry or allocator validity is implied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationMode3RgbCoverage {
    /// Complete1DC words copied, always four for a completed group plan.
    pub stored_words_copied: usize,
    /// Complete group-parent words copied, zero or eight.
    pub group_words_copied: usize,
    /// Selected RGB words, zero or three.
    pub selected_words: usize,
    /// Locally written destination words, zero or three.
    pub written_words: usize,
    /// Whether the local destination bounds were checked before publication.
    pub destination_bounds_checked: bool,
    /// True; word comparisons and copies are exact, without sampling.
    pub exact_word_checks: bool,
    /// False; geometry is outside the local color contract.
    pub geometry_validated: bool,
    /// Zero for exact raw-word operations.
    pub achieved_pattern_error_bits: u32,
}
/// Exclusions accompany every local completed result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationMode3RgbDiagnostic {
    /// The target was not executed.
    NativeNotExecuted,
    /// Pass admission and full traversal are supplied externally.
    WholeCallerUnmodeled,
    /// Renderer state, alpha paths, reflection setup and exceptions remain external.
    RendererStateUnmodeled,
    /// No allocator was invoked and no geometry or draw is established.
    AllocatorAndGeometryUnmodeled,
}
/// Explicit array facts, including a separately supplied post-growth destination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationMode3RgbArrayContext {
    /// Signed native capacity; values at most5 reach growth.
    pub capacity_raw: Option<i32>,
    /// Known existing destination prefix; None differs from known empty.
    pub existing: Option<SimulationMode3RgbTriples>,
    /// External growth result; only exactly zero admits its destination.
    pub growth_result_raw: Option<i32>,
    /// Actual supplied post-growth prefix; no cells are invented.
    pub after_growth: Option<SimulationMode3RgbTriples>,
}
/// The exact destination parent used by the isolated write.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationMode3RgbArrayParent {
    /// Existing storage admitted by signed capacity above5.
    Existing,
    /// Explicit storage supplied after a known zero growth result.
    AfterGrowth,
}
/// One derived raw-word relation; all other cells retain the selected parent's words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationMode3RgbWriteHistory {
    /// The source destination parent retained in the result context.
    pub parent: SimulationMode3RgbArrayParent,
    /// Fixed native triple index5.
    pub index: usize,
    /// All three original destination words.
    pub before: [u32; 3],
    /// All three derived selected RGB words.
    pub after: [u32; 3],
}
