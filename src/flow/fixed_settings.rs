//! Strict raw settings arrays; runtime normalizations never alter this view.
use super::*;

/// Native fixed-array profile, independent of per-slot configuration meanings.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFixedSettingsKind {
    /// Record 7: one hundred signed 32-bit slots.
    Integer,
    /// Record 102: three hundred twenty binary64 slots, preserving all bits.
    Numeric,
}

impl FlowFixedSettingsKind {
    /// Returns the ordinary lookup word; e.g. `assert_eq!(kind.identifier(), 7)`.
    pub fn identifier(self) -> u32 {
        match self {
            Self::Integer => 7,
            Self::Numeric => 102,
        }
    }
    /// Returns the native fixed slot count; e.g. `let slots = kind.slot_count()`.
    pub fn slot_count(self) -> usize {
        match self {
            Self::Integer => 100,
            Self::Numeric => 320,
        }
    }
    fn width(self) -> usize {
        match self {
            Self::Integer => 4,
            Self::Numeric => 8,
        }
    }
}

/// Exact raw slot value, with binary64 bits carried instead of float equality.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFixedSlot {
    /// All integer bits, including negative two's-complement representations.
    Integer(u32),
    /// All binary64 bits, including signed zero and NaN payloads.
    Numeric(u64),
}

impl FlowFixedSlot {
    /// Interprets integer slots only; e.g. `assert_eq!(slot.signed(), Some(-1))`.
    pub fn signed(self) -> Option<i32> {
        match self {
            Self::Integer(bits) => Some(i32::from_le_bytes(bits.to_le_bytes())),
            _ => None,
        }
    }
    /// Interprets numeric slots without normalizing their bits; e.g. `let value = slot.numeric()`.
    pub fn numeric(self) -> Option<f64> {
        match self {
            Self::Numeric(bits) => Some(f64::from_bits(bits)),
            _ => None,
        }
    }
}

/// Immutable selected record and complete source checkpoint.
#[derive(Clone, Debug)]
pub struct FlowFixedSettings {
    source: FlowCarrier,
    record: FlowRecordId,
    kind: FlowFixedSettingsKind,
    payload: Arc<[u8]>,
}

/// Explicit check scope for raw slots, without per-field meaning or normalization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFixedSettingsCheckMode {
    /// Complete exact fixed extent and little-endian slot partition.
    ExactRawFixedSlots,
}

/// Completed exact slot checks; no sampled or native application acceptance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowFixedSettingsCoverage {
    /// Scope of the completed validation.
    pub mode: FlowFixedSettingsCheckMode,
    /// All slots in the selected array.
    pub checked_slots: usize,
    /// Complete selected payload bytes.
    pub checked_bytes: usize,
    /// Exact byte disagreement, zero for completed views.
    pub achieved_byte_error: usize,
}

/// Exclusions that remain after raw array validation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlowFixedSettingsDiagnostic {
    /// Slot domains and units remain outside the raw array profile.
    SlotSemanticsNotChecked,
    /// Constructor and post-load adjustments have not been applied.
    RuntimeNormalizationNotApplied,
    /// No native loader or application was executed.
    NativeApplicationNotChecked,
}

impl FlowCarrier {
    /// Reads the last ordinary occurrence; e.g. `let settings = carrier.fixed_settings(kind)?`.
    pub fn fixed_settings(&self, kind: FlowFixedSettingsKind) -> Result<Option<FlowFixedSettings>> {
        let Some(record) = self.latest(kind.identifier())? else {
            return Ok(None);
        };
        let expected = kind.slot_count() * kind.width();
        if record.payload.len() != expected {
            return Err(fixed_failure(format!(
                "record {} has {} bytes; expected exactly {expected} raw bytes",
                kind.identifier(),
                record.payload.len()
            )));
        }
        Ok(Some(FlowFixedSettings {
            source: self.clone(),
            record: record.id,
            kind,
            payload: Arc::from(record.payload),
        }))
    }
}

