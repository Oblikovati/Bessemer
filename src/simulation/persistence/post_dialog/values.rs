use super::super::thermal::{SimulationThermalField, SimulationThermalValue};
use super::{SimulationPostDialogBytes, SimulationPostDialogLimits, SimulationPostDialogText};
use crate::{
    Result,
    model_archives::SimulationEmbeddedArchive,
    simulation::{SimulationPersistText, SimulationPersistVersion, invalid},
};
use std::sync::Arc;

/// Complete physical dialog value with distinct text, byte-frame and embedded archive boundaries.
#[derive(Debug, Clone)]
pub enum SimulationPostDialogValue {
    /// Signed stored word or declared collection count.
    Signed(i32),
    /// Unresolved four-byte pattern.
    RawWordBits(u32),
    /// Exact float32 component or scalar bits.
    Float32Bits(u32),
    /// Opaque eight-byte pattern without inferred numeric meaning.
    Raw64([u8; 8]),
    /// Persisted byte-count text retaining null and allocated-empty distinctions.
    PersistText(SimulationPersistText),
    /// Atomic character-count frame retaining raw UTF-16 code units.
    CharacterText(SimulationPostDialogText),
    /// Atomic opaque byte count and payload.
    CountedBytes(SimulationPostDialogBytes),
    /// Complete independently versioned scoped Simulation archive.
    Embedded(Box<SimulationEmbeddedArchive>),
}

impl PartialEq for SimulationPostDialogValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Signed(a), Self::Signed(b)) => a == b,
            (Self::RawWordBits(a), Self::RawWordBits(b)) => a == b,
            (Self::Float32Bits(a), Self::Float32Bits(b)) => a == b,
            (Self::Raw64(a), Self::Raw64(b)) => a == b,
            (Self::PersistText(a), Self::PersistText(b)) => {
                a.header() == b.header() && a.payload() == b.payload()
            }
            (Self::CharacterText(a), Self::CharacterText(b)) => a == b,
            (Self::CountedBytes(a), Self::CountedBytes(b)) => a == b,
            (Self::Embedded(a), Self::Embedded(b)) => a.fields() == b.fields(),
            _ => false,
        }
    }
}
impl Eq for SimulationPostDialogValue {}

/// Immutable dialog field parts in complete physical order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostDialogField {
    /// Canonical parser-derived structural path.
    pub path: Arc<str>,
    /// Complete typed physical value.
    pub value: SimulationPostDialogValue,
}

pub(super) fn wire_value(value: &SimulationPostDialogValue) -> SimulationThermalValue {
    match value {
        SimulationPostDialogValue::Signed(word) => SimulationThermalValue::Signed(*word),
        SimulationPostDialogValue::RawWordBits(bits)
        | SimulationPostDialogValue::Float32Bits(bits) => SimulationThermalValue::Unsigned(*bits),
        SimulationPostDialogValue::Raw64(raw) => {
            SimulationThermalValue::Bytes(Arc::from(raw.as_slice()))
        }
        SimulationPostDialogValue::PersistText(text) => SimulationThermalValue::Text(text.clone()),
        SimulationPostDialogValue::CharacterText(text) => {
            SimulationThermalValue::Bytes(text.to_bytes().into())
        }
        SimulationPostDialogValue::CountedBytes(frame) => {
            SimulationThermalValue::Bytes(frame.to_bytes().into())
        }
        SimulationPostDialogValue::Embedded(archive) => {
            SimulationThermalValue::Embedded(archive.clone())
        }
    }
}

pub(super) fn to_wire_fields(fields: &[SimulationPostDialogField]) -> Vec<SimulationThermalField> {
    fields
        .iter()
        .map(|field| SimulationThermalField {
            path: field.path.clone(),
            value: wire_value(&field.value),
        })
        .collect()
}

pub(super) fn from_wire_fields(
    fields: &[SimulationThermalField],
    version: SimulationPersistVersion,
    limits: SimulationPostDialogLimits,
) -> Result<Vec<SimulationPostDialogField>> {
    fields
        .iter()
        .map(|field| {
            Ok(SimulationPostDialogField {
                path: field.path.clone(),
                value: typed_value(field, version, limits)?,
            })
        })
        .collect()
}

