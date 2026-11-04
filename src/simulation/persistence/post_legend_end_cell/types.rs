use super::super::{SimulationRgbaColor, SimulationThermalRelation};

/// Versioned stored E8/EC roles, separate from actual native read-time facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendEndCellInputs {
    /// Signed stored E8, absent in earlier profiles; nonzero is not proof of activation.
    pub enabled_raw: Option<i32>,
    /// All four stored EC words, without an allowed-color or alpha interpretation.
    pub color: Option<SimulationRgbaColor>,
}

/// Explicit facts at their native reads; earlier stored snapshots supply no defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendEndCellRuntime {
    /// Actual runtime plot524, required before the E8 read is reached.
    pub plot_word_524_raw: Option<i32>,
    /// Actual E8 after earlier helpers, required only when plot524 is nonzero.
    pub enabled_at_gate_raw: Option<i32>,
    /// Actual EC at the later RGB copy, required only for an active gate.
    pub color_at_rgb_copy: Option<SimulationRgbaColor>,
}

/// Completed local projection, distinct from missing facts or native rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendEndCellCompletion {
    /// A reached zero gate produced no RGB copy.
    CompletedInactive,
    /// Three explicitly supplied RGB words were copied without floating arithmetic.
    CompletedRgbCopy,
}

/// All supplied parents remain available even when short-circuiting skips their reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendEndCellHistory {
    /// Original read-time facts, including the unused fourth color word.
    pub runtime: SimulationLegendEndCellRuntime,
    /// The local decision and optional RGB result derive from these facts.
    pub relation: SimulationThermalRelation,
}

/// Exact reached scalar reads with explicit downstream exclusions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendEndCellCoverage {
    /// Bits0..2 identify plot524, E8 and the three-word EC RGB copy.
    pub reached_read_mask: u8,
    /// Zero; no file snapshot is silently substituted for read-time facts.
    pub stored_snapshot_words_read: usize,
    /// Zero or three, never four: stored/read-time alpha is not copied to RGB.
    pub rgb_words_copied: usize,
    /// True; comparisons and MOVSS copies preserve exact raw patterns.
    pub exact_gate_and_rgb_bits: bool,
    /// Zero local raw-pattern error, without geometric error claims.
    pub achieved_pattern_error_bits: u32,
    /// False; supplied scalar facts establish no live pointer safety.
    pub runtime_pointer_validated: bool,
    /// False; dimensions, vertices, helper calls and rendering remain unmodeled.
    pub geometry_renderer_modeled: bool,
}

/// Qualifications retained by every completed local projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendEndCellDiagnostic {
    /// Target code was inspected without execution.
    NativeNotExecuted,
    /// Every reached field value must be supplied at its actual read boundary.
    ReadTimeFactsExplicit,
    /// Stored words do not establish live object initialization or helper isolation.
    StoredRuntimeIdentityNotInferred,
    /// The fourth supplied color word survives in history without alpha compositing.
    AlphaNotConsumed,
    /// No pointer, geometry, floating coordinates or rendered output is validated.
    GeometryRendererNotModeled,
}
