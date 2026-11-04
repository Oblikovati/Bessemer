use super::super::{SimulationLegendStepInputs, SimulationThermalRelation};

/// Caller-supplied facts at the reads following the three text helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepLayoutAfterHelpers {
    /// Spacing argument read after the helpers; no entry-value identity is assumed.
    pub spacing_raw: Option<i32>,
    /// Actual second FC read; it need not equal the stored entry snapshot.
    pub layout_switch_raw: Option<i32>,
    /// Runtime1B0; any nonzero value skips the1B4 read and triples spacing.
    pub word_1b0_raw: Option<i32>,
    /// Runtime1B4, required only when the reached1B0 value is zero.
    pub word_1b4_raw: Option<i32>,
}

/// Explicit native arguments/runtime words; no point, size, enable or post-state comes from FC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepLayoutRuntime {
    /// Raw explicit enable argument; exactly0 returns before all legend/point reads.
    pub enable_raw: i32,
    /// Supplied pointX, read before the text helper calls.
    pub point_x_raw: Option<i32>,
    /// Supplied pointY at the initial vertical-position reads.
    pub point_y_raw: Option<i32>,
    /// Supplied CSize vertical component, distinct from a font metric.
    pub size_y_raw: Option<i32>,
    /// Runtime12C, required only by a nonzero entry FC.
    pub word_12c_raw: Option<i32>,
    /// Runtime124, subtracted with low32 wrapping before signed division by20.
    pub word_124_raw: Option<i32>,
    /// Actual facts following the three helpers, required only for enabled projection.
    pub after_helpers: Option<SimulationLegendStepLayoutAfterHelpers>,
}

/// Exact local integer patterns, without font/drawing/pixel validity claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepLayoutValues {
    /// The supplied pointX copied before the helpers.
    pub point_x_raw: i32,
    /// Initial vertical result before the text helper calls.
    pub initial_y_raw: i32,
    /// Post-helper FC/flag-adjusted spacing, including wrapping results.
    pub spacing_raw: i32,
    /// The native low32 initialY-spacing result.
    pub vertical_after_spacing_raw: i32,
}

/// Completed early return and computed zero-valued layouts remain distinguishable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendStepLayoutCompletion {
    /// Explicit enable0 completed without reading any optional layout fact.
    CompletedDisabledEarlyReturn,
    /// Every reached scalar dependency was supplied and computed exactly.
    CompletedIntegerLayout,
}

/// Every original supplied parent survives without assuming helper isolation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepLayoutHistory {
    /// Stored entry FC, including an unconsumed value on early return.
    pub inputs: SimulationLegendStepInputs,
    /// All supplied facts, including optional unconsumed range/flag words.
    pub runtime: SimulationLegendStepLayoutRuntime,
    /// Derived scalar results leave the supplied facts unchanged.
    pub relation: SimulationThermalRelation,
}

/// Exact scalar dependency coverage, separate from real renderer execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepLayoutCoverage {
    /// Bits0..5 identify entry FC,12C,124,sizeY,pointY,pointX reads.
    pub entry_member_mask: u8,
    /// Bits0..3 identify post-helper spacing, FC,1B0 and1B4 reads.
    pub post_helper_member_mask: u8,
    /// Three represented native helper boundaries when enabled, zero on early return.
    pub native_text_helper_boundaries: usize,
    /// Whether the nonzero entry FC reached the exact signed division by20.
    pub division_20_reached: bool,
    /// True; all local additions/subtractions/multiplications preserve low32 bits.
    pub exact_wrapping_signed32: bool,
    /// Zero local integer-pattern error.
    pub achieved_pattern_error_bits: u32,
    /// False; no text helper or complete renderer was modeled.
    pub renderer_modeled: bool,
    /// False; signed integer coordinates do not establish valid pixels or pointers.
    pub pixel_coordinates_validated: bool,
}

/// Qualifications accompanying each completed local projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendStepLayoutDiagnostic {
    /// Target code was inspected without execution.
    NativeNotExecuted,
    /// Post-helper reads come from explicit facts rather than an entry snapshot.
    AfterHelperFactsExplicit,
    /// The three text callees, their arguments and localized output remain unmodeled.
    TextHelpersNotModeled,
    /// Fonts, drawing and later renderer state changes remain outside the projection.
    CompleteRendererNotModeled,
    /// No point/size pointer, pixel extent or display geometry was validated.
    PixelValidityNotEstablished,
}
