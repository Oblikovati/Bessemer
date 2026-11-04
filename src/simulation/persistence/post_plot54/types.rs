use super::super::SimulationRgbaColor;

/// Complete stored pair; the2BC forwarding role does not establish its consumer purpose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPlot54Colors {
    /// Original2AC RGBA, read by the named GetColor4fv boundary.
    pub color_2ac: SimulationRgbaColor,
    /// Original2BC RGBA, copied and forwarded separately without a bound purpose.
    pub forwarded_2bc: SimulationRgbaColor,
}

/// Canonical available file facts; absence never becomes an initialization default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPlot54Inputs {
    /// Parent170 code, or None for a standalone style without supplied parent context.
    pub plot_type_raw_code: Option<i32>,
    /// Present only when both complete stored color groups exist.
    pub colors: Option<SimulationPlot54Colors>,
}

/// Explicit reached runtime facts; None is unknown, distinct from known false/null.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPlot54Runtime {
    /// Raw core2C68; only3 and4 admit the next caller check.
    pub mode_2c68_raw: Option<i32>,
    /// Presence of helper core2520, read after the caller's color copies.
    pub core_object_present: Option<bool>,
    /// Whether the key10 lookup returned a nonnull callback; its ABI remains unbound.
    pub callback_key10_present: Option<bool>,
}

/// Furthest local normal-flow stage; this does not assert native execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPlot54Stage {
    /// Mode was outside the unsigned subtraction window for3 and4.
    ModeBypassed,
    /// The known parent plot code differed from54.
    PlotBypassed,
    /// Caller copied both colors but the helper core was known null.
    CoreRejected,
    /// Caller copied both colors but key10 was known null.
    CallbackRejected,
    /// Both guards admitted the local read/clamp projection.
    ColorReadAdmitted,
}

/// Completion concerns only bounded color requests/projections, never a draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPlot54Completion {
    /// No local renderer color read was admitted; caller copies may still exist.
    CompletedEmpty,
    /// Exact local read and alpha projection were admitted.
    CompletedColorProjection,
}

/// Exact local color coverage under explicitly named floating-point assumptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationPlot54Coverage {
    /// Number of complete caller color arguments copied, zero or two.
    pub copied_color_arguments: usize,
    /// Number of renderer components projected at the read boundary, zero or four.
    pub renderer_components_read: usize,
    /// True; no callback geometry or graphics-state validity is implied.
    pub local_color_only: bool,
    /// Exact raw color operations have zero achieved pattern error.
    pub achieved_pattern_error_bits: u32,
    /// True; unmasked COMISS exceptions are outside this model.
    pub exceptions_masked: bool,
    /// False; subnormal inputs retain their original patterns.
    pub daz_enabled: bool,
    /// False; no flush-to-zero behavior is inferred.
    pub ftz_enabled: bool,
}

/// Exclusions accompany every completed local plan, including bypassed callers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPlot54Diagnostic {
    /// SolidWorks code was not executed.
    NativeNotExecuted,
    /// Virtual state changes, later graphics state and reflection are not modeled.
    RendererStateNotModeled,
    /// Callback ABI, geometry records and scene acceptance remain unbound.
    CallbackGeometryUnknown,
    /// The separate2BC copy retains an unknown consumer purpose.
    Forwarded2bcPurposeUnknown,
}
