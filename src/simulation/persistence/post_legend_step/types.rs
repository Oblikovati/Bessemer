use super::super::SimulationThermalRelation;

/// Stored FC layout role; it never supplies a runtime availability argument.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepInputs {
    /// Optional signed FC word, present only in the applicable versioned profile.
    pub layout_switch_raw: Option<i32>,
}

impl SimulationLegendStepInputs {
    /// Inspect the raw layout branch without assigning visibility; e.g. `inputs.step_layout_nonzero()`.
    pub fn step_layout_nonzero(self) -> Option<bool> {
        self.layout_switch_raw.map(|word| word != 0)
    }
}

/// Six explicitly known signed plot words; no runtime pointer or word is inferred from a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepRuntime {
    /// Runtime plot1C, compared as signed32 with1.
    pub step_count_raw: i32,
    /// Runtime plot10; exactly21 supplies the count-qualified override.
    pub plot_type_raw: i32,
    /// Runtime plotC; exactly8 reaches the special return branch.
    pub analysis_type_raw: i32,
    /// Runtime plotBD8; exactly0 admits the ordinary count-qualified branch.
    pub word_bd8_raw: i32,
    /// Runtime plotBE8; exactly2 reaches the raw BEC return.
    pub word_be8_raw: i32,
    /// Runtime plotBEC, returned without boolean normalization when reached.
    pub word_bec_raw: i32,
}

/// Completion of the integer policy, without claiming rendering or native execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendStepCompletion {
    /// Complete policy evaluation returned raw0.
    CompletedZeroReturn,
    /// Complete policy evaluation returned a nonzero signed pattern, including negative BEC.
    CompletedNonzeroReturn,
}

/// Original runtime parents survive even when their words are not read by the reached branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepHistory {
    /// All six original caller-supplied words.
    pub runtime: SimulationLegendStepRuntime,
    /// The local return derives from runtime facts alone.
    pub relation: SimulationThermalRelation,
}

/// Exact reached native word reads, separate from the complete supplied context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationLegendStepCoverage {
    /// Bits0..5 identify count, plot type, analysis, BD8, BE8 and BEC reads.
    pub runtime_member_mask: u8,
    /// Zero; FC and other serialized words are not read by availability.
    pub stored_words_read: usize,
    /// True; all reached signed comparisons and raw return bits are exact.
    pub exact_signed32_policy: bool,
    /// Zero error in the local signed32 result pattern.
    pub achieved_pattern_error_bits: u32,
    /// False; supplied word values establish no native pointer safety.
    pub runtime_plot_pointer_validated: bool,
    /// False; layout arithmetic and drawing remain separate operations.
    pub renderer_modeled: bool,
}

/// Explicit qualifications accompanying every completed local availability result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationLegendStepDiagnostic {
    /// Target code was inspected without execution.
    NativeNotExecuted,
    /// All six runtime words were explicitly supplied by the caller.
    RuntimePlotWordsExplicit,
    /// No live plot pointer was accessed or validated.
    PointerSafetyNotEstablished,
    /// FC is a separate stored layout role, not this availability policy.
    LayoutSwitchNotAvailability,
    /// Text, layout and drawing output are not produced by this integer operation.
    RendererNotModeled,
}
