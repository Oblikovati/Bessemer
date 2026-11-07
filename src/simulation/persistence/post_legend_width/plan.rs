use super::super::SimulationThermalRelation;
use super::{
    SimulationLegendWidthCompletion, SimulationLegendWidthCoverage,
    SimulationLegendWidthDiagnostic, SimulationLegendWidthHistory, SimulationLegendWidthOperation,
    SimulationLegendWidthRuntime, SimulationLegendWidthValues,
};

/// Immutable exact local width patterns, independent of ambient floating-point state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendWidthPlan {
    history: SimulationLegendWidthHistory,
    values: SimulationLegendWidthValues,
}

impl SimulationLegendWidthPlan {
    pub(super) fn complete(
        runtime: SimulationLegendWidthRuntime,
        values: SimulationLegendWidthValues,
    ) -> Self {
        Self {
            history: SimulationLegendWidthHistory {
                runtime,
                relation: SimulationThermalRelation::Derived,
            },
            values,
        }
    }

    /// Inspect all explicit entry parents; e.g. `plan.history().runtime`.
    pub fn history(&self) -> SimulationLegendWidthHistory {
        self.history
    }

    /// Inspect all intermediate and selected patterns; e.g. `plan.values().selected_width_bits`.
    pub fn values(&self) -> SimulationLegendWidthValues {
        self.values
    }

    /// Distinguish completed zero from signed nonzero widths; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegendWidthCompletion {
        if self.values.selected_width_bits & 0x7fffffff == 0 {
            return SimulationLegendWidthCompletion::CompletedZeroWidth;
        }
        SimulationLegendWidthCompletion::CompletedNonzeroWidth
    }

    /// Inspect exact local arithmetic and native-state exclusions; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationLegendWidthCoverage {
        SimulationLegendWidthCoverage {
            runtime_words_read: 2,
            stored_snapshot_words_read: 0,
            rounded_multiplications: 2 + usize::from(
                self.values.operation == SimulationLegendWidthOperation::MultiplyByFour,
            ),
            addition_reached: self.values.operation
                == SimulationLegendWidthOperation::AddInitialToItself,
            exact_nearest_even_patterns: true,
            achieved_pattern_error_bits: 0,
            ambient_sse_state_validated: false,
            pointer_geometry_renderer_validated: false,
        }
    }

    /// Retain rounding-policy and renderer qualifications; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendWidthDiagnostic; 5] {
        use SimulationLegendWidthDiagnostic::*;
        [
            NativeNotExecuted,
            ReadTimeFactsExplicit,
            NearestEvenPolicyExplicit,
            PointerGeometryNotValidated,
            RendererAndUnitsNotEstablished,
        ]
    }
}
