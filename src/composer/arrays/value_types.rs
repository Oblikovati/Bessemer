use super::{ComposerArrayCoverage, ComposerArrayHistory};

/// Explicit version and arithmetic context for component reconstruction (14.97).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerVectorProfile {
    /// Behavior 1 for version 7.9: nearest-even, gradual underflow and separate binary32 operations.
    NearestEvenGradual79V1,
}

/// Completion state of a successful numeric reconstruction, including an empty output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerVectorValueCompletion {
    /// Every requested component was reconstructed under the declared profile.
    Completed,
}

/// Achieved error domain of the returned component patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerVectorValueFidelity {
    /// Zero bit error relative to the declared arithmetic profile; no source-geometry error bound.
    ExactProfilePatterns,
}

/// Explicit numeric check mode, independent of native application state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerVectorValueCheckMode {
    /// Every component evaluated under the selected software profile.
    ExactDeclaredProfile,
}

/// Checks not implied by a completed component reconstruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComposerVectorValueDiagnostic {
    /// Native MXCSR/FTZ/DAZ/traps and exception flags were not measured or reproduced.
    NativeControlStateNotChecked,
    /// Topology, geometry, self-intersection and tolerance checks were not executed.
    GeometryNotChecked,
    /// Error against unavailable original unquantized geometry is unknown.
    QuantizationErrorNotEstablished,
    /// Loaded-mesh/application acceptance was not checked.
    NativeApplicationNotChecked,
}

/// Exact source and output coverage for one completed reconstruction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComposerVectorValueCoverage {
    /// Declared check mode.
    pub mode: ComposerVectorValueCheckMode,
    /// Entire validated original packed block.
    pub source: ComposerArrayCoverage,
    /// Number of returned scalar components, including constants.
    pub components: usize,
    /// Direct/constant lanes only; no quantized varying lane was present.
    pub raw_copy_only: bool,
}

/// Completion, fidelity, coverage, provenance and exclusions returned together.
#[derive(Debug, Clone)]
pub struct ComposerVectorReconstructionReport {
    /// Explicit version/arithmetic context.
    pub profile: ComposerVectorProfile,
    /// Completed empty/nonempty output state.
    pub completion: ComposerVectorValueCompletion,
    /// Achieved profile-bit fidelity.
    pub fidelity: ComposerVectorValueFidelity,
    /// Complete source/output validation scope.
    pub coverage: ComposerVectorValueCoverage,
    /// Derived component-array revision and all parents (one packed array).
    pub history: ComposerArrayHistory,
    /// Required distinctions from native application or geometry acceptance.
    pub diagnostics: [ComposerVectorValueDiagnostic; 4],
}
