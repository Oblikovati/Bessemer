use super::*;

/// Validated inner body version; the separate outer version is always zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSettingsBodyVersion(pub(super) u8);

impl TryFrom<u32> for FlowSettingsBodyVersion {
    type Error = BessemerError;
    fn try_from(value: u32) -> Result<Self> {
        if value <= 11 {
            return Ok(Self(value as u8));
        }
        Err(body_failure(
            FailureClass::Unsupported,
            format!("body version {value}; expected 0 through 11"),
        ))
    }
}

impl FlowSettingsBodyVersion {
    /// Returns the parsed inner version; e.g. `let version = body.version().map(|version| version.wire_value())`.
    pub fn wire_value(self) -> u32 {
        u32::from(self.0)
    }
}

/// Native destination-member labels, not wire offsets or guessed engineering meanings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsBodyMember {
    /// Native destination +0x34.
    Member34,
    /// Native destination +0x38.
    Member38,
    /// Native destination +0x4c.
    Member4c,
    /// Native destination +0x4d.
    Member4d,
    /// Native destination +0x50.
    Member50,
    /// Native destination +0x2d.
    Member2d,
    /// Native destination +0x44.
    Member44,
    /// Native destination +0x0f; absent in versions zero and one.
    Member0f,
    /// Native destination +0x10.
    Member10,
    /// Native destination +0x11.
    Member11,
    /// Native destination +0x88.
    Member88,
    /// Native destination +0x3c.
    Member3c,
    /// Native destination +0x41.
    Member41,
    /// Native destination +0x40.
    Member40,
    /// Native destination +0x1e.
    Member1e,
    /// Native destination +0x89.
    Member89,
    /// Native destination +0x8a.
    Member8a,
    /// Native destination +0x48.
    Member48,
}

/// Exact established representation; raw fields do not acquire numeric interpretations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsBodyValue {
    /// Full stored byte, without Boolean coercion.
    Byte(u8),
    /// Established little-endian u32 bit pattern.
    Word(u32),
    /// Four uninterpreted bytes in wire order.
    Raw4([u8; 4]),
    /// Eight uninterpreted bytes in wire order.
    Raw8([u8; 8]),
}

/// One present field with exact payload offset and representation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSettingsBodyField {
    pub(super) member: FlowSettingsBodyMember,
    pub(super) offset: usize,
    pub(super) value: FlowSettingsBodyValue,
}

/// Stored legacy member or explicit runtime-context projection, including unresolved context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsLegacyMember {
    /// Actual full byte from a body version at least two.
    Stored(u8),
    /// Derived from explicit supplied runtime context; no stored byte was consumed.
    DerivedRuntime {
        /// Signed runtime slot-88 value supplied by the caller.
        context: i32,
        /// One for zero context, otherwise zero.
        value: u8,
    },
    /// Missing record or old version without an established runtime slot-88 value.
    RequiresRuntimeIntegerSlot88,
}

#[derive(Clone, Debug)]
pub(super) struct ParsedBody {
    pub version: FlowSettingsBodyVersion,
    pub payload: Arc<[u8]>,
    pub fields: Arc<[FlowSettingsBodyField]>,
}

/// Complete optional body view, source checkpoint and explicitly supplied runtime context.
#[derive(Clone, Debug)]
pub struct FlowSettingsBody {
    pub(super) source: FlowCarrier,
    pub(super) record: Option<FlowRecordId>,
    pub(super) parsed: Option<ParsedBody>,
    pub(super) runtime_integer_slot_88: Option<i32>,
}

/// Explicit optional-body validation scope.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsBodyCheckMode {
    /// Exact outer/inner versions, gated fields and complete payload EOF, or completed absence.
    ExactOptionalVersionedBody,
}

/// Stored-byte checks, independent of runtime-context resolution and member semantics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowSettingsBodyCoverage {
    /// Exact check scope.
    pub mode: FlowSettingsBodyCheckMode,
    /// Zero for absence, otherwise all selected payload bytes.
    pub checked_bytes: usize,
    /// Present native members, excluding the two version words.
    pub checked_members: usize,
    /// Stored member bytes without established engineering meanings.
    pub opaque_member_bytes: usize,
    /// Exact stored-byte disagreement, zero for completed views.
    pub achieved_byte_error: usize,
}

/// Explicit exclusions from body framing and supplied-context projection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowSettingsBodyDiagnostic {
    /// Native member labels establish destinations, not engineering semantics.
    MemberSemanticsNotEstablished,
    /// Supplied runtime context was not reconstructed from stored record 7.
    RuntimeContextNotReconstructed,
    /// Native loading, mutation and success-return behavior were not executed.
    NativeApplicationNotChecked,
}
