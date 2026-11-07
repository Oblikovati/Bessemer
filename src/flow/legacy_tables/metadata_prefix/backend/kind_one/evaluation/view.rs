use super::*;
const COPIED_DIAGNOSTICS: &[FlowMediumEvaluationDiagnostic] = &[
    FlowMediumEvaluationDiagnostic::EngineeringUnitsNotAssigned,
    FlowMediumEvaluationDiagnostic::NativeRuntimeNotReproduced,
    FlowMediumEvaluationDiagnostic::NativeApplicationNotChecked,
];
const LINEAR_DIAGNOSTICS: &[FlowMediumEvaluationDiagnostic] = &[
    FlowMediumEvaluationDiagnostic::EngineeringUnitsNotAssigned,
    FlowMediumEvaluationDiagnostic::NativeRuntimeNotReproduced,
    FlowMediumEvaluationDiagnostic::NativeApplicationNotChecked,
    FlowMediumEvaluationDiagnostic::ForwardErrorNotBounded,
];
const NONFINITE_DIAGNOSTICS: &[FlowMediumEvaluationDiagnostic] = &[
    FlowMediumEvaluationDiagnostic::EngineeringUnitsNotAssigned,
    FlowMediumEvaluationDiagnostic::NativeRuntimeNotReproduced,
    FlowMediumEvaluationDiagnostic::NativeApplicationNotChecked,
    FlowMediumEvaluationDiagnostic::ForwardErrorNotBounded,
    FlowMediumEvaluationDiagnostic::NonFiniteArithmeticResult,
];
impl FlowMediumCurveEvaluation {
    /// Borrows the unchanged input checkpoint identity; e.g. `let record=result.source_record()`.
    pub fn source_record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns the selected complete body interval; e.g. `let span=result.body_payload_interval()`.
    pub fn body_payload_interval(&self) -> (usize, usize) {
        self.body
    }
    /// Returns the selected curve interval in payload coordinates; e.g. `let span=result.curve_payload_interval()`.
    pub fn curve_payload_interval(&self) -> (usize, usize) {
        self.curve
    }
    /// Returns the typed documented accessor; e.g. `let accessor=result.accessor()`.
    pub fn accessor(&self) -> FlowMediumCurveAccessor {
        self.accessor
    }
    /// Returns every query bit, including signed zero; e.g. `let bits=result.query_bits()`.
    pub fn query_bits(&self) -> u64 {
        self.query
    }
    /// Returns the computed bits; NaN payload/sign is not portable; e.g. `let bits=result.result_bits()`.
    pub fn result_bits(&self) -> u64 {
        self.bits
    }
    /// Returns the computed binary64 value; e.g. `let value=result.value()`.
    pub fn value(&self) -> f64 {
        f64::from_bits(self.bits)
    }
    /// Returns the normalized arithmetic classification; e.g. `let class=result.classification()`.
    pub fn classification(&self) -> FlowMediumArithmeticClass {
        self.class
    }
    /// Returns the branch and selected stored-point bracket; e.g. `let branch=result.branch()`.
    pub fn branch(&self) -> FlowMediumEvaluationBranch {
        self.branch
    }
    /// Returns the stable documented behavior version; e.g. `let behavior=result.behavior()`.
    pub fn behavior(&self) -> FlowMediumEvaluationBehavior {
        FlowMediumEvaluationBehavior::FiniteOrderedV1
    }
    /// Reports all executed selected-input checks and exclusions; e.g. `let coverage=result.coverage()`.
    pub fn coverage(&self) -> FlowMediumEvaluationCoverage {
        FlowMediumEvaluationCoverage {
            mode: FlowMediumEvaluationCheckMode::ExactSelectedFiniteOrderedV1,
            table_points: self.points,
            finite_values_checked: if self.points == 0 {
                2
            } else {
                2 * self.points + 1
            },
            ordered_pairs_checked: self.points.saturating_sub(1),
            all_selected_inputs_validated: true,
            scalar_evaluated: self.points == 0,
            extra_arrays_evaluated: false,
            engineering_units_assigned: false,
            native_application_checked: false,
        }
    }
    /// Returns zero for copied values and no certificate for interpolation; e.g. `let bound=result.achieved_error_bound()`.
    pub fn achieved_error_bound(&self) -> Option<f64> {
        if matches!(self.branch, FlowMediumEvaluationBranch::Linear { .. }) {
            return None;
        }
        Some(0.)
    }
    /// Borrows exclusions and actual nonfinite-result diagnostics; e.g. `let diagnostics=result.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowMediumEvaluationDiagnostic] {
        if self.class != FlowMediumArithmeticClass::Finite {
            return NONFINITE_DIAGNOSTICS;
        }
        if matches!(self.branch, FlowMediumEvaluationBranch::Linear { .. }) {
            return LINEAR_DIAGNOSTICS;
        }
        COPIED_DIAGNOSTICS
    }
}
