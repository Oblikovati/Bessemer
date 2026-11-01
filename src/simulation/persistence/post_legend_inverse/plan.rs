use super::super::SimulationThermalRelation;
use super::{
    SimulationLegendPreferenceCompletion, SimulationLegendPreferenceCoverage,
    SimulationLegendPreferenceDiagnostic, SimulationLegendPreferenceEffect,
    SimulationLegendPreferenceHistory, SimulationLegendPreferencePlot,
    SimulationLegendPreferenceRoute, SimulationLegendPreferenceSource,
};

/// Immutable local conversion or completed empty assignment, with original provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPreferencePlan {
    history: SimulationLegendPreferenceHistory,
    route: SimulationLegendPreferenceRoute,
    effect: SimulationLegendPreferenceEffect,
}

impl SimulationLegendPreferencePlan {
    pub(super) fn complete(
        source: SimulationLegendPreferenceSource,
        route: SimulationLegendPreferenceRoute,
        effect: SimulationLegendPreferenceEffect,
    ) -> Self {
        Self {
            history: SimulationLegendPreferenceHistory {
                source,
                relation: SimulationThermalRelation::Derived,
            },
            route,
            effect,
        }
    }

    /// Inspect every original fact, including unused values; e.g. `plan.history().source`.
    pub fn history(&self) -> SimulationLegendPreferenceHistory {
        self.history
    }

    /// Inspect the native branch without inferring unrelated palette work; e.g. `plan.route()`.
    pub fn route(&self) -> SimulationLegendPreferenceRoute {
        self.route
    }

    /// Inspect the sole derived destination or empty event; e.g. `plan.effect()`.
    pub fn effect(&self) -> SimulationLegendPreferenceEffect {
        self.effect
    }

    /// Distinguish completed empty selection from failed missing input; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegendPreferenceCompletion {
        if self.effect == SimulationLegendPreferenceEffect::NoAssignment {
            return SimulationLegendPreferenceCompletion::CompletedNoAssignment;
        }
        SimulationLegendPreferenceCompletion::CompletedWord
    }

    /// Inspect exact reached local reads/stores and exclusions; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationLegendPreferenceCoverage {
        let assigned = usize::from(self.effect != SimulationLegendPreferenceEffect::NoAssignment);
        let (plot_context_inspected, plot_type_word_read) = self.plot_reads();
        SimulationLegendPreferenceCoverage {
            source_word_read: assigned == 1,
            plot_context_inspected,
            plot_type_word_read,
            runtime_words_assigned: assigned,
            runtime_store_bytes: assigned * 4,
            native_store_instructions: assigned,
            additional_wire_bytes: 0,
            exact_integer_policy: true,
            achieved_pattern_error_bits: 0,
            wire_bytes_modified: false,
            whole_methods_emulated: false,
            native_object_pointer_validated: false,
        }
    }

    fn plot_reads(&self) -> (bool, bool) {
        match self.history.source {
            SimulationLegendPreferenceSource::StoredA8(_) => (false, false),
            SimulationLegendPreferenceSource::RuntimePreference(runtime) => (
                true,
                matches!(
                    runtime.plot_context,
                    Some(SimulationLegendPreferencePlot::PresentType(_))
                ),
            ),
        }
    }

    /// Keep original-value and whole-method qualifications explicit; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendPreferenceDiagnostic; 5] {
        use SimulationLegendPreferenceDiagnostic::*;
        [
            NativeNotExecuted,
            LocalZeroTestOnly,
            OriginalWordsRetained,
            WholePreferenceMethodsNotEmulated,
            NativePointerStateNotValidated,
        ]
    }
}
