use super::super::SimulationThermalRelation;
use super::{
    SimulationLegendEndCellCompletion, SimulationLegendEndCellCoverage,
    SimulationLegendEndCellDiagnostic, SimulationLegendEndCellHistory,
    SimulationLegendEndCellRuntime,
};

/// Immutable local gate and optional raw RGB copy, without geometry or rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendEndCellPlan {
    history: SimulationLegendEndCellHistory,
    rgb_words: Option<[u32; 3]>,
    reached_read_mask: u8,
}

impl SimulationLegendEndCellPlan {
    pub(super) fn complete(
        runtime: SimulationLegendEndCellRuntime,
        rgb_words: Option<[u32; 3]>,
        reached_read_mask: u8,
    ) -> Self {
        Self {
            history: SimulationLegendEndCellHistory {
                runtime,
                relation: SimulationThermalRelation::Derived,
            },
            rgb_words,
            reached_read_mask,
        }
    }

    /// Inspect all original read-time facts, including skipped parents; e.g. `plan.history().runtime`.
    pub fn history(&self) -> SimulationLegendEndCellHistory {
        self.history
    }

    /// Inspect the exact three copied words without alpha or floating conversion; e.g. `plan.rgb_words()`.
    pub fn rgb_words(&self) -> Option<[u32; 3]> {
        self.rgb_words
    }

    /// Distinguish completed inactivity from a completed local RGB copy; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegendEndCellCompletion {
        if self.rgb_words.is_some() {
            return SimulationLegendEndCellCompletion::CompletedRgbCopy;
        }
        SimulationLegendEndCellCompletion::CompletedInactive
    }

    /// Inspect exact reached reads and explicit geometric exclusions; e.g. `plan.coverage().reached_read_mask`.
    pub fn coverage(&self) -> SimulationLegendEndCellCoverage {
        SimulationLegendEndCellCoverage {
            reached_read_mask: self.reached_read_mask,
            stored_snapshot_words_read: 0,
            rgb_words_copied: if self.rgb_words.is_some() { 3 } else { 0 },
            exact_gate_and_rgb_bits: true,
            achieved_pattern_error_bits: 0,
            runtime_pointer_validated: false,
            geometry_renderer_modeled: false,
        }
    }

    /// Retain native-execution, phase and alpha qualifications; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendEndCellDiagnostic; 5] {
        use SimulationLegendEndCellDiagnostic::*;
        [
            NativeNotExecuted,
            ReadTimeFactsExplicit,
            StoredRuntimeIdentityNotInferred,
            AlphaNotConsumed,
            GeometryRendererNotModeled,
        ]
    }
}
