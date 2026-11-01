use super::super::SimulationThermalRelation;

/// Optional canonical A8 wire word; older structural absence remains unknown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPreferenceInputs {
    /// Original signed32 pattern, without constructor or preference defaults.
    pub stored_a8_raw: Option<i32>,
}

/// Actual caller-supplied runtime plot state for the local setter gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPreferencePlot {
    /// A known null plot pointer, not unknown plot state.
    NullPlot,
    /// A known nonnull plot with its original signed32 type word.
    PresentType(i32),
}

/// Explicit local setter facts, separate from archived words and whole-method validity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPreferenceRuntime {
    /// None means unknown; Some(NullPlot) means a known null pointer.
    pub plot_context: Option<SimulationLegendPreferencePlot>,
    /// Actual runtime preference1528; an early return need not consume it.
    pub preference_word_1528_raw: Option<i32>,
}

/// Reached local destination, with no resulting whole-object state inferred.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPreferenceEffect {
    /// Getter's derived first plotAdvPreference word.
    PreferenceWord0(i32),
    /// Setter's derived A8 word on a reached assignment path.
    LegendWordA8(i32),
    /// The early return assigns nothing; no prior A8 value is inferred.
    NoAssignment,
}

/// Native branch identity for the represented local conversion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPreferenceRoute {
    /// Export A8 through the getter's zero test.
    ExportA8,
    /// Reach the ordinary setter's local A8 assignment.
    ApplyOrdinary,
    /// Reach A8 assignment in the type70 branch; palette work remains separate.
    ApplyType70,
    /// Return before reading preference1528 for type22,57 or73.
    SkipPlotType,
}

/// Original facts remain separate from the lossy derived value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPreferenceSource {
    /// Actual optional wire control supplied to the getter projection.
    StoredA8(SimulationLegendPreferenceInputs),
    /// Explicit runtime setter facts, including any unconsumed preference word.
    RuntimePreference(SimulationLegendPreferenceRuntime),
}

/// Original inputs and the derived relation, without native execution claims.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPreferenceHistory {
    /// Every original input, including unused values on an early return.
    pub source: SimulationLegendPreferenceSource,
    /// The local effect derives from these facts.
    pub relation: SimulationThermalRelation,
}

/// Completed empty assignment selection is distinct from unknown required input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPreferenceCompletion {
    /// Exactly one local destination word was derived.
    CompletedWord,
    /// An early return completed with no local assignment.
    CompletedNoAssignment,
}

/// Exact local read/store coverage with explicit whole-method exclusions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendPreferenceCoverage {
    /// Whether A8 or preference1528 was consumed by the selected path.
    pub source_word_read: bool,
    /// Whether the setter inspected explicit plot-pointer state.
    pub plot_context_inspected: bool,
    /// Whether a nonnull plot's signed type word was inspected.
    pub plot_type_word_read: bool,
    /// Zero on an early return, otherwise one modeled word store.
    pub runtime_words_assigned: usize,
    /// Zero or four modeled runtime bytes, not additional wire bytes.
    pub runtime_store_bytes: usize,
    /// Zero or one native memory store represented by this local result.
    pub native_store_instructions: usize,
    /// Zero; derived values add no serialized field.
    pub additional_wire_bytes: usize,
    /// True; integer gates and zero tests are exact over signed32 inputs.
    pub exact_integer_policy: bool,
    /// Zero local raw-pattern error.
    pub achieved_pattern_error_bits: u32,
    /// False; original archives are unchanged.
    pub wire_bytes_modified: bool,
    /// False; other preference assignments, calls and pointer uses remain separate.
    pub whole_methods_emulated: bool,
    /// False; explicit scalar facts do not establish native pointer validity.
    pub native_object_pointer_validated: bool,
}

/// Qualifications attached to every completed local result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendPreferenceDiagnostic {
    /// Installed bodies were inspected without executing target code.
    NativeNotExecuted,
    /// Only the reached zero-test conversion or early return is represented.
    LocalZeroTestOnly,
    /// Arbitrary original signed words are retained rather than normalized.
    OriginalWordsRetained,
    /// Palette, color and remaining preference behavior is separate.
    WholePreferenceMethodsNotEmulated,
    /// Neither a completed local store nor an early return validates native pointers.
    NativePointerStateNotValidated,
}
