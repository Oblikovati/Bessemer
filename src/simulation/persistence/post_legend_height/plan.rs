use super::super::SimulationThermalRelation;
use super::{
    SimulationLegendHeightCompletion, SimulationLegendHeightCoverage,
    SimulationLegendHeightDiagnostic, SimulationLegendHeightHistory, SimulationLegendHeightInputs,
};

/// Immutable final-instruction projection without partial font/tick output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendHeightPlan {
    history: SimulationLegendHeightHistory,
    total_height_raw: i32,
}

impl SimulationLegendHeightPlan {
    pub(super) fn complete(
        inputs: SimulationLegendHeightInputs,
        cell_height_raw: i32,
        total_height_raw: i32,
    ) -> Self {
        Self {
            history: SimulationLegendHeightHistory {
                inputs,
                cell_height_raw,
                relation: SimulationThermalRelation::Derived,
            },
            total_height_raw,
        }
    }

    /// Inspect both original scalar parents; e.g. `plan.history().cell_height_raw`.
    pub fn history(&self) -> SimulationLegendHeightHistory {
        self.history
    }

    /// Inspect the signed result without pixel-validity inference; e.g. `plan.total_height_raw()`.
    pub fn total_height_raw(&self) -> i32 {
        self.total_height_raw
    }

    /// Distinguish a complete zero product from a nonzero one; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegendHeightCompletion {
        if self.total_height_raw == 0 {
            return SimulationLegendHeightCompletion::CompletedZeroProduct;
        }
        SimulationLegendHeightCompletion::CompletedNonzeroProduct
    }

    /// Inspect exact instruction coverage and achieved error; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationLegendHeightCoverage {
        SimulationLegendHeightCoverage {
            stored_words_read: 1,
            runtime_scalars_read: 1,
            exact_wrapping_signed32: true,
            cell_helper_modeled: false,
            pixel_coordinates_validated: false,
            wire_edited: false,
            achieved_pattern_error_bits: 0,
        }
    }

    /// Retain native/font/title/tick exclusions; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendHeightDiagnostic; 5] {
        [
            SimulationLegendHeightDiagnostic::NativeNotExecuted,
            SimulationLegendHeightDiagnostic::CellHeightHelperNotModeled,
            SimulationLegendHeightDiagnostic::RuntimeFontMetricsNotInferred,
            SimulationLegendHeightDiagnostic::TitleAndTicksNotModeled,
            SimulationLegendHeightDiagnostic::PixelValidityNotEstablished,
        ]
    }
}
