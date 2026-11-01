use super::super::{SimulationLegendStepInputs, SimulationThermalRelation};
use super::{
    SimulationLegendStepLayoutCompletion, SimulationLegendStepLayoutCoverage,
    SimulationLegendStepLayoutDiagnostic, SimulationLegendStepLayoutHistory,
    SimulationLegendStepLayoutRuntime, SimulationLegendStepLayoutValues,
};

/// Immutable completed local scalar projection with all caller assumptions retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepLayoutPlan {
    history: SimulationLegendStepLayoutHistory,
    values: Option<SimulationLegendStepLayoutValues>,
}

impl SimulationLegendStepLayoutPlan {
    pub(super) fn complete(
        inputs: SimulationLegendStepInputs,
        runtime: SimulationLegendStepLayoutRuntime,
        values: Option<SimulationLegendStepLayoutValues>,
    ) -> Self {
        Self {
            history: SimulationLegendStepLayoutHistory {
                inputs,
                runtime,
                relation: SimulationThermalRelation::Derived,
            },
            values,
        }
    }

    /// Inspect all original supplied facts; e.g. `plan.history().runtime.after_helpers`.
    pub fn history(&self) -> SimulationLegendStepLayoutHistory {
        self.history
    }

    /// Inspect scalar values conditioned on the supplied post-state, or disabled completion; e.g. `plan.values()`.
    pub fn values(&self) -> Option<SimulationLegendStepLayoutValues> {
        self.values
    }

    /// Distinguish disabled completion from computed zero-valued output; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegendStepLayoutCompletion {
        if self.values.is_none() {
            return SimulationLegendStepLayoutCompletion::CompletedDisabledEarlyReturn;
        }
        SimulationLegendStepLayoutCompletion::CompletedIntegerLayout
    }

    /// Inspect exact reached reads and scalar error bounds; e.g. `plan.coverage().division_20_reached`.
    pub fn coverage(&self) -> SimulationLegendStepLayoutCoverage {
        let (entry, post) = self.read_masks();
        SimulationLegendStepLayoutCoverage {
            entry_member_mask: entry,
            post_helper_member_mask: post,
            native_text_helper_boundaries: if self.values.is_some() { 3 } else { 0 },
            division_20_reached: entry & 2 != 0,
            exact_wrapping_signed32: true,
            achieved_pattern_error_bits: 0,
            renderer_modeled: false,
            pixel_coordinates_validated: false,
        }
    }

    fn read_masks(&self) -> (u8, u8) {
        if self.values.is_none() {
            return (0, 0);
        }
        let entry = if self.history.inputs.layout_switch_raw == Some(0) {
            57
        } else {
            63
        };
        let first = self
            .history
            .runtime
            .after_helpers
            .and_then(|after| after.word_1b0_raw);
        (entry, if first == Some(0) { 15 } else { 7 })
    }

    /// Retain native, helper, renderer and pixel qualifications; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendStepLayoutDiagnostic; 5] {
        [
            SimulationLegendStepLayoutDiagnostic::NativeNotExecuted,
            SimulationLegendStepLayoutDiagnostic::AfterHelperFactsExplicit,
            SimulationLegendStepLayoutDiagnostic::TextHelpersNotModeled,
            SimulationLegendStepLayoutDiagnostic::CompleteRendererNotModeled,
            SimulationLegendStepLayoutDiagnostic::PixelValidityNotEstablished,
        ]
    }
}
