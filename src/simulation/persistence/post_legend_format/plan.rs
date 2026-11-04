use super::super::SimulationThermalRelation;
use super::{
    SimulationLegendFormatCheckMode, SimulationLegendFormatCoverage,
    SimulationLegendFormatDiagnostic, SimulationLegendFormatHistory, SimulationLegendFormatInputs,
    SimulationLegendFormatRuntime, SimulationLegendFormatSelection,
};

/// Immutable completed local result; None output means skipped assignment, never an empty string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLegendFormatPlan {
    history: SimulationLegendFormatHistory,
    selection: SimulationLegendFormatSelection,
    format_string: Option<String>,
    coverage: SimulationLegendFormatCoverage,
}

impl SimulationLegendFormatPlan {
    pub(super) fn complete(
        inputs: SimulationLegendFormatInputs,
        runtime: Option<SimulationLegendFormatRuntime>,
        selection: SimulationLegendFormatSelection,
        format_string: Option<String>,
        consumed_member_mask: u8,
    ) -> Self {
        let alternate = runtime.is_some();
        let output_relation = format_string
            .as_ref()
            .map(|_| SimulationThermalRelation::Derived);
        let coverage = format_coverage(selection, alternate, consumed_member_mask, &format_string);
        Self {
            history: SimulationLegendFormatHistory {
                inputs,
                runtime,
                output_relation,
            },
            selection,
            format_string,
            coverage,
        }
    }

    /// Inspect all retained parents without exposing mutation; e.g. `plan.history().inputs`.
    pub fn history(&self) -> SimulationLegendFormatHistory {
        self.history
    }

    /// Inspect the completed primary/alternate or specific bypass; e.g. `plan.selection()`.
    pub fn selection(&self) -> SimulationLegendFormatSelection {
        self.selection
    }

    /// Borrow the constructed string; skipped branches return None; e.g. `plan.format_string()`.
    pub fn format_string(&self) -> Option<&str> {
        self.format_string.as_deref()
    }

    /// Inspect exact consumed dependencies and local check mode; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationLegendFormatCoverage {
        self.coverage
    }

    /// Retain native rendering and runtime exclusions with each result; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendFormatDiagnostic; 4] {
        [
            SimulationLegendFormatDiagnostic::NativeNotExecuted,
            SimulationLegendFormatDiagnostic::NativeFormattingNotModeled,
            SimulationLegendFormatDiagnostic::LocaleNotModeled,
            SimulationLegendFormatDiagnostic::RuntimeNotInferred,
        ]
    }
}

fn format_coverage(
    selection: SimulationLegendFormatSelection,
    alternate: bool,
    mask: u8,
    output: &Option<String>,
) -> SimulationLegendFormatCoverage {
    use SimulationLegendFormatSelection::{Alternate, MagnitudeOutsideInterval};
    SimulationLegendFormatCoverage {
        check_mode: if alternate {
            SimulationLegendFormatCheckMode::ExactMaskedExceptionsDazOff
        } else {
            SimulationLegendFormatCheckMode::ExactWordArithmetic
        },
        consumed_member_mask: mask,
        pointer_checked: alternate,
        value_checked: matches!(selection, Alternate | MagnitudeOutsideInterval),
        output_ascii_bytes: output.as_ref().map_or(0, String::len),
        wire_edited: false,
        achieved_pattern_error_bits: 0,
    }
}
