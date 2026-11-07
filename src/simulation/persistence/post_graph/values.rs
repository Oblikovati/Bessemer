use super::super::thermal::{SimulationThermalField, SimulationThermalValue};
use super::{SimulationPostGraphLimits, SimulationPostGraphText};
use crate::{Result, simulation::invalid};
use std::sync::Arc;

/// Complete documented graph value, preserving unassigned eight-byte patterns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimulationPostGraphValue {
    /// Four-byte wire word without runtime enum normalization.
    Signed(i32),
    /// Unassigned eight-byte wire pattern.
    Raw64([u8; 8]),
    /// Atomic character count and raw UTF-16 payload.
    Text(SimulationPostGraphText),
}

/// Immutable complete graph field parts in physical order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimulationPostGraphField {
    /// Parser-derived structural path.
    pub path: Arc<str>,
    /// Complete typed physical value.
    pub value: SimulationPostGraphValue,
}

pub(super) fn wire_value(value: &SimulationPostGraphValue) -> SimulationThermalValue {
    match value {
        SimulationPostGraphValue::Signed(word) => SimulationThermalValue::Signed(*word),
        SimulationPostGraphValue::Raw64(raw) => {
            SimulationThermalValue::Bytes(Arc::from(raw.as_slice()))
        }
        SimulationPostGraphValue::Text(text) => {
            SimulationThermalValue::Bytes(text.to_bytes().into())
        }
    }
}

pub(super) fn to_wire_fields(fields: &[SimulationPostGraphField]) -> Vec<SimulationThermalField> {
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
    limits: SimulationPostGraphLimits,
) -> Result<Vec<SimulationPostGraphField>> {
    fields
        .iter()
        .map(|field| {
            Ok(SimulationPostGraphField {
                path: field.path.clone(),
                value: typed_value(field, limits.maximum_text_bytes)?,
            })
        })
        .collect()
}

fn typed_value(
    field: &SimulationThermalField,
    maximum_text_bytes: usize,
) -> Result<SimulationPostGraphValue> {
    match &field.value {
        SimulationThermalValue::Signed(word) => Ok(SimulationPostGraphValue::Signed(*word)),
        SimulationThermalValue::Bytes(raw) => typed_bytes(&field.path, raw, maximum_text_bytes),
        value => Err(invalid(
            &field.path,
            format!(
                "{:?}; expected graph signed word or complete byte frame",
                std::mem::discriminant(value)
            ),
        )),
    }
}

fn typed_bytes(
    path: &str,
    raw: &[u8],
    maximum_text_bytes: usize,
) -> Result<SimulationPostGraphValue> {
    if path.starts_with("text") {
        return Ok(SimulationPostGraphValue::Text(
            SimulationPostGraphText::from_frame(raw, maximum_text_bytes)?,
        ));
    }
    raw.try_into()
        .map(SimulationPostGraphValue::Raw64)
        .map_err(|_| {
            invalid(
                path,
                format!("{} raw bytes; expected exactly eight", raw.len()),
            )
        })
}

pub(super) fn require_kind(
    original: &SimulationPostGraphValue,
    replacement: &SimulationPostGraphValue,
    path: &str,
) -> Result<()> {
    if std::mem::discriminant(original) == std::mem::discriminant(replacement) {
        return Ok(());
    }
    Err(invalid(
        path,
        format!(
            "replacement {:?}; expected {:?}",
            graph_kind_name(replacement),
            graph_kind_name(original)
        ),
    ))
}

fn graph_kind_name(value: &SimulationPostGraphValue) -> &'static str {
    match value {
        SimulationPostGraphValue::Signed(_) => "Signed",
        SimulationPostGraphValue::Raw64(_) => "Raw64",
        SimulationPostGraphValue::Text(_) => "Text",
    }
}
