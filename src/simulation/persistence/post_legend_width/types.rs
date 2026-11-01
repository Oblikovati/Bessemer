use super::super::SimulationThermalRelation;

/// Optional stored B4 role, without a font input or live initialization default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendWidthInputs {
    /// Raw signed selector; values other than0 and1 remain unchanged.
    pub selector_raw: Option<i32>,
}

/// Actual entry reads, separate from earlier serialized snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendWidthRuntime {
    /// Signed word68 from the supplied font, read before B4.
    pub font_size_at_entry_raw: Option<i32>,
    /// Actual legend B4; absence is unknown, never a default selector.
    pub selector_at_entry_raw: Option<i32>,
}

/// Distinct bound native operations, rather than normalized enum labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendWidthOperation {
    /// Every raw selector other than0 and1 keeps the initial pattern.
    KeepInitial,
    /// Selector1 uses ADDSS of the initial width to itself.
    AddInitialToItself,
    /// Selector0 uses MULSS with exact constant4.0.
    MultiplyByFour,
}

/// Every local binary32 stage under the explicitly selected nearest-even policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendWidthValues {
    /// Rounded signed32-to-binary32 font conversion.
    pub converted_font_bits: u32,
    /// Rounded product with exact source constant0x3a83126f.
    pub scaled_font_bits: u32,
    /// Rounded product of the scaled font and0.625.
    pub initial_width_bits: u32,
    /// Final local width pattern; this is not a validated pixel or physical size.
    pub selected_width_bits: u32,
    /// Exact reached B4 branch, preserving addition versus multiplication.
    pub operation: SimulationLegendWidthOperation,
}

/// Completion of the local numerical projection, without renderer completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendWidthCompletion {
    /// Signed font0 produced an exact zero width.
    CompletedZeroWidth,
    /// A nonzero signed font produced a finite normal width, including negative values.
    CompletedNonzeroWidth,
}

/// All original read-time facts survive independently of later caller mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendWidthHistory {
    /// Explicit font68 and B4 entry reads.
    pub runtime: SimulationLegendWidthRuntime,
    /// All projected patterns derive from the supplied entry facts.
    pub relation: SimulationThermalRelation,
}

/// Exact local numerical coverage with explicit native-state exclusions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendWidthCoverage {
    /// Two explicitly supplied signed entry reads.
    pub runtime_words_read: usize,
    /// Zero; no stored snapshot was substituted automatically.
    pub stored_snapshot_words_read: usize,
    /// Two initial multiplications, plus the selector0 multiplication when reached.
    pub rounded_multiplications: usize,
    /// Whether selector1 reached its distinct ADDSS operation.
    pub addition_reached: bool,
    /// True; integer arithmetic implements the explicit nearest-even policy.
    pub exact_nearest_even_patterns: bool,
    /// Zero local pattern error under that policy.
    pub achieved_pattern_error_bits: u32,
    /// False; the live process's MXCSR or other floating state was not read.
    pub ambient_sse_state_validated: bool,
    /// False; no native pointer, geometry, physical unit or rendered output was validated.
    pub pointer_geometry_renderer_validated: bool,
}

/// Qualifications retained with every completed local width result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendWidthDiagnostic {
    /// Target code was inspected without execution.
    NativeNotExecuted,
    /// Entry font and selector facts must be supplied explicitly.
    ReadTimeFactsExplicit,
    /// Nearest-even is the declared model policy, not observed ambient native state.
    NearestEvenPolicyExplicit,
    /// Supplied scalars establish no live pointer or geometric validity.
    PointerGeometryNotValidated,
    /// No universal width units or rendered output are established.
    RendererAndUnitsNotEstablished,
}
