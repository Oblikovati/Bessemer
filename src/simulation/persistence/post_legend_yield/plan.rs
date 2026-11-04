use super::super::{SimulationRgbaColor, SimulationThermalRelation};
use super::{
    SimulationLegendYieldCall, SimulationLegendYieldCompletion, SimulationLegendYieldCoverage,
    SimulationLegendYieldDiagnostic, SimulationLegendYieldHistory, SimulationLegendYieldInputs,
    SimulationLegendYieldRuntime,
};

/// Immutable complete call plan retaining colors, caller facts and ordered requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLegendYieldPlan {
    history: SimulationLegendYieldHistory,
    copied_base_pair: [SimulationRgbaColor; 2],
    calls: Vec<SimulationLegendYieldCall>,
}

impl SimulationLegendYieldPlan {
    pub(super) fn complete(
        inputs: SimulationLegendYieldInputs,
        runtime: SimulationLegendYieldRuntime,
        copied_base_pair: [SimulationRgbaColor; 2],
        calls: Vec<SimulationLegendYieldCall>,
    ) -> Self {
        Self {
            history: SimulationLegendYieldHistory {
                inputs,
                runtime,
                relation: SimulationThermalRelation::Derived,
            },
            copied_base_pair,
            calls,
        }
    }

    /// Inspect all original supplied parents; e.g. `plan.history().runtime`.
    pub fn history(&self) -> SimulationLegendYieldHistory {
        self.history
    }

    /// Inspect unconditional D8/white setup, including fallback; e.g. `plan.copied_base_pair()[0].bits()`.
    pub fn copied_base_pair(&self) -> [SimulationRgbaColor; 2] {
        self.copied_base_pair
    }

    /// Inspect calls in native suffix-before-prefix order; e.g. `plan.calls().len()`.
    pub fn calls(&self) -> &[SimulationLegendYieldCall] {
        &self.calls
    }

    /// Distinguish completed empty planning from mapping requests; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegendYieldCompletion {
        if self.calls.is_empty() {
            return SimulationLegendYieldCompletion::CompletedFallbackPlan;
        }
        SimulationLegendYieldCompletion::CompletedMappingCallPlan
    }

    /// Inspect the native return only if every recorded call completes; e.g. `plan.return_raw_if_calls_complete()`.
    pub fn return_raw_if_calls_complete(&self) -> i32 {
        i32::from(!self.calls.is_empty())
    }

    /// Inspect local exactness and downstream exclusions; e.g. `plan.coverage().post_suffix_state_read`.
    pub fn coverage(&self) -> SimulationLegendYieldCoverage {
        SimulationLegendYieldCoverage {
            stored_color_copied: true,
            entry_reversal_read: !self.calls.is_empty(),
            post_suffix_state_read: self.calls.len() == 2,
            exact_local_integer_plan: true,
            achieved_pattern_error_bits: 0,
            mapping_helper_modeled: false,
            output_memory_validated: false,
        }
    }

    /// Retain native, floating-point, helper and memory qualifications; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendYieldDiagnostic; 6] {
        [
            SimulationLegendYieldDiagnostic::NativeNotExecuted,
            SimulationLegendYieldDiagnostic::TickAdjustmentNotModeled,
            SimulationLegendYieldDiagnostic::MappingHelperNotModeled,
            SimulationLegendYieldDiagnostic::OutputMemoryNotValidated,
            SimulationLegendYieldDiagnostic::PostSuffixFactsExplicit,
            SimulationLegendYieldDiagnostic::NearestEvenWhiteConstruction,
        ]
    }
}