impl FlowFixedSettings {
    /// Borrows the complete source checkpoint; e.g. `let original = settings.source()`.
    pub fn source(&self) -> &FlowCarrier {
        &self.source
    }
    /// Borrows the selected occurrence identity; e.g. `let id = settings.record()`.
    pub fn record(&self) -> &FlowRecordId {
        &self.record
    }
    /// Returns the parsed profile; e.g. `let kind = settings.kind()`.
    pub fn kind(&self) -> FlowFixedSettingsKind {
        self.kind
    }
    /// Borrows every raw payload byte; e.g. `let bytes = settings.as_bytes()`.
    pub fn as_bytes(&self) -> &[u8] {
        &self.payload
    }
    /// Returns a bit-exact slot, or absence outside the array; e.g. `let first = settings.slot(0)`.
    pub fn slot(&self, index: usize) -> Option<FlowFixedSlot> {
        if index >= self.kind.slot_count() {
            return None;
        }
        let start = index * self.kind.width();
        let bytes = &self.payload[start..start + self.kind.width()];
        Some(match self.kind {
            FlowFixedSettingsKind::Integer => {
                FlowFixedSlot::Integer(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
            }
            FlowFixedSettingsKind::Numeric => FlowFixedSlot::Numeric(u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ])),
        })
    }
    /// Reports complete raw checks; e.g. `assert_eq!(settings.coverage().achieved_byte_error, 0)`.
    pub fn coverage(&self) -> FlowFixedSettingsCoverage {
        FlowFixedSettingsCoverage {
            mode: FlowFixedSettingsCheckMode::ExactRawFixedSlots,
            checked_slots: self.kind.slot_count(),
            checked_bytes: self.payload.len(),
            achieved_byte_error: 0,
        }
    }
    /// Borrows semantic exclusions; e.g. `let exclusions = settings.diagnostics()`.
    pub fn diagnostics(&self) -> &[FlowFixedSettingsDiagnostic] {
        &[
            FlowFixedSettingsDiagnostic::SlotSemanticsNotChecked,
            FlowFixedSettingsDiagnostic::RuntimeNormalizationNotApplied,
            FlowFixedSettingsDiagnostic::NativeApplicationNotChecked,
        ]
    }
    /// Replaces one same-profile slot on a copy; e.g. `let edit = settings.replace_slot(0, value)?`.
    pub fn replace_slot(&self, index: usize, value: FlowFixedSlot) -> Result<FlowCarrierEdit> {
        let bytes = replacement_bytes(self.kind, value)?;
        if index >= self.kind.slot_count() {
            return Err(fixed_failure(format!(
                "slot {index}; expected index below {}",
                self.kind.slot_count()
            )));
        }
        let start = index * self.kind.width();
        let mut payload = self.payload.to_vec();
        payload[start..start + bytes.len()].copy_from_slice(&bytes);
        let edit = self.source.replace_payload(&self.record, &payload)?;
        verify_replacement(&edit, self.kind, index, value, &payload)?;
        Ok(edit)
    }
}

fn replacement_bytes(kind: FlowFixedSettingsKind, value: FlowFixedSlot) -> Result<Vec<u8>> {
    match (kind, value) {
        (FlowFixedSettingsKind::Integer, FlowFixedSlot::Integer(bits)) => {
            Ok(bits.to_le_bytes().to_vec())
        }
        (FlowFixedSettingsKind::Numeric, FlowFixedSlot::Numeric(bits)) => {
            Ok(bits.to_le_bytes().to_vec())
        }
        _ => Err(fixed_failure(format!(
            "slot value {value:?}; expected profile {kind:?}"
        ))),
    }
}

fn verify_replacement(
    edit: &FlowCarrierEdit,
    kind: FlowFixedSettingsKind,
    index: usize,
    value: FlowFixedSlot,
    payload: &[u8],
) -> Result<()> {
    let selected = edit.result.fixed_settings(kind)?;
    if selected
        .as_ref()
        .is_some_and(|view| view.slot(index) == Some(value) && view.as_bytes() == payload)
    {
        return Ok(());
    }
    Err(fixed_failure(format!(
        "reopened slot {index} disagrees; expected {value:?} and complete patched payload"
    )))
}

fn fixed_failure(cause: String) -> BessemerError {
    BessemerError::new(
        "Flow fixed settings",
        "raw array",
        FailureClass::Invalid,
        cause,
    )
}
