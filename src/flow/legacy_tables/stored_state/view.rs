use super::*;
impl FlowStoredStateMember {
    /// Returns the fixed serialized byte offset; e.g. `let offset = FlowStoredStateMember::Member5D8.payload_offset()`.
    pub fn payload_offset(self) -> usize {
        match self {
            Self::Member5E0 => 4,
            Self::Member5D8 => 8,
            Self::Member5DC => 12,
            Self::Discarded => 16,
        }
    }
}
impl FlowStoredState {
    /// Borrows the immutable original carrier; e.g. `let raw = state.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.wire.source
    }
    /// Borrows the selected revision-bound occurrence; e.g. `let ordinal = state.record().ordinal()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.wire.record
    }
    /// Returns the observed nonnegative version; e.g. `let version = state.version()`.
    pub fn version(&self) -> i32 {
        self.wire.version
    }
    /// Borrows every complete raw byte; e.g. `let raw = state.decoded_bytes()`.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.wire.decoded
    }
    /// Returns the admitted payload bound; e.g. `let bound = state.options().maximum_payload_bytes`.
    pub fn options(&self) -> FlowStoredStateOptions {
        self.options
    }
    /// Reads a stored signed member, retaining discarded-member absence; e.g. `let first = state.member(FlowStoredStateMember::Member5E0)`.
    pub fn member(&self, member: FlowStoredStateMember) -> Option<i32> {
        let offset = member.payload_offset();
        (offset < self.wire.decoded.len()).then(|| wire::signed_at(&self.wire.decoded, offset))
    }
    /// Reports the native version rule without running recomputation; e.g. `let required = state.native_recomputation_required()`.
    pub fn native_recomputation_required(&self) -> bool {
        self.version() < 2
    }
    /// Reports exact framing and stored-member coverage; e.g. `let coverage = state.coverage()`.
    pub fn coverage(&self) -> FlowStoredStateCoverage {
        FlowStoredStateCoverage {
            mode: wire::mode(&self.wire),
            payload_bytes: self.wire.decoded.len(),
            stored_members: self.wire.decoded.len() / 4 - 1,
            native_recomputation_required: self.native_recomputation_required(),
            achieved_error: 0,
        }
    }
    /// Borrows native and engineering exclusions; e.g. `let exclusions = state.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowStoredStateDiagnostic] {
        use FlowStoredStateDiagnostic::*;
        &[
            NativeRecomputationNotPerformed,
            NativeRuntimeNotReproduced,
            EngineeringRolesNotResolved,
            NativeApplicationNotChecked,
        ]
    }
}
impl FlowStoredStateEdit {
    /// Borrows the validated original checkpoint; e.g. `let before = edit.source()`.
    pub fn source(&self) -> &FlowStoredState {
        &self.source
    }
    /// Borrows the completely reopened result; e.g. `let after = edit.result()`.
    pub fn result(&self) -> &FlowStoredState {
        &self.result
    }
    /// Borrows complete physical history and carrier checkpoints; e.g. `let history = edit.carrier_edit().history()`.
    pub fn carrier_edit(&self) -> &FlowCarrierEdit {
        &self.carrier_edit
    }
}
