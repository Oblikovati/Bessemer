use super::super::SimulationThermalRelation;
use super::{
    SimulationLegendStepCompletion, SimulationLegendStepCoverage, SimulationLegendStepDiagnostic,
    SimulationLegendStepHistory, SimulationLegendStepRuntime,
};

/// Immutable complete signed availability result without stored/runtime inference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepPlan {
    history: SimulationLegendStepHistory,
    availability_raw: i32,
    runtime_member_mask: u8,
}

impl SimulationLegendStepPlan {
    pub(super) fn complete(
        runtime: SimulationLegendStepRuntime,
        availability_raw: i32,
        runtime_member_mask: u8,
    ) -> Self {
        Self {
            history: SimulationLegendStepHistory {
                runtime,
                relation: SimulationThermalRelation::Derived,
            },
            availability_raw,
            runtime_member_mask,
        }
    }

    /// Inspect every supplied runtime parent; e.g. `plan.history().runtime.word_bec_raw`.
    pub fn history(&self) -> SimulationLegendStepHistory {
        self.history
    }

    /// Retain the signed return without boolean normalization; e.g. `plan.availability_raw()`.
    pub fn availability_raw(&self) -> i32 {
        self.availability_raw
    }

    /// Distinguish completed zero and nonzero returns; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegendStepCompletion {
        if self.availability_raw == 0 {
            return SimulationLegendStepCompletion::CompletedZeroReturn;
        }
        SimulationLegendStepCompletion::CompletedNonzeroReturn
    }

    /// Inspect exact reached reads and downstream exclusions; e.g. `plan.coverage().runtime_member_mask`.
    pub fn coverage(&self) -> SimulationLegendStepCoverage {
        SimulationLegendStepCoverage {
            runtime_member_mask: self.runtime_member_mask,
            stored_words_read: 0,
            exact_signed32_policy: true,
            achieved_pattern_error_bits: 0,
            runtime_plot_pointer_validated: false,
            renderer_modeled: false,
        }
    }

    /// Retain explicit runtime, pointer and rendering qualifications; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendStepDiagnostic; 5] {
        [
            SimulationLegendStepDiagnostic::NativeNotExecuted,
            SimulationLegendStepDiagnostic::RuntimePlotWordsExplicit,
            SimulationLegendStepDiagnostic::PointerSafetyNotEstablished,
            SimulationLegendStepDiagnostic::LayoutSwitchNotAvailability,
            SimulationLegendStepDiagnostic::RendererNotModeled,
        ]
    }
}
