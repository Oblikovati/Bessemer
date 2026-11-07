use super::{
    Result, SimulationElfInitialArchive, SimulationElfParameter, SimulationElfParameterId,
};

/// Exact native initializer check scope, separate from populated ELF acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfCheckMode {
    /// Every byte of the complete observed initializer, including all zero continuation bytes.
    ExactNativeInitialImage,
}

/// Quantified complete initializer checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationElfInitialCoverage {
    /// Explicit exact check mode.
    pub mode: SimulationElfCheckMode,
    /// Complete wrapped bytes checked and reconstructed.
    pub bytes: usize,
    /// Every stored four-byte header word.
    pub header_words: usize,
    /// Complete native zero continuation.
    pub zero_continuation_bytes: usize,
    /// All empty signed directory pairs.
    pub empty_directory_pairs: usize,
}

/// Exact stored-byte scope of a complete initializer parameter edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfInitialFidelity {
    /// Only the selected four-byte parameter differs; every other complete-image byte is exact.
    ExactBytesExceptSelectedParameter,
}

/// Required acceptance responsibilities outside the initial-image profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationElfDiagnostic {
    /// The complete populated-file lifecycle has not been established.
    PopulatedLifecycleNotChecked,
    /// Raw constructor parameters and scalar slots do not establish physical meanings.
    PhysicalSemanticsNotChecked,
    /// No native runtime gate, calculation or file-I/O path was executed.
    NativeRuntimeNotExecuted,
    /// The native application did not load this output.
    NativeApplicationNotChecked,
}

/// Complete isolated initial-parameter replacement and its identity derivation.
#[derive(Debug, Clone)]
pub struct SimulationElfInitialEdit {
    /// Immutable complete initializer result.
    pub archive: SimulationElfInitialArchive,
    /// Whether exact parameter bits changed.
    pub completion: crate::display_lists::DisplayEditCompletion,
    /// Exact complete-image byte preservation scope.
    pub fidelity: SimulationElfInitialFidelity,
    /// Original constructor parameter identity.
    pub parent: SimulationElfParameterId,
    /// Derived corresponding parameter identity.
    pub result: SimulationElfParameterId,
    /// Exact complete native initializer coverage.
    pub coverage: SimulationElfInitialCoverage,
    /// Achieved requested parameter-pattern error, zero; no physical error was established.
    pub parameter_pattern_error_bits: u32,
    /// Required unchecked responsibilities.
    pub diagnostics: [SimulationElfDiagnostic; 4],
}

impl SimulationElfInitialArchive {
    /// Quantify complete initializer validation, e.g. `initial.coverage().empty_directory_pairs`.
    pub fn coverage(&self) -> SimulationElfInitialCoverage {
        SimulationElfInitialCoverage {
            mode: SimulationElfCheckMode::ExactNativeInitialImage,
            bytes: 1224,
            header_words: 7,
            zero_continuation_bytes: 1192,
            empty_directory_pairs: 100,
        }
    }

    /// Explicit semantic and runtime exclusions, e.g. `initial.diagnostics()`.
    pub fn diagnostics(&self) -> [SimulationElfDiagnostic; 4] {
        diagnostics()
    }

    /// Replace one constructor parameter on an isolated complete image, e.g. `initial.edit_parameter(&id, bits)?`.
    pub fn edit_parameter(
        &self,
        id: &SimulationElfParameterId,
        bits: u32,
    ) -> Result<SimulationElfInitialEdit> {
        let changed = self.parameter(id)? != bits;
        let archive = if changed {
            self.replaced_parameter(id.parameter, bits)
        } else {
            self.clone()
        };
        Ok(SimulationElfInitialEdit {
            fidelity: SimulationElfInitialFidelity::ExactBytesExceptSelectedParameter,
            completion: if changed {
                crate::display_lists::DisplayEditCompletion::Changed
            } else {
                crate::display_lists::DisplayEditCompletion::Unchanged
            },
            parent: id.clone(),
            result: archive.parameter_id(id.parameter),
            coverage: archive.coverage(),
            diagnostics: archive.diagnostics(),
            parameter_pattern_error_bits: 0,
            archive,
        })
    }

    fn replaced_parameter(&self, parameter: SimulationElfParameter, bits: u32) -> Self {
        let mut parameters = self.parameters;
        match parameter {
            SimulationElfParameter::A => parameters.parameter_a = bits,
            SimulationElfParameter::B => parameters.parameter_b = bits,
        }
        Self::from_parameters(parameters)
    }
}

pub(super) fn diagnostics() -> [SimulationElfDiagnostic; 4] {
    [
        SimulationElfDiagnostic::PopulatedLifecycleNotChecked,
        SimulationElfDiagnostic::PhysicalSemanticsNotChecked,
        SimulationElfDiagnostic::NativeRuntimeNotExecuted,
        SimulationElfDiagnostic::NativeApplicationNotChecked,
    ]
}
