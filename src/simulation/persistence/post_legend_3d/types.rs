use super::super::{SimulationLegendHeightInputs, SimulationRgbaColor, SimulationThermalRelation};

/// Explicit facts at the mapping branch, not inferred from an earlier object snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegend3DBodyRuntime {
    /// Actual count14 read after vertex construction, which may differ from the allocation snapshot.
    pub cell_count_at_mapping_raw: Option<i32>,
    /// Plain by-value integer rendering argument; no stored ContourType identity is assigned.
    pub rendering_argument_raw: Option<i32>,
}

/// Local requests only; no buffer, geometry or successful rendering is published.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegend3DBodyValues {
    /// Low32 entry-count+1 allocation pair count, not a validated vertex-loop extent.
    pub allocation_pairs_raw: i32,
    /// Low32 six-float-per-pair temporary storage request.
    pub vertex_float_count_raw: i32,
    /// Native signed-extension/unsigned-multiply request, saturated to u64::MAX on overflow.
    pub vertex_allocation_bytes_raw: u64,
    /// Raw signed mapping count, incremented only for rendering argument2.
    pub mapping_request_count_raw: i32,
    /// Low32 three-float-per-color storage request with one extra temporary color.
    pub color_float_count_raw: i32,
    /// Saturated native byte request, without any allocation-success claim.
    pub color_allocation_bytes_raw: u64,
    /// Initial glShadeModel argument selected by the plain rendering argument.
    pub initial_shade_model_raw: u32,
    /// Initial glBegin argument7 or8; no mesh is produced.
    pub begin_primitive_raw: u32,
    /// Argument at the later smooth-shading call, not a claim about final GL state.
    pub later_smooth_argument_raw: u32,
}

/// Original entry snapshot and explicit later facts remain separate parents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegend3DBodyHistory {
    /// Original stored/allocation-entry count snapshot, including its raw signed value.
    pub inputs: SimulationLegendHeightInputs,
    /// Explicit post-vertex read and by-value rendering argument.
    pub runtime: SimulationLegend3DBodyRuntime,
    /// Local integer requests derive from both supplied parents.
    pub relation: SimulationThermalRelation,
}

/// Explicit reads for constructor, width-factor and lighting decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegend3DBackgroundRuntime {
    /// Packed color argument, unrelated to an inferred stored EC/100 identity.
    pub packed_color_raw: Option<u32>,
    /// Actual BC value at its width read after earlier helpers; not inferred from an entry snapshot.
    pub precision_at_width_raw: Option<i32>,
    /// Raw GLboolean result of glIsEnabled, required after precision processing.
    pub lighting_result_raw: Option<u8>,
}

/// Constructed local color and integer/call decisions, without float-getter or dimension output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegend3DBackgroundValues {
    /// Existing explicit nearest-even packed constructor result, not an emulated GetColorfv buffer.
    pub constructed_color: SimulationRgbaColor,
    /// Low32 precision+10 before conversion/multiplication by runtime character metrics.
    pub precision_factor_raw: i32,
    /// Background polygon glBegin argument9.
    pub begin_primitive_raw: u32,
    /// Lighting capability argument0xB50 used by the bound calls.
    pub lighting_capability_raw: u32,
    /// Whether the raw supplied GLboolean reaches the later glEnable call.
    pub restore_lighting: bool,
}

/// All original background facts survive without implicit file/runtime initialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegend3DBackgroundHistory {
    /// Original packed argument, BC read and lighting result.
    pub runtime: SimulationLegend3DBackgroundRuntime,
    /// The local constructor/factor/lighting decisions are derived.
    pub relation: SimulationThermalRelation,
}

/// Completion of local requests is distinct from renderer completion or missing facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegend3DCompletion {
    /// Completed a body projection whose raw mapping request is zero.
    CompletedZeroMappingRequest,
    /// Completed a body projection with a nonzero signed mapping request.
    CompletedNonzeroMappingRequest,
    /// Completed the local background input projection.
    CompletedBackgroundInputs,
}

/// Exact local coverage with explicit exclusions for allocation and renderer behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegend3DCoverage {
    /// One entry count snapshot for body; zero stored snapshots for background.
    pub stored_snapshot_words_read: usize,
    /// Two explicit later body facts or three explicit background facts.
    pub runtime_scalars_read: usize,
    /// True; every local integer step preserves the bound register pattern.
    pub exact_integer_patterns: bool,
    /// Whether saturated temporary allocation requests were computed.
    pub allocation_requests_computed: bool,
    /// Whether the explicit nearest-even packed constructor was modeled.
    pub nearest_even_constructor: bool,
    /// False; no memory allocation was attempted.
    pub actual_allocation_performed: bool,
    /// False; mapping, float getter, dimensions and rendering remain unmodeled.
    pub renderer_modeled: bool,
    /// False; raw requests and factors establish neither pointer nor geometry validity.
    pub pointer_geometry_validated: bool,
    /// Zero local raw-pattern error.
    pub achieved_pattern_error_bits: u32,
}

/// Qualifications accompanying completed local projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegend3DDiagnostic {
    /// Target code was inspected without execution.
    NativeNotExecuted,
    /// Buffers, loop bounds, coordinates and native allocation success were not validated.
    AllocationAndGeometryNotValidated,
    /// Color mapping and later renderer/helper behavior remain unmodeled.
    RendererHelpersNotModeled,
    /// Later field/GL results must be supplied explicitly rather than copied from an earlier snapshot.
    LaterFactsExplicit,
    /// A plain argument has no bound stored contour or color identity.
    StoredArgumentIdentityNotInferred,
    /// Constructor output does not establish getter output, floating dimensions or alpha compositing.
    BackgroundFloatingOutputNotModeled,
    /// Lighting restoration is not full GL-state restoration or a final-shading guarantee.
    CompleteGlStateNotEstablished,
}
