use super::{SimulationBodyQueryContext, SimulationBodyQueryMesh};
use std::sync::Arc;

/// Complete operation distinguished from the native routine's always-zero return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationBodyQueryCompletion {
    /// No destination was written; prior/unknown marks remain intact.
    CompletedEmpty,
    /// All inspected writes completed in deterministic visit order.
    CompletedWrites,
}

/// Source parent within the immutable mesh retained by the result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationBodyWriteSource {
    /// Mode-1 active-node visit, with no element or callback processing.
    NodeActivity,
    /// Element reference selected by the supplied query context.
    ElementReference {
        /// Zero-based source element index.
        element_index: usize,
        /// Zero-based selected reference slot within that element.
        reference_slot: usize,
    },
}

/// One ordered write, including all overwritten values and duplicate destinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationBodyNodeWrite {
    /// Zero-based node slot in the retained source revision.
    pub node_slot: usize,
    /// Complete source relationship, combined with the result's source mesh revision.
    pub source: SimulationBodyWriteSource,
    /// Captured previous value; None means unknown, not zero.
    pub previous_pattern: Option<u32>,
    /// Exact written bits.
    pub pattern: u32,
}

/// Explicit exact query scope; neither mode validates geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationBodyQueryCheckMode {
    /// Every stored node activity, bypassing element and callback processing.
    ExactAllNodeActivity,
    /// Selected element references and active destination nodes only.
    ExactSelectedReferencesAndNodeActivity,
}

/// Complete selected-query checks with untouched values and exclusions explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationBodyQueryCoverage {
    /// Declared exact validation scope.
    pub mode: SimulationBodyQueryCheckMode,
    /// Every element visited before its selection gates, zero in mode 1.
    pub visited_elements: usize,
    /// Elements passing activity/body/family selection.
    pub selected_elements: usize,
    /// Selected stored references examined, including zeros and inactive destinations.
    pub visited_references: usize,
    /// Every performed write, including duplicate destinations.
    pub write_visits: usize,
    /// Final unknown slots, preserved rather than invented as zero.
    pub unknown_slots: usize,
}

/// Responsibilities deliberately outside this pure supplied-context operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationBodyQueryDiagnostic {
    /// No CAD or assembly-body identity conversion has been established.
    CadOwnershipUnassigned,
    /// Supplied callback values were consumed; no callback was executed.
    NativeCallbackNotExecuted,
    /// No native runtime, pointer guards or historical session was executed/recovered.
    NativeRuntimeNotExecuted,
    /// Geometry/topology consistency, intersections and tolerance validity remain unchecked.
    GeometryNotValidated,
}

/// Immutable completed marks, exact checks and complete ordered write history.
#[derive(Debug, Clone)]
pub struct SimulationBodyQueryResult {
    /// Owned immutable source blocks bind every history index and parent.
    pub source: SimulationBodyQueryMesh,
    /// Exact normalized supplied context.
    pub context: SimulationBodyQueryContext,
    /// Captured caller marks before this operation; untouched entries remain these values.
    pub initial_marks: Arc<[Option<u32>]>,
    /// Final marks, retaining unknown values where no initial value/write exists.
    pub marks: Arc<[Option<u32>]>,
    /// Every write and overwritten value, in native visit order.
    pub writes: Arc<[SimulationBodyNodeWrite]>,
    /// Completed empty versus completed writes.
    pub completion: SimulationBodyQueryCompletion,
    /// Exact selected scope and coverage counters.
    pub coverage: SimulationBodyQueryCoverage,
    /// Native scalar return value, always zero, independent of performed writes.
    pub native_return_value: i32,
    /// Final numeric type flag, without API enum conversion.
    pub type_flag: i32,
    /// Whether native branch rules resolved the type flag, even if its value remains -1.
    pub type_flag_resolved: bool,
    /// Supplied body low word presented to the callback path; absent in mode 1.
    pub callback_initial_i16: Option<i16>,
    /// Exact achieved mark-pattern error; zero, without a physical-error claim.
    pub mark_pattern_error_bits: u32,
    /// Runtime, ownership and geometry exclusions.
    pub diagnostics: [SimulationBodyQueryDiagnostic; 4],
}
