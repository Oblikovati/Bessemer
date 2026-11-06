use super::*;

impl FlowSettingsBody {
    /// Borrows the entire source checkpoint; e.g. `let original = body.source().as_bytes()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Returns the selected occurrence or absence; e.g. `let present = body.record().is_some()`.
    pub fn record(&self) -> Option<&FlowRecordId> {
        self.record.as_ref()
    }
    /// Returns the validated inner version or absence; e.g. `let version = body.version()`.
    pub fn version(&self) -> Option<FlowSettingsBodyVersion> {
        self.parsed.as_ref().map(|body| body.version)
    }
    /// Borrows present raw payload bytes, distinguishing absent records; e.g. `let raw = body.payload()`.
    pub fn payload(&self) -> Option<&[u8]> {
        self.parsed.as_ref().map(|body| body.payload.as_ref())
    }
    /// Borrows present members in exact wire order; e.g. `let fields = body.fields()`.
    pub fn fields(&self) -> &[FlowSettingsBodyField] {
        self.parsed
            .as_ref()
            .map_or(&[], |body| body.fields.as_ref())
    }
    /// Finds one stored member without inventing omitted defaults; e.g. `let field = body.field(FlowSettingsBodyMember::Member0f)`.
    pub fn field(&self, member: FlowSettingsBodyMember) -> Option<&FlowSettingsBodyField> {
        self.fields().iter().find(|field| field.member == member)
    }
    /// Returns stored member 0f or explicit legacy context; e.g. `let legacy = body.legacy_member_0f()`.
    pub fn legacy_member_0f(&self) -> FlowSettingsLegacyMember {
        if let Some(field) = self.field(FlowSettingsBodyMember::Member0f)
            && let FlowSettingsBodyValue::Byte(byte) = field.value
        {
            return FlowSettingsLegacyMember::Stored(byte);
        }
        match self.runtime_integer_slot_88 {
            Some(context) => FlowSettingsLegacyMember::DerivedRuntime {
                context,
                value: u8::from(context == 0),
            },
            None => FlowSettingsLegacyMember::RequiresRuntimeIntegerSlot88,
        }
    }
    /// Returns the supplied runtime value without inference from record 7; e.g. `let context = body.runtime_integer_slot_88()`.
    pub fn runtime_integer_slot_88(&self) -> Option<i32> {
        self.runtime_integer_slot_88
    }
    /// Reports exact selected-body checks; e.g. `let checked = body.coverage()`.
    pub fn coverage(&self) -> FlowSettingsBodyCoverage {
        let checked_bytes = self.payload().map_or(0, <[u8]>::len);
        FlowSettingsBodyCoverage {
            mode: FlowSettingsBodyCheckMode::ExactOptionalVersionedBody,
            checked_bytes,
            checked_members: self.fields().len(),
            opaque_member_bytes: checked_bytes.saturating_sub(8),
            achieved_byte_error: 0,
        }
    }
    /// Borrows native execution and semantic exclusions; e.g. `let exclusions = body.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowSettingsBodyDiagnostic] {
        &[
            FlowSettingsBodyDiagnostic::MemberSemanticsNotEstablished,
            FlowSettingsBodyDiagnostic::RuntimeContextNotReconstructed,
            FlowSettingsBodyDiagnostic::NativeApplicationNotChecked,
        ]
    }
}

impl FlowSettingsBodyField {
    /// Returns the native destination label; e.g. `let member = field.member()`.
    pub fn member(&self) -> FlowSettingsBodyMember {
        self.member
    }
    /// Returns the payload-relative wire offset; e.g. `let offset = field.payload_offset()`.
    pub fn payload_offset(&self) -> usize {
        self.offset
    }
    /// Returns all established bits with their raw representation; e.g. `let stored = field.value()`.
    pub fn value(&self) -> FlowSettingsBodyValue {
        self.value
    }
}
