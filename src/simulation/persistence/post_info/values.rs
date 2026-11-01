use super::super::thermal::{SimulationThermalField, SimulationThermalValue};
use super::{SimulationPostInfoLimits, SimulationPostInfoText};
use crate::{Result, simulation::invalid};
use std::sync::Arc;

/// Complete stored PostInfo value; float payloads remain exact bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationPostInfoValue {
    /// Signed count, enum or contour word without normalization.
    Signed(i32),
    /// Four-byte word with an unresolved physical purpose.
    RawWordBits(u32),
    /// Stored float32 pattern, including negative zero and nonfinite values.
    Float32Bits(u32),
    /// Atomic version700002 character count and raw UTF-16 code units.
    Text(SimulationPostInfoText),
}

/// Immutable complete PostInfo field in physical order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostInfoField {
    /// Parser-derived structural path.
    pub path: Arc<str>,
    /// Complete typed stored value.
    pub value: SimulationPostInfoValue,
}

pub(in crate::simulation::persistence) fn wire_value(
    value: &SimulationPostInfoValue,
) -> SimulationThermalValue {
    match value {
        SimulationPostInfoValue::Signed(word) => SimulationThermalValue::Signed(*word),
        SimulationPostInfoValue::RawWordBits(bits) | SimulationPostInfoValue::Float32Bits(bits) => {
            SimulationThermalValue::Unsigned(*bits)
        }
        SimulationPostInfoValue::Text(text) => {
            SimulationThermalValue::Bytes(text.to_bytes().into())
        }
    }
}

pub(in crate::simulation::persistence) fn to_wire_fields(
    fields: &[SimulationPostInfoField],
) -> Vec<SimulationThermalField> {
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
    limits: SimulationPostInfoLimits,
) -> Result<Vec<SimulationPostInfoField>> {
    fields
        .iter()
        .map(|field| {
            Ok(SimulationPostInfoField {
                path: field.path.clone(),
                value: typed_value(field, limits.maximum_text_bytes)?,
            })
        })
        .collect()
}

fn typed_value(
    field: &SimulationThermalField,
    maximum_bytes: usize,
) -> Result<SimulationPostInfoValue> {
    match &field.value {
        SimulationThermalValue::Signed(word) => Ok(SimulationPostInfoValue::Signed(*word)),
        SimulationThermalValue::Unsigned(bits) if is_float_path(&field.path) => {
            Ok(SimulationPostInfoValue::Float32Bits(*bits))
        }
        SimulationThermalValue::Unsigned(bits) => Ok(SimulationPostInfoValue::RawWordBits(*bits)),
        SimulationThermalValue::Bytes(raw) if field.path.as_ref() == "legacy.text" => Ok(
            SimulationPostInfoValue::Text(SimulationPostInfoText::from_frame(raw, maximum_bytes)?),
        ),
        value => Err(invalid(
            &field.path,
            format!(
                "{:?}; expected PostInfo signed/raw32/float32 or legacy text",
                std::mem::discriminant(value)
            ),
        )),
    }
}

fn is_float_path(path: &str) -> bool {
    path.starts_with("post_info.")
        && (path.contains(".normal[")
            || path.contains(".center[")
            || path.ends_with(".shared_scalar"))
}

pub(in crate::simulation::persistence) fn require_kind(
    original: &SimulationPostInfoValue,
    replacement: &SimulationPostInfoValue,
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

fn kind_name(value: &SimulationPostInfoValue) -> &'static str {
    match value {
        SimulationPostInfoValue::Signed(_) => "Signed",
        SimulationPostInfoValue::RawWordBits(_) => "RawWordBits",
        SimulationPostInfoValue::Float32Bits(_) => "Float32Bits",
        SimulationPostInfoValue::Text(_) => "Text",
    }
}
