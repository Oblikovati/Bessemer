use super::super::SimulationThermalRelation;

/// Stored cell-count role; absence does not supply a runtime default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendHeightInputs {
    /// Raw signed object14 word, distinct from font metrics and requested tick count.
    pub cell_count_raw: Option<i32>,
}

/// Complete zero/nonzero scalar products remain distinct from missing inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendHeightCompletion {
    /// Completed the exact signed32 product with zero result bits.
    CompletedZeroProduct,
    /// Completed the exact signed32 product with nonzero result bits.
    CompletedNonzeroProduct,
}

/// Both original scalar parents survive without deriving a runtime font.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendHeightHistory {
    /// Original optional stored count.
    pub inputs: SimulationLegendHeightInputs,
    /// Explicit supplied return value of the cell-height helper.
    pub cell_height_raw: i32,
    /// The local product derives from both supplied parents.
    pub relation: SimulationThermalRelation,
}

/// Exact final-instruction coverage, without validating display geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendHeightCoverage {
    /// One reached stored multiplier.
    pub stored_words_read: usize,
    /// One reached explicitly supplied helper result.
    pub runtime_scalars_read: usize,
    /// True; low32 product bits match the observed IMUL instruction.
    pub exact_wrapping_signed32: bool,
    /// False; no LOGFONTW/SSE helper calculation was performed.
    pub cell_helper_modeled: bool,
    /// False; a scalar product is not validated pixel geometry.
    pub pixel_coordinates_validated: bool,
    /// False; the projection never changes file bytes.
    pub wire_edited: bool,
    /// Exact local product bits have zero achieved pattern error.
    pub achieved_pattern_error_bits: u32,
}

/// Native/font/tick qualifications accompany every completed scalar result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendHeightDiagnostic {
    /// SolidWorks target code was not executed.
    NativeNotExecuted,
    /// Runtime font conversion and exceptional SSE behavior remain unmodeled.
    CellHeightHelperNotModeled,
    /// Actual font metrics are not inferred from the count.
    RuntimeFontMetricsNotInferred,
    /// Title positioning, tick generation and final adjustment are not modeled.
    TitleAndTicksNotModeled,
    /// A complete raw product is not validated display geometry.
    PixelValidityNotEstablished,
}
