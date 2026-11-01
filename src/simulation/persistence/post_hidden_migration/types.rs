use super::super::{SimulationPersistVersion, SimulationThermalRelation};

/// Original explicit context and optional wire word, without an invented stored1EC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationHiddenPartsMigrationInputs {
    /// Once-normalized caller version; migration applies only to700006..840007.
    pub version: SimulationPersistVersion,
    /// Original signed wire1F0, retained even when its runtime copy changes.
    pub stored_word_1f0_raw: Option<i32>,
}

/// Exact native store identity, including the eight-byte overwrite of the adjacent word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationHiddenPartsStore {
    /// A four-byte zero store to1EC, leaving original zero1F0 unchanged.
    ClearOptionWord,
    /// One eight-byte store of1 at1EC, producing the adjacent pair(1,0).
    WriteAdjacentPairOne,
    /// Separate four-byte stores of1 to1EC and1F0.
    WriteBothWordsOne,
}

/// Derived object words; neither value adds a serialized field or changes source bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationHiddenPartsMigrationValues {
    /// Derived runtime1EC, exactly0 or1 under this policy.
    pub hidden_option_raw: i32,
    /// Resulting runtime1F0, which may differ from the original stored word.
    pub runtime_word_1f0_raw: i32,
    /// The exact reached native store sequence.
    pub store: SimulationHiddenPartsStore,
}

/// Completion of this bounded load policy, without complete hidden-parts visibility.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationHiddenPartsMigrationCompletion {
    /// Original zero produced derived pair(0,0).
    CompletedClearedPair,
    /// Nonzero original produced option1 with runtime1F0 equal0 or1.
    CompletedOptionOne,
}

/// Complete original parents remain available after the derived runtime overwrite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationHiddenPartsMigrationHistory {
    /// Original typed version and optional stored value.
    pub inputs: SimulationHiddenPartsMigrationInputs,
    /// Both runtime words derive from these original inputs.
    pub relation: SimulationThermalRelation,
}

/// Exact local store coverage, separate from wire bytes and broader visibility checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimulationHiddenPartsMigrationCoverage {
    /// One original signed1F0 read after the version gate.
    pub stored_words_read: usize,
    /// One or two runtime words assigned by the reached stores.
    pub runtime_words_assigned: usize,
    /// One or two native store instructions represented by the result.
    pub native_store_instructions: usize,
    /// Four or eight runtime bytes assigned, without a wire-offset claim.
    pub runtime_store_bytes: usize,
    /// Zero; the migration consumes no new serialized bytes.
    pub additional_wire_bytes: usize,
    /// True; signed input patterns and the unsigned native branch are exact.
    pub exact_integer_policy: bool,
    /// Zero local raw-pattern error.
    pub achieved_pattern_error_bits: u32,
    /// False; no stored bytes were rewritten.
    pub wire_bytes_modified: bool,
    /// False; derived scalar pairs establish no native object pointer validity.
    pub native_object_pointer_validated: bool,
    /// False; application visibility and native pointer state remain outside this policy.
    pub full_visibility_validated: bool,
}

/// Qualifications retained by every completed migration result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationHiddenPartsMigrationDiagnostic {
    /// Target code was inspected without execution.
    NativeNotExecuted,
    /// This policy excludes the raw class and the later direct-field branch.
    ExplicitFieldVersionOnly,
    /// The derived option does not introduce an absent serialized field.
    DerivedPairNotStoredField,
    /// History retains original1F0 even after its runtime copy becomes zero or one.
    OriginalWordRetained,
    /// The complete visibility decision and all visual meanings of1F0 are not modeled.
    FullVisibilityNotModeled,
}
