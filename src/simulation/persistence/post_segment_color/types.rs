use super::super::SimulationRgbaColor;

/// Available stored1BC facts; older opaque regions remain absent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationSegmentColorInputs {
    /// Original complete RGBA, including the purpose-unresolved fourth word.
    pub color_1bc: Option<SimulationRgbaColor>,
}

/// Explicit caller facts; unknown is distinct from known zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationSegmentColorRuntime {
    /// Core2C68 raw mode; only3 and4 reach the next check.
    pub mode_2c68_raw: Option<i32>,
    /// Core274C signed first element index.
    pub first_element_274c: Option<i32>,
    /// Core2750 signed last element index.
    pub last_element_2750: Option<i32>,
    /// CoreDA4 raw flags; zero or a set mask0x4 admits the read.
    pub flags_da4_raw: Option<u32>,
}

/// Furthest local caller stage, without a native invocation claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSegmentColorStage {
    /// Mode was outside3 and4.
    ModeBypassed,
    /// First index was nonpositive.
    FirstIndexRejected,
    /// Last index was nonpositive.
    LastIndexRejected,
    /// First index exceeded last index.
    RangeRejected,
    /// Nonzero flags lacked mask0x4.
    FlagsRejected,
    /// Caller guards admitted the local three-channel read.
    RgbReadAdmitted,
}

/// Completion applies only to the local RGB read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSegmentColorCompletion {
    /// No read was admitted.
    CompletedEmpty,
    /// Three exact raw color words were projected.
    CompletedRgbRead,
}

/// Explicit local coverage; geometry cannot receive a validity claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationSegmentColorCoverage {
    /// Zero or three read components.
    pub components_read: usize,
    /// False; endpoint membership, transforms and separation remain unchecked.
    pub geometry_validated: bool,
    /// Exact bit-copy projection has zero achieved pattern error.
    pub achieved_pattern_error_bits: u32,
}

/// Unresolved scope accompanies every completed local result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationSegmentColorDiagnostic {
    /// SolidWorks code was not executed.
    NativeNotExecuted,
    /// Pre-read state calls and their exceptional paths remain outside the model.
    PreReadStateUnmodeled,
    /// Endpoint lookup, transforms, separation, primitive calls and drawing remain unproven.
    EndpointGeometryUnknown,
    /// Reading three words does not establish the fourth word's purpose or global nonuse.
    StoredAlphaPurposeUnknown,
}
