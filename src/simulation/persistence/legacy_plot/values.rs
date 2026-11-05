use super::super::thermal::{SimulationThermalField, SimulationThermalValue};
use super::{SimulationLegacyPlotLimits, SimulationLegacyPlotText};
use crate::{Result, simulation::invalid};
use std::sync::Arc;

/// Complete historical stored value without physical or Unicode interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationLegacyPlotValue {
    /// Exact four-byte pattern, including unknown enums and float-looking values.
    RawWordBits(u32),
    /// Exact eight-byte pattern; no floating-point meaning is assigned.
    RawQwordBits(u64),
    /// One entire historical bulk block or fixed buffer, including bytes after NUL.
    RawBytes(Arc<[u8]>),
    /// Atomic signed character count and its exact raw code units.
    Text(SimulationLegacyPlotText),
}

/// One immutable physical field in the explicit version-selected grammar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationLegacyPlotField {
    /// Parser-derived structural path, distinct from native member byte offsets.
    pub path: Arc<str>,
    /// Complete owned stored value.
    pub value: SimulationLegacyPlotValue,
}

pub(super) fn wire_value(value: &SimulationLegacyPlotValue) -> SimulationThermalValue {
    match value {
        SimulationLegacyPlotValue::RawWordBits(bits) => SimulationThermalValue::Unsigned(*bits),
        SimulationLegacyPlotValue::RawQwordBits(bits) => SimulationThermalValue::DoubleBits(*bits),
        SimulationLegacyPlotValue::RawBytes(bytes) => SimulationThermalValue::Bytes(bytes.clone()),
        SimulationLegacyPlotValue::Text(text) => {
            SimulationThermalValue::Bytes(text.to_bytes().into())
        }
    }
}

pub(super) fn wire_fields(fields: &[SimulationLegacyPlotField]) -> Vec<SimulationThermalField> {
    fields
        .iter()
        .map(|field| SimulationThermalField {
            path: field.path.clone(),
            value: wire_value(&field.value),
        })
        .collect()
}

pub(super) fn typed_fields(
    fields: &[SimulationThermalField],
    limits: SimulationLegacyPlotLimits,
) -> Result<Vec<SimulationLegacyPlotField>> {
    fields
        .iter()
        .map(|field| {
            Ok(SimulationLegacyPlotField {
                path: field.path.clone(),
                value: typed_value(field, limits.maximum_text_bytes)?,
            })
        })
        .collect()
}

fn typed_value(
    field: &SimulationThermalField,
    maximum_text_bytes: usize,
) -> Result<SimulationLegacyPlotValue> {
    match &field.value {
        SimulationThermalValue::Unsigned(bits) => Ok(SimulationLegacyPlotValue::RawWordBits(*bits)),
        SimulationThermalValue::DoubleBits(bits) => {
            Ok(SimulationLegacyPlotValue::RawQwordBits(*bits))
        }
        SimulationThermalValue::Bytes(bytes) if field.path.starts_with("legacy_plot.text_") => {
            Ok(SimulationLegacyPlotValue::Text(
                SimulationLegacyPlotText::from_frame(bytes, maximum_text_bytes)?,
            ))
        }
        SimulationThermalValue::Bytes(bytes) => {
            Ok(SimulationLegacyPlotValue::RawBytes(bytes.clone()))
        }
        other => Err(invalid(
            &field.path,
            format!("{other:?}; expected historical raw32/raw64/buffer or character text"),
        )),
    }
}

pub(super) fn require_kind(
    original: &SimulationLegacyPlotValue,
    replacement: &SimulationLegacyPlotValue,
    path: &str,
) -> Result<()> {
    if std::mem::discriminant(original) == std::mem::discriminant(replacement) {
        return Ok(());
    }
    Err(invalid(
        path,
        format!("replacement {replacement:?}; expected the stored kind of {original:?}"),
    ))
}