fn typed_value(
    field: &SimulationThermalField,
    version: SimulationPersistVersion,
    limits: SimulationPostDialogLimits,
) -> Result<SimulationPostDialogValue> {
    match &field.value {
        SimulationThermalValue::Signed(word) => Ok(SimulationPostDialogValue::Signed(*word)),
        SimulationThermalValue::Unsigned(bits) if float_path(&field.path) => {
            Ok(SimulationPostDialogValue::Float32Bits(*bits))
        }
        SimulationThermalValue::Unsigned(bits) => Ok(SimulationPostDialogValue::RawWordBits(*bits)),
        SimulationThermalValue::Text(text) => {
            Ok(SimulationPostDialogValue::PersistText(text.clone()))
        }
        SimulationThermalValue::Embedded(archive) => {
            Ok(SimulationPostDialogValue::Embedded(archive.clone()))
        }
        SimulationThermalValue::Bytes(raw) => typed_bytes(&field.path, raw, version, limits),
        value => Err(invalid(
            &field.path,
            format!("{value:?}; expected a complete declared dialog value kind"),
        )),
    }
}

fn typed_bytes(
    path: &str,
    raw: &[u8],
    version: SimulationPersistVersion,
    limits: SimulationPostDialogLimits,
) -> Result<SimulationPostDialogValue> {
    if path.ends_with(".blob")
        || (path.starts_with("dialog.string_list[") && version.value() < 890611)
    {
        let frame = SimulationPostDialogBytes::from_frame(raw, limits.maximum_bytes)?;
        return Ok(SimulationPostDialogValue::CountedBytes(frame));
    }
    if character_path(path) {
        let text = SimulationPostDialogText::from_frame(raw, limits.maximum_text_bytes)?;
        return Ok(SimulationPostDialogValue::CharacterText(text));
    }
    let width = raw.len();
    raw.try_into()
        .map(SimulationPostDialogValue::Raw64)
        .map_err(|_| {
            invalid(
                path,
                format!("{width} bytes; expected exactly eight opaque bytes"),
            )
        })
}

fn character_path(path: &str) -> bool {
    path.starts_with("text")
        || path.contains(".text")
        || path.ends_with(".name")
        || path.starts_with("dialog.string_list[")
}

fn float_path(path: &str) -> bool {
    if path.contains(".color_") || matches!(path, "legend.word_ac" | "legend.word_b0") {
        return true;
    }
    if path.starts_with("post_info.")
        && (path.contains(".normal[")
            || path.contains(".center[")
            || path.ends_with(".shared_scalar"))
    {
        return true;
    }
    matches!(
        path,
        "eight_words528_544[0]"
            | "eight_words528_544[1]"
            | "eight_words528_544[2]"
            | "eight_words528_544[3]"
            | "eight_words528_544[4]"
            | "eight_words528_544[5]"
    )
}

pub(super) fn require_kind(
    original: &SimulationPostDialogValue,
    replacement: &SimulationPostDialogValue,
    path: &str,
) -> Result<()> {
    if std::mem::discriminant(original) == std::mem::discriminant(replacement) {
        return Ok(());
    }
    Err(invalid(
        path,
        format!(
            "replacement {}; expected {}",
            kind_name(replacement),
            kind_name(original)
        ),
    ))
}

fn kind_name(value: &SimulationPostDialogValue) -> &'static str {
    match value {
        SimulationPostDialogValue::Signed(_) => "Signed",
        SimulationPostDialogValue::RawWordBits(_) => "RawWordBits",
        SimulationPostDialogValue::Float32Bits(_) => "Float32Bits",
        SimulationPostDialogValue::Raw64(_) => "Raw64",
        SimulationPostDialogValue::PersistText(_) => "PersistText",
        SimulationPostDialogValue::CharacterText(_) => "CharacterText",
        SimulationPostDialogValue::CountedBytes(_) => "CountedBytes",
        SimulationPostDialogValue::Embedded(_) => "Embedded",
    }
}
