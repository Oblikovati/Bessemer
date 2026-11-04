use super::super::super::SimulationThermalRelation;
use super::{SimulationStyleComparisonCheck, SimulationStyleComparisonState};

/// Completion of the exact selected dependency path, not universal rendering equivalence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStyleComparisonOutcome {
    /// A reached native dependency differs; later inputs were not read.
    CompletedDifferent(SimulationStyleComparisonCheck),
    /// Every reached dependency checked without a difference.
    CompletedNoDifference,
}

/// Explicit comparison environment and bounded native method scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStyleComparisonCheckMode {
    /// Exact native dependencies with masked SSE exceptions and DAZ disabled.
    ExactDependenciesMaskedExceptionsDazOff,
}

/// Both owned source states remain available independently of later caller mutation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationStyleComparisonHistory {
    /// Every original left-side fact, including unconsumed values.
    pub left: SimulationStyleComparisonState,
    /// Every original right-side fact, including unconsumed values.
    pub right: SimulationStyleComparisonState,
    /// The outcome derives from both source states.
    pub relation: SimulationThermalRelation,
}

/// Exact paired-check coverage, without claims about unread inputs or the entire object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationStyleComparisonCoverage {
    /// Declared native dependency and FP model.
    pub check_mode: SimulationStyleComparisonCheckMode,
    /// Completed paired checks, not counts of individual native memory instructions.
    pub compared_entities: usize,
    /// True; short-circuit check order matches the named body.
    pub exact_dependency_order: bool,
    /// Zero raw decision-pattern error within the declared model.
    pub achieved_pattern_error_bits: u32,
    /// False; inactive or later inputs are retained without being validated.
    pub unread_inputs_validated: bool,
    /// False; this is not a full-object or geometry validator.
    pub full_object_validated: bool,
    /// False; comparison does not rewrite any archive.
    pub wire_bytes_modified: bool,
    /// False; supplied scalar facts establish no native pointer validity.
    pub native_object_pointer_validated: bool,
}

/// Explicit limits on interpreting the comparison result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationStyleComparisonDiagnostic {
    /// Target code was inspected without execution.
    NativeNotExecuted,
    /// Actual loaded/runtime facts must be supplied explicitly.
    RuntimeFactsRequired,
    /// Stored values are not silently replaced by migration or initializer results.
    WireInputsNotMigrated,
    /// This named comparator is not universal render equivalence or an edit no-op policy.
    NotUniversalRenderEquivalence,
}

/// Immutable completed comparison, full original history and native-order paired checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationStyleComparisonPlan {
    history: SimulationStyleComparisonHistory,
    checks: Box<[SimulationStyleComparisonCheck]>,
    outcome: SimulationStyleComparisonOutcome,
}

impl SimulationStyleComparisonPlan {
    pub(super) fn complete(
        left: SimulationStyleComparisonState,
        right: SimulationStyleComparisonState,
        checks: Vec<SimulationStyleComparisonCheck>,
        first: Option<SimulationStyleComparisonCheck>,
    ) -> Self {
        let outcome = first.map_or(
            SimulationStyleComparisonOutcome::CompletedNoDifference,
            SimulationStyleComparisonOutcome::CompletedDifferent,
        );
        Self {
            history: SimulationStyleComparisonHistory {
                left,
                right,
                relation: SimulationThermalRelation::Derived,
            },
            checks: checks.into_boxed_slice(),
            outcome,
        }
    }

    /// Borrow both immutable source states; e.g. `plan.history().left.word_bits(member)`.
    pub fn history(&self) -> &SimulationStyleComparisonHistory {
        &self.history
    }

    /// Inspect completed checks in native order; e.g. `plan.checks()`.
    pub fn checks(&self) -> &[SimulationStyleComparisonCheck] {
        &self.checks
    }

    /// Inspect completion and the first reached difference; e.g. `plan.outcome()`.
    pub fn outcome(&self) -> SimulationStyleComparisonOutcome {
        self.outcome
    }

    /// Inspect exact local coverage and exclusions; e.g. `plan.coverage()`.
    pub fn coverage(&self) -> SimulationStyleComparisonCoverage {
        SimulationStyleComparisonCoverage {
            check_mode: SimulationStyleComparisonCheckMode::ExactDependenciesMaskedExceptionsDazOff,
            compared_entities: self.checks.len(),
            exact_dependency_order: true,
            achieved_pattern_error_bits: 0,
            unread_inputs_validated: false,
            full_object_validated: false,
            wire_bytes_modified: false,
            native_object_pointer_validated: false,
        }
    }

    /// Keep runtime/whole-object qualifications attached to success; e.g. `plan.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationStyleComparisonDiagnostic; 4] {
        use SimulationStyleComparisonDiagnostic::*;
        [
            NativeNotExecuted,
            RuntimeFactsRequired,
            WireInputsNotMigrated,
            NotUniversalRenderEquivalence,
        ]
    }
}
