use super::*;

impl FlowContextCounts {
    /// Borrows the complete original carrier; e.g. `let wire = counts.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = counts.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Borrows every decoded prefix/body byte; e.g. `let raw = counts.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.decoded
    }
    /// Borrows the unchecked body bytes without interpreting a delegated serializer; e.g. `let opaque = counts.opaque_body()`.
    pub fn opaque_body(&self) -> &[u8] {
        &self.decoded[12..]
    }
    /// Returns the observed positive version; e.g. `let version = counts.version()`.
    pub fn version(&self) -> i32 {
        self.version
    }
    /// Returns the validated native G.Fluid label; e.g. `let fluid = counts.g_fluid()`.
    pub fn g_fluid(&self) -> i32 {
        self.counts.pair()[0]
    }
    /// Returns the validated native G.Solid label; e.g. `let solid = counts.g_solid()`.
    pub fn g_solid(&self) -> i32 {
        self.counts.pair()[1]
    }
    /// Returns the exact mathematical sum, without signed wrapping; e.g. `let total = counts.mathematical_total()`.
    pub fn mathematical_total(&self) -> u64 {
        self.counts.mathematical_total()
    }
    /// Returns the native signed 32-bit wrapped sum; e.g. `let wrapped = counts.native_total()`.
    pub fn native_total(&self) -> i32 {
        self.counts.native_total()
    }
    /// Classifies the version-selected body branch without decoding it; e.g. `let branch = counts.body_branch()`.
    pub fn body_branch(&self) -> FlowCountBodyBranch {
        if self.version <= 17 {
            FlowCountBodyBranch::Legacy1To17
        } else {
            FlowCountBodyBranch::Restored18Plus
        }
    }
    /// Returns the exact transport/byte options; e.g. `let maximum = counts.options().maximum_decoded_bytes`.
    pub fn options(&self) -> FlowCountPrefixOptions {
        self.options
    }
    /// Returns known-prefix and opaque-body coverage separately; e.g. `let unchecked = counts.coverage().opaque_body_bytes`.
    pub fn coverage(&self) -> FlowCountPrefixCoverage {
        self.coverage
    }
    /// Borrows the prefix-only acceptance exclusions; e.g. `let exclusions = counts.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowCountPrefixDiagnostic] {
        &[
            FlowCountPrefixDiagnostic::OpaqueBodyNotValidated,
            FlowCountPrefixDiagnostic::Record433ContextNotProven,
            FlowCountPrefixDiagnostic::OwnerCompressionContextNotReconstructed,
            FlowCountPrefixDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowInitializedCountBinding {
    /// Borrows the complete prefix source checkpoint and coverage; e.g. `let original = binding.counts()`.
    pub fn counts(&self) -> &FlowContextCounts {
        &self.counts
    }
    /// Returns the validated conditional record-433 count; e.g. `let expected = binding.expected_count()`.
    pub fn expected_count(&self) -> u32 {
        self.count
    }
    /// Returns the required initialization assumption; e.g. `let assumed = binding.assumption()`.
    pub fn assumption(&self) -> FlowAbsentMapInitialization {
        self.assumption
    }
    /// Projects native pointer partition layout without allocation; e.g. `let bytes = binding.pointer_layout().pointer_array_bytes`.
    pub fn pointer_layout(&self) -> FlowInitializedPointerLayout {
        let fluid = self.counts.g_fluid() as u32;
        FlowInitializedPointerLayout {
            pointer_slot_bytes: 8,
            pointer_array_bytes: u64::from(self.count) * 8,
            fluid_start_index: 0,
            solid_start_index: fluid,
            fluid_pointer_byte_offset: 0,
            solid_pointer_byte_offset: u64::from(fluid) * 8,
        }
    }
    /// Reports the documented assignment only under the retained assumption; e.g. `assert!(binding.count_assignment_proven_under_assumption())`.
    pub fn count_assignment_proven_under_assumption(&self) -> bool {
        true
    }
    /// Reports absence of native execution; e.g. `assert!(!binding.native_context_executed())`.
    pub fn native_context_executed(&self) -> bool {
        false
    }
    /// Reports absence of pointer population; e.g. `assert!(!binding.pointer_population_reproduced())`.
    pub fn pointer_population_reproduced(&self) -> bool {
        false
    }
    /// Reports exclusion of arbitrary reused maps; e.g. `assert!(!binding.existing_map_context_supported())`.
    pub fn existing_map_context_supported(&self) -> bool {
        false
    }
}

impl FlowBoundMapArrays {
    /// Borrows the conditional count provenance and original prefix; e.g. `let counts = bound.binding().counts()`.
    pub fn binding(&self) -> &FlowInitializedCountBinding {
        &self.binding
    }
    /// Borrows the complete arrays with existing fixed-cell edit support; e.g. `let values = bound.arrays().second()`.
    pub fn arrays(&self) -> &FlowMapArrays {
        &self.arrays
    }
}
