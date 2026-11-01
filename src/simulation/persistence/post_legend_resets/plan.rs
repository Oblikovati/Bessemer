use super::super::{SimulationPersistVersion, SimulationThermalRelation};
use super::{
    SimulationLegendResetAssignment, SimulationLegendResetCompletion,
    SimulationLegendResetCoverage, SimulationLegendResetDiagnostic, SimulationLegendResetHistory,
    SimulationLegendResetMember,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LegendResetAssignmentSet {
    None,
    FormatPair([SimulationLegendResetAssignment; 2]),
    AllThree([SimulationLegendResetAssignment; 3]),
}

impl LegendResetAssignmentSet {
    pub(super) fn formats() -> Self {
        use SimulationLegendResetMember::*;
        Self::FormatPair([zero_assignment(WordC4), zero_assignment(WordC8)])
    }

    pub(super) fn all() -> Self {
        use SimulationLegendResetMember::*;
        Self::AllThree([
            zero_assignment(WordC0),
            zero_assignment(WordC4),
            zero_assignment(WordC8),
        ])
    }

    fn as_slice(&self) -> &[SimulationLegendResetAssignment] {
        match self {
            Self::None => &[],
            Self::FormatPair(pair) => pair,
            Self::AllThree(triple) => triple,
        }
    }
}

fn zero_assignment(member: SimulationLegendResetMember) -> SimulationLegendResetAssignment {
    SimulationLegendResetAssignment {
        member,
        value_raw: 0,
    }
}

/// Immutable ordered zero events with the original version and exact local qualifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendResetPlan {
    history: SimulationLegendResetHistory,
    assignments: LegendResetAssignmentSet,
}

impl SimulationLegendResetPlan {
    pub(super) fn complete(
        version: SimulationPersistVersion,
        assignments: LegendResetAssignmentSet,
    ) -> Self {
        Self {
            history: SimulationLegendResetHistory {
                version,
                relation: SimulationThermalRelation::Derived,
            },
            assignments,
        }
    }

    /// Inspect the sole original version parent; e.g. `plan.history().version`.
    pub fn history(&self) -> SimulationLegendResetHistory {
        self.history
    }

    /// Read only reached assignments in native store order; e.g. `plan.assignments()`.
    pub fn assignments(&self) -> &[SimulationLegendResetAssignment] {
        self.assignments.as_slice()
    }

    /// Distinguish a completed empty event list from unsupported context; e.g. `plan.completion()`.
    pub fn completion(&self) -> SimulationLegendResetCompletion {
        if self.assignments().is_empty() {
            return SimulationLegendResetCompletion::CompletedNoAssignments;
        }
        SimulationLegendResetCompletion::CompletedZeroAssignments
    }

    /// Inspect policy-only stores and explicit full-reader exclusions; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationLegendResetCoverage {
        let count = self.assignments().len();
        SimulationLegendResetCoverage {
            version_inspected: true,
            stored_words_read_for_reset_policy: 0,
            runtime_words_assigned: count,
            native_store_instructions: count,
            runtime_store_bytes: count * 4,
            additional_wire_bytes: 0,
            exact_version_policy: true,
            achieved_pattern_error_bits: 0,
            wire_bytes_modified: false,
            full_reader_emulated: false,
            constructor_state_inferred: false,
            native_object_pointer_validated: false,
        }
    }

    /// Keep other missing state and constructor qualifications explicit; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationLegendResetDiagnostic; 5] {
        use SimulationLegendResetDiagnostic::*;
        [
            NativeNotExecuted,
            ExplicitReaderPolicyOnly,
            DerivedAssignmentsHaveNoWireOffsets,
            OtherSkippedFieldsRemainUnknown,
            ConstructorAndPriorStateNotInferred,
        ]
    }
}
