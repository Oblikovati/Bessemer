use super::*;
/// Independent bounds for one complete kind-one body.
#[derive(Clone, Copy, Debug)]
pub struct FlowMediumBodyOptions {
    /// Maximum consumed body bytes, excluding following bytes.
    pub maximum_body_bytes: usize,
    /// Maximum total raw wide patterns, including all scalar fields.
    pub maximum_wide_words: usize,
    /// Maximum stored name units including a terminal NUL.
    pub maximum_name_units: usize,
}
/// Physical curve-array role, without engineering column labels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumCurveArray {
    /// First always-stored array.
    First,
    /// Second always-stored array.
    Second,
    /// First conditional array, only with a nonzero flag.
    ExtraFirst,
    /// Second conditional array, only with a nonzero flag.
    ExtraSecond,
}
/// Existing raw wide-field selection, parsed once and reused for reading/editing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumWideField {
    /// Single curve scalar.
    CurveScalar {
        /// Zero-based curve ordinal.
        curve: usize,
    },
    /// One of four curve array roles.
    CurveArray {
        /// Zero-based curve ordinal.
        curve: usize,
        /// Physical role, without engineering meaning.
        array: FlowMediumCurveArray,
    },
    /// One of the six arrays following the curves; first two share the first count.
    SharedArray {
        /// Zero-based array ordinal, 0..6.
        array: usize,
    },
    /// Four final raw scalar patterns.
    FinalScalars,
}
/// Exact serialized body coverage, independently of complete record acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMediumBodyCoverage {
    /// Complete checked body bytes.
    pub body_bytes: usize,
    /// Complete checked raw wide patterns.
    pub wide_words: usize,
    /// Stored raw UTF-16 name units.
    pub name_units: usize,
    /// Every documented body field was decoded.
    pub body_framing_complete: bool,
    /// The remaining backend and outer record were not checked.
    pub full_record_schema: bool,
    /// Exact byte error, without approximation.
    pub achieved_error: usize,
}
/// Exclusions separate stored fields from native loaded state and semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumBodyDiagnostic {
    /// Array engineering meanings and numeric evaluation were not checked.
    EngineeringAndEvaluationNotChecked,
    /// Native runtime state, including load-time state replacement, was not reproduced.
    NativeRuntimeNotReproduced,
    /// Following references and outer collections remain opaque.
    FollowingRecordNotDecoded,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BodySpan {
    pub start: usize,
    pub end: usize,
    pub name: TextSpan,
    pub curves: Arc<[CurveSpan]>,
    pub arrays: [(usize, usize); 6],
    pub scalars: (usize, usize),
    pub state: usize,
    pub words: usize,
}
/// Immutable complete body retaining all following bytes through its backend checkpoint.
#[derive(Clone, Debug)]
pub struct FlowMediumKindOne {
    pub(super) backend: FlowMediumBackendPrefix,
    pub(super) options: FlowMediumBodyOptions,
    pub(super) span: BodySpan,
}
/// Borrowed curve preserving raw flags and numeric bit patterns.
#[derive(Clone, Copy, Debug)]
pub struct FlowMediumCurve<'a> {
    pub(super) owner: &'a FlowMediumKindOne,
    pub(super) ordinal: usize,
}
/// Isolated existing-field body edit with typed checkpoints and full carrier history.
#[derive(Clone, Debug)]
pub struct FlowMediumBodyEdit {
    pub(super) source: FlowMediumKindOne,
    pub(super) result: FlowMediumKindOne,
    pub(super) prefix_edit: FlowMetadataPrefixEdit,
}
