use super::{SimulationScalarArchive, SimulationScalarId, SimulationScalarKind};

/// Exact validation scope for a bare metadata word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationScalarCheckMode {
    /// All four bytes, without any transfer envelope or extra tail.
    ExactBareScalar,
}

/// Quantified bare-word representation and known semantic binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationScalarCoverage {
    /// Explicit exact check mode.
    pub mode: SimulationScalarCheckMode,
    /// Complete scalar bytes checked.
    pub bytes: usize,
    /// Normalized stream grammar.
    pub kind: SimulationScalarKind,
    /// Timestamp binding is known; RunState enum meaning is unassigned.
    pub semantic_binding_complete: bool,
    /// Always false: neither bare scalar has a transfer-count prefix.
    pub transfer_prefix: bool,
}

/// Explicit boundaries beyond complete stored scalar representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationScalarDiagnostic {
    /// An update stamp is not a wall-clock timestamp or a cross-session unique identity.
    ClockAndUniquenessNotAssigned,
    /// No complete state enum or solver-outcome meaning has been established.
    RunStateEnumUnassigned,
    /// Current-build binding does not identify every historical producer.
    HistoricalProducerNotIdentified,
    /// No native accessor, command-line conversion or storage I/O was executed.
    NativeRuntimeNotExecuted,
    /// The native application did not load this output.
    NativeApplicationNotChecked,
}

/// Complete isolated bare-word replacement with exact bit fidelity and identity derivation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationScalarFidelity {
    /// All requested four-byte patterns are stored exactly without numeric conversion.
    ExactBarePattern,
}

/// Complete isolated bare-word replacement and its acceptance scope.
#[derive(Debug, Clone)]
pub struct SimulationScalarEdit {
    /// Complete immutable resulting scalar.
    pub archive: SimulationScalarArchive,
    /// Whether stored bits changed.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Exact stored-pattern preservation scope.
    pub fidelity: SimulationScalarFidelity,
    /// Source scalar identity.
    pub parent: SimulationScalarId,
    /// Derived scalar identity.
    pub result: SimulationScalarId,
    /// Complete exact bare-word coverage.
    pub coverage: SimulationScalarCoverage,
    /// Requested pattern error, zero; no clock/physical error was established.
    pub pattern_error_bits: u32,
    /// Required semantic/runtime exclusions.
    pub diagnostics: [SimulationScalarDiagnostic; 4],
}

impl SimulationScalarArchive {
    /// Quantify exact representation, e.g. `scalar.coverage().semantic_binding_complete`.
    pub fn coverage(&self) -> SimulationScalarCoverage {
        SimulationScalarCoverage {
            mode: SimulationScalarCheckMode::ExactBareScalar,
            bytes: 4,
            kind: self.kind(),
            semantic_binding_complete: self.kind() == SimulationScalarKind::TimeStamp,
            transfer_prefix: false,
        }
    }

    /// Explicit binding limitations, e.g. `scalar.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationScalarDiagnostic; 4] {
        let meaning = match self.kind() {
            SimulationScalarKind::TimeStamp => {
                SimulationScalarDiagnostic::ClockAndUniquenessNotAssigned
            }
            SimulationScalarKind::RunState => SimulationScalarDiagnostic::RunStateEnumUnassigned,
        };
        [
            meaning,
            SimulationScalarDiagnostic::HistoricalProducerNotIdentified,
            SimulationScalarDiagnostic::NativeRuntimeNotExecuted,
            SimulationScalarDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

pub(super) fn edit_result(
    parent: &SimulationScalarArchive,
    archive: SimulationScalarArchive,
) -> SimulationScalarEdit {
    SimulationScalarEdit {
        fidelity: SimulationScalarFidelity::ExactBarePattern,
        completion: if parent.id == archive.id {
            crate::display_lists::DisplayEditCompletion::Unchanged
        } else {
            crate::display_lists::DisplayEditCompletion::Changed
        },
        parent: parent.id.clone(),
        result: archive.id.clone(),
        coverage: archive.coverage(),
        diagnostics: archive.diagnostics(),
        pattern_error_bits: 0,
        archive,
    }
}
