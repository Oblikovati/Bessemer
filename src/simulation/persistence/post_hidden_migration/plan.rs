use super::super::SimulationThermalRelation;
use super::{
    SimulationHiddenPartsMigrationCompletion, SimulationHiddenPartsMigrationCoverage,
    SimulationHiddenPartsMigrationDiagnostic, SimulationHiddenPartsMigrationHistory,
    SimulationHiddenPartsMigrationInputs, SimulationHiddenPartsMigrationValues,
    SimulationHiddenPartsStore,
};

/// Immutable complete derived pair with original wire parents and exact store coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationHiddenPartsMigrationPlan {
    history: SimulationHiddenPartsMigrationHistory,
    values: SimulationHiddenPartsMigrationValues,
}

impl SimulationHiddenPartsMigrationPlan {
    pub(super) fn complete(
        inputs: SimulationHiddenPartsMigrationInputs,
        values: SimulationHiddenPartsMigrationValues,
    ) -> Self {
        Self {
            history: SimulationHiddenPartsMigrationHistory {
                inputs,
                relation: SimulationThermalRelation::Derived,
            },
            values,
        }
    }

    /// Inspect the original version and stored word; e.g. `plan.history().inputs`.
    pub fn history(&self) -> SimulationHiddenPartsMigrationHistory {
        self.history
    }

    /// Inspect both derived words and the exact store kind; e.g. `plan.values().runtime_word_1f0_raw`.
    pub fn values(&self) -> SimulationHiddenPartsMigrationValues {
        self.values
    }

    /// Distinguish cleared and option-one completion; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationHiddenPartsMigrationCompletion {
        if self.values.hidden_option_raw == 0 {
            return SimulationHiddenPartsMigrationCompletion::CompletedClearedPair;
        }
        SimulationHiddenPartsMigrationCompletion::CompletedOptionOne
    }

    /// Inspect exact represented stores and explicit wire/visibility exclusions; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationHiddenPartsMigrationCoverage {
        let clear = self.values.store == SimulationHiddenPartsStore::ClearOptionWord;
        let separate = self.values.store == SimulationHiddenPartsStore::WriteBothWordsOne;
        SimulationHiddenPartsMigrationCoverage {
            stored_words_read: 1,
            runtime_words_assigned: if clear { 1 } else { 2 },
            native_store_instructions: if separate { 2 } else { 1 },
            runtime_store_bytes: if clear { 4 } else { 8 },
            additional_wire_bytes: 0,
            exact_integer_policy: true,
            achieved_pattern_error_bits: 0,
            wire_bytes_modified: false,
            native_object_pointer_validated: false,
            full_visibility_validated: false,
        }
    }

    /// Retain load-scope and original-word qualifications; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationHiddenPartsMigrationDiagnostic; 5] {
        use SimulationHiddenPartsMigrationDiagnostic::*;
        [
            NativeNotExecuted,
            ExplicitFieldVersionOnly,
            DerivedPairNotStoredField,
            OriginalWordRetained,
            FullVisibilityNotModeled,
        ]
    }
}
