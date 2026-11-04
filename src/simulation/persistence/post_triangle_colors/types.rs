use super::super::SimulationRgbaColor;

/// Stored controls for either documented C889 caller, without inferred runtime facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationTriangleColorControls {
    /// Style1F0 and1F4; unsigned selectors above1 bypass this caller branch.
    HiddenParts {
        /// Exact signed32 wire pattern; only equality to1 selects group RGB.
        selector_raw: i32,
        /// All four original supplied color patterns.
        color: SimulationRgbaColor,
    },
    /// Style21C,230 and220; only selector1 requests group RGB.
    ModelTessellation {
        /// Style21C; zero takes other caller paths.
        override_raw: i32,
        /// Exact signed32 wire pattern; only equality to1 selects group RGB.
        selector_raw: i32,
        /// All four original supplied color patterns.
        color: SimulationRgbaColor,
    },
}

/// Explicit external results; absent facts are required only when their branch is reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationTriangleColorCaller {
    /// Named accessor result and the later core678 load.
    HiddenParts {
        /// Known accessor result; None remains unknown.
        option_result: Option<i32>,
        /// Known reached core678 value; None remains unknown.
        core_678_raw: Option<i32>,
    },
    /// Renderer type and the later1800FF240 result.
    ModelTessellation {
        /// Explicit renderer type;5 bypasses the model caller.
        renderer_type_code: i32,
        /// Known1800FF240 result, evaluated after the renderer guard.
        precheck_result: Option<i32>,
    },
}

/// Explicit helper facts for the separate40C/410 group table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationTriangleColorRuntime {
    /// Typed facts for the same caller as the stored controls.
    pub caller: SimulationTriangleColorCaller,
    /// Presence of core2520, never inferred from file bytes.
    pub core_object_present: bool,
    /// Raw core40C, interpreted as signed32 for iteration.
    pub group_count_raw: u32,
    /// Presence of core410.
    pub group_array_present: bool,
    /// Core8C8; any nonzero value enables lookup and chooses color1C.
    pub selection_flag_raw: i32,
}

/// One complete owned group snapshot; payload identities never become validated geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationTriangleColorGroup {
    /// Group8; zero and nonzero select distinct caller classes.
    pub class_raw: i32,
    /// Signed group2C; only positive values reach slot58.
    pub triangle_count: i32,
    /// Unchanged group30/38 identities, including zero.
    pub payload_tokens: [u64; 2],
    /// Original group+C components; only RGB may be selected.
    pub color_c: SimulationRgbaColor,
    /// Original group+1C components; only RGB may be selected.
    pub color_1c: SimulationRgbaColor,
    /// Opaque input to1800AAFC0; no map semantics are inferred.
    pub lookup_key_raw: u32,
    /// Whether the external lookup returned the supplied end identity.
    pub lookup_matches_end: Option<bool>,
}

/// Both color parents remain available in the plan's owned group snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationTriangleColorOrigin {
    /// Supplied caller RGB.
    Supplied,
    /// Selected group+C RGB.
    GroupC,
    /// Selected group+1C RGB.
    Group1C,
}

/// One ordered local slot58 request with exact parent provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationTriangleColorCall {
    /// Original ordered parent index.
    pub group_index: u32,
    /// Signed group2C; only positive values reach slot58.
    pub triangle_count: i32,
    /// Unchanged group30/38 identities, including zero.
    pub payload_tokens: [u64; 2],
    /// All four original supplied color patterns.
    pub color: SimulationRgbaColor,
    /// Exact RGB parent; alpha always derives from supplied color.
    pub origin: SimulationTriangleColorOrigin,
}

/// Local stage distinguishes bypass, other unmodeled caller paths and admitted setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationTriangleColorStage {
    /// Accessor returned zero.
    HiddenDisabled,
    /// Unsigned hidden selector was above1.
    HiddenSelectorBypassed,
    /// Core678 selected other unmodeled rendering paths.
    HiddenOtherPath,
    /// Renderer type5 bypassed the model caller.
    ModelRendererExcluded,
    /// The explicit precheck result was nonzero.
    ModelPrecheckRejected,
    /// Stored21C was zero; other paths remain excluded.
    ModelOtherPath,
    /// Core2520 was absent.
    HelperCoreMissing,
    /// Raw group count was zero.
    HelperCountZero,
    /// The group array was absent.
    HelperArrayMissing,
    /// Setup admitted, including signed-empty iteration.
    HelperEntered,
}

/// Completion applies only to the bounded local request plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationTriangleColorCompletion {
    /// Planning completed with no local triangle requests.
    CompletedEmpty,
    /// Planning completed with ordered local requests.
    CompletedCalls,
}

/// Exact local coverage; exceptions are masked and DAZ/FTZ are explicitly off.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationTriangleColorCoverage {
    /// Original raw count, including a possible sign bit.
    pub declared_group_count: u32,
    /// Number of owned group snapshots actually inspected.
    pub inspected_groups: usize,
    /// Reached external lookup comparisons, including rejected groups.
    pub required_lookups: usize,
    /// Explicit resource budget applied before publication.
    pub maximum_groups: usize,
    /// True; this coverage does not assert geometry validity.
    pub local_requests_only: bool,
    /// Zero for exact local request patterns.
    pub achieved_pattern_error_bits: u32,
    /// True; native unmasked exceptions are not modeled.
    pub exceptions_masked: bool,
    /// False; subnormal source patterns remain significant.
    pub daz_enabled: bool,
    /// False; no flush-to-zero assumptions are inferred.
    pub ftz_enabled: bool,
}

/// Exclusions accompany every completed local plan, including empty results.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationTriangleColorDiagnostic {
    /// SolidWorks code was not invoked.
    NativeNotExecuted,
    /// Opaque tokens do not prove valid triangle storage.
    PayloadNotValidated,
    /// Other caller branches and state changes remain excluded.
    OtherCallerPathsNotModeled,
    /// No complete scene result is claimed.
    SceneTraversalNotModeled,
}
