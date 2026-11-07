use super::*;
/// The documented kind-one curve ordinals; other tables have no proven accessor here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum FlowMediumCurveAccessor {
    /// Ordinal1, native object offset `0xC0` and vtable slot `0x48`.
    CurveOne = 1,
    /// Ordinal2, native object offset `0xF0` and vtable slot `0x38`.
    CurveTwo = 2,
    /// Ordinal4, native object offset `0x150` and vtable slot `0x28`.
    CurveFour = 4,
}
impl TryFrom<i32> for FlowMediumCurveAccessor {
    type Error = BessemerError;
    fn try_from(value: i32) -> Result<Self> {
        match value {
            1 => Ok(Self::CurveOne),
            2 => Ok(Self::CurveTwo),
            4 => Ok(Self::CurveFour),
            _ => Err(table_failure(
                424,
                FailureClass::Unsupported,
                format!(
                    "evaluation accessor {value}; expected documented kind-one ordinal 1, 2 or 4"
                ),
            )),
        }
    }
}
impl FlowMediumCurveAccessor {
    /// Returns the original signed ordinal; e.g. `assert_eq!(FlowMediumCurveAccessor::CurveTwo.as_i32(),2)`.
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}
/// Stable documented arithmetic behavior; earlier behavior remains selectable by this versioned API.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumEvaluationBehavior {
    /// Finite selected inputs, strict coordinates, clamps and the documented binary64 operation order.
    FiniteOrderedV1,
}
/// Exact executed input-check scope rather than a sampled pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumEvaluationCheckMode {
    /// Finite query and all selected values; strict ordering of every adjacent coordinate pair.
    ExactSelectedFiniteOrderedV1,
}
/// Selected evaluation branch and physical array-point provenance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumEvaluationBranch {
    /// Empty table copied the stored scalar.
    Constant,
    /// Query at or below the first coordinate copied point0.
    LowerClamp,
    /// Query at or above the last coordinate copied the last point.
    UpperClamp,
    /// Interpolation used these adjacent zero-based points.
    Linear {
        /// Lower stored point.
        lower: usize,
        /// Upper stored point.
        upper: usize,
    },
}
/// Arithmetic classification without assigning portable NaN payload/sign semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumArithmeticClass {
    /// Finite binary64 result, including signed zero.
    Finite,
    /// Positive infinity produced by documented arithmetic.
    PositiveInfinity,
    /// Negative infinity produced by documented arithmetic.
    NegativeInfinity,
    /// NaN produced by documented arithmetic on finite inputs.
    NotANumber,
}
/// Complete selected-input validation with explicit evaluation exclusions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowMediumEvaluationCoverage {
    /// Exact executed scope.
    pub mode: FlowMediumEvaluationCheckMode,
    /// Declared selected table size.
    pub table_points: usize,
    /// Query plus scalar, or query plus both populated arrays.
    pub finite_values_checked: usize,
    /// Every adjacent selected input-coordinate pair.
    pub ordered_pairs_checked: usize,
    /// Every input required by the selected behavior was checked.
    pub all_selected_inputs_validated: bool,
    /// The empty-table scalar was the numeric input.
    pub scalar_evaluated: bool,
    /// Conditional extra arrays are outside these accessors.
    pub extra_arrays_evaluated: bool,
    /// Engineering units are not assigned by this numeric profile.
    pub engineering_units_assigned: bool,
    /// No native application execution was performed.
    pub native_application_checked: bool,
}
/// Exclusions and actual arithmetic outcomes returned with the value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowMediumEvaluationDiagnostic {
    /// No physical engineering units were inferred.
    EngineeringUnitsNotAssigned,
    /// Native allocation, hidden state and floating-point environment were not reproduced.
    NativeRuntimeNotReproduced,
    /// Native application execution was not checked.
    NativeApplicationNotChecked,
    /// Interpolation carries no runtime exact-real forward-error certificate.
    ForwardErrorNotBounded,
    /// The documented sequence produced infinity or NaN on finite inputs.
    NonFiniteArithmeticResult,
}
/// Complete immutable evaluation, provenance, coverage and achieved-error information.
#[derive(Clone, Debug)]
pub struct FlowMediumCurveEvaluation {
    pub(super) record: FlowRecordId,
    pub(super) body: (usize, usize),
    pub(super) curve: (usize, usize),
    pub(super) accessor: FlowMediumCurveAccessor,
    pub(super) query: u64,
    pub(super) bits: u64,
    pub(super) class: FlowMediumArithmeticClass,
    pub(super) branch: FlowMediumEvaluationBranch,
    pub(super) points: usize,
}
