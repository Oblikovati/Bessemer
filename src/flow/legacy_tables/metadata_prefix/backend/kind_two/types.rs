use super::*;
/// Explicit kind-two bounds, retaining existing common-body option compatibility.
#[derive(Clone, Copy, Debug)]
pub struct FlowMediumKindTwoOptions {
    /// Per-body bytes, raw wide patterns and name units.
    pub body: FlowMediumBodyOptions,
    /// Positive maximum raw 32-bit model-five words.
    pub maximum_word_patterns: usize,
}
/// The six documented signed model tags, normalized at the boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum FlowMediumModelKind {
    /// Two curves and one scalar.
    TwoCurves = 0,
    /// Three curves, one scalar and two state bytes.
    ThreeCurvesWithState = 1,
    /// Three curves and one scalar.
    ThreeCurves = 2,
    /// Seven raw scalars.
    SevenScalars = 3,
    /// Six scalars, one state byte and a last scalar.
    ScalarsWithState = 4,
    /// Three independent array counts, a signed word, scalars and states.
    Tables = 5,
}
impl TryFrom<i32> for FlowMediumModelKind {
    type Error = BessemerError;
    fn try_from(value: i32) -> Result<Self> {
        match value {
            0 => Ok(Self::TwoCurves),
            1 => Ok(Self::ThreeCurvesWithState),
            2 => Ok(Self::ThreeCurves),
            3 => Ok(Self::SevenScalars),
            4 => Ok(Self::ScalarsWithState),
            5 => Ok(Self::Tables),
            _ => Err(table_failure(
                424,
                FailureClass::Unsupported,
                format!("kind-two model tag {value}; expected 0..=5"),
            )),
        }
    }
}
impl FlowMediumModelKind {
    /// Returns the original signed tag; e.g. `assert_eq!(FlowMediumModelKind::Tables.as_i32(),5)`.
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}
/// Curve namespace; model curves have their own zero-based ordinals.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumCurveScope {
    /// The seven shared body curves.
    Body,
    /// The two or three curves of model zero, one or two.
    Model,
}
/// Physical wide-field selector, without inferred engineering labels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumKindTwoWideField {
    /// One scalar in the selected curve namespace.
    CurveScalar {
        /// Body or model namespace.
        scope: FlowMediumCurveScope,
        /// Zero-based ordinal.
        curve: usize,
    },
    /// One physical curve-array role.
    CurveArray {
        /// Body or model namespace.
        scope: FlowMediumCurveScope,
        /// Zero-based ordinal.
        curve: usize,
        /// Physical array role.
        array: FlowMediumCurveArray,
    },
    /// One of six arrays following the shared curves.
    SharedArray {
        /// Zero-based array ordinal, 0..6.
        array: usize,
    },
    /// The single scalar before the model tag.
    BodyScalar,
    /// Model-zero/one/two scalar, model-three seven scalars, or model-five two scalars.
    ModelScalars,
    /// Six initial scalars of model four.
    ModelFirstScalars,
    /// Last scalar of model four.
    ModelLastScalar,
    /// One of the two first-count arrays of model five.
    ModelFirstArray {
        /// Zero-based ordinal, 0..2.
        array: usize,
    },
    /// The second-count wide array of model five.
    ModelSecondArray,
    /// The third-count wide array of model five.
    ModelThirdArray,
}
/// Exact serialized scope; installed-file and full-record acceptance remain distinct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMediumKindTwoCoverage {
    /// Complete checked body bytes.
    pub body_bytes: usize,
    /// Complete checked raw wide patterns.
    pub wide_words: usize,
    /// Complete checked raw 32-bit patterns.
    pub word_patterns: usize,
    /// Complete stored name units.
    pub name_units: usize,
    /// Normalized documented model selector.
    pub model_kind: FlowMediumModelKind,
    /// Every documented body field was decoded.
    pub body_framing_complete: bool,
    /// Following reference/outer record framing was not checked.
    pub full_record_schema: bool,
    /// No installed native kind-two fixture has been validated.
    pub installed_fixture_validated: bool,
    /// Exact byte error, without approximation.
    pub achieved_error: usize,
}
/// Native and engineering exclusions accompanying generated-schema body acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumKindTwoDiagnostic {
    /// Generated fixtures do not establish installed native acceptance.
    InstalledFixtureNotValidated,
    /// Engineering/model evaluation remains unchecked.
    EngineeringAndEvaluationNotChecked,
    /// Native runtime and loaded object state were not reproduced.
    NativeRuntimeNotReproduced,
    /// Following reference tables and outer collections remain opaque.
    FollowingRecordNotDecoded,
    /// Native application acceptance was not executed.
    NativeApplicationNotChecked,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum ModelPayload {
    Curves {
        curves: Arc<[CurveSpan]>,
        scalars: (usize, usize),
        states: (usize, usize),
    },
    Scalars((usize, usize)),
    Switched {
        first: (usize, usize),
        state: usize,
        last: (usize, usize),
    },
    Tables(TableModelSpan),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TableModelSpan {
    pub counts: [usize; 3],
    pub first: [(usize, usize); 2],
    pub second: (usize, usize),
    pub words: (usize, usize),
    pub third: (usize, usize),
    pub member: usize,
    pub scalars: (usize, usize),
    pub states: (usize, usize),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ModelSpan {
    pub kind: FlowMediumModelKind,
    pub start: usize,
    pub end: usize,
    pub payload: ModelPayload,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct KindTwoSpan {
    pub start: usize,
    pub end: usize,
    pub prefix: PrefixSpan,
    pub scalar: (usize, usize),
    pub model: ModelSpan,
    pub state: usize,
    pub words: usize,
}
/// Immutable complete kind-two body with exact following bytes and typed model metadata.
#[derive(Clone, Debug)]
pub struct FlowMediumKindTwo {
    pub(super) backend: FlowMediumBackendPrefix,
    pub(super) options: FlowMediumKindTwoOptions,
    pub(super) span: KindTwoSpan,
}
/// Isolated fixed-field kind-two edit with typed checkpoints and complete carrier history.
#[derive(Clone, Debug)]
pub struct FlowMediumKindTwoEdit {
    pub(super) source: FlowMediumKindTwo,
    pub(super) result: FlowMediumKindTwo,
    pub(super) prefix_edit: FlowMetadataPrefixEdit,
}
