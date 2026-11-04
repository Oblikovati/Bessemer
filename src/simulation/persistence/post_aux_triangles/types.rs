use super::super::SimulationRgbaColor;

/// Explicit caller runtime facts; these are not inferred from an archive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxTriangleRuntime {
    /// Presence of core2520.
    pub core_object_present: bool,
    /// Supplied core170 word.
    pub plot_word_170: i32,
    /// Raw count, interpreted as signed32 for iteration.
    pub group_count_raw: u32,
    /// Presence of the group array.
    pub group_array_present: bool,
    /// Supplied type code;3 through5 are excluded.
    pub renderer_type_code: i32,
}

/// One owned group snapshot; payload tokens are opaque and never dereferenced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxTriangleGroup {
    /// Only positive signed counts reach slot58.
    pub triangle_count: i32,
    /// Both payload identities, including zero when supplied.
    pub payload_tokens: [u64; 2],
    /// Only RGB may be selected; group alpha is ignored.
    pub color: SimulationRgbaColor,
}

/// Exact RGB derivation; alpha always comes from the supplied caller color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxTriangleColorSource {
    /// Supplied RGB with capped supplied alpha.
    Supplied,
    /// Group RGB with capped supplied alpha.
    GroupRgbWithSuppliedAlpha,
}

/// One derived slot58 request; it does not establish a draw or valid geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxTriangleCall {
    /// Parent group index in the original ordered input.
    pub group_index: u32,
    /// Positive original count.
    pub triangle_count: i32,
    /// Unchanged opaque payload identities.
    pub payload_tokens: [u64; 2],
    /// Exact color requested for this call.
    pub color: SimulationRgbaColor,
    /// Derivation from the supplied color and, when selected, this group.
    pub color_source: SimulationAuxTriangleColorSource,
}

/// Ordered helper rejection, separate from missing required group snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxTriangleRejection {
    /// Core2520 is null.
    MissingCoreObject,
    /// Core170 equals51.
    ExcludedPlot,
    /// Raw count is zero.
    EmptyGroupCount,
    /// The group array is null.
    MissingGroupArray,
    /// Renderer type is3,4 or5.
    ExcludedRenderer,
}

/// Furthest admitted local stage, without claiming callback completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxTriangleStage {
    /// Caller did not request the helper.
    CallerBypassed,
    /// Helper rejected the context before setup.
    HelperRejected(SimulationAuxTriangleRejection),
    /// Setup was admitted; iteration may still produce no calls.
    HelperEntered,
}

/// Completion concerns only the bounded local request plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxTriangleCompletion {
    /// Planning completed with no triangle requests.
    CompletedEmpty,
    /// Planning completed with ordered triangle requests.
    CompletedCalls,
}

/// Exact local coverage and budget, excluding geometry validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationAuxTriangleCoverage {
    /// Coverage concerns local requests, not geometry or scene validation.
    pub local_requests_only: bool,
    /// Original raw runtime count, including a possible sign bit.
    pub declared_group_count: u32,
    /// Number of provided groups inspected.
    pub inspected_groups: usize,
    /// Caller budget used for this plan.
    pub maximum_groups: usize,
    /// Exact supplied request patterns have zero achieved error.
    pub achieved_pattern_error_bits: u32,
}

/// Exclusions returned alongside every completed local plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationAuxTriangleDiagnostic {
    /// SolidWorks code was not executed.
    NativeNotExecuted,
    /// Opaque identities do not prove valid triangle storage.
    PayloadNotValidated,
    /// Other passes and unbound callback effects remain outside this plan.
    SceneTraversalNotModeled,
}
