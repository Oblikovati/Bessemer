use super::super::SimulationThermalRelation;
use super::{
    SimulationLegendPaletteCompletion, SimulationLegendPaletteCoverage,
    SimulationLegendPaletteDiagnostic, SimulationLegendPaletteDispatch,
    SimulationLegendPaletteHistory, SimulationLegendPaletteInputs, SimulationLegendPaletteRuntime,
};

/// Immutable completed local dispatcher plan, with no partial mapper output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPalettePlan {
    history: SimulationLegendPaletteHistory,
    dispatch: SimulationLegendPaletteDispatch,
    coverage: SimulationLegendPaletteCoverage,
}

impl SimulationLegendPalettePlan {
    pub(super) fn complete(
        inputs: SimulationLegendPaletteInputs,
        runtime: SimulationLegendPaletteRuntime,
        dispatch: SimulationLegendPaletteDispatch,
    ) -> Self {
        Self {
            history: SimulationLegendPaletteHistory {
                inputs,
                runtime,
                relation: SimulationThermalRelation::Derived,
            },
            dispatch,
            coverage: palette_dispatch_coverage(inputs, dispatch),
        }
    }

    /// Inspect every original supplied parent; e.g. `plan.history().inputs`.
    pub fn history(&self) -> SimulationLegendPaletteHistory {
        self.history
    }

    /// Inspect the completed local route without claiming mapper execution; e.g. `plan.dispatch()`.
    pub fn dispatch(&self) -> SimulationLegendPaletteDispatch {
        self.dispatch
    }

    /// Distinguish early return, regular and tail selections; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegendPaletteCompletion {
        match self.dispatch {
            SimulationLegendPaletteDispatch::ReturnedAfterGrayHelper => {
                SimulationLegendPaletteCompletion::CompletedEarlyReturnSelection
            }
            SimulationLegendPaletteDispatch::RegularMapper => {
                SimulationLegendPaletteCompletion::CompletedRegularSelection
            }
            _ => SimulationLegendPaletteCompletion::CompletedTailSelection,
        }
    }

    /// Inspect exact consumed dependencies and zero local error; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationLegendPaletteCoverage {
        self.coverage
    }

    /// Retain native mapper, runtime and count qualifications; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendPaletteDiagnostic; 5] {
        [
            SimulationLegendPaletteDiagnostic::NativeNotExecuted,
            SimulationLegendPaletteDiagnostic::MappingAlgorithmsNotModeled,
            SimulationLegendPaletteDiagnostic::GrayOutputNotEstablished,
            SimulationLegendPaletteDiagnostic::RuntimeProductionNotInferred,
            SimulationLegendPaletteDiagnostic::NativeCountValidityUnknown,
        ]
    }
}

fn palette_dispatch_coverage(
    inputs: SimulationLegendPaletteInputs,
    dispatch: SimulationLegendPaletteDispatch,
) -> SimulationLegendPaletteCoverage {
    let early = dispatch == SimulationLegendPaletteDispatch::ReturnedAfterGrayHelper;
    let shell = matches!(
        dispatch,
        SimulationLegendPaletteDispatch::ShellMaterialMapper
            | SimulationLegendPaletteDispatch::ShellThicknessMapper
    );
    SimulationLegendPaletteCoverage {
        consumed_member_mask: if early { 8 } else { 9 },
        gray_helper_result_read: inputs.gray_above_yield_raw.is_some_and(|word| word != 0),
        plot_pointer_read: shell,
        plot_word_read: shell,
        exact_local_dispatch: true,
        native_count_validated: false,
        selected_mapper_modeled: false,
        wire_edited: false,
        achieved_pattern_error_bits: 0,
    }
}
