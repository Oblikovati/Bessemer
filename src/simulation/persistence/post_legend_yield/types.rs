use super::super::{SimulationRgbaColor, SimulationThermalRelation};

/// Stored inputs read by the gray mapper; presence does not establish actual invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendYieldInputs {
    /// All four D8 words, without equal-channel or alpha constraints.
    pub gray_base_color: Option<SimulationRgbaColor>,
    /// Entry reversal wordC; a later call rereads its own explicit value.
    pub reverse_base_palette_raw: Option<i32>,
}

/// Explicit scalar reads after the suffix helper returns; these are not inferred snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendYieldPrefixState {
    /// Reread wordC, including any helper-induced change.
    pub reverse_base_palette_raw: i32,
    /// Reread word10, without clamping to the nine serialized slots.
    pub base_palette_count_raw: i32,
    /// Reread runtime word178; no entry-index identity is assumed.
    pub split_index_raw: i32,
}

/// Caller facts for one native invocation; yield/tick adjustment is a separate operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendYieldRuntime {
    /// Raw signed EDX, forwarded unchanged by the -2 path even when negative.
    pub requested_output_count_raw: i32,
    /// Runtime word178 at the entry read; absent is unknown, not -1.
    pub split_index_raw: Option<i32>,
    /// Required only for the second call on the partial path, including index0.
    pub after_suffix: Option<SimulationLegendYieldPrefixState>,
}

/// Identity of the native base area passed to an unmodeled mapping helper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendYieldBase {
    /// Local copied D8 and constructed packed-white pair, retained by the plan.
    CopiedYieldPair,
    /// Original legend+18 base area; this descriptor does not validate its extent.
    OrdinaryLegendArea,
}

/// One ordered call request; no output pointer or mapped colors are materialized.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendYieldCall {
    /// Relative destination byte displacement after native signed32 wrapping.
    pub destination_byte_offset: i64,
    /// Signed destination count passed through EDX.
    pub output_count_raw: i32,
    /// Signed base count passed through R9D.
    pub base_count_raw: i32,
    /// Raw reversal argument, retained without boolean normalization.
    pub reversal_raw: i32,
    /// Exact source-area identity, independent of pointer validity.
    pub base: SimulationLegendYieldBase,
}

/// Completion of local planning, distinct from successful interpolation or native execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendYieldCompletion {
    /// Complete empty call list, with conditional native return0.
    CompletedFallbackPlan,
    /// Complete ordered nonempty call list, with return1 only if those calls complete.
    CompletedMappingCallPlan,
}

/// All original supplied parents of a completed local plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendYieldHistory {
    /// Original stored facts, including supplied but unconsumed reversal.
    pub inputs: SimulationLegendYieldInputs,
    /// Original caller facts, including an unused supplied post-suffix state.
    pub runtime: SimulationLegendYieldRuntime,
    /// Derived call requests leave every input unchanged.
    pub relation: SimulationThermalRelation,
}

/// Exact local coverage; a complete plan proves no downstream output validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendYieldCoverage {
    /// True even on fallback, because color setup precedes index testing.
    pub stored_color_copied: bool,
    /// Whether entry wordC was consumed by a reached mapping call.
    pub entry_reversal_read: bool,
    /// Whether the post-suffix rereads were consumed.
    pub post_suffix_state_read: bool,
    /// True; signed comparisons and low32 offset arithmetic are exact.
    pub exact_local_integer_plan: bool,
    /// Zero error in retained raw words and derived integer patterns.
    pub achieved_pattern_error_bits: u32,
    /// False; helper reads/writes and mapped colors remain unmodeled.
    pub mapping_helper_modeled: bool,
    /// False; signed counts and displacements do not establish valid memory access.
    pub output_memory_validated: bool,
}

/// Explicit qualifications accompanying every completed local call plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendYieldDiagnostic {
    /// Installed code was inspected without executing the target.
    NativeNotExecuted,
    /// Runtime yield-to-index selection remains outside this operation.
    TickAdjustmentNotModeled,
    /// Mapping helper interpolation and state changes remain outside this operation.
    MappingHelperNotModeled,
    /// No count, pointer, base extent or mapped geometry was validated.
    OutputMemoryNotValidated,
    /// Reached post-suffix reads require explicit facts rather than entry snapshots.
    PostSuffixFactsExplicit,
    /// Packed-white construction uses the existing explicit nearest-even model.
    NearestEvenWhiteConstruction,
}
